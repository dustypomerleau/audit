use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // todo: We need to populate a surgeon struct at login and put it in a signal.
    // We can then directly access that signal to get things like their default SIA values.
    // This also means we probably don't need a global cur_user in the DB - consider.
    // Alternatively, we could put the user in the URL
    // https://book.leptos.dev/15_global_state.html#global-state-management
    // for auth: https://github.com/ramosbugs/oauth2-rs
    // or less tested: https://github.com/HeroicKatora/oxide-auth/tree/master
    // or using AWS cognito with axum sessions: https://www.youtube.com/watch?v=epX_Bzq1zxs
    // https://github.com/leptos-rs/leptos/tree/f84f1422f447f35adb917582c882ccbc4e1483a7/examples/session_auth_axum
    // You can get the user's information from the Oauth JWT:
    // https://www.oauth.com/oauth2-servers/signing-in-with-google/verifying-the-user-info/
    // validating the token locally: https://developers.google.com/identity/openid-connect/openid-connect#validatinganidtoken

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/muffin.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
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
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
