use crate::components::game_ui;

use yew::{function_component, html, Html};

// TODO: 구현 필요
// 기존 GameUI 컴포넌트를 재사용 가능하게 만들어서 양쪽에 하나씩 두는 식으로 생각중

#[function_component(MultiPlay)]
pub fn multiplay() -> Html {
    html! {
        <div>
            <game_ui::GameUI/>
        </div>
    }
}
