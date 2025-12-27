#[cfg(feature = "ssr")] use audit::error::AppError;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), AppError> {
    use std::env;
    use std::sync::Arc;
    use std::sync::RwLock;

    use audit::auth::handle_kill_session;
    use audit::auth::handle_pkce_code;
    use audit::auth::handle_sign_in;
    use audit::mail::MAILER;
    use audit::routes::App;
    use audit::routes::shell;
    use audit::state::AppState;
    use axum::Router;
    use axum::routing::get;
    #[cfg(debug_assertions)] use dotenvy::dotenv;
    use leptos::logging::log;
    use leptos::prelude::get_configuration;
    use leptos_axum::LeptosRoutes;
    use leptos_axum::generate_route_list;
    use sqlx::PgPool;
    use sqlx::migrate;

    #[cfg(debug_assertions)]
    dotenv().ok();

    // Use default values for the `cargo-leptos` config:
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // TODO: customise the connection to like max 90 and handle env var errors.
    // see sqlx PgConnectOptions for options
    //
    // let options = PgPoolOptions::new().max_connections(90);
    // let options = PgConnectOptions::new();
    let pool = PgPool::connect(env::var("DATABASE_URL").unwrap().as_str()).await?;

    // ./migrations is the default, but we explicitly set it here. `.` is the same directory as the
    // Cargo.toml for `audit` in dev, and the same directory as the audit binary in prod.
    migrate!("./migrations").run(&pool).await?;

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        db: pool,
        mailer: Arc::new(MAILER.clone()),
        surgeon: Arc::new(RwLock::new(None)),
    };

    let app = Router::new()
        .route("/code", get(handle_pkce_code))
        .route("/killsession", get(handle_kill_session))
        .route("/signin", get(handle_sign_in))
        .leptos_routes(&app_state, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
