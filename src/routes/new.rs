use crate::db::is_signed_in;
use leptos::{
    prelude::{IntoAny, IntoView, Suspend, Suspense, component, view},
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
            {move || Suspend::new(async move {
                if let Ok(true) = surgeon_resource.await {
                    view! { <Outlet /> }.into_any()
                } else {
                    let navigate = use_navigate();
                    navigate("/signin", Default::default());
                    ().into_any()
                }
            })}
        </Suspense>
    }
}
