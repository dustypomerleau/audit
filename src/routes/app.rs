use crate::{
    components::SignedOut,
    plots::DeltaCyl,
    routes::{Add, Gateway, Instructions, Landing, List, Protected, Report, SignUp, Terms},
};
use leptos::prelude::{
    AutoReload, ElementChild, GlobalAttributes, HydrationScripts, IntoMaybeErased, IntoView,
    LeptosOptions, component, view,
};
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
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
                <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>
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
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=Landing />
                    <Route path=StaticSegment("gateway") view=Gateway />
                    <Route path=StaticSegment("signedout") view=SignedOut />
                    <Route path=StaticSegment("signup") view=SignUp />
                    <Route path=StaticSegment("terms") view=Terms />
                    <ParentRoute path=StaticSegment("protected") view=Protected>
                        // todo: consider making instructions a sidebar inside Add
                        <Route path=StaticSegment("add") view=Add />
                        <Route path=StaticSegment("instructions") view=Instructions />
                        <Route path=StaticSegment("list") view=List />
                        <Route path=StaticSegment("report") view=Report />
                        // just for testing purposes
                        <Route path=StaticSegment("test") view=DeltaCyl />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
