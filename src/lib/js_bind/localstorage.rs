use super::window::window;

pub fn get_local_value(id: &str) -> Option<String> {
    let local_storage = window().local_storage().ok().flatten()?;

    local_storage.get_item(id).ok().flatten()
}
