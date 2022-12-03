use yew::{function_component, html, use_state,Callback};
use web_sys::KeyboardEvent;

// game_manager on)play가 아닌경우, 값을 초기화
#[function_component(ScoreBox)]
pub fn score_box() -> html {
    let time_count = use_state(|| 1);

    let onkeypress = Callback::from(move |event: KeyboardEvent| {
     let time_count = time_count.clone();
        time_count.set(*time_count + 1);
    });
    html! {
        <section {onkeypress}>
                < div class="flex flex-col justify-between mb-[30px]">
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
                                        <dl class="flex flex-row justify-between">
                        <dt class="font-mono text-base	">{"timeCount"}</dt>
                        <dd id="pc">{*time_count}</dd>
                    </dl>
                </div>
                </section>
    }
}
