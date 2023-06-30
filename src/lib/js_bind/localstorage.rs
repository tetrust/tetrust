use super::window::window;

pub fn get_local_value(key: &str) -> Option<String> {
    let local_storage = window().local_storage().ok().unwrap().unwrap();

    local_storage.get_item(key).ok().flatten()
}

pub fn set_local_value(key: &str, value: String) {
    let local_storage = window().local_storage().ok().unwrap().unwrap();

    let _ = local_storage.set_item(key, &value);
}
