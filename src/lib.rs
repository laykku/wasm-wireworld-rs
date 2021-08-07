mod utils;
use wasm_bindgen::prelude::*;

mod world;
pub use world::{Cell, World};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn print_log(s: &str) {
    log(s);
}

#[wasm_bindgen(start)]
pub fn run() {
    print_log("Wireworld")
}