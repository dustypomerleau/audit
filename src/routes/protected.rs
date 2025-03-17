use crate::{components::Nav, db::get_authorized_surgeon};
use leptos::prelude::{
    ElementChild, IntoAny, IntoView, OnceResource, Suspend, Suspense, component, provide_context,
    view,
};
use leptos_router::{components::Outlet, hooks::use_navigate};

#[component]
pub fn Protected() -> impl IntoView {
    let surgeon_resource = OnceResource::new(get_authorized_surgeon());

    view! {
        <Suspense fallback=move || {
            view! { "Loading the current surgeon..." }
        }>
            {move || Suspend::new(async move {
                if let Ok(Some(surgeon)) = surgeon_resource.await {
                    if surgeon.terms.is_some() {
                        provide_context(surgeon);

                        view! {
                            <Nav />
                            <Outlet />
                        }
                            .into_any()
                    } else {
                        let navigate = use_navigate();
                        navigate("/terms", Default::default());
                        ().into_any()
                    }
                } else {
                    let navigate = use_navigate();
                    navigate("/signin", Default::default());
                    ().into_any()
                }
            })}
        </Suspense>
    }
}
