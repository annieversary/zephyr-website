use wasm_bindgen::prelude::*;

use zephyr::{scraping::get_classes, Zephyr};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate(classes: Box<[JsValue]>) -> String {
    let z = Zephyr::new();
    let c = classes
        .iter()
        .filter_map(JsValue::as_string)
        .collect::<Vec<_>>();
    z.generate_classes(c.iter().map(String::as_str))
}

#[wasm_bindgen]
pub fn scrape_and_generate(html: String) -> String {
    let classes = get_classes(&html);
    let z = Zephyr::new();
    z.generate_classes(classes.iter().map(String::as_str))
}
