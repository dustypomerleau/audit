// Allows using existing fields to change from MyType<Zst1> to MyType<Zst2> without
// repeating all of them.
#![feature(type_changing_struct_update)]

// for RA support only
mod refs;

pub mod app;
pub mod axis;
pub mod case;
pub mod check;
pub mod cyl;
pub mod error_template;
#[cfg(feature = "ssr")] pub mod fileserv;
pub mod iol;
pub mod refraction;
pub mod sca;
pub mod sia;
pub mod surgeon;
pub mod target;
pub mod va;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
