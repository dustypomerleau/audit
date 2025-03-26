#[cfg(feature = "ssr")] use crate::auth::get_jwt_cookie;
#[cfg(feature = "ssr")] use crate::db::db;
use leptos::{
    prelude::{IntoAny, IntoView, ServerFnError, Suspend, Suspense, component, server, view},
    server::OnceResource,
};
use leptos_router::{components::Outlet, hooks::use_navigate};

#[component]
pub fn New() -> impl IntoView {
    let surgeon_resource = OnceResource::new(is_signed_in());

    view! {
        <Suspense fallback=move || {
            view! { "Checking the signin status..." }
        }>
            // bookmark: todo: this redirect is happening even after a successful
            // signin when the user is new. They should be able to click the new
            // user link and get to this form
            {move || Suspend::new(async move {
                if let Ok(true) = surgeon_resource.await {
                    view! { <Outlet /> }.into_any()
                } else {
                    let navigate = use_navigate();
                    navigate("/", Default::default());
                    ().into_any()
                }
            })}
        </Suspense>
    }
}

#[server]
pub async fn is_signed_in() -> Result<bool, ServerFnError> {
    let auth_token = get_jwt_cookie().await?;

    let query = format!(r#"select "{auth_token}" = (select global ext::auth::client_token);"#);

    let has_auth_token = db()
        .await?
        .query_required_single::<bool, _>(query, &())
        .await
        // unwrap_or_default() works here, but for such an important check, be explicit.
        .unwrap_or(false);

    Ok(has_auth_token)
}
