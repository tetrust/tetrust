pub mod bot;
pub mod components;
pub mod constants;
pub mod game;
pub mod js_bind;
pub mod options;
pub mod util;
pub mod wasm_bind;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<components::main::MainComponent>::new().render();
}
