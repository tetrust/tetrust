use std::rc::Rc;

use core::cell::RefCell;
use log::info;
use std::collections::HashMap;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html};

use crate::components::scorebox::ScoreBox;
use crate::constants::keycode;
use crate::game::renderer::GameRenderer;
use crate::game::{Event, GameMode, GameState};
use crate::js_bind::document::document;

#[function_component(GameUI)]
pub fn game_ui() -> Html {
    let game_renderer = GameRenderer::new();

    let _event_queue = Rc::clone(&game_renderer.event_queue);

    let _game_info = Rc::clone(&game_renderer.game_info);
    let game_info1 = Rc::clone(&game_renderer.game_info);
    let game_info2 = Rc::clone(&game_renderer.game_info);

    let _das = _game_info.borrow().das.clone();

    let start_disabled = use_state(|| false);

    let _start_disabled = start_disabled.clone();
    let onclick = {
        let _start_disabled = _start_disabled;

        Callback::from(move |_: _| {
            if !game_renderer.playing() {
                //start_disabled.set(true);
                game_renderer.start_game();
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

    let event_queue = Rc::clone(&_event_queue);

    let _key_states = Rc::new(RefCell::new(HashMap::new()));
    let key_states = Rc::clone(&_key_states);

    let keydown = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        match event.key_code() {
            keycode::LEFT => {
                event.prevent_default(); // Prevent scrolling down by hitting the spacebar
                if !event.repeat() {
                    key_states.borrow_mut().insert("left", true);
                    event_queue.borrow_mut().push_back(Event::LeftMove);
                }

                //
                //     game_info.borrow_mut().on_right_move = None;
                //     game_info.borrow_mut().left_move();

                //     let _game_info = Rc::clone(&game_info);
                //     let game_info = Rc::clone(&_game_info);

                //     Timeout::new(das, move || {
                //         if game_info.borrow().on_left_move.is_some() {
                //             game_info.borrow_mut().left_move_end();
                //         }
                //     })
                //     .forget();

                //     let game_info = Rc::clone(&_game_info);

                //     game_info.borrow_mut().on_left_move = Some(instant::Instant::now());
                // }
            } // left move
            keycode::RIGHT => {
                key_states.borrow_mut().insert("right", true);
                event_queue.borrow_mut().push_back(Event::RightMove);

                // if !event.repeat() {
                //     game_info.borrow_mut().on_right_move = None;
                //     game_info.borrow_mut().right_move();

                //     let _game_info = Rc::clone(&game_info);
                //     let game_info = Rc::clone(&_game_info);

                //     Timeout::new(das, move || {
                //         if game_info.borrow().on_right_move.is_some() {
                //             game_info.borrow_mut().right_move_end();
                //         }
                //     })
                //     .forget();

                //     let game_info = Rc::clone(&_game_info);

                //     game_info.borrow_mut().on_right_move = Some(instant::Instant::now());
                // }
            } // right move
            keycode::DOWN => {
                key_states.borrow_mut().insert("down", true);
                event_queue.borrow_mut().push_back(Event::SoftDrop);

                // if !event.repeat() {
                //     if game_info.borrow().sdf_is_infinity {
                //         game_info.borrow_mut().down_move_end();
                //     } else {
                //         game_info.borrow_mut().on_down_move = None;
                //         game_info.borrow_mut().soft_drop();

                //         let _game_info = Rc::clone(&game_info);
                //         let game_info = Rc::clone(&_game_info);

                //         Timeout::new(das, move || {
                //             if game_info.borrow().on_down_move.is_some() {
                //                 game_info.borrow_mut().down_move_end();
                //             }
                //         })
                //         .forget();

                //         let game_info = Rc::clone(&_game_info);

                //         game_info.borrow_mut().on_down_move = Some(instant::Instant::now());
                //     }
                // }
            } // down move
            keycode::Z => {
                event_queue.borrow_mut().push_back(Event::LeftRotate);

                // if !event.repeat() {
                //     game_info.borrow_mut().left_rotate();
                // }
            } // z
            keycode::X => {
                event_queue.borrow_mut().push_back(Event::RightRotate);

                // if !event.repeat() {
                //     game_info.borrow_mut().right_rotate();
                // }
            } // x
            keycode::A => {
                event_queue.borrow_mut().push_back(Event::DoubleRotate);

                // if !event.repeat() {
                //     game_info.borrow_mut().double_rotate();
                // }
            } // a
            keycode::SPACE => {
                event.prevent_default(); // Prevent scrolling down by hitting the spacebar
                event_queue.borrow_mut().push_back(Event::HardDrop);

                // if !event.repeat() {
                //     game_info.borrow_mut().hard_drop();
                // }
            } // spacebar
            keycode::SHIFT => {
                event_queue.borrow_mut().push_back(Event::Hold);

                // if !event.repeat() {
                //     game_info.borrow_mut().hold();
                // }
            } // shift
            _ => {}
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);

    document()
        .add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())
        .unwrap();

    keydown.forget();

    let event_queue = Rc::clone(&_event_queue);
    let key_states = Rc::clone(&_key_states);

    let keyup = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        match event.key_code() {
            keycode::LEFT => {
                key_states.borrow_mut().remove("left");
                event_queue.borrow_mut().push_back(Event::LeftMoveStop);

                //game_info.borrow_mut().on_left_move = None;
            } // left move
            keycode::RIGHT => {
                key_states.borrow_mut().remove("right");
                event_queue.borrow_mut().push_back(Event::RightMoveStop);

                //game_info.borrow_mut().on_right_move = None;
            } // right move
            keycode::DOWN => {
                key_states.borrow_mut().remove("down");
                // event_queue.borrow_mut().push_back(Event::LeftMoveStop);

                // game_info.borrow_mut().on_down_move = None;
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
            GameRenderer::empty_render();
            || ()
        },
        (),
    );

    html! {
        <article id="game_ui" tabindex="0" class="flex justify-center">
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
