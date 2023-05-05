use std::rc::Rc;

use log::info;
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
    let _key_states = Rc::clone(&game_renderer.key_states);

    let _game_info = Rc::clone(&game_renderer.game_info);

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

    let game_info = Rc::clone(&_game_info);
    let to_normal_mode = {
        Callback::from(move |_: _| {
            let mut game_info = game_info.borrow_mut();
            if game_info.game_state != GameState::PLAYING {
                info!("Switching to normal mode");
                game_info.game_mode = GameMode::NORMAL;
            }
        })
    };

    let game_info = Rc::clone(&_game_info);
    let to_sprint_mode = {
        Callback::from(move |_| {
            let mut game_info = game_info.borrow_mut();
            if game_info.game_state != GameState::PLAYING {
                info!("Switching to sprint mode");
                game_info.game_mode = GameMode::SPRINT;
            }
        })
    };

    let game_info = Rc::clone(&_game_info);
    let to_cheese_mode = {
        Callback::from(move |_| {
            let mut game_info = game_info.borrow_mut();
            if game_info.game_state != GameState::PLAYING {
                info!("Switching to sprint mode");
                game_info.game_mode = GameMode::CHEESE;
            }
        })
    };

    let event_queue = Rc::clone(&_event_queue);
    let key_states = Rc::clone(&_key_states);

    let keydown = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        match event.key_code() {
            keycode::LEFT => {
                event.prevent_default(); 
                if !event.repeat() {
                    key_states.borrow_mut().set_left(true);
                    event_queue.borrow_mut().push_back(Event::LeftMove);
                }
            } // left move
            keycode::RIGHT => {
                event.prevent_default(); 
                if !event.repeat() {
                    key_states.borrow_mut().set_right(true);
                    event_queue.borrow_mut().push_back(Event::RightMove);
                }
            } // right move
            keycode::DOWN => {
                event.prevent_default(); // Prevent scrolling down by hitting the down key
                if !event.repeat() {
                    key_states.borrow_mut().set_down(true);
                    event_queue.borrow_mut().push_back(Event::SoftDrop);
                }
            } // down move
            keycode::Z => {
                event_queue.borrow_mut().push_back(Event::LeftRotate);
            } // z
            keycode::X => {
                event_queue.borrow_mut().push_back(Event::RightRotate);
            } // x
            keycode::A => {
                event_queue.borrow_mut().push_back(Event::DoubleRotate);
            } // a
            keycode::SPACE => {
                event.prevent_default(); // Prevent scrolling down by hitting the spacebar
                event_queue.borrow_mut().push_back(Event::HardDrop);
            } // spacebar
            keycode::SHIFT => {
                event_queue.borrow_mut().push_back(Event::Hold);
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
                key_states.borrow_mut().set_left(true);
                event_queue.borrow_mut().push_back(Event::LeftMoveStop);
            } // left move
            keycode::RIGHT => {
                key_states.borrow_mut().set_right(true);
                event_queue.borrow_mut().push_back(Event::RightMoveStop);
            } // right move
            keycode::DOWN => {
                key_states.borrow_mut().set_down(true);
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
                <div>
                <input
                    type="radio" id="cheese" name="mode"
                    onclick={to_cheese_mode}
                />
                <label for="normal">{"Cheese race)"}</label>
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
