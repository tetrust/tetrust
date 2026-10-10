//! 브라우저에서 엔진을 돌리는 계층.
//!
//! - 시뮬레이션은 `requestAnimationFrame` 루프 하나가 게임 시계를 진행시킨다.
//!   setTimeout·setInterval 타이머는 쓰지 않는다.
//! - 키 입력은 `event.timeStamp`로 눌린 시각을 잡아 엔진 입력 로그에 기록한다.
//! - 이벤트 리스너와 rAF 루프는 [`PlayApp`]이 소유하고, 드롭될 때 모두 해제한다.

pub mod input;
pub mod render;
pub mod settings;
pub mod stats;

use std::cell::RefCell;
use std::rc::Rc;

use tetrust_engine::{Event, EventKind, Game};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, KeyboardEvent, Window};

use crate::lib::components::audio::game_audio::{play_game_audio, stop_game_audio};

use render::Renderer;
use stats::{format_time, Mode, Stats};

/// 한 판의 상태.
struct Session {
    game: Game,
    /// 매치 시작 시각 (`performance.now()` 기준 ms).
    start: f64,
    seq: u64,
    /// 리플레이·서버 검증용 입력 로그.
    log: Vec<Event>,
    mode: Mode,
    stats: Stats,
}

impl Session {
    fn new(mode: Mode, start: f64) -> Self {
        let seed = (js_sys::Math::random() * (1u64 << 53) as f64) as u64;
        Self {
            // 설정 페이지에서 바꾼 값은 다음 판부터 적용된다.
            game: Game::new(settings::load_config(), seed),
            start,
            seq: 0,
            log: Vec::new(),
            mode,
            stats: Stats::default(),
        }
    }

    fn collect_outcomes(&mut self) {
        for outcome in self.game.drain_outcomes() {
            self.stats.apply(&outcome, self.mode);
        }
    }

    fn input(&mut self, timestamp: f64, kind: EventKind) {
        if self.stats.is_finished() {
            return;
        }
        let time = input::game_time(timestamp, self.start, self.game.clock());
        let event = Event::new(time, self.seq, kind);
        self.seq += 1;
        match self.game.handle(event) {
            Ok(()) => self.log.push(event),
            Err(e) => log::warn!("dropped input {event:?}: {e:?}"),
        }
        self.collect_outcomes();
    }

    fn frame(&mut self, timestamp: f64) {
        if self.stats.is_finished() {
            return;
        }
        let time = input::game_time(timestamp, self.start, self.game.clock());
        self.game.advance_to(time);
        self.collect_outcomes();
    }

    /// 화면에 보여 줄 경과 시간. 스프린트가 끝났으면 완료 시각에서 멈춘다.
    fn display_time(&self) -> u64 {
        self.stats.finished_at.unwrap_or(self.game.clock())
    }
}

/// 사이드바 텍스트. 값이 바뀐 요소만 DOM에 쓴다.
struct StatsView {
    document: Document,
    last: Vec<(&'static str, String)>,
}

impl StatsView {
    fn new(document: Document) -> Self {
        Self {
            document,
            last: Vec::new(),
        }
    }

    fn set(&mut self, id: &'static str, text: String) {
        if let Some(entry) = self.last.iter_mut().find(|(k, _)| *k == id) {
            if entry.1 == text {
                return;
            }
            entry.1 = text.clone();
        } else {
            self.last.push((id, text.clone()));
        }
        if let Some(el) = self.document.get_element_by_id(id) {
            el.set_text_content(Some(&text));
        }
    }

    fn update(&mut self, session: &Session) {
        let s = &session.stats;
        let time = session.display_time();
        self.set("time", format_time(time));
        self.set("lines", s.lines.to_string());
        self.set("pieces", s.pieces.to_string());
        self.set("pps", format!("{:.2}", s.pieces_per_second(time)));
        self.set("attack", s.attack.to_string());
        self.set("quad", s.quads.to_string());
        self.set("pc", s.perfect_clears.to_string());
        self.set(
            "combo",
            match s.combo {
                Some(c) if c > 0 => format!("{c} Combo"),
                _ => String::new(),
            },
        );
        self.set(
            "back2back",
            match s.back_to_back {
                Some(b) if b > 0 => format!("B2B x{b}"),
                _ => String::new(),
            },
        );
        let message = if s.topped_out {
            "Game Over".to_string()
        } else if s.finished_at.is_some() {
            format!("Finished {}", format_time(time))
        } else {
            s.message.clone()
        };
        self.set("message", message);
    }
}

struct State {
    session: Option<Session>,
    renderer: Renderer,
    view: StatsView,
    /// 다음 판의 모드. R 키 재시작도 이 값을 쓴다.
    mode: Mode,
    /// 지금 판이 끝난 것을 처리했는지 (배경음악 정지).
    finish_handled: bool,
}

impl State {
    fn start(&mut self, now: f64) {
        self.session = Some(Session::new(self.mode, now));
        self.finish_handled = false;
        play_game_audio();
    }

    fn draw(&mut self) {
        match &self.session {
            Some(session) => {
                self.renderer.draw(&session.game);
                self.view.update(session);
                if session.stats.is_finished() && !self.finish_handled {
                    self.finish_handled = true;
                    stop_game_audio();
                }
            }
            None => self.renderer.draw_idle(),
        }
    }
}

type FrameCallback = Closure<dyn FnMut(f64)>;
type KeyCallback = Closure<dyn FnMut(KeyboardEvent)>;

/// 화면 하나에 붙는 플레이 루프. 드롭하면 리스너와 rAF가 해제된다.
pub struct PlayApp {
    window: Window,
    document: Document,
    state: Rc<RefCell<State>>,
    keydown: KeyCallback,
    keyup: KeyCallback,
    frame: Rc<RefCell<Option<FrameCallback>>>,
    frame_handle: Rc<std::cell::Cell<i32>>,
}

fn now(window: &Window) -> f64 {
    window.performance().map_or(0.0, |p| p.now())
}

impl PlayApp {
    pub fn mount() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("no window")?;
        let document = window.document().ok_or("no document")?;
        let state = Rc::new(RefCell::new(State {
            session: None,
            renderer: Renderer::new(&document)?,
            view: StatsView::new(document.clone()),
            mode: Mode::default(),
            finish_handled: false,
        }));

        let keydown = {
            let state = Rc::clone(&state);
            let window = window.clone();
            Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                let code = event.code();
                if code == "KeyR" && !event.repeat() {
                    state.borrow_mut().start(now(&window));
                    return;
                }
                let Some(action) = input::action_for(&code) else {
                    return;
                };
                // 스크롤, 버튼 활성화 같은 브라우저 기본 동작을 막는다.
                event.prevent_default();
                if event.repeat() {
                    return;
                }
                if let Some(session) = state.borrow_mut().session.as_mut() {
                    session.input(event.time_stamp(), EventKind::Press(action));
                }
            })
        };
        let keyup = {
            let state = Rc::clone(&state);
            Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                let Some(action) = input::action_for(&event.code()) else {
                    return;
                };
                event.prevent_default();
                if let Some(session) = state.borrow_mut().session.as_mut() {
                    session.input(event.time_stamp(), EventKind::Release(action));
                }
            })
        };
        document.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
        document.add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())?;

        let frame: Rc<RefCell<Option<FrameCallback>>> = Rc::new(RefCell::new(None));
        let frame_handle = Rc::new(std::cell::Cell::new(0));
        {
            let state = Rc::clone(&state);
            let window_for_loop = window.clone();
            let next = Rc::clone(&frame);
            let handle = Rc::clone(&frame_handle);
            *frame.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
                {
                    let mut state = state.borrow_mut();
                    if let Some(session) = state.session.as_mut() {
                        session.frame(timestamp);
                    }
                    state.draw();
                }
                if let Some(callback) = next.borrow().as_ref() {
                    if let Ok(id) =
                        window_for_loop.request_animation_frame(callback.as_ref().unchecked_ref())
                    {
                        handle.set(id);
                    }
                }
            }));
        }
        if let Some(callback) = frame.borrow().as_ref() {
            frame_handle.set(window.request_animation_frame(callback.as_ref().unchecked_ref())?);
        }

        Ok(Self {
            window,
            document,
            state,
            keydown,
            keyup,
            frame,
            frame_handle,
        })
    }

    /// 새 판을 시작한다.
    pub fn start(&self, mode: Mode) {
        let mut state = self.state.borrow_mut();
        state.mode = mode;
        state.start(now(&self.window));
    }

    /// 지금 판의 입력 로그. 리플레이와 대전 서버 검증(4단계)에서 쓴다.
    #[allow(dead_code)]
    pub fn input_log(&self) -> Vec<Event> {
        self.state
            .borrow()
            .session
            .as_ref()
            .map(|s| s.log.clone())
            .unwrap_or_default()
    }
}

impl Drop for PlayApp {
    fn drop(&mut self) {
        let _ = self.window.cancel_animation_frame(self.frame_handle.get());
        // 클로저가 자기 자신을 붙잡고 있는 순환 참조를 끊는다.
        self.frame.borrow_mut().take();
        let _ = self
            .document
            .remove_event_listener_with_callback("keydown", self.keydown.as_ref().unchecked_ref());
        let _ = self
            .document
            .remove_event_listener_with_callback("keyup", self.keyup.as_ref().unchecked_ref());
    }
}
