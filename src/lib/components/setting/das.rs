use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent};

use crate::lib::game::local_manager;

#[function_component(DasInput)]
pub fn das() -> Html {
    let das_value = local_manager::get_das_or_set_default();

    let das_state = use_state(|| das_value);

    let oninput = Callback::from(move |input_event: InputEvent| {
        let name = das_state.clone();

        let target: HtmlInputElement = input_event
            .target()
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();

        let value = target.value();
        local_manager::set_das(value.parse::<u32>().unwrap());

        let _ = move |_: HtmlInputElement| name.set(value.parse::<u32>().unwrap());
    });

    html! {
        <div>
            <span>{"DAS:"}</span> <input oninput={oninput} type="number" min="0" max="1000" step="1" value={das_value.to_string()}/>
        </div>
    }
}
