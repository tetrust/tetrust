use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent};

use crate::lib::constants::setting::INFINITY;
use crate::lib::js_bind::localstorage::{get_local_value, set_local_value};

#[function_component(SdfInput)]
pub fn sdf() -> Html {
    if let None = get_local_value("sdf") {
        set_local_value("sdf", 5.to_string());
    }

    let sdf_value = get_local_value("sdf")
        .map(|v| v.parse::<u32>().ok())
        .flatten()
        .unwrap_or(5);

    let sdf_state = use_state(|| sdf_value);

    let oninput = Callback::from(move |input_event: InputEvent| {
        let name = sdf_state.clone();

        let target: HtmlInputElement = input_event
            .target()
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();

        let value = target.value();

        let value = match value.as_str() {
            "41" => INFINITY.to_string(),
            _ => value,
        };

        set_local_value("sdf", value.clone());

        let _ = move |_: HtmlInputElement| name.set(value.parse::<u32>().unwrap());
    });

    html! {
        <div>
            <span>{"SDF:"}</span> <input oninput={oninput} type="number" min="5" max="41" step="1" value={sdf_value.to_string()}/>
        </div>
    }
}
