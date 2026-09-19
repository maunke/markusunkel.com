pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod content;
mod error;
pub mod markdown;
pub mod routes;
#[cfg(feature = "ssr")]
pub mod rss;
#[cfg(feature = "ssr")]
pub mod sitemap;

pub use error::{Error, Result};

pub const SITE_URL: &str = "https://markusunkel.com/";

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
