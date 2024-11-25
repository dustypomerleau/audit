use crate::routes::{sign_in::BASE_AUTH_URL, Add, List, Register, Report, SignIn};
use leptos::prelude::{AutoReload, HydrationScripts, *};
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
    // for auth:
    // edgedb has an auth solution actually
    // https://github.com/ramosbugs/oauth2-rs
    // or less tested: https://github.com/HeroicKatora/oxide-auth/tree/master
    // or using AWS cognito with axum sessions: https://www.youtube.com/watch?v=epX_Bzq1zxs
    // https://github.com/leptos-rs/leptos/tree/f84f1422f447f35adb917582c882ccbc4e1483a7/examples/session_auth_axum
    // You can get the user's information from the Oauth JWT:
    // https://www.oauth.com/oauth2-servers/signing-in-with-google/verifying-the-user-info/
    // validating the token locally: https://developers.google.com/identity/openid-connect/openid-connect#validatinganidtoken

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <head></head>
        <Stylesheet id="leptos" href="/pkg/audit.css" />
        <Title text="Cataract audit" />

        <Router>
            <main>
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
