mod refs; // for rust analyzer support only

use cfg_if::cfg_if;

pub mod app;
pub mod case;
pub mod csv;
pub mod error_template;
pub mod fileserv;
pub mod powers;
pub mod refraction;
pub mod surgeon;
pub mod target;
pub mod vision;

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
