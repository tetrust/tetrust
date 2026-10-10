use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_state, Callback, Html};

use crate::lib::components::audio::game_audio::{stop_game_audio, GameAudio};
use crate::lib::play::stats::Mode;
use crate::lib::play::PlayApp;

#[function_component(GameUI)]
pub fn game_ui() -> Html {
    let app = use_mut_ref(|| None::<PlayApp>);
    let mode = use_state(Mode::default);

    {
        let app = app.clone();
        use_effect_with_deps(
            move |_| {
                match PlayApp::mount() {
                    Ok(play) => *app.borrow_mut() = Some(play),
                    Err(e) => log::error!("failed to start play loop: {e:?}"),
                }
                // 화면을 떠나면 리스너와 rAF 루프를 해제하고 배경음악을 끈다.
                move || {
                    drop(app.borrow_mut().take());
                    stop_game_audio();
                }
            },
            (),
        );
    }

    let onclick = {
        let app = app.clone();
        let mode = mode.clone();
        Callback::from(move |_| {
            if let Some(play) = app.borrow().as_ref() {
                play.start(*mode);
            }
        })
    };
    let to_normal_mode = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(Mode::Normal))
    };
    let to_sprint_mode = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(Mode::Sprint))
    };

    let stat = |label: &'static str, id: &'static str, initial: &'static str| {
        html! {
            <dl class="flex flex-row justify-between">
                <dt class="font-mono text-base">{label}</dt>
                <dd id={id} class="font-mono text-base">{initial}</dd>
            </dl>
        }
    };

    html! {
        <article id="game_ui" tabindex="0" class="flex justify-center">
            <aside class="flex flex-col m-5 justify-between">
                <dl class="mb-[10px] side-canvas">
                    <dt class="font-mono text-2xl text-center">{"Hold"}</dt>
                    <dd><canvas id="hold-canvas" width="120" height="120"></canvas></dd>
                </dl>
                <dl class="flex flex-col justify-between mb-[10px]">
                    <dt id="combo" class="font-mono text-base text-center">{" "}</dt>
                    <dt id="back2back" class="font-mono text-base text-center">{" "}</dt>
                    <dt id="message" class="font-mono text-base text-center">{" "}</dt>
                </dl>

                <div class="flex flex-col justify-between mb-[30px]">
                    {stat("Time", "time", "0.00")}
                    {stat("Lines", "lines", "0")}
                    {stat("Pieces", "pieces", "0")}
                    {stat("PPS", "pps", "0.00")}
                    {stat("Attack", "attack", "0")}
                    {stat("Quad", "quad", "0")}
                    {stat("PC", "pc", "0")}
                </div>

                <button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                    onclick={onclick}
                >
                    {"Start (R)"}
                </button>

                <div>
                    <input
                        type="radio" id="normal" name="mode"
                        checked={*mode == Mode::Normal}
                        onclick={to_normal_mode}
                    />
                    <label for="normal">{"Normal"}</label>
                </div>
                <div>
                    <input
                        type="radio" id="sprint" name="mode"
                        checked={*mode == Mode::Sprint}
                        onclick={to_sprint_mode}
                    />
                    <label for="sprint">{"Sprint (40 Lines)"}</label>
                </div>
                <p class="font-mono text-xs mt-2 leading-5">
                    {"← → 이동 · ↓ 소프트 드롭 · Space 하드 드롭"}<br/>
                    {"↑/X 시계 · Z 반시계 · A 180 · Shift/C 홀드"}
                </p>
            </aside>
            <dl class="mt-[20px] mr-[10px] side-canvas">
                <dd><canvas id="garbage-gauge-canvas" width="30" height="660"></canvas></dd>
            </dl>

            <section class="my-5">
                <canvas id="game-canvas" width="300" height="660"></canvas>
            </section>

            <aside class="m-5 side-canvas">
                <p class="font-mono text-2xl text-center">{"Next"}</p>
                <canvas id="next-canvas" width="120" height="520"></canvas>
            </aside>

            <GameAudio/>
        </article>
    }
}
