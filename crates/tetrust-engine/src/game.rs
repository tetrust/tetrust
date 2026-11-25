use std::collections::VecDeque;

use crate::board::{Board, VISIBLE_HEIGHT, WIDTH};
use crate::config::Config;
use crate::event::{
    Action, Event, EventError, EventKind, LockResult, Millis, Outcome, SpinKind, TopOutReason,
};
use crate::kicks::kicks;
use crate::piece::{ActivePiece, PieceKind, Rotation};
use crate::rng::{Rng, SevenBag};
use crate::rules;

/// 같은 시각에 만료되는 타이머끼리의 순서. 값이 작을수록 먼저.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Timer {
    Lock,
    Gravity,
    Shift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PendingGarbage {
    lines: u32,
    ready_at: Millis,
}

/// 한 플레이어의 게임 상태.
#[derive(Debug, Clone)]
pub struct Game {
    config: Config,
    clock: Millis,
    last_event: Option<(Millis, u64)>,

    board: Board,
    active: Option<ActivePiece>,
    hold: Option<PieceKind>,
    hold_used: bool,
    queue: VecDeque<PieceKind>,
    bag: SevenBag,
    garbage_rng: Rng,

    // 타이머. None이면 꺼져 있다.
    gravity_at: Option<Millis>,
    lock_at: Option<Millis>,
    shift_at: Option<Millis>,

    lock_resets: u32,
    lowest_y: i32,
    /// 마지막으로 성공한 동작이 회전이었으면 사용한 킥 번호.
    last_rotation_kick: Option<usize>,

    left_held: bool,
    right_held: bool,
    soft_drop_held: bool,
    /// 현재 자동 이동 방향 (-1, +1).
    shift_dir: Option<i32>,
    /// DAS가 다 차서 자동 이동 중인지.
    das_charged: bool,

    combo: Option<u32>,
    back_to_back: Option<u32>,
    incoming: VecDeque<PendingGarbage>,

    over: Option<TopOutReason>,
    outcomes: Vec<Outcome>,
}

impl Game {
    pub fn new(config: Config, seed: u64) -> Self {
        assert!(config.gravity > 0, "gravity must be at least 1ms");
        let mut game = Self {
            config,
            clock: 0,
            last_event: None,
            board: Board::default(),
            active: None,
            hold: None,
            hold_used: false,
            queue: VecDeque::new(),
            bag: SevenBag::new(seed),
            garbage_rng: Rng::new(seed ^ 0x6A09_E667_F3BC_C908),
            gravity_at: None,
            lock_at: None,
            shift_at: None,
            lock_resets: 0,
            lowest_y: 0,
            last_rotation_kick: None,
            left_held: false,
            right_held: false,
            soft_drop_held: false,
            shift_dir: None,
            das_charged: false,
            combo: None,
            back_to_back: None,
            incoming: VecDeque::new(),
            over: None,
            outcomes: Vec::new(),
        };
        game.spawn_next();
        game
    }

    /// 이벤트 로그 전체를 처음부터 재생한다. 서버 검증과 리플레이가 쓰는 경로다.
    pub fn replay(config: Config, seed: u64, events: &[Event]) -> Result<Self, EventError> {
        let mut game = Self::new(config, seed);
        for event in events {
            game.handle(*event)?;
        }
        Ok(game)
    }

    // ---- 조회 ----

    pub fn clock(&self) -> Millis {
        self.clock
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn active(&self) -> Option<ActivePiece> {
        self.active
    }

    /// 하드 드롭하면 놓일 위치.
    pub fn ghost(&self) -> Option<ActivePiece> {
        self.active.map(|p| self.drop_position(p))
    }

    pub fn hold_piece(&self) -> Option<PieceKind> {
        self.hold
    }

    pub fn next_pieces(&self) -> impl Iterator<Item = PieceKind> + '_ {
        self.queue.iter().copied().take(self.config.next_previews)
    }

    /// 아직 보드에 들어가지 않은 가비지 줄 수 (게이지 표시용).
    pub fn pending_garbage(&self) -> u32 {
        self.incoming.iter().map(|g| g.lines).sum()
    }

    pub fn is_over(&self) -> Option<TopOutReason> {
        self.over
    }

    /// 지난 호출 이후 쌓인 결과를 꺼낸다.
    pub fn drain_outcomes(&mut self) -> Vec<Outcome> {
        std::mem::take(&mut self.outcomes)
    }

    /// 테스트·연습 모드용: 보드와 현재 조각을 직접 지정한다.
    #[doc(hidden)]
    pub fn set_state_for_test(&mut self, board: Board, active: ActivePiece) {
        self.board = board;
        self.active = Some(active);
        self.gravity_at = None;
        self.lock_at = None;
        self.lock_resets = 0;
        self.lowest_y = active.y;
        self.last_rotation_kick = None;
        self.after_change();
    }

    // ---- 시간 진행 ----

    /// 게임 시계를 `time`까지 진행한다. `time` 이하에 만료되는 타이머를 모두 처리한다.
    ///
    /// 렌더 루프가 매 프레임 불러도 되고 안 불러도 된다. 결과는 호출 빈도와 무관하다.
    pub fn advance_to(&mut self, time: Millis) {
        while self.over.is_none() {
            let Some((at, timer)) = self.next_timer() else {
                break;
            };
            if at > time {
                break;
            }
            self.clock = self.clock.max(at);
            self.fire(timer, at);
        }
        self.clock = self.clock.max(time);
    }

    /// 입력 로그 한 줄을 처리한다. 같은 시각에 만료되는 타이머가 먼저 처리된다.
    pub fn handle(&mut self, event: Event) -> Result<(), EventError> {
        if let Some(last) = self.last_event {
            if (event.time, event.seq) <= last {
                return Err(EventError::OutOfOrder);
            }
        }
        if event.time < self.clock {
            return Err(EventError::OutOfOrder);
        }
        self.last_event = Some((event.time, event.seq));

        self.advance_to(event.time);
        if self.over.is_some() {
            return Ok(());
        }
        match event.kind {
            EventKind::Press(action) => self.press(action),
            EventKind::Release(action) => self.release(action),
            EventKind::Garbage { lines } => self.incoming.push_back(PendingGarbage {
                lines,
                ready_at: event.time + self.config.garbage_delay,
            }),
        }
        // 입력이 지금 시각으로 만든 타이머(예: 리셋 한도 초과 후 즉시 고정)를 처리한다.
        self.advance_to(event.time);
        Ok(())
    }

    fn next_timer(&self) -> Option<(Millis, Timer)> {
        [
            (self.lock_at, Timer::Lock),
            (self.gravity_at, Timer::Gravity),
            (self.shift_at, Timer::Shift),
        ]
        .into_iter()
        .filter_map(|(at, t)| at.map(|at| (at, t)))
        .min()
    }

    fn fire(&mut self, timer: Timer, at: Millis) {
        match timer {
            Timer::Lock => {
                self.lock_at = None;
                self.lock_piece();
            }
            Timer::Gravity => {
                self.gravity_at = None;
                if self.step_down() {
                    self.gravity_at = Some(at + self.gravity_interval());
                }
                self.after_change();
            }
            Timer::Shift => {
                self.shift_at = None;
                self.das_charged = true;
                if let Some(dir) = self.shift_dir {
                    if self.config.arr == 0 {
                        self.shift_to_wall(dir);
                    } else {
                        self.try_shift(dir);
                        self.shift_at = Some(at + self.config.arr);
                    }
                }
                self.after_change();
            }
        }
    }

    // ---- 입력 ----

    fn press(&mut self, action: Action) {
        match action {
            Action::Left => {
                self.left_held = true;
                self.start_shift(-1);
            }
            Action::Right => {
                self.right_held = true;
                self.start_shift(1);
            }
            Action::SoftDrop => {
                self.soft_drop_held = true;
                if self.config.soft_drop > 0 {
                    self.step_down();
                    self.gravity_at = Some(self.clock + self.config.soft_drop);
                }
            }
            Action::HardDrop => {
                if let Some(piece) = self.active {
                    let dropped = self.drop_position(piece);
                    if dropped.y != piece.y {
                        self.last_rotation_kick = None;
                    }
                    self.active = Some(dropped);
                    self.lock_piece();
                    return;
                }
            }
            Action::RotateCw => self.try_rotate(|r| r.cw()),
            Action::RotateCcw => self.try_rotate(|r| r.ccw()),
            Action::Rotate180 => self.try_rotate(|r| r.flip()),
            Action::Hold => self.do_hold(),
        }
        self.after_change();
    }

    fn release(&mut self, action: Action) {
        match action {
            Action::Left | Action::Right => {
                let dir = if action == Action::Left { -1 } else { 1 };
                if action == Action::Left {
                    self.left_held = false;
                } else {
                    self.right_held = false;
                }
                if self.shift_dir == Some(dir) {
                    let other_held = if dir == -1 {
                        self.right_held
                    } else {
                        self.left_held
                    };
                    if other_held {
                        // 반대쪽 키가 아직 눌려 있으면 그쪽으로 DAS를 새로 시작한다.
                        self.shift_dir = Some(-dir);
                        self.das_charged = false;
                        self.shift_at = Some(self.clock + self.config.das);
                    } else {
                        self.shift_dir = None;
                        self.das_charged = false;
                        self.shift_at = None;
                    }
                }
            }
            Action::SoftDrop => {
                self.soft_drop_held = false;
                if self.gravity_at.is_some() {
                    self.gravity_at = Some(self.clock + self.config.gravity);
                }
            }
            _ => {}
        }
        self.after_change();
    }

    fn start_shift(&mut self, dir: i32) {
        self.shift_dir = Some(dir);
        self.das_charged = false;
        self.try_shift(dir);
        self.shift_at = Some(self.clock + self.config.das);
    }

    // ---- 이동 ----

    fn gravity_interval(&self) -> Millis {
        if self.soft_drop_held && self.config.soft_drop > 0 {
            self.config.soft_drop.min(self.config.gravity)
        } else {
            self.config.gravity
        }
    }

    fn drop_position(&self, mut piece: ActivePiece) -> ActivePiece {
        while self.board.fits(&piece.shifted(0, -1)) {
            piece = piece.shifted(0, -1);
        }
        piece
    }

    fn is_grounded(&self, piece: &ActivePiece) -> bool {
        !self.board.fits(&piece.shifted(0, -1))
    }

    /// 플레이어 동작(이동·회전)이 성공했을 때. 바닥에 닿아 있었다면 락 딜레이를 다시 채운다.
    fn on_player_move(&mut self) {
        if self.lock_at.is_some() && self.lock_resets < self.config.lock_reset_limit {
            self.lock_resets += 1;
            self.lock_at = Some(self.clock + self.config.lock_delay);
        }
    }

    fn try_shift(&mut self, dx: i32) -> bool {
        let Some(piece) = self.active else {
            return false;
        };
        let moved = piece.shifted(dx, 0);
        if !self.board.fits(&moved) {
            return false;
        }
        self.active = Some(moved);
        self.last_rotation_kick = None;
        self.on_player_move();
        true
    }

    fn shift_to_wall(&mut self, dir: i32) {
        while self.try_shift(dir) {}
    }

    fn step_down(&mut self) -> bool {
        let Some(piece) = self.active else {
            return false;
        };
        let moved = piece.shifted(0, -1);
        if !self.board.fits(&moved) {
            return false;
        }
        self.active = Some(moved);
        self.last_rotation_kick = None;
        if moved.y < self.lowest_y {
            self.lowest_y = moved.y;
            self.lock_resets = 0;
        }
        true
    }

    fn try_rotate(&mut self, target: impl Fn(Rotation) -> Rotation) {
        let Some(piece) = self.active else {
            return;
        };
        let to = target(piece.rotation);
        for (i, &(dx, dy)) in kicks(piece.kind, piece.rotation, to).iter().enumerate() {
            let candidate = ActivePiece {
                rotation: to,
                x: piece.x + dx,
                y: piece.y + dy,
                ..piece
            };
            if self.board.fits(&candidate) {
                self.active = Some(candidate);
                self.last_rotation_kick = Some(i);
                if candidate.y < self.lowest_y {
                    self.lowest_y = candidate.y;
                    self.lock_resets = 0;
                }
                self.on_player_move();
                return;
            }
        }
    }

    fn do_hold(&mut self) {
        if self.hold_used {
            return;
        }
        let Some(piece) = self.active else {
            return;
        };
        self.hold_used = true;
        match self.hold.replace(piece.kind) {
            Some(kind) => self.spawn(kind),
            None => self.spawn_next(),
        }
    }

    /// 상태가 바뀐 뒤 타이머를 정리한다.
    fn after_change(&mut self) {
        let Some(_) = self.active else {
            return;
        };
        // ARR 0으로 자동 이동 중이면 조각이 바뀌거나 회전해도 계속 벽에 붙는다.
        if self.das_charged && self.config.arr == 0 {
            if let Some(dir) = self.shift_dir {
                self.shift_to_wall(dir);
            }
        }
        if self.soft_drop_held && self.config.soft_drop == 0 {
            while self.step_down() {}
        }
        let piece = self.active.expect("checked above");
        if self.is_grounded(&piece) {
            self.gravity_at = None;
            if self.lock_at.is_none() {
                self.lock_at = Some(if self.lock_resets >= self.config.lock_reset_limit {
                    self.clock
                } else {
                    self.clock + self.config.lock_delay
                });
            }
        } else {
            self.lock_at = None;
            if self.gravity_at.is_none() {
                self.gravity_at = Some(self.clock + self.gravity_interval());
            }
        }
    }

    // ---- 스폰과 고정 ----

    fn fill_queue(&mut self) {
        while self.queue.len() < self.config.next_previews.max(1) + 1 {
            self.queue.extend(self.bag.next_bag());
        }
    }

    fn spawn_next(&mut self) {
        self.fill_queue();
        let kind = self.queue.pop_front().expect("queue filled");
        self.fill_queue();
        self.spawn(kind);
    }

    fn spawn(&mut self, kind: PieceKind) {
        let (x, y) = match kind {
            PieceKind::I => (3, VISIBLE_HEIGHT - 2),
            PieceKind::O => (4, VISIBLE_HEIGHT),
            _ => (3, VISIBLE_HEIGHT - 1),
        };
        let piece = ActivePiece {
            kind,
            rotation: Rotation(0),
            x,
            y,
        };
        self.gravity_at = None;
        self.lock_at = None;
        self.lock_resets = 0;
        self.lowest_y = y;
        self.last_rotation_kick = None;
        if !self.board.fits(&piece) {
            self.active = None;
            self.top_out(TopOutReason::BlockOut);
            return;
        }
        self.active = Some(piece);
        self.after_change();
    }

    fn top_out(&mut self, reason: TopOutReason) {
        self.over = Some(reason);
        self.gravity_at = None;
        self.lock_at = None;
        self.shift_at = None;
        self.outcomes.push(Outcome::TopOut {
            time: self.clock,
            reason,
        });
    }

    /// T 조각의 3코너 규칙. 고정 직전 보드 기준.
    fn detect_spin(&self, piece: &ActivePiece) -> SpinKind {
        let Some(kick) = self.last_rotation_kick else {
            return SpinKind::None;
        };
        if piece.kind != PieceKind::T {
            return SpinKind::None;
        }
        let corner = |cx: i32, cy: i32| self.board.is_blocked(piece.x + cx, piece.y + cy);
        let filled = [(0, 0), (2, 0), (0, 2), (2, 2)]
            .iter()
            .filter(|&&(cx, cy)| corner(cx, cy))
            .count();
        if filled < 3 {
            return SpinKind::None;
        }
        // 튀어나온 쪽(앞쪽) 두 코너.
        let front = match piece.rotation.0 {
            0 => [(0, 2), (2, 2)],
            1 => [(2, 2), (2, 0)],
            2 => [(0, 0), (2, 0)],
            _ => [(0, 0), (0, 2)],
        };
        // 마지막(5번째) 킥으로 들어간 경우는 앞 코너와 상관없이 정식 T-스핀으로 친다.
        if front.iter().all(|&(cx, cy)| corner(cx, cy)) || kick == 4 {
            SpinKind::Full
        } else {
            SpinKind::Mini
        }
    }

    fn lock_piece(&mut self) {
        let Some(piece) = self.active.take() else {
            return;
        };
        let time = self.clock;
        let spin = self.detect_spin(&piece);
        self.board.place(&piece);
        if piece.cells().iter().all(|&(_, y)| y >= VISIBLE_HEIGHT) {
            self.top_out(TopOutReason::LockOut);
            return;
        }

        let lines = self.board.clear_lines();
        let perfect_clear = lines > 0 && self.board.is_empty();
        if lines > 0 {
            self.combo = Some(self.combo.map_or(0, |c| c + 1));
            self.back_to_back = if rules::is_difficult(lines, spin) {
                Some(self.back_to_back.map_or(0, |b| b + 1))
            } else {
                None
            };
        } else {
            self.combo = None;
        }
        let attack = if lines > 0 {
            rules::attack(lines, spin, self.combo, self.back_to_back, perfect_clear)
        } else {
            0
        };
        self.outcomes.push(Outcome::Locked {
            time,
            result: LockResult {
                piece: piece.kind,
                lines,
                spin,
                combo: self.combo,
                back_to_back: self.back_to_back,
                perfect_clear,
                attack,
            },
        });

        if attack > 0 {
            let remaining = self.cancel_incoming(attack);
            if remaining > 0 {
                self.outcomes.push(Outcome::AttackSent {
                    time,
                    lines: remaining,
                });
            }
        } else if lines == 0 && !self.apply_ready_garbage(time) {
            self.top_out(TopOutReason::GarbageOut);
            return;
        }

        self.hold_used = false;
        self.spawn_next();
    }

    /// 들어오는 가비지를 먼저 온 것부터 상쇄하고 남은 공격량을 돌려준다.
    fn cancel_incoming(&mut self, mut attack: u32) -> u32 {
        while attack > 0 {
            let Some(front) = self.incoming.front_mut() else {
                break;
            };
            let used = front.lines.min(attack);
            front.lines -= used;
            attack -= used;
            if front.lines == 0 {
                self.incoming.pop_front();
            }
        }
        attack
    }

    /// 준비된(ready_at <= time) 가비지를 한도까지 넣는다. 블록이 밀려나면 false.
    fn apply_ready_garbage(&mut self, time: Millis) -> bool {
        let mut budget = self.config.garbage_cap;
        while budget > 0 {
            let Some(front) = self.incoming.front_mut() else {
                break;
            };
            if front.ready_at > time {
                break;
            }
            let lines = front.lines.min(budget);
            front.lines -= lines;
            budget -= lines;
            if front.lines == 0 {
                self.incoming.pop_front();
            }
            let hole = self.garbage_rng.below(WIDTH as u64) as i32;
            self.outcomes
                .push(Outcome::GarbageApplied { time, lines, hole });
            if !self.board.insert_garbage(lines, hole) {
                return false;
            }
        }
        true
    }
}
