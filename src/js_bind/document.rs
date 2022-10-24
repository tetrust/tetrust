use super::window::window;

pub(crate) fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}
