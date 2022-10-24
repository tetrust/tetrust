use std::sync::Arc;

use web_sys::KeyboardEvent;
use yew::{function_component, html, use_state, Callback};

use crate::game::manager::GameManager;
use crate::js_bind::focus::focus;

#[function_component(GameBox)]
pub fn game_box() -> Html {
    let game_manager = GameManager::new();
    let game_info = Arc::clone(&game_manager.game_info);
    let _game_info = Arc::clone(&game_manager.game_info);

    let start_disabled = use_state(|| false);

    //let _start_disabled = start_disabled.clone();
    let onclick = {
        //let start_disabled = _start_disabled;

        Callback::from(move |_| {
            focus("gamebox");

            if !game_manager.on_play() {
                // start_disabled.set(true); // Enabling this causes problems.
                game_manager.start_game(); /*Using different mutex objects "GameInfo" */
            }
        })
    };

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        match event.key_code() {
            37 => {
                game_info.lock().unwrap().left_move();
            } // left move
            39 => {
                game_info.lock().unwrap().right_move();
            } // right move
            38 => {} // up move
            40 => {
                game_info.lock().unwrap().soft_drop();
            } // down move
            90 => {
                game_info.lock().unwrap().left_rotate();
            } // z
            88 => {
                game_info.lock().unwrap().right_rotate();
            } // x
            65 => {
                game_info.lock().unwrap().double_rotate();
            } // a
            32 => {
                game_info.lock().unwrap().hard_drop();
            } // spacebar
            16 => {
                game_info.lock().unwrap().hold();
            } // shift
            _ => {}
        }
    });

    let game_info = _game_info;

    let onkeyup = Callback::from(move |event: KeyboardEvent| {
        match event.key_code() {
            37 => {
                game_info.lock().unwrap().on_left_move = None;
            } // left move
            39 => {
                game_info.lock().unwrap().on_right_move = None;
            } // right move
            38 => {} // up move
            40 => {
                game_info.lock().unwrap().on_down_move = None;
            } // down move
            _ => {}
        }
    });

    html! {
        <div id="gamebox" tabindex="0" class="flex content-start" {onkeydown} {onkeyup} onclick={Callback::from(|_| {
            log::info!("test");
            GameManager::empty_render();
        })}>
            <div class="flex flex-col m-5 justify-start">
                <div class="mb-[150px]">
                    <p class="font-mono text-2xl text-center">{"Hold"}</p>
                    <canvas id="hold-canvas" class="" width="120" height="120"></canvas>
                </div>

                <div class="flex flex-col justify-between mb-[80px]">
                    <div id="combo" class="font-mono text-base text-center">{" "}</div>
                    <div id="back2back" class="font-mono text-base text-center">{" "}</div>
                    <div id="message" class="font-mono text-base text-center">{" "}</div>
                </div>

                <div class="flex flex-col justify-between mb-[30px]">
                    <div class="flex flex-row justify-between">
                        <div class="font-mono text-base	">{"Score"}</div>
                        <div id="score">{"0"}</div>
                    </div>
                    <div class="flex flex-row justify-between">
                        <div class="font-mono text-base	content-start">{"Quad"}</div>
                        <div id="quad">{"0"}</div>
                    </div>
                    <div class="flex flex-row justify-between">
                        <div class="font-mono text-base	">{"PC"}</div>
                        <div id="pc">{"0"}</div>
                    </div>
                </div>

                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full" onclick={onclick} disabled={*start_disabled}>{"Start"}</button>
            </div>

            <div class="my-5">
                <canvas id="game-canvas" width="300" height="600"></canvas>
            </div>

            <div class="m-5">
                <p class="font-mono text-2xl text-center">{"Next"}</p>
                <canvas id="next-canvas" class="" width="120" height="520"></canvas>
            </div>

            <audio autoplay={true} loop={true}>
                <source src={"resource/sound/tetrust.ogg"} type={"audio/mp3"}/>
            </audio>
        </div>
    }
}
