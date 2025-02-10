#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use audit::{
        auth::{handle_pkce_code, handle_sign_in},
        routes::{App, shell},
    };
    use axum::{Router, routing::get};
    use dotenvy::dotenv;
    use leptos::{
        logging::log,
        prelude::{get_configuration, provide_context},
    };
    use leptos_axum::{LeptosRoutes, generate_route_list};

    #[cfg(debug_assertions)]
    dotenv().ok();

    //     let client = create_client()
    //         .await
    //         .expect("DB client to be initialized")
    //         .with_globals_fn(|c| c.set("ext::auth::client_token", auth_token));
    //
    //     // todo: auth + protected routes:
    //     // https://docs.rs/leptos_router/latest/leptos_router/fn.ProtectedRoute.html
    //     // https://docs.rs/oauth2/latest/oauth2/
    //     // https://docs.edgedb.com/guides/auth

    // Use default values for the `cargo-leptos` config:
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let db_client = edgedb_tokio::create_client()
        .await
        .expect("expected the DB client to be initialized");

    // We can't provide our DB client here via context, because we need to set auth globals when we
    // create the client, and that can only be done after auth is complete. So we will use a
    // reactive store instead.
    let app = Router::new()
        .route("/code", get(handle_pkce_code))
        .route("/signin", get(handle_sign_in))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
