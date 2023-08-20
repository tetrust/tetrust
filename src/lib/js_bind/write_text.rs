use super::document::document;

pub fn write_text(id: &str, text: String) {
    let element = document().get_element_by_id(id).unwrap();

    element.set_inner_html(text.as_str());
}
