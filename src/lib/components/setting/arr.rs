use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent};

use crate::lib::game::local_manager;

#[function_component(ArrInput)]
pub fn arr() -> Html {
    let arr_value = local_manager::get_arr_or_set_default();

    let arr_state = use_state(|| arr_value);

    let oninput = Callback::from(move |input_event: InputEvent| {
        let name = arr_state.clone();

        let target: HtmlInputElement = input_event
            .target()
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();

        let value = target.value();
        local_manager::set_arr(value.parse::<u32>().unwrap());

        let _ = move |_: HtmlInputElement| name.set(value.parse::<u32>().unwrap());
    });

    html! {
        <div>
            <span>{"ARR:"}</span> <input oninput={oninput} type="number" min="0" max="1000" step="1" value={arr_value.to_string()}/>
        </div>
    }
}
