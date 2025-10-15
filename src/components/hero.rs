use crate::components::ArrowIol;
use leptos::{
    prelude::{ClassAttribute, ElementChild, IntoView, StyleAttribute, component},
    view,
};

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="hero hero-text neon-pink-text">
            <h1>"Level UP"</h1>
            <div class="hero-arrow neon-pink-svg">
                <ArrowIol />
            </div>
        </div>
    }
}
