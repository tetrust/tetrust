#![allow(special_module_name)]

use lib::components;

pub mod lib;
pub mod pages;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<pages::router::MainRouterComponent>::new().render();
}
