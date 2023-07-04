use super::document::document;

pub(crate) fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}
