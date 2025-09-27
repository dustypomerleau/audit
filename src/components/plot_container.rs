use crate::error::AppError;
#[cfg(feature = "ssr")] use crate::plots::{Reference, get_compare};
use leptos::prelude::{
    ElementChild, Get, InnerHtmlAttribute, IntoAny, IntoView, Resource, RwSignal, StyleAttribute,
    Suspense, component, server, view,
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
    let compare = get_compare(year, Reference::Cohort).await?;
    // It's not elegant, but for now just get the previous year comparison as a separate query.
    let self_compare = get_compare(year, Reference::Surgeon).await?;

    // plots to create:
    // 1. preop corneal cylinder polar plot
    // 2. postop refractive cylinder vertexed to cornea polar plot
    // 3. target error polar plot (vertexed to cornea)
    // 4. preop corneal cylinder x, postop refractive cylinder vertexed to cornea y, cartesian
    // the same 4 plots again, but now comparing the surgeon the themselves the previous year.
    let cyl_before = compare
        .polar_cyl_before()
        // fix the plot function to add ellipses for both groups rather than 2 surgeon ellipses
        .plot()
        .to_inline_html(Some("cyl-before"));

    // bookmark: todo:
    // let cyl_after = todo!();
    // let target_error = todo!();

    let cyl_both = compare
        .cartesian_delta_cyl()
        .plot()
        .to_inline_html(Some("cyl-both"));

    Ok(vec![cyl_before])
}
