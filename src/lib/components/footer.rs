use stylist::css;
use stylist::yew::Global;
use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    let css = css!(
        r##"
        footer {
            width: 100%;
            position: relative;
            text-align: center;
        }
    "##
    );

    html! {
        <>
            <Global css={css} />

            <footer>
                <p>{"Copyright © 2023 - All rights Reserved - by myyrakle"}</p>
            </footer>
        </>
    }
}
