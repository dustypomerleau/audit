use leptos::prelude::{IntoView, OnceResource, ServerFnError, component, server, view};
use serde::{Deserialize, Serialize};

#[component]
pub fn Repro() -> impl IntoView {
    let resource = OnceResource::new(get_value());
    view! { Suspend::new(resource.await) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    inner: u32,
}

#[server]
pub async fn get_value() -> Result<Sample, ServerFnError> {
    Ok(Sample { inner: 5 })
}
