use std::collections::BTreeSet;

use tetrust_engine::*;

fn press(time: Millis, seq: u64, action: Action) -> Event {
    Event::new(time, seq, EventKind::Press(action))
}

fn release(time: Millis, seq: u64, action: Action) -> Event {
    Event::new(time, seq, EventKind::Release(action))
}

fn garbage(time: Millis, seq: u64, lines: u32) -> Event {
    Event::new(time, seq, EventKind::Garbage { lines })
}

fn piece(kind: PieceKind, rotation: u8, x: i32, y: i32) -> ActivePiece {
    ActivePiece {
        kind,
        rotation: Rotation(rotation),
        x,
        y,
    }
}

/// 중력이 사실상 꺼진 설정. 이동·회전만 보고 싶을 때 쓴다.
fn still() -> Config {
    Config {
        gravity: 1_000_000,
        ..Config::default()
    }
}

fn game_with(config: Config, board: Board, active: ActivePiece) -> Game {
    let mut game = Game::new(config, 1);
    game.set_state_for_test(board, active);
    game
}

fn locks(outcomes: &[Outcome]) -> Vec<LockResult> {
    outcomes
        .iter()
        .filter_map(|o| match o {
            Outcome::Locked { result, .. } => Some(*result),
            _ => None,
        })
        .collect()
}

fn filled_columns(board: &Board, y: usize) -> Vec<usize> {
    board.rows()[y]
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.is_empty())
        .map(|(x, _)| x)
        .collect()
}

// ---- 랜덤 ----

#[test]
fn seven_bag_deals_each_piece_once_per_bag() {
    let config = Config {
        next_previews: 13,
        ..Config::default()
    };
    let game = Game::new(config, 42);
    let mut pieces = vec![game.active().unwrap().kind];
    pieces.extend(game.next_pieces());
    assert_eq!(pieces.len(), 14);
    for bag in pieces.chunks(7) {
        let set: BTreeSet<_> = bag.iter().copied().collect();
        assert_eq!(set.len(), 7, "bag {bag:?} is not a permutation");
    }
}

#[test]
fn same_seed_same_pieces_different_seed_different_pieces() {
    let config = Config {
        next_previews: 13,
        ..Config::default()
    };
    let seq = |seed| {
        let g = Game::new(config.clone(), seed);
        g.next_pieces().collect::<Vec<_>>()
    };
    assert_eq!(seq(7), seq(7));
    assert_ne!(seq(7), seq(8));
}

// ---- SRS ----

#[test]
fn t_kicks_off_left_wall() {
    // R 상태 T가 왼쪽 벽에 붙어 있다. 0으로 돌리면 제자리는 벽에 걸리고 2번째 킥 (+1, 0)으로 들어간다.
    let mut game = game_with(still(), Board::default(), piece(PieceKind::T, 1, -1, 5));
    game.handle(press(1, 0, Action::RotateCcw)).unwrap();
    assert_eq!(game.active(), Some(piece(PieceKind::T, 0, 0, 5)));
}

#[test]
fn i_kicks_off_right_wall() {
    // 세로 I가 맨 오른쪽 열에 있다. R -> 2는 제자리가 벽 밖이라 (-1, 0)으로 들어간다.
    let mut game = game_with(still(), Board::default(), piece(PieceKind::I, 1, 7, 5));
    game.handle(press(1, 0, Action::RotateCw)).unwrap();
    assert_eq!(game.active(), Some(piece(PieceKind::I, 2, 6, 5)));
}

#[test]
fn o_does_not_move_when_rotated() {
    let mut game = game_with(still(), Board::default(), piece(PieceKind::O, 0, 4, 5));
    game.handle(press(1, 0, Action::RotateCw)).unwrap();
    let o = game.active().unwrap();
    assert_eq!((o.x, o.y), (4, 5));
}

// ---- T-스핀과 공격 ----

#[test]
fn t_spin_double_sends_four_lines() {
    let board = Board::from_rows(&[
        "...X......", //
        "XXX...XXXX",
        "XXXX.XXXXX",
    ]);
    let mut game = game_with(still(), board, piece(PieceKind::T, 1, 3, 0));
    game.handle(press(1, 0, Action::RotateCw)).unwrap();
    game.handle(press(2, 1, Action::HardDrop)).unwrap();

    let outcomes = game.drain_outcomes();
    let lock = locks(&outcomes)[0];
    assert_eq!((lock.lines, lock.spin, lock.attack), (2, SpinKind::Full, 4));
    assert!(outcomes.contains(&Outcome::AttackSent { time: 2, lines: 4 }));
}

#[test]
fn t_spin_mini_when_front_corners_are_open() {
    let board = Board::from_rows(&[
        "...X.X....", //
        "..........",
        "...X......",
    ]);
    let mut game = game_with(still(), board, piece(PieceKind::T, 1, 3, 0));
    game.handle(press(1, 0, Action::RotateCw)).unwrap();
    game.handle(press(2, 1, Action::HardDrop)).unwrap();
    assert_eq!(locks(&game.drain_outcomes())[0].spin, SpinKind::Mini);
}

#[test]
fn moving_after_rotation_cancels_t_spin() {
    let board = Board::from_rows(&[
        "...X......", //
        "XXX...XXXX",
        "XXXX.XXXXX",
    ]);
    let mut game = game_with(still(), board, piece(PieceKind::T, 1, 3, 0));
    game.handle(press(1, 0, Action::RotateCw)).unwrap();
    // 옆으로 못 움직이면 마지막 동작은 여전히 회전이다.
    game.handle(press(2, 1, Action::Left)).unwrap();
    game.handle(press(3, 2, Action::HardDrop)).unwrap();
    assert_eq!(locks(&game.drain_outcomes())[0].spin, SpinKind::Full);
}

#[test]
fn attack_table() {
    assert_eq!(attack(1, SpinKind::None, Some(0), None, false), 0);
    assert_eq!(attack(4, SpinKind::None, Some(0), Some(0), false), 4);
    // B2B 테트리스
    assert_eq!(attack(4, SpinKind::None, Some(0), Some(1), false), 5);
    // 퍼펙트 클리어 테트리스
    assert_eq!(attack(4, SpinKind::None, Some(0), Some(0), true), 14);
    // 3콤보 더블: 1 + 콤보 1
    assert_eq!(attack(2, SpinKind::None, Some(3), None, false), 2);
    assert_eq!(attack(3, SpinKind::Full, Some(0), Some(0), false), 6);
    assert_eq!(attack(2, SpinKind::Mini, Some(0), Some(0), false), 1);
    assert_eq!(attack(0, SpinKind::Full, None, None, false), 0);
}

// ---- 중력과 소프트 드롭 ----

#[test]
fn gravity_drops_on_the_deadline() {
    let mut game = game_with(
        Config::default(),
        Board::default(),
        piece(PieceKind::T, 0, 3, 15),
    );
    game.advance_to(999);
    assert_eq!(game.active().unwrap().y, 15);
    game.advance_to(1000);
    assert_eq!(game.active().unwrap().y, 14);
    game.advance_to(2000);
    assert_eq!(game.active().unwrap().y, 13);
}

#[test]
fn instant_soft_drop_reaches_floor_without_locking() {
    let config = Config {
        soft_drop: 0,
        ..still()
    };
    let mut game = game_with(config, Board::default(), piece(PieceKind::T, 0, 3, 15));
    game.handle(press(10, 0, Action::SoftDrop)).unwrap();
    assert_eq!(game.active().unwrap().y, -1);
    assert!(game.board().is_empty());
}

// ---- DAS / ARR ----

fn handling(das: Millis, arr: Millis) -> Config {
    Config {
        das,
        arr,
        ..still()
    }
}

#[test]
fn das_then_arr_on_half_open_deadlines() {
    let mut game = game_with(
        handling(100, 20),
        Board::default(),
        piece(PieceKind::T, 0, 3, 15),
    );
    game.handle(press(10, 0, Action::Left)).unwrap();
    assert_eq!(game.active().unwrap().x, 2, "first tap moves immediately");
    game.advance_to(109);
    assert_eq!(game.active().unwrap().x, 2);
    game.advance_to(110);
    assert_eq!(
        game.active().unwrap().x,
        1,
        "DAS fires exactly at press + das"
    );
    game.advance_to(129);
    assert_eq!(game.active().unwrap().x, 1);
    game.advance_to(130);
    assert_eq!(game.active().unwrap().x, 0);
    game.advance_to(500);
    assert_eq!(game.active().unwrap().x, 0, "stops at the wall");
}

#[test]
fn zero_arr_slides_to_wall_at_das() {
    let mut game = game_with(
        handling(100, 0),
        Board::default(),
        piece(PieceKind::T, 0, 3, 15),
    );
    game.handle(press(10, 0, Action::Right)).unwrap();
    game.advance_to(109);
    assert_eq!(game.active().unwrap().x, 4);
    game.advance_to(110);
    assert_eq!(game.active().unwrap().x, 7);
}

#[test]
fn release_before_das_cancels_auto_shift() {
    let mut game = game_with(
        handling(100, 20),
        Board::default(),
        piece(PieceKind::T, 0, 3, 15),
    );
    game.handle(press(10, 0, Action::Left)).unwrap();
    game.handle(release(50, 1, Action::Left)).unwrap();
    game.advance_to(1000);
    assert_eq!(game.active().unwrap().x, 2);
}

#[test]
fn last_pressed_direction_wins_and_restarts_das() {
    let mut game = game_with(
        handling(100, 20),
        Board::default(),
        piece(PieceKind::T, 0, 3, 15),
    );
    game.handle(press(10, 0, Action::Left)).unwrap();
    game.handle(press(60, 1, Action::Right)).unwrap();
    assert_eq!(game.active().unwrap().x, 3);
    game.advance_to(159);
    assert_eq!(game.active().unwrap().x, 3, "left DAS no longer applies");
    game.advance_to(160);
    assert_eq!(game.active().unwrap().x, 4);
}

// ---- 락 딜레이 ----

#[test]
fn lock_delay_is_half_open() {
    let mut game = game_with(still(), Board::default(), piece(PieceKind::T, 0, 3, -1));
    game.advance_to(499);
    assert!(game.board().is_empty());
    game.advance_to(500);
    assert_eq!(filled_columns(game.board(), 0), vec![3, 4, 5]);
}

#[test]
fn input_at_the_lock_deadline_is_too_late() {
    let mut game = game_with(still(), Board::default(), piece(PieceKind::T, 0, 3, -1));
    game.handle(press(500, 0, Action::Left)).unwrap();
    // 타이머가 입력보다 먼저: T는 제자리에 고정되고, 왼쪽 입력은 다음 조각이 받는다.
    assert_eq!(filled_columns(game.board(), 0), vec![3, 4, 5]);
}

#[test]
fn input_just_before_deadline_resets_lock_delay() {
    let mut game = game_with(
        handling(100_000, 0),
        Board::default(),
        piece(PieceKind::T, 0, 3, -1),
    );
    game.handle(press(499, 0, Action::Left)).unwrap();
    game.advance_to(998);
    assert!(game.board().is_empty());
    game.advance_to(999);
    assert_eq!(filled_columns(game.board(), 0), vec![2, 3, 4]);
}

#[test]
fn lock_resets_are_limited() {
    let config = Config {
        lock_reset_limit: 2,
        ..handling(100_000, 0)
    };
    let mut game = game_with(config, Board::default(), piece(PieceKind::T, 0, 3, -1));
    game.handle(press(100, 0, Action::Left)).unwrap(); // 600
    game.handle(press(200, 1, Action::Right)).unwrap(); // 700
    game.handle(press(300, 2, Action::Left)).unwrap(); // 한도 초과, 그대로 700
    game.advance_to(699);
    assert!(game.board().is_empty());
    game.advance_to(700);
    assert!(!game.board().is_empty());
}

// ---- 가비지 ----

#[test]
fn garbage_enters_on_a_lock_that_clears_nothing() {
    let mut game = Game::new(Config::default(), 3);
    game.handle(garbage(1, 0, 2)).unwrap();
    assert_eq!(game.pending_garbage(), 2);
    game.handle(press(2, 1, Action::HardDrop)).unwrap();
    assert_eq!(game.pending_garbage(), 0);

    let outcomes = game.drain_outcomes();
    let hole = outcomes
        .iter()
        .find_map(|o| match o {
            Outcome::GarbageApplied { lines: 2, hole, .. } => Some(*hole as usize),
            _ => None,
        })
        .expect("garbage applied");
    for y in 0..2 {
        let row = &game.board().rows()[y];
        assert_eq!(row.iter().filter(|c| **c == Cell::Garbage).count(), 9);
        assert!(row[hole].is_empty());
    }
}

#[test]
fn attack_cancels_incoming_garbage_first() {
    // 맨 위 한 칸을 남겨 퍼펙트 클리어가 되지 않게 한다.
    let board = Board::from_rows(&[
        "X.........",
        "XXXXXXXXX.",
        "XXXXXXXXX.",
        "XXXXXXXXX.",
        "XXXXXXXXX.",
    ]);
    let mut game = game_with(still(), board, piece(PieceKind::I, 1, 7, 10));
    game.handle(garbage(1, 0, 3)).unwrap();
    game.handle(press(2, 1, Action::HardDrop)).unwrap();

    let outcomes = game.drain_outcomes();
    assert_eq!(locks(&outcomes)[0].attack, 4);
    assert!(outcomes.contains(&Outcome::AttackSent { time: 2, lines: 1 }));
    assert_eq!(game.pending_garbage(), 0);
}

#[test]
fn garbage_delay_only_allows_cancelling_until_ready() {
    let config = Config {
        garbage_delay: 500,
        ..Config::default()
    };
    let mut game = Game::new(config, 3);
    game.handle(garbage(0, 0, 2)).unwrap();
    game.handle(press(499, 1, Action::HardDrop)).unwrap();
    assert_eq!(game.pending_garbage(), 2, "not ready before 0 + 500");
    game.handle(press(500, 2, Action::HardDrop)).unwrap();
    assert_eq!(game.pending_garbage(), 0, "ready exactly at 500");
}

// ---- 홀드와 탑아웃 ----

#[test]
fn hold_once_per_piece() {
    let mut game = Game::new(still(), 5);
    let first = game.active().unwrap().kind;
    let second = game.next_pieces().next().unwrap();
    game.handle(press(1, 0, Action::Hold)).unwrap();
    assert_eq!(game.hold_piece(), Some(first));
    assert_eq!(game.active().unwrap().kind, second);
    game.handle(press(2, 1, Action::Hold)).unwrap();
    assert_eq!(
        game.active().unwrap().kind,
        second,
        "second hold is ignored"
    );
}

#[test]
fn blocked_spawn_tops_out() {
    let mut rows = vec![".XXXXXXXXX"; 20];
    rows.extend(["..........", ".........."]);
    let mut game = game_with(
        still(),
        Board::from_rows(&rows),
        piece(PieceKind::O, 0, 4, 0),
    );
    game.handle(press(1, 0, Action::HardDrop)).unwrap();
    assert_eq!(game.is_over(), Some(TopOutReason::BlockOut));
    // 게임이 끝난 뒤의 입력은 무시된다.
    game.handle(press(2, 1, Action::Left)).unwrap();
}

// ---- 순서와 결정성 ----

#[test]
fn events_must_be_strictly_ordered() {
    let mut game = Game::new(still(), 1);
    game.handle(press(10, 1, Action::Left)).unwrap();
    assert_eq!(
        game.handle(release(10, 1, Action::Left)),
        Err(EventError::OutOfOrder)
    );
    assert_eq!(
        game.handle(release(5, 2, Action::Left)),
        Err(EventError::OutOfOrder)
    );
    game.handle(release(10, 2, Action::Left)).unwrap();
}

fn script() -> Vec<Event> {
    let mut events = Vec::new();
    let mut t = 0;
    let mut seq = 0;
    let mut push = |dt: Millis, kind: EventKind| {
        t += dt;
        events.push(Event::new(t, seq, kind));
        seq += 1;
    };
    for i in 0..30u64 {
        push(37, EventKind::Press(Action::Left));
        push(5 + i % 7 * 40, EventKind::Release(Action::Left));
        push(11, EventKind::Press(Action::RotateCw));
        push(0, EventKind::Press(Action::SoftDrop));
        push(90, EventKind::Release(Action::SoftDrop));
        if i % 3 == 0 {
            push(3, EventKind::Garbage { lines: 1 });
        }
        push(23, EventKind::Press(Action::Right));
        push(150 + i % 5 * 30, EventKind::Release(Action::Right));
        push(500 + i % 4 * 300, EventKind::Press(Action::HardDrop));
    }
    events
}

/// 화면 주사율(= advance_to 호출 빈도)이 결과를 바꾸지 않는다.
#[test]
fn refresh_rate_does_not_change_the_result() {
    let config = Config {
        das: 120,
        arr: 30,
        ..Config::default()
    };
    let run = |frame: Option<Millis>| {
        let mut game = Game::new(config.clone(), 99);
        let mut outcomes = Vec::new();
        let mut now = 0;
        for event in script() {
            if let Some(frame) = frame {
                while now + frame < event.time {
                    now += frame;
                    game.advance_to(now);
                }
            }
            game.handle(event).unwrap();
            now = event.time;
        }
        game.advance_to(now + 5000);
        outcomes.extend(game.drain_outcomes());
        (game.board().clone(), game.active(), outcomes)
    };
    let events_only = run(None);
    assert!(!locks(&events_only.2).is_empty());
    assert_eq!(events_only, run(Some(16)), "60Hz");
    assert_eq!(events_only, run(Some(7)), "144Hz");
    assert_eq!(events_only, run(Some(1)), "1000Hz");
}

#[test]
fn replay_reproduces_a_live_game() {
    let events = script();
    let mut live = Game::new(Config::default(), 1234);
    for e in &events {
        live.handle(*e).unwrap();
    }
    let replayed = Game::replay(Config::default(), 1234, &events).unwrap();
    assert_eq!(live.board(), replayed.board());
    assert_eq!(live.active(), replayed.active());
    assert_eq!(live.clock(), replayed.clock());
}
