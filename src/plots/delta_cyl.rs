// In terms of structure, we want something like a parent Plot function/component that takes a
// struct holding a vec of SurgeonCase and a comparison vec of Case. Then this parent component
// passes the right fields for analysis to all of the subplots, so the subplots need to be able to
// accept data by reference.
//
// The idea is to use the builder syntax to make separate traces for the surgeon and the cohort,
// and then put both of the traces on one plot. `let trace = ScatterPolar::new(...);`.
//
// As a test case, perhaps start with a simple Histogram that shows the distribution of
// postoperative refractive astigmatism for surgeon and cohort. This simple plot would not take
// preop data into account, only the goal of complete cyl elimination. And then follow with polar
// scatter double angle plots and the like.
//
// Actually start with x = preop corneal astigmatism, y = postop refractive astigmatism (scatter)

// i think we basically want the route to be Report, and then the view on Report contains
// PlotContainer(compare: Compare) and then all the subviews take &Compare and return a view
// if there's common patterns then maybe you do some work in PlotContainer and supply the plot with
// a ScatterCompare, if it's the same for several plots, but the data is shown differently
use crate::{
    model::{Email, Surgeon},
    plots::{ScatterCompare, get_compare},
};
use leptos::{
    prelude::{
        Get, GlobalAttributes, IntoAny, IntoView, Resource, RwSignal, Suspense, component,
        expect_context, use_context, view,
    },
    reactive::spawn_local,
};
// use plotly::{Plot, Scatter};

// todo: create a view to test this plot.
// Once you have a plot working, turn to deploy, then styles
#[component]
pub fn DeltaCyl() -> impl IntoView {
    // todo: expecting this context is failing
    // you fool you need another SUSPENSE for compare_resource!
    // It's possible that either surgeon_resource is failing, or provide_context() is failing.
    // todo: migrate to use_context and handle Nones
    // let email = expect_context::<RwSignal<Option<Surgeon>>>()
    //     .get()
    //     .unwrap()
    //     .email;
    // dbg!(&email);

    let year = RwSignal::new(2025_u32);
    let compare_resource = Resource::new(move || year.get(), move |year| get_compare(year));

    // for each `Scatter` we are plotting magnitude of preop corneal cyl versus magnitude of
    // postoperative refractive cyl (do they need to be in the same plane, or is it ok that the
    // outcome measure is apples:apples)
    view! {
        <Suspense fallback=move || {
            view! { "Getting plot data to compare..." }
        }>
            {move || {
                let ScatterCompare { surgeon, cohort } = if let Some(Ok(compare)) = compare_resource
                    .get()
                {
                    compare.scatter_delta_cyl()
                } else {
                    return view! { "Query for the surgeon and cohort was not successful" }
                        .into_any();
                };
                dbg!((&surgeon, &cohort));
                // let surgeon = Scatter::new(surgeon.x, surgeon.y).name("Surgeon");
                // let cohort = Scatter::new(cohort.x, cohort.y).name("Cohort");
                // let mut plot = Plot::new();
                // plot.add_traces(vec![surgeon, cohort]);
                // plot.show();
                view! { "Plot should be loading with show()" }
                    .into_any()
            }}
        </Suspense>
    }
    .into_any()
}
// spawn_local(async move {
//     plotly::bindings::new_plot("plotly-delta-cyl", &plot).await
// });
//
// view! { <div id="plotly-delta-cyl"></div> }
//     .into_any()

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "ssr")]
    #[tokio::test]
    async fn exports_a_plot() {
        use crate::plots::Compare;
        use dotenvy::dotenv;
        use plotly::{Plot, Scatter, common::Mode};

        dotenv().ok();

        async fn mock_get_compare(year: u32) -> Compare {
            use crate::{db::tests::test_db, query::query_select_compare};
            use std::env;

            let query = query_select_compare(year);

            let query_result = test_db()
                .await
                .query_single_json(query, &())
                .await
                .unwrap()
                .unwrap();

            serde_json::from_str::<Compare>(query_result.as_ref()).unwrap()
        }

        let ScatterCompare { surgeon, cohort } = mock_get_compare(2025).await.scatter_delta_cyl();

        let surgeon = Scatter::new(surgeon.x, surgeon.y)
            .name("Surgeon")
            .mode(Mode::Markers);

        let cohort = Scatter::new(cohort.x, cohort.y)
            .name("Cohort")
            .mode(Mode::Markers);

        let mut plot = Plot::new();
        // note: the surgeon should be added after the cohort, because that allows hover on their
        // points, which are "on top" in the layered plot
        plot.add_traces(vec![cohort, surgeon]);
        plot.show();
    }
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
