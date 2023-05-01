use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use js_sys::Date;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::spawn_local;

use crate::constants::character::SPECIAL_SPACE;
use crate::constants::time::GRAVITY_DROP_INTERVAL;
use crate::game::game_info::GameInfo;
use crate::game::Event;
use crate::js_bind::request_animation_frame::request_animation_frame;
use crate::js_bind::write_text::write_text;
use crate::options::game_option::GameOption;
use crate::types::key_state::KeyState;
use crate::wasm_bind;

use super::GameState;

pub struct GameRenderer {
    pub game_info: Rc<RefCell<GameInfo>>,
    pub event_queue: Rc<RefCell<VecDeque<Event>>>,
    pub key_states: Rc<RefCell<KeyState>>,
}

impl GameRenderer {
    pub fn empty_render() {
        let renderer = Self::new();

        let game_info = renderer.game_info.borrow();
        let board = game_info.board.clone();

        wasm_bind::render_board(
            board.unfold(),
            board.board_width,
            board.board_height,
            board.column_count,
            board.row_count,
            board.hidden_row_count,
        );

        let next = game_info.bag.iter().map(|e| e.block.into()).collect();
        wasm_bind::render_next(next, 120, 520, 6, 26);

        wasm_bind::render_hold(game_info.hold.map(|e| e.block.into()), 120, 120, 6, 6);
    }

    pub fn new() -> Self {
        Self::with_option(Default::default())
    }

    pub fn with_option(option: GameOption) -> Self {
        let game_info = GameInfo::with_option(option);

        let game_info = Rc::new(RefCell::new(game_info));

        let event_queue = Rc::new(RefCell::new(VecDeque::new()));

        let key_states = Rc::new(RefCell::new(KeyState::new()));

        Self {
            game_info,
            event_queue,
            key_states,
        }
    }

    pub fn playing(&self) -> bool {
        self.game_info.borrow_mut().game_state == GameState::PLAYING
    }

    pub fn start_game(&self) -> Option<()> {
        if self.playing() {
            return None;
        }

        /* FIXME? */
        self.game_info.borrow_mut().init_game()?;

        self.game_info.borrow_mut().game_state = GameState::PLAYING;
        self.game_info.borrow_mut().start_time.set_time(Date::now());
        self.game_info.borrow_mut().lose = false;

        log::info!("GAME START");

        let _game_info = Rc::clone(&self.game_info);
        let game_info = Rc::clone(&_game_info);
        let event_queue = Rc::clone(&self.event_queue);
        let _key_states = Rc::clone(&self.key_states);

        let mut former_lock_delay_count: u8 = 0;

        // gravity_drop - 중력 스레드
        spawn_local(async move {
            // 마지막으로 중력 처리한 시점
            let mut last_tick_point = instant::Instant::now();

            let mut left_keydown_time: Option<instant::Instant> = None;
            let mut right_keydown_time: Option<instant::Instant> = None;

            let mut future_list = IntervalStream::new(GRAVITY_DROP_INTERVAL).map(move |_| {
                // 락 딜레이 처리
                if former_lock_delay_count != game_info.borrow().lock_delay_count {
                    if game_info.borrow().lock_delay_count < 8 {
                        last_tick_point = instant::Instant::now();
                    }
                    former_lock_delay_count = game_info.borrow().lock_delay_count;
                }

                // 현재 실행시간 기록
                game_info.borrow_mut().running_time = {
                    let elapsed_time = Date::now() - game_info.borrow().start_time.get_time();
                    elapsed_time as u128
                };

                let duration = last_tick_point.elapsed();

                // gravity_drop이 발생하지 않은 시점에서 경과된 시간.
                let elapsed_time = duration.as_millis();

                // 여기서 딜레이 커스텀하면 될듯
                let delay = game_info.borrow().gravity_drop_interval as u128
                    + (game_info.borrow().lock_delay as u128);

                // 지정된 딜레이만큼 지났다면 다시 초기화하고 gravity_drop 한칸 수행
                if elapsed_time >= delay {
                    last_tick_point = instant::Instant::now();
                    game_info.borrow_mut().gravity_drop();
                }

                // TODO: das에 arr 적용 필요
                // 일단은 전부 arr 0으로 가정해서 구현했는데,
                // arr 0가 아닐때는 move_end가 아니라 arr 시간이 지날때마다 한칸씩 이동하게 해야할듯

                // 키 이벤트 핸들링
                if !event_queue.borrow().is_empty() {
                    let event = event_queue.borrow_mut().pop_front().unwrap();

                    match event {
                        Event::LeftMove => {
                            game_info.borrow_mut().left_move();
                            left_keydown_time = Some(instant::Instant::now());
                        }
                        Event::LeftMoveStop => {
                            left_keydown_time = None;
                        }
                        Event::RightMove => {
                            game_info.borrow_mut().right_move();
                            right_keydown_time = Some(instant::Instant::now());
                        }
                        Event::RightMoveStop => {
                            right_keydown_time = None;
                        }
                        Event::LeftRotate => {
                            game_info.borrow_mut().left_rotate();
                        }
                        Event::RightRotate => {
                            game_info.borrow_mut().right_rotate();
                        }
                        Event::SoftDrop => {
                            game_info.borrow_mut().soft_drop();
                        }
                        Event::HardDrop => {
                            game_info.borrow_mut().hard_drop();
                        }
                        Event::Hold => {
                            game_info.borrow_mut().hold();
                        }
                        Event::DoubleRotate => {
                            game_info.borrow_mut().double_rotate();
                        }
                    }
                }

                // left das 처리
                if let Some(keydown_time) = left_keydown_time {
                    let elapsed_time = keydown_time.elapsed().as_millis();

                    if elapsed_time >= game_info.borrow().das as u128 {
                        game_info.borrow_mut().left_move_end();
                        left_keydown_time = None;
                    }
                }

                // right das 처리
                if let Some(keydown_time) = right_keydown_time {
                    let elapsed_time = keydown_time.elapsed().as_millis();

                    if elapsed_time >= game_info.borrow().das as u128 {
                        game_info.borrow_mut().right_move_end();
                        right_keydown_time = None;
                    }
                }
            });

            let game_info = _game_info;
            loop {
                if game_info.borrow_mut().game_state == GameState::PLAYING {
                    let next = future_list.next();
                    next.await;
                } else {
                    break;
                }
            }
        });

        // 렌더링 스레드
        let _game_info = Rc::clone(&self.game_info);
        let event_queue = Rc::clone(&self.event_queue);

        spawn_local(async move {
            let f = Rc::new(RefCell::new(None));
            let g = f.clone();

            *g.borrow_mut() = Some(Closure::new(move || {
                if let Some(event) = event_queue.borrow_mut().pop_front() {
                    match event {
                        Event::LeftMove => {
                            _game_info.borrow_mut().left_move();
                        }
                        Event::LeftMoveStop => {}
                        Event::RightMove => {
                            _game_info.borrow_mut().right_move();
                        }
                        Event::RightMoveStop => {}
                        Event::LeftRotate => {
                            _game_info.borrow_mut().left_rotate();
                        }
                        Event::RightRotate => {
                            _game_info.borrow_mut().right_rotate();
                        }
                        Event::HardDrop => {
                            _game_info.borrow_mut().hard_drop();
                        }
                        Event::Hold => {
                            _game_info.borrow_mut().hold();
                        }
                        Event::SoftDrop => {
                            _game_info.borrow_mut().soft_drop();
                        }
                        Event::DoubleRotate => {}
                    }
                }

                event_queue.borrow_mut().clear();

                let game_info = _game_info.borrow_mut();

                if game_info.game_state == GameState::GAMEOVER {
                    // Drop our handle to this closure so that it will get cleaned
                    // up once we return.
                    let _ = f.borrow_mut().take();
                    return;
                }

                let board = match game_info.current_block {
                    Some(current_block) => {
                        let mut board = game_info.board.clone();
                        board.write_current_block(current_block.cells, game_info.current_position);

                        let ghost_position = game_info.get_hard_drop_position().unwrap();
                        board.write_current_block(
                            current_block.clone().to_ghost().cells,
                            ghost_position,
                        );

                        board
                    }
                    None => game_info.board.clone(),
                };

                // 렌더링 수행
                {
                    wasm_bind::render_board(
                        board.unfold(),
                        board.board_width,
                        board.board_height,
                        board.column_count,
                        board.row_count,
                        board.hidden_row_count,
                    );

                    let next = game_info.bag.iter().map(|e| e.block.into()).collect();
                    wasm_bind::render_next(next, 120, 520, 6, 26);

                    wasm_bind::render_hold(game_info.hold.map(|e| e.block.into()), 120, 120, 6, 6);
                    wasm_bind::render_garbage_gauge(game_info.garbage_gauge_count);

                    write_text(
                        "time",
                        format!("{:.2}", game_info.running_time as f64 / 1000.0f64),
                    );
                    write_text("score", game_info.record.score.to_string());
                    write_text("pc", game_info.record.perfect_clear_count.to_string());
                    write_text("quad", game_info.record.quad_count.to_string());
                    write_text(
                        "lineclearcount",
                        format!("{}", game_info.record.line_clear_count),
                    );

                    if let Some(back2back) = game_info.back2back {
                        if back2back != 0 {
                            write_text("back2back", format!("Back2Back {}", back2back));
                        }
                    } else {
                        write_text("back2back", SPECIAL_SPACE.into());
                    }

                    if let Some(combo) = game_info.combo {
                        if combo > 0 {
                            write_text("combo", format!("Combo {}", combo));
                        }
                    } else {
                        write_text("combo", SPECIAL_SPACE.into());
                    }

                    if let Some(message) = game_info.message.clone() {
                        write_text("message", message);
                    } else {
                        write_text("message", SPECIAL_SPACE.into());
                    }
                }

                request_animation_frame(f.borrow().as_ref().unwrap());
            }));

            request_animation_frame(g.borrow().as_ref().unwrap());
        });

        Some(())
    }

    pub fn end_game(&self) -> Option<()> {
        self.game_info.borrow_mut().game_state = GameState::GAMEOVER;

        Some(())
    }

    pub fn init_running_time(&self) -> Option<()> {
        let mut game_info = self.game_info.borrow_mut();
        game_info.running_time = 0;
        Some(())
    }
}
