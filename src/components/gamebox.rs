use std::rc::Rc;

use gloo_timers::callback::Timeout;
use web_sys::KeyboardEvent;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback};

use crate::constants::keycode;
use crate::game::manager::GameManager;
use crate::js_bind::focus::focus;

#[function_component(GameBox)]
pub fn game_box() -> Html {
    let game_manager = GameManager::new();
    let _game_info = Rc::clone(&game_manager.game_info);

    let das = _game_info.borrow().das.clone();

    let start_disabled = use_state(|| false);

    let _start_disabled = start_disabled.clone();
    let onclick = {
        let _start_disabled = _start_disabled;

        Callback::from(move |_| {
            focus("gamebox");

            if !game_manager.on_play() {
                //start_disabled.set(true);
                game_manager.start_game();
            }
        })
    };

    let game_info = Rc::clone(&_game_info);

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        match event.key_code() {
            keycode::LEFT => {
                game_info.borrow_mut().on_right_move = None;

                game_info.borrow_mut().left_move();
            } // left move
            keycode::RIGHT => {
                game_info.borrow_mut().on_left_move = None;

                game_info.borrow_mut().right_move();
            } // right move
            keycode::DOWN => {
                game_info.borrow_mut().soft_drop();
            } // down move
            keycode::Z => {
                game_info.borrow_mut().left_rotate();
            } // z
            keycode::X => {
                game_info.borrow_mut().right_rotate();
            } // x
            keycode::A => {
                game_info.borrow_mut().double_rotate();
            } // a
            keycode::SPACE => {
                game_info.borrow_mut().hard_drop();
            } // spacebar
            keycode::SHIFT => {
                game_info.borrow_mut().hold();
            } // shift
            _ => {}
        }
    });

    let game_info = Rc::clone(&_game_info);

    let onkeypress = Callback::from(move |event: KeyboardEvent| {
        match event.key_code() {
            keycode::LEFT => {
                game_info.borrow_mut().left_move();
                game_info.borrow_mut().on_left_move = Some(instant::Instant::now());

                let game_info = Rc::clone(&game_info);

                Timeout::new(das, move || {
                    if game_info.borrow().on_left_move.is_some() {
                        game_info.borrow_mut().left_move_end();
                    }
                })
                .forget();
            } // left move
            keycode::RIGHT => {
                game_info.borrow_mut().right_move();
                game_info.borrow_mut().on_right_move = Some(instant::Instant::now());

                let game_info = Rc::clone(&game_info);

                Timeout::new(das, move || {
                    if game_info.borrow().on_right_move.is_some() {
                        game_info.borrow_mut().right_move_end();
                    }
                })
                .forget();
            } // right move
            keycode::DOWN => {
                game_info.borrow_mut().soft_drop();
            } // down move
            _ => {}
        }
    });

    let game_info = Rc::clone(&_game_info);

    let _onkeyup = Callback::from(move |event: KeyboardEvent| {
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
    });

    // 최초 렌더링시 호출
    use_effect_with_deps(
        move |_| {
            GameManager::empty_render();
            || ()
        },
        (),
    );

    html! {
        <article id="gamebox" tabindex="0" class="flex justify-between" {onkeydown} {onkeypress}>
            <aside class="flex flex-col m-5 justify-between">
                <dl class="mb-[150px] side-canvas">
                    <dt class="font-mono text-2xl text-center">{"Hold"}</dt>
                    <dd><canvas id="hold-canvas" class="" width="120" height="120"></canvas></dd>
                </dl>

                <dl class="flex flex-col justify-between mb-[80px]">
                    <dt id="combo" class="font-mono text-base text-center">{" "}</dt>
                    <dt id="back2back" class="font-mono text-base text-center">{" "}</dt>
                    <dt id="message" class="font-mono text-base text-center">{" "}</dt>
                </dl>

                <div class="flex flex-col justify-between mb-[30px]">
                    <dl class="flex flex-row justify-between">
                        <dt class="font-mono text-base	">{"Score"}</dt>
                        <dd id="score">{"0"}</dd>
                    </dl>
                    <dl class="flex flex-row justify-between">
                        <dt class="font-mono text-base	content-start">{"Quad"}</dt>
                        <dd id="quad">{"0"}</dd>
                    </dl>
                    <dl class="flex flex-row justify-between">
                        <dt class="font-mono text-base	">{"PC"}</dt>
                        <dd id="pc">{"0"}</dd>
                    </dl>
                </div>

                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full" onclick={onclick} disabled={*start_disabled}>{"Start"}</button>
            </aside>

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
