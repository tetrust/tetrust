use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent};

use crate::lib::js_bind::localstorage::{get_local_value, set_local_value};

#[function_component(DasInput)]
pub fn das() -> Html {
    if let None = get_local_value("das") {
        set_local_value("das", "300".to_string());
    }

    let das_value = get_local_value("das")
        .map(|v| v.parse::<u32>().ok())
        .flatten()
        .unwrap_or(300);

    let das_state = use_state(|| das_value);

    let oninput = Callback::from(move |input_event: InputEvent| {
        let name = das_state.clone();
        log::info!("das input");

        let target: HtmlInputElement = input_event
            .target()
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();

        let value = target.value();
        set_local_value("das", value.clone());

        let _ = move |_: HtmlInputElement| name.set(value.parse::<u32>().unwrap());
    });

    html! {
        <div>
            <span>{"DAS:"}</span> <input oninput={oninput} type="number" min="0" max="1000" step="1" value={das_value.to_string()}/>
        </div>
    }
}
