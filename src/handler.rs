// use crate::{
//     case::Case, distance::Far, flatcase::FlatCase, refraction::{OpRefraction, Refraction},
// sca::Sca, surgeon::Surgeon, va::{FarVaSet, OpVa, Va} };
// // todo: do you really want axum to be an optional dep? Because it could mean a lot of
// // `#[cfg(...)]`
// #[cfg(feature = "ssr")] use axum::Extension;
// use edgedb_tokio::Client;
// use leptos::{server, ServerFnError};
// use uuid::Uuid;
//
// // todo: reenable server function macro once you have other errors dealt with
// #[server]
// // you previously had `client: Extension<Client>` as an argument, but it appears that maybe the
// // axum integration already provides access to this state.
// // note: https://www.edgedb.com/docs/clients/rust/arguments
// async fn insert_case(case: Case) -> Result<Uuid, ServerFnError> {
//
//     let fc = FlatCase {};
//
//     // for now, just make a new client to get this working, you can learn later how to pass it in
//     // from a `Layer`:
//     let client = edgedb_tokio::create_client().await.expect("edgedb client to be initiated");
//
//
//     // this may not be implemented for big enough tuples
//     let args = (,);
//     let query = "";
//     // how are we getting the client if we don't pass it in?
//     // we want to I think pass in a Vec of Cases, which means we really want to derive Queryable
// on     // Case, and it also means this won't be single
//     let res = client.query_required_single(query, &(args)).await?;
//     // return an error just to get this compiling so you can check the rest
//     Err(ServerFnError::Args("a placeholder error".to_string()))
// }
