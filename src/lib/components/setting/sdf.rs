use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent};

use crate::lib::game::local_manager;
use crate::lib::game::sdf::sdf_value::SdfValue;

#[function_component(SdfInput)]
pub fn sdf() -> Html {
    let sdf_value = local_manager::get_sdf_or_set_default();

    let sdf_state = use_state(|| sdf_value);

    let oninput = Callback::from(move |input_event: InputEvent| {
        let state = sdf_state.clone();

        let target: HtmlInputElement = input_event
            .target()
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();

        let value = target.value();

        let value = match value.as_str() {
            "41" => SdfValue::Infinity,
            _ => SdfValue::Number(value.parse::<u32>().unwrap()),
        };

        local_manager::set_sdf(value.clone());

        let _ = move |_: HtmlInputElement| state.set(value);
    });

    html! {
        <div>
            <span>{"SDF:"}</span> <input oninput={oninput} type="number" min="5" max="41" step="1" value={sdf_value.to_string()}/>
        </div>
    }
}
