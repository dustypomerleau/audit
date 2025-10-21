use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

use crate::components::PlotSet;

#[component]
pub fn Report() -> impl IntoView {
    view! { <PlotSet /> }
}
