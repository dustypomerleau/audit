use crate::{components::Nav, db::get_authorized_surgeon};
use leptos::prelude::{
    ElementChild, IntoAny, IntoView, OnceResource, Suspend, Suspense, component, provide_context,
    view,
};
use leptos_router::{components::Outlet, hooks::use_navigate};

#[component]
pub fn Protected() -> impl IntoView {
    // resource call blows up with:
    //
    // thread 'tokio-runtime-worker' panicked at
    // /Users/dn/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/js-sys-0.3.77/src/lib.rs:6063:
    // 9: cannot access imported statics on non-wasm targets
    let surgeon_resource = OnceResource::new(get_authorized_surgeon());

    view! {
        <Suspense fallback=move || {
            view! { "Checking authorization for the current surgeon..." }
        }>
            {move || Suspend::new(async move {
                if let Ok(Some(surgeon)) = surgeon_resource.await {
                    if surgeon.terms.is_some() {
                        dbg!(&surgeon);
                        provide_context(surgeon);

                        view! {
                            <Nav />
                            <Outlet />
                        }
                            .into_any()
                    } else {
                        let navigate = use_navigate();
                        navigate("/new/terms", Default::default());
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
