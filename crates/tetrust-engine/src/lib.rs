//! tetrust 게임 엔진.
//!
//! 웹, 서버, 봇이 같은 코드를 쓰도록 플랫폼 의존성이 없다. 엔진은 결정적이다:
//! 같은 [`Config`], 같은 시드, 같은 이벤트 로그면 어디서 돌리든 같은 결과가 나온다.
//!
//! # 시간 규칙
//!
//! - 게임 시계는 매치 시작부터 센 정수 밀리초([`Millis`]) 하나뿐이다. 화면 주사율은 결과에
//!   영향을 주지 않는다.
//! - 시뮬레이션은 고정 틱이 아니라 이벤트 단위로 진행한다. 다음 입력과 다음 타이머 만료 중
//!   이른 시각으로 바로 넘어간다.
//! - 모든 타이머는 반열린 구간 `[시작, 만료)`로 동작한다. 만료 시각에 정확히 들어온 입력은
//!   이미 늦은 것이다.
//! - 같은 시각의 처리 순서는 `(시각, 종류, seq)`다. 종류는 "만료되는 타이머 → 입력 로그"이고,
//!   타이머끼리는 락 → 중력 → 자동 이동 순이다.

mod board;
mod config;
mod event;
mod game;
mod kicks;
mod piece;
mod rng;
mod rules;

pub use board::{Board, Cell, Row, HEIGHT, VISIBLE_HEIGHT, WIDTH};
pub use config::Config;
pub use event::{
    Action, Event, EventError, EventKind, LockResult, Millis, Outcome, SpinKind, TopOutReason,
};
pub use game::Game;
pub use piece::{ActivePiece, PieceKind, Rotation};
pub use rules::{attack, base_attack, is_difficult};
