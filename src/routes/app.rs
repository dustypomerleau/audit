use crate::routes::{AppError, ErrorTemplate, SignIn};
#[allow(unused_imports)] use leptos::{component, template, view, Errors, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

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
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/audit.css"/>
        <Title text="Cataract audit"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=SignIn />
                </Routes>
            </main>
        </Router>
    }
}
