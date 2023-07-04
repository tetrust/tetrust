use wasm_bindgen::JsCast;
use yew::{function_component, html, Html};

use crate::lib::js_bind::document::document;

pub fn play_game_audio() {
    let audio = document()
        .get_element_by_id("game-audio")
        .unwrap()
        .dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(|_| ())
        .unwrap();

    if let Err(error) = audio.play() {
        log::error!("Failed to play audio: {:?}", error);
    }
}

pub fn stop_game_audio() {
    let audio = document()
        .get_element_by_id("game-audio")
        .unwrap()
        .dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(|_| ())
        .unwrap();

    if let Err(error) = audio.pause() {
        log::error!("Failed to stop audio: {:?}", error);
    }
}

#[function_component(GameAudio)]
pub fn game_audio() -> Html {
    document()
        .get_element_by_id("game-audio")
        .unwrap()
        .set_attribute("autoplay", "true")
        .unwrap();

    html! {
        <>
            <audio id="game-audio" loop={true}>
                <source src={"/resource/sound/tetrust.ogg"} type={"audio/mp3"}/>
            </audio>
        </>
    }
}
