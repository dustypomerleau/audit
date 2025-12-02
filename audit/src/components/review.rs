use leptos::prelude::IntoView;
use leptos::prelude::component;

use crate::model::SurgeonCase;

#[component]
pub fn ReviewCase(case: String) -> impl IntoView { case }
