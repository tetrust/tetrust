pub(crate) fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
