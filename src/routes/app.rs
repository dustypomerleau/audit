use crate::routes::{Add, Code, List, Register, Report, SignIn};
#[cfg(feature = "ssr")] use edgedb_tokio::create_client;
use leptos::prelude::{
    component, provide_context, view, AutoReload, ElementChild, GlobalAttributes, HydrationScripts,
    IntoView, LeptosOptions,
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use reactive_stores::Store;

// #[derive(Clone, Debug, Default, Store)]
// pub struct GlobalState {
//     db_client: Option<edgedb_tokio::Client>,
// }

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
    // Start by providing global state with a `None` DB client, and then create the client with
    // correct globals after completing the auth flow.
    // provide_context(Store::new(GlobalState::default()));

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/audit.css" />
        <Title text="Cataract audit" />

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=SignIn />
                    <Route path=StaticSegment("add") view=Add />
                    // <Route path=StaticSegment("code") view=Code />
                    <Route path=StaticSegment("list") view=List />
                    <Route path=StaticSegment("register") view=Register />
                    <Route path=StaticSegment("report") view=Report />
                </Routes>
            </main>
        </Router>
    }
}
