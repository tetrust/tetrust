use wasm_bindgen::JsCast;
use web_sys::HtmlAudioElement;
use yew::{function_component, html, Html};

use crate::lib::js_bind::document::document;

fn get_audio() -> Option<HtmlAudioElement> {
    let audio = document()
        .get_element_by_id("game-audio")?
        .dyn_into::<web_sys::HtmlAudioElement>()
        .ok()?;

    Some(audio)
}

pub fn play_game_audio() {
    if let Some(audio) = get_audio() {
        if let Err(error) = audio.play() {
            log::error!("Failed to play audio: {:?}", error);
        }
    }
}

pub fn stop_game_audio() {
    if let Some(audio) = get_audio() {
        if let Err(error) = audio.pause() {
            log::error!("Failed to stop audio: {:?}", error);
        }
    }
}

#[function_component(GameAudio)]
pub fn game_audio() -> Html {
    html! {
        <>
            <audio id="game-audio" loop={true}>
                <source src={"/resource/sound/tetrust.ogg"} type={"audio/mp3"}/>
            </audio>
        </>
    }
}
