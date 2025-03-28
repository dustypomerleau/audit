use crate::routes::{
    Add, Gateway, Instructions, Landing, List, New, Protected, Report, SignUp, Terms,
};
use leptos::prelude::{
    AutoReload, ElementChild, GlobalAttributes, HydrationScripts, IntoView, LeptosOptions,
    component, view,
};
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    SsrMode, StaticSegment,
    components::{ParentRoute, Route, Router, Routes},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, shrink-to-fit=no"
                />
                <meta name="description" content="Vic Eye cataract audit" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>

            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/audit.css" />
        <Title text="Cataract audit" />
        <Router>
            <main>
                // note: plain Axum server routes are not represented here,
                // as they are added directly to the router in `src/main.rs`.
                //
                // todo: you need 2 types of protected routes:
                //
                // 1. protected route that simply checks for a global
                // ext::auth::ClientTokenIdentity. Failure redirects to "/signin".
                //
                // 2. protected route that checks for a Surgeon with .identity =
                // ext::auth::ClientTokenIdentity (protects all the logged-in content).
                // Failure redirects to "/signin".
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=Landing />
                    <Route path=StaticSegment("gateway") view=Gateway />
                    <ParentRoute path=StaticSegment("new") view=New>
                        <Route path=StaticSegment("signup") view=SignUp />
                        <Route path=StaticSegment("terms") view=Terms />
                    </ParentRoute>
                    <ParentRoute path=StaticSegment("protected") view=Protected>
                        // <Route path=StaticSegment("") view=Fallback />
                        <Route path=StaticSegment("add") view=Add />
                        // todo: consider making instructions a sidebar inside Add
                        <Route path=StaticSegment("instructions") view=Instructions />
                        <Route path=StaticSegment("list") view=List />
                        <Route path=StaticSegment("report") view=Report />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
