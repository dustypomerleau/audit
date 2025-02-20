use crate::{
    routes::{Add, List, Register, Report, SignIn},
    surgeon::Surgeon,
};
use leptos::prelude::{
    AutoReload, ElementChild, GlobalAttributes, HydrationScripts, IntoView, LeptosOptions,
    component, expect_context, provide_context, signal, view,
};
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use reactive_stores::Store;
use serde_json::from_str;

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
                // as they are added directly to the router in `src/main.rs`
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=SignIn />
                    <Route path=StaticSegment("add") view=Add />
                    <Route path=StaticSegment("list") view=List />
                    <Route path=StaticSegment("register") view=Register />
                    <Route path=StaticSegment("report") view=Report />
                </Routes>
            </main>
        </Router>
    }
}
