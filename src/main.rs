pub mod components;
pub mod constants;
pub mod game;
pub mod js_bind;
pub mod options;
pub mod util;
pub mod wasm_bind;
pub mod bot;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<components::main::MainComponent>();
}
