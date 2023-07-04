#![allow(special_module_name)]

use lib::components;

pub mod lib;
pub mod pages;
pub mod router;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<router::MainRouterComponent>::new().render();
}
