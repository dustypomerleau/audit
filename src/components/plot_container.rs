use crate::error::AppError;
#[cfg(feature = "ssr")] use crate::plots::get_compare;
use leptos::{
    html::InnerHtmlAttribute,
    prelude::{
        ElementChild, Get, GlobalAttributes, IntoAny, IntoView, Resource, StyleAttribute, Suspense,
        component, server, view,
    },
};
use leptos_meta::Script;

#[component]
pub fn PlotContainer() -> impl IntoView {
    let plot_resource = Resource::new_blocking(|| (), |_| test_a_polar_plot());

    view! {
        <Suspense fallback=|| {
            "waiting for the plot_resource to load..."
        }>
            {move || {
                if let Some(Ok(inner)) = plot_resource.get() {
                    dbg!(&inner);
                    view! { <div inner_html=inner></div> }.into_any()
                } else {
                    "no inner!".into_any()
                }
            }}
        </Suspense>
    }
}

// todo: this is just a quick way to make some plot to get embedding working
#[server]
pub async fn test_a_polar_plot() -> Result<String, AppError> {
    // todo: note: get compare fails if you aren't signed in, but once you put it inside protected
    // does that matter?
    let plot_string = get_compare(2025)
        .await
        .unwrap()
        .polar_cyl_before()
        .polar_plot()
        .to_inline_html(Some("plot"));
    dbg!(&plot_string);

    Ok(plot_string)
}
