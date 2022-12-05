use std::rc::Rc;

use gloo_timers::callback::Timeout;
use log::info;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html};

use crate::components::scorebox::ScoreBox;
use crate::constants::keycode;
use crate::game::manager::GameManager;
use crate::game::{GameMode, GameState};
use crate::js_bind::document::document;

#[function_component(GameBox)]
pub fn game_box() -> Html {
    let game_manager = GameManager::new();
    let _game_info = Rc::clone(&game_manager.game_info);
    let game_info1 = Rc::clone(&game_manager.game_info);
    let game_info2 = Rc::clone(&game_manager.game_info);

    let das = _game_info.borrow().das.clone();

    let start_disabled = use_state(|| false);

    let _start_disabled = start_disabled.clone();
    let onclick = {
        let _start_disabled = _start_disabled;

        Callback::from(move |_: _| {
            if !game_manager.playing() {
                //start_disabled.set(true);
                game_manager.start_game();
            }
        })
    };

    let to_normal_mode = {
        Callback::from(move |_: _| {
            let mut game_info = game_info1.borrow_mut();
            if game_info.game_state != GameState::PLAYING {
                info!("Switching to normal mode");
                game_info.game_mode = GameMode::NORMAL;
            }
        })
    };

    let to_sprint_mode = {
        Callback::from(move |_| {
            let mut game_info = game_info2.borrow_mut();
            if game_info.game_state != GameState::PLAYING {
                info!("Switching to sprint mode");
                game_info.game_mode = GameMode::SPRINT;
            }
        })
    };

    let game_info = Rc::clone(&_game_info);

    let keydown = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        match event.key_code() {
            keycode::LEFT => {
                if !event.repeat() {
                    game_info.borrow_mut().on_right_move = None;
                    game_info.borrow_mut().left_move();

                    let _game_info = Rc::clone(&game_info);
                    let game_info = Rc::clone(&_game_info);

                    Timeout::new(das, move || {
                        if game_info.borrow().on_left_move.is_some() {
                            game_info.borrow_mut().left_move_end();
                        }
                    })
                    .forget();

                    let game_info = Rc::clone(&_game_info);

                    game_info.borrow_mut().on_left_move = Some(instant::Instant::now());
                }
            } // left move
            keycode::RIGHT => {
                if !event.repeat() {
                    game_info.borrow_mut().on_right_move = None;
                    game_info.borrow_mut().right_move();

                    let _game_info = Rc::clone(&game_info);
                    let game_info = Rc::clone(&_game_info);

                    Timeout::new(das, move || {
                        if game_info.borrow().on_right_move.is_some() {
                            game_info.borrow_mut().right_move_end();
                        }
                    })
                    .forget();

                    let game_info = Rc::clone(&_game_info);

                    game_info.borrow_mut().on_right_move = Some(instant::Instant::now());
                }
            } // right move
            keycode::DOWN => {
                if !event.repeat() {
                    game_info.borrow_mut().soft_drop();
                }
            } // down move
            keycode::Z => {
                if !event.repeat() {
                    game_info.borrow_mut().left_rotate();
                }
            } // z
            keycode::X => {
                if !event.repeat() {
                    game_info.borrow_mut().right_rotate();
                }
            } // x
            keycode::A => {
                if !event.repeat() {
                    game_info.borrow_mut().double_rotate();
                }
            } // a
            keycode::SPACE => {
                if !event.repeat() {
                    game_info.borrow_mut().hard_drop();
                }
            } // spacebar
            keycode::SHIFT => {
                if !event.repeat() {
                    game_info.borrow_mut().hold();
                }
            } // shift
            _ => {}
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);

    document()
        .add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())
        .unwrap();

    keydown.forget();

    // let onkeypress = Callback::from(move |event: KeyboardEvent| {
    //     match event.key_code() {
    //         keycode::LEFT => {
    //             game_info.borrow_mut().left_move();
    //             game_info.borrow_mut().on_left_move = Some(instant::Instant::now());

    //             let game_info = Rc::clone(&game_info);

    //             Timeout::new(das, move || {
    //                 if game_info.borrow().on_left_move.is_some() {
    //                     game_info.borrow_mut().left_move_end();
    //                 }
    //             })
    //             .forget();
    //         } // left move
    //         keycode::RIGHT => {
    //             game_info.borrow_mut().right_move();
    //             game_info.borrow_mut().on_right_move = Some(instant::Instant::now());

    //             let game_info = Rc::clone(&game_info);

    //             Timeout::new(das, move || {
    //                 if game_info.borrow().on_right_move.is_some() {
    //                     game_info.borrow_mut().right_move_end();
    //                 }
    //             })
    //             .forget();
    //         } // right move
    //         keycode::DOWN => {
    //             game_info.borrow_mut().soft_drop();
    //         } // down move
    //         _ => {}
    //     }
    // });

    let game_info = Rc::clone(&_game_info);

    let keyup = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        match event.key_code() {
            keycode::LEFT => {
                game_info.borrow_mut().on_left_move = None;
            } // left move
            keycode::RIGHT => {
                game_info.borrow_mut().on_right_move = None;
            } // right move
            keycode::DOWN => {
                game_info.borrow_mut().on_down_move = None;
            } // down move
            _ => {}
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);

    document()
        .add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())
        .unwrap();

    keyup.forget();

    // 최초 렌더링시 호출
    use_effect_with_deps(
        move |_| {
            GameManager::empty_render();
            || ()
        },
        (),
    );

    html! {
        <article id="gamebox" tabindex="0" class="flex justify-center">
            <aside class="flex flex-col m-5 justify-between">
                <dl class="mb-[10px] side-canvas">
                    <dt class="font-mono text-2xl text-center">{"Hold"}</dt>
                    <dd><canvas id="hold-canvas" class="" width="120" height="120"></canvas></dd>
                </dl>
                <dl class="flex flex-col justify-between mb-[10px]">
                    <dt id="combo" class="font-mono text-base text-center">{" "}</dt>
                    <dt id="back2back" class="font-mono text-base text-center">{" "}</dt>
                    <dt id="message" class="font-mono text-base text-center">{" "}</dt>
                </dl>

                <div class="flex flex-col justify-between mb-[30px]">
                    <dl class="flex flex-row justify-between">
                        <dt class="font-mono text-base	">{"Time"}</dt>
                        <dd id="time" class="font-mono text-base">{"0.00"}</dd>
                    </dl>
                    <dl class="flex flex-row justify-between">
                        <dt class="font-mono text-base	">{"Lines"}</dt>
                        <dd id="lineclearcount" class="font-mono text-base">{"0"}</dd>
                    </dl>
                </div>

                <ScoreBox/>

                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full" onclick={onclick} disabled={*start_disabled}>{"Start"}</button>

                <div>
                    <input
                        type="radio" id="normal" name="mode" checked=true
                        onclick={to_normal_mode}
                    />
                    <label for="normal">{"Normal"}</label>
                </div>
                <div>
                    <input
                        type="radio" id="sprint" name="mode"
                        onclick={to_sprint_mode}
                    />
                    <label for="normal">{"Sprint(40 Lines)"}</label>
                </div>
            </aside>
            <dl class="mt-[20px] mr-[10px] side-canvas">
                <dd><canvas id="garbage-gauge-canvas" class="" width="30" height="600"></canvas></dd>
            </dl>

            <section class="my-5">
                <canvas id="game-canvas" width="300" height="600"></canvas>
            </section>

            <aside class="m-5  side-canvas">
                <p class="font-mono text-2xl text-center">{"Next"}</p>
                <canvas id="next-canvas" class="" width="120" height="520"></canvas>
            </aside>

            <audio autoplay={true} loop={true}>
                <source src={"/resource/sound/tetrust.ogg"} type={"audio/mp3"}/>
            </audio>
        </article>
    }
}
