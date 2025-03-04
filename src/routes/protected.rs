use crate::{
    components::Nav,
    routes::Terms,
    surgeon::{Surgeon, get_current_surgeon},
};
use leptos::{
    either::Either,
    prelude::{
        ElementChild, Get, IntoAny, IntoView, Suspend, Suspense, component, provide_context,
        signal, view,
    },
    server::OnceResource,
};
use leptos_router::components::Outlet;

#[component]
pub fn Protected() -> impl IntoView {
    let surgeon_resource = OnceResource::new(get_current_surgeon());
    dbg!(surgeon_resource);
    // I think we need to deserialize this oh shoot it DOES need to be a #[server]
    // function to handle the deserialization automatically, that's the issue-go back to
    // that approach now that you know the contexts are different

    // todo:
    // https://github.com/leptos-rs/leptos/discussions/3390
    // looks like this componenst can consist only of the Outlet as long as the other
    // routes are nested inside it in the Router component definition
    // so basically you've worked it out properly, you just need to sort your eithers
    //
    // hold on:
    // you are still panicking if you manually navigate to this route, because
    // [`surgeon::get_current_surgeon`] uses `expect_context`, which is failing for some
    // reason on a None value
    // (see above for answer)
    view! {
        <Suspense fallback=move || {
            view! { "still loading the current surgeon" }
        }>
            {move || Suspend::new(async move {
                let surgeon = surgeon_resource.await.unwrap_or(None);
                if surgeon.as_ref().is_none() {
                    view! {
                        "are you sure you didn't want to "
                        <a href="/signin">"sign in?"</a>
                    }
                        .into_any()
                } else if surgeon.as_ref().unwrap().terms.is_some() {
                    provide_context(surgeon);
                    view! {
                        <Nav />
                        <Outlet />
                    }
                        .into_any()
                } else {
                    view! { <Terms /> }.into_any()
                }
            })}
        </Suspense>
    }
}
