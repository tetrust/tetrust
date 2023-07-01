use wasm_bindgen::{prelude::Closure, JsCast};

use super::window::window;

pub(crate) fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
