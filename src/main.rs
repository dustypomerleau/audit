#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use audit::{app::App, fileserv::file_and_error_handler};
    use axum::{routing::post, Router};
    use leptos::get_configuration;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower::ServiceBuilder;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // probably expect or throw a specific error here
    let client = edgedb_tokio::create_client().await?;
    // Client wraps a Pool and a Config. The Pool wraps an Arc<PoolInner>, so it is thread
    // safe
    // PoolInner wraps a BlockingMutex<VecDeque>, which is I assume the actual job queue
    // that the database operates on

    // build our application with a route
    // todo: this is where you need to add your DB connection, see:
    // https://docs.rs/axum/latest/axum/middleware/index.html#sharing-state-with-handlers
    // and
    // https://docs.rs/axum/latest/axum/struct.Extension.html
    // you need your handler to take client: Extension<Client>
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .layer(ServiceBuilder::new().layer(client))
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
