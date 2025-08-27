use crate::error::AppError;
#[cfg(feature = "ssr")] use crate::plots::get_compare;
use leptos::prelude::{
    Get, InnerHtmlAttribute, IntoAny, IntoView, Resource, RwSignal, Suspense, component, server,
    view,
};

#[component]
pub fn PlotContainer() -> impl IntoView {
    let year = RwSignal::new(2025_u32);
    let plot_resource = Resource::new_blocking(move || year.get(), move |_| get_plots(year.get()));

    view! {
        <Suspense fallback=|| {
            "waiting for the plot_resource to load..."
        }>
            {move || {
                if let Some(Ok(plots)) = plot_resource.get() {
                    plots
                        .into_iter()
                        .map(|plot| view! { <div inner_html=plot></div> })
                        .collect::<Vec<_>>()
                        .into_any()
                } else {
                    "no inner!".into_any()
                }
            }}
        </Suspense>
    }
}

#[server]
pub async fn get_plots(year: u32) -> Result<Vec<String>, AppError> {
    // bookmark: todo:
    // - separate this assignment into 2 parts: get the Compare, and generate the polar plot
    // - use the same compare to generate each plot you need for the whole report
    // - return the HTML as a Vec and iterate over it to create the views.
    let plot_string = get_compare(year)
        .await
        .unwrap()
        .polar_cyl_before()
        .polar_plot()
        .to_inline_html(Some("plot"));

    Ok(vec![plot_string])
}
