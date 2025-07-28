// i think we basically want the route to be Report, and then the view on Report contains
// PlotContainer(compare: Compare) and then all the subviews take &Compare and return a view
// if there's common patterns then maybe you do some work in PlotContainer and supply the plot with
// a ScatterCompare, if it's the same for several plots, but the data is shown differently
use crate::plots::get_compare;
use leptos::prelude::{Get, IntoAny, IntoView, Resource, RwSignal, Suspense, component, view};
// use plotly::{Plot, Scatter};

// todo: create a view to test this plot.
// Once you have a plot working, turn to deploy, then styles
#[component]
pub fn DeltaCyl() -> impl IntoView {
    let year = RwSignal::new(2025_u32);
    let compare_resource = Resource::new(move || year.get(), move |year| get_compare(year));

    // for each `Scatter` we are plotting magnitude of preop corneal cyl versus magnitude of
    // postoperative refractive cyl (do they need to be in the same plane, or is it ok that the
    // outcome measure is apples:apples)
    //
    // todo: handle the plot entirely on the server, and just ship it back to the browser by
    // setting inner html.
    // Get it working correctly in the test below first.
    //
    // So in that case the resource won't return the Compare, but will instead just return the
    // view!{}
    view! {
        <Suspense fallback=move || {
            view! { "Getting plot data to compare..." }
        }>
            {
                view! { "todo" }
            }
        </Suspense>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    // #[cfg(feature = "ssr")]
    // #[tokio::test]
    // async fn exports_a_plot() {
    //     use crate::plots::{Compare, ScatterCompare};
    //     use dotenvy::dotenv;
    //     use plotly::{Plot, Scatter, common::Mode};
    //
    //     dotenv().ok();
    //
    //     async fn mock_get_compare(year: u32) -> Compare {
    //         use crate::{db::tests::test_db, query::query_select_compare};
    //
    //         let query = query_select_compare(year);
    //
    //         let query_result = test_db()
    //             .await
    //             .query_single_json(query, &())
    //             .await
    //             .unwrap()
    //             .unwrap();
    //
    //         serde_json::from_str::<Compare>(query_result.as_ref()).unwrap()
    //     }
    //
    //     let ScatterCompare { surgeon, cohort } =
    // mock_get_compare(2025).await.scatter_delta_cyl();
    //
    //     let surgeon = Scatter::new(surgeon.x, surgeon.y)
    //         .name("Surgeon")
    //         .mode(Mode::Markers);
    //
    //     let cohort = Scatter::new(cohort.x, cohort.y)
    //         .name("Cohort")
    //         .mode(Mode::Markers);
    //
    //     let mut plot = Plot::new();
    //     // note: the surgeon should be added after the cohort, because that allows hover on their
    //     // points, which are "on top" in the layered plot
    //     plot.add_traces(vec![cohort, surgeon]);
    //     plot.show();
    // }
}

// For reference, the exported HTML that Plotly produces looks like:
// <!doctype html>
// <html lang="en">
//     <head>
//         <meta charset="utf-8" />
//     </head>
//
//     <body>
//         <div>
//             <script src="https://cdn.plot.ly/plotly-2.12.1.min.js"></script>
//             <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
//             <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.0/es5/tex-mml-chtml.js"></script>
//
//             <div
//                 id="plotly-html-element"
//                 class="plotly-graph-div"
//                 style="height:100%; width:100%;"
//             ></div>
//
//             <script type="module">
//                 const graph_div = document.getElementById("plotly-html-element");
//                 await Plotly.newPlot(graph_div, {
//                     data: [
//                         { type: "scatter", name: "Surgeon", x: [5.0, 0.0], y: [0.0, 0.0] },
//                         { type: "scatter", name: "Cohort", x: [0.0], y: [0.0] },
//                     ],
//                     layout: {},
//                     config: {},
//                 });
//             </script>
//         </div>
//     </body>
// </html>
