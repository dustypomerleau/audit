use crate::routes::{Add, Code, List, Register, Report, SignIn};
use leptos::prelude::{
    component, view, AutoReload, ElementChild, GlobalAttributes, HydrationScripts, IntoView,
    LeptosOptions,
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
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
    // todo: We need to populate a surgeon struct at login and put it in a signal.
    // We can then directly access that signal to get things like their default SIA values.
    // Alternatively, we could put the user in the URL
    // https://book.leptos.dev/15_global_state.html#global-state-management
    //
    // Keep in mind that we also have a DB global `cur_surgeon`.
    // It may be possible to simply use that to populate a struct.

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
                    <Route path=StaticSegment("code") view=Code />
                    <Route path=StaticSegment("list") view=List />
                    <Route path=StaticSegment("register") view=Register />
                    <Route path=StaticSegment("report") view=Report />
                </Routes>
            </main>
        </Router>
    }
}
