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
        // get this working with Excel first, then add CSV for other users
        <input type="file" accept=".xlsx"/>
    }
}
