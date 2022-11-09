<<<<<<< HEAD
=======
pub mod bot;
>>>>>>> 116ae438d93b6b859648d1e0079664d29c9f142c
pub mod components;
pub mod constants;
pub mod game;
pub mod js_bind;
pub mod options;
pub mod util;
pub mod wasm_bind;
<<<<<<< HEAD
pub mod bot;
=======
>>>>>>> 116ae438d93b6b859648d1e0079664d29c9f142c

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<components::main::MainComponent>();
}
