#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use audit::{
        auth::{handle_pkce_code, handle_sign_in},
        routes::{App, shell},
        state::AppState,
    };
    use axum::{Router, routing::get};
    use dotenvy::dotenv;
    use leptos::{
        logging::log,
        prelude::{get_configuration, provide_context},
    };
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use std::sync::{Arc, RwLock};

    #[cfg(debug_assertions)]
    dotenv().ok();

    // Use default values for the `cargo-leptos` config:
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let db_client = edgedb_tokio::create_client()
        .await
        .expect("expected the DB client to be initialized");

    let db = Arc::new(RwLock::new(db_client));

    let app_state = AppState {
        leptos_options,
        db: Arc::clone(&db),
    };

    let app = Router::new()
        .route("/code", get(handle_pkce_code))
        .route("/signin", get(handle_sign_in))
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            App,
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

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
