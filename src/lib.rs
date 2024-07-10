// Allows using existing fields to change from MyType<Zst1> to MyType<Zst2> without
// repeating all of them.
#![feature(type_changing_struct_update)]

// for rust analyzer support only, not part of the crate
mod refs;

mod app;
mod axis;
mod case;
mod check;
mod cyl;
mod error_template;
mod fileserv;
mod handler;
mod iol;
mod refraction;
mod sca;
mod sia;
mod surgeon;
mod target;
mod va;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
