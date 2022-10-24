use wasm_bindgen::JsCast;

use super::document::document;

pub(crate) fn focus(id: &str) {
    let element = document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    element.focus().unwrap();
}
