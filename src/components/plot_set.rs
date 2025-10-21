use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::InnerHtmlAttribute;
use leptos::prelude::IntoAny;
use leptos::prelude::IntoView;
use leptos::prelude::Resource;
use leptos::prelude::RwSignal;
use leptos::prelude::Suspense;
use leptos::prelude::component;
use leptos::prelude::server;
use leptos::prelude::view;
use serde::Deserialize;
use serde::Serialize;

use crate::error::AppError;
#[cfg(feature = "ssr")] use crate::plots::AsPlot;
#[cfg(feature = "ssr")] use crate::plots::Cohort;
#[cfg(feature = "ssr")] use crate::plots::get_compare;

#[component]
pub fn PlotSet() -> impl IntoView {
    let year = RwSignal::new(2025_u32);
    let plot_resource = Resource::new_blocking(move || year.get(), move |_| get_plots(year.get()));

    view! {
        <Suspense fallback=|| { "waiting for the plot_resource to load..." }>
            <div class="plot-group">
                {move || {
                    if let Some(Ok(plots)) = plot_resource.get() {
                        plots
                            .into_iter()
                            .map(|PlotSet { title, info, plot }| {
                                view! {
                                    <div class="plot-container">
                                        <div class="plot">
                                            <h2 class="plot-title">{title}</h2>
                                            <div class="plot-traces" inner_html=plot></div>
                                        </div>
                                        <div class="plot-info">{info}</div>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                            .into_any()
                    } else {
                        "no inner!".into_any()
                    }
                }}
            </div>
        </Suspense>
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlotSet {
    pub title: Option<String>,
    pub info: Option<String>,
    pub plot: String,
}

#[server]
pub async fn get_plots(year: u32) -> Result<Vec<PlotSet>, AppError> {
    let compare = get_compare(year, Cohort::Peers).await?;
    // Eventually, we will want the surgeon to be able to compare to their prior data.
    //
    // let self_compare = get_compare(year, Cohort::Surgeon).await?;
    //
    // and then create the same 4 plots.

    let plot = compare
        .polar_cyl_before()
        .plot()
        .to_inline_html(Some("cyl-before"));

    let (title, info) = (
        String::from("Preop astigmatism"),
        String::from("cyl before info"),
    );

    let cyl_before = PlotSet {
        title: Some(title),
        info: Some(info),
        plot,
    };

    let plot = compare
        .polar_cyl_after()
        .plot()
        .to_inline_html(Some("cyl-after"));

    let (title, info) = (
        String::from("Postop astigmatism"),
        String::from("cyl after info"),
    );

    let cyl_after = PlotSet {
        title: Some(title),
        info: Some(info),
        plot,
    };

    let plot = compare
        .polar_cyl_target_error()
        .plot()
        .to_inline_html(Some("cyl-target-error"));

    let (title, info) = (
        String::from("Astigmatic target error"),
        String::from("cyl target error info"),
    );

    let cyl_target_error = PlotSet {
        title: Some(title),
        info: Some(info),
        plot,
    };

    let plot = compare
        .cartesian_delta_cyl()
        .plot()
        .to_inline_html(Some("cyl-delta"));

    let (title, info) = (
        String::from("Astigmatism magnitude"),
        String::from("cyl delta info"),
    );

    let cyl_delta = PlotSet {
        title: Some(title),
        info: Some(info),
        plot,
    };

    Ok(vec![cyl_before, cyl_after, cyl_target_error, cyl_delta])
}
