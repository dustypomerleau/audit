#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use audit::routes::{shell, App};
    use axum::Router;
    use dotenvy::dotenv;
    use edgedb_tokio::create_client;
    use leptos::{
        logging::log,
        prelude::{get_configuration, provide_context},
    };
    use leptos_axum::{generate_route_list, LeptosRoutes};

    #[cfg(debug_assertions)]
    dotenv().ok();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    //
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    //
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    // getting the auth token requires something like:
    // https://docs.edgedb.com/guides/auth/email_password#retrieve-auth-token
    // Which is still a todo
    // todo: remove this - temporarily mock the auth token:
    let auth_token = String::from("fake_token");

    let client = create_client()
        .await
        .expect("DB client to be initialized")
        .with_globals_fn(|c| c.set("ext::auth::client_token", auth_token));

    // todo: auth + protected routes:
    // https://docs.rs/leptos_router/latest/leptos_router/fn.ProtectedRoute.html
    // https://docs.rs/oauth2/latest/oauth2/
    // https://docs.edgedb.com/guides/auth
    // https://www.shuttle.rs/blog/2024/02/13/clerk-in-rust
    //
    // plan: sign-in page is the landing page
    // sign-in has a link to register
    // register has a form, where you enter your details first, and then select your oauth provider

    // example of using context to pass the DB client:
    // https://book.leptos.dev/server/26_extractors.html#axum-state
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(client.clone()),
            App,
        )
        // .fallback(file_and_error_handler)
        // If you need to expand on the basic functionality in the provided file and
        // error handler, uncomment the above fallback and update your custom code using
        // the provided file_and_error_handler as a starting point.
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log!("listening on http://{}", &addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
