// Allows using existing fields to change from MyType<Zst1> to MyType<Zst2> without
// repeating all of them.
#![feature(type_changing_struct_update)]

/// The [`audit`](self) library provides tools for analyzing and plotting the results of cataract
/// surgery. Powers that would typically be represented in diopters (1 m e-1) (refractions, IOLs,
/// refractive targets) are instead represented in centidiopters (1 hm^-1) to allow integer math.
// for RA support only
mod refs;

pub mod bounds_check;
pub mod case;
pub mod components;
pub mod cyl;
#[cfg(feature = "ssr")] pub mod db;
#[cfg(feature = "ssr")] pub mod fileserv;
pub mod iol;
pub mod plots;
pub mod refraction;
pub mod routes;
pub mod sca;
pub mod sia;
pub mod surgeon;
pub mod target;
pub mod va;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::routes::App;

    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
