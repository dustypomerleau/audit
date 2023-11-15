use crate::error_template::{AppError, ErrorTemplate};
use leptos::{component, create_signal, island, tracing, view, Errors, IntoView, SignalUpdate};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, SsrMode};

#[component]
pub fn App() -> impl IntoView {
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
