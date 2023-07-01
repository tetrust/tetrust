use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent};

use crate::lib::js_bind::localstorage::{get_local_value, set_local_value};

#[function_component(ArrInput)]
pub fn arr() -> Html {
    if let None = get_local_value("arr") {
        set_local_value("arr", 0.to_string());
    }

    let arr_value = get_local_value("arr")
        .map(|v| v.parse::<u32>().ok())
        .flatten()
        .unwrap_or(0);

    let arr_state = use_state(|| arr_value);

    let oninput = Callback::from(move |input_event: InputEvent| {
        let name = arr_state.clone();

        let target: HtmlInputElement = input_event
            .target()
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();

        let value = target.value();
        set_local_value("arr", value.clone());

        let _ = move |_: HtmlInputElement| name.set(value.parse::<u32>().unwrap());
    });

    html! {
        <div>
            <span>{"ARR:"}</span> <input oninput={oninput} type="number" min="0" max="1000" step="1" value={arr_value.to_string()}/>
        </div>
    }
}
