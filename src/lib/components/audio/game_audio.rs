use yew::{function_component, html, Html};

#[function_component(GameAudio)]
pub fn game_audio() -> Html {
    html! {
        <>
            <audio autoplay={true} loop={true}>
                <source src={"/resource/sound/tetrust.ogg"} type={"audio/mp3"}/>
            </audio>
        </>
    }
}
