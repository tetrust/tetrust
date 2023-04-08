use crate::components::game_ui;

use yew::{function_component, html, Html};

#[function_component(MultiPlay)]
pub fn multiplay() -> Html {
    html! {
        <div>
            <game_ui::GameUI/>
        </div>
    }
}
