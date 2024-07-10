// Allows using existing fields to change from MyType<Zst1> to MyType<Zst2> without
// repeating all of them.
#![feature(type_changing_struct_update)]

// for rust analyzer support only, not part of the crate
mod refs;
// mod trash;

use cfg_if::cfg_if;

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

cfg_if! { if #[cfg(feature = "hydrate")] {
    use crate::app::App;
    use leptos::mount_to_body;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        // commented out to try islands architecture:
        // leptos::mount_to_body(App);
        // added in to replace above in islands architecture:
        leptos::leptos_dom::HydrationCtx::stop_hydrating();
    }
}}
