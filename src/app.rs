use crate::error_template::{AppError, ErrorTemplate};
use leptos::{component, create_signal, island, tracing, view, Errors, IntoView, SignalUpdate};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, SsrMode};

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

    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/main.css"/>
        <Title text="Vic Eye cataract audit | Upload"/>

        <Router
            fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }
        >
            <main>
                <Routes>
                    <Route path="" view=Upload />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Upload() -> impl IntoView {
    view! {
        <h1>"Surgeon data upload"</h1>
        // limit to CSV for now
        // possibly wrap the input in form as shown in the autosubmit example
        // then presumably you can write a server-side handler for that URL
        <input type="file" accept=".csv" id="source" />
    }
}

// show a view that prompts the user to upload a file
// call std::fs::File::open(source) (start by mocking data in a r#"" and then add a folder for test
// CSV data)
//
// autosubmit after selection:
//
// <form id="form" action="http://example.com">
//    <input type="file" id="file">
// </form>
//
// document.getElementById("file").onchange = function() {
//     document.getElementById("form").submit();
// }
