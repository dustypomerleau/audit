#![feature(const_trait_impl, effects)] // allows using range syntax on constants in powers.rs

mod refs; // for rust analyzer support only

use cfg_if::cfg_if;

pub mod app;
pub mod axis;
pub mod case;
pub mod csv;
pub mod cyl;
pub mod error_template;
pub mod fileserv;
pub mod flatcase;
pub mod incision;
pub mod iol;
pub mod powers;
pub mod refraction;
pub mod sca;
pub mod surgeon;
pub mod target;
pub mod va;

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
