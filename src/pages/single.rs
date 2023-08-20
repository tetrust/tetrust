use crate::components::footer::Footer;
use crate::components::game_ui;
use crate::components::header::Header;

use stylist::css;
use stylist::yew::Global;

use yew::{function_component, html, Html};

#[function_component(SinglePlay)]
pub fn single() -> Html {
    let css = css!(
        r##"
        .container {
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 90vh;
            max-width: none;
        }"##
    );

    html! {
        <div>
            <Global css={css} />

            <Header/>

            <main class="container">
                <game_ui::GameUI/>
            </main>

            <Footer/>
        </div>
    }
}
