#[cfg(feature = "ssr")] use audit::error::AppError;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), AppError> {
    use std::env;
    use std::sync::Arc;
    use std::sync::RwLock;

    // use audit::auth::handle_kill_session;
    // use audit::auth::handle_pkce_code;
    // use audit::auth::handle_sign_in;
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
    use oauth2::AuthUrl;
    use oauth2::ClientId;
    use oauth2::ClientSecret;
    use oauth2::CsrfToken;
    use oauth2::RedirectUrl;
    use oauth2::RevocationUrl;
    use oauth2::TokenUrl;
    use oauth2::basic::BasicClient;
    use sqlx::Pool;
    use sqlx::Postgres;
    use sqlx::migrate;

    #[cfg(debug_assertions)]
    dotenv().ok();

    let client_id = ClientId::new(
        env::var("OAUTH_CLIENT_ID")
            .expect("expected OAUTH_CLIENT_ID environment variable to be present"),
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;

    let redirect_url = RedirectUrl::new(
        env::var("OAUTH_REDIRECT_URI")
            .expect("expected OAUTH_REDIRECT_URI environment variable to be present"),
    )?;

    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;
    let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())?;

    let oauth_client = BasicClient::new(client_id)
        .set_auth_uri(auth_url)
        .set_redirect_uri(redirect_url)
        .set_token_uri(token_url)
        .set_revocation_url(revocation_url);

    // TODO: customise the connection to like max 90 and handle env var errors.
    // see sqlx PgConnectOptions for options
    //
    // let options = PgPoolOptions::new().max_connections(90);
    // let options = PgConnectOptions::new();
    let pool = Pool::<Postgres>::connect(env::var("DATABASE_URL").unwrap().as_str()).await?;

    // ./migrations is the default, but we explicitly set it here. `.` is the same directory as the
    // Cargo.toml for `audit` in dev, and the same directory as the audit binary in prod.
    migrate!("./migrations").run(&pool).await?;

    let app_state = AppState::builder()
        .oauth_client(oauth_client)
        .http_client(http_client)
        .leptos_options(leptos_options)
        .db(pool)
        .mailer(MAILER.clone())
        .build()?;

    // Use default values for the `cargo-leptos` config:
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // BOOKMARK: TODO: add:
    // session store using postgres adapter
    // SessionManagerLayer and
    // AuthManagerLayer
    // (see oauth example in axum_login in app.rs)
    let app = Router::new()
        // .route("/code", get(handle_pkce_code))
        // .route("/killsession", get(handle_kill_session))
        // .route("/signin", get(handle_sign_in))
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
