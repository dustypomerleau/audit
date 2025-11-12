use leptos::prelude::AutoReload;
use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::HydrationScripts;
use leptos::prelude::IntoView;
use leptos::prelude::LeptosOptions;
use leptos::prelude::component;
use leptos::prelude::view;
use leptos_meta::MetaTags;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_meta::provide_meta_context;
use leptos_router::StaticSegment;
use leptos_router::components::ParentRoute;
use leptos_router::components::Route;
use leptos_router::components::Router;
use leptos_router::components::Routes;

use crate::components::Nav;
use crate::components::SignedOut;
use crate::routes::Add;
use crate::routes::Gateway;
use crate::routes::Instructions;
use crate::routes::Landing;
use crate::routes::List;
use crate::routes::Protected;
use crate::routes::Report;
use crate::routes::SignUp;
use crate::routes::Terms;

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
                // Plotly releases: https://github.com/plotly/plotly.js/releases
                // At present, we load this up front, but it really should load only on
                // the report page. The issue is that <Script> doesn't seem to inject it
                // properly.
                <script src="https://cdn.plot.ly/plotly-3.1.0.min.js"></script>
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
                // NOTE: plain Axum server routes are not represented here,
                // as they are added directly to the router in `src/main.rs`.
                <Routes fallback=|| "Page not found.".into_view()>
                    <ParentRoute path=StaticSegment("") view=Nav>
                        <Route path=StaticSegment("") view=Landing />
                        <Route path=StaticSegment("gateway") view=Gateway />
                        <Route path=StaticSegment("signedout") view=SignedOut />
                        <Route path=StaticSegment("signup") view=SignUp />
                        <Route path=StaticSegment("terms") view=Terms />
                        <ParentRoute path=StaticSegment("protected") view=Protected>
                            // TODO: consider making instructions a sidebar inside Add
                            <Route path=StaticSegment("add") view=Add />
                            <Route path=StaticSegment("instructions") view=Instructions />
                            <Route path=StaticSegment("list") view=List />
                            <Route path=StaticSegment("report") view=Report />
                        </ParentRoute>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}