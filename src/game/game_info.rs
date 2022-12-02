use std::collections::VecDeque;

use instant::Instant;
use js_sys::Date;
use js_sys::Math::{floor, random};

use crate::game::{
    valid_block, valid_tspin, BagType, BlockShape, Board, Cell, ClearInfo, GameRecord, Point,
    SpinType,
};

use crate::js_bind::write_text::write_text;
use crate::options::game_option::GameOption;
use crate::util::{random, rotate_left, rotate_right, KICK_INDEX_3BY3, KICK_INDEX_I};

use super::{calculate_score, Block};

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    IDLE,
    PLAYING,
    GAMEOVER,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameMode {
    NORMAL,
    SPRINT,
}

#[derive(Debug)]
pub struct GameInfo {
    pub record: GameRecord,

    pub start_time: Date,
    pub running_time: u128, // 실행시간 (밀리초)

    pub game_mode: GameMode,               //게임 모드
    pub game_state: GameState,             //게임 진행중 여부
    pub current_position: Point,           //현재 블럭 좌표
    pub current_block: Option<BlockShape>, //현재 블럭 형태

    pub freezed: bool, //현재 블럭이 보드에 붙었는지?
    pub lose: bool,    //현재 게임 오버 여부

    pub next_count: i32,             // 넥스트 개수
    pub bag: VecDeque<BlockShape>,   // 현재 가방
    pub garbage_queue: VecDeque<u8>, // 쓰레기라인대기열
    pub garbage_gauge_count: u64,    // 쌓인 쓰레기줄 수, 새 block write 시점에 남아있으면 올라옴

    pub board: Board, // 보드

    pub render_interval: u64,       //렌더링 시간간격(밀리초)
    pub gravity_drop_interval: u64, //틱당 시간간격(밀리초)

    pub bag_mode: BagType, //가방 순환 규칙 사용여부 (false면 완전 랜덤. true면 한 묶음에서 랜덤)
    pub block_list: Vec<BlockShape>, //블럭 리스트

    pub hold: Option<BlockShape>, // 홀드한 블럭
    pub hold_used: bool,          // 현재 홀드 사용권을 소모했는지 여부

    pub combo: Option<u32>, // 현재 콤보 (제로콤보는 None, 지웠을 경우 0부터 시작)
    pub back2back: Option<u32>, // 현재 백투백 스택 (제로는 None, 지웠을 경우 0부터 시작)

    pub message: Option<String>, // 렌더링할 메세지

    pub in_spin: SpinType, // 현재 스핀 상태 확인

    pub lock_delay: u32,      // 바닥에 닿을때 고정하기까지의 딜레이. 밀리초 단위.
    pub lock_delay_count: u8, // 하좌우이동, 좌우회전 성공 시 록딜레이 카운트가 올라감. 틱스레드에서 변화를 읽고 start를 초기화. 8이상이면 안올라감

    pub sdf: u32, // SDF: Soft Drop Factor. 소프트 드랍 Key가 눌러졌을 때 자연드랍속도를 몇배 더 빠르게할지 설정
    pub das: u32, // DAS: Delayed Auto Shift의 약자. Key를 Holding하여 Auto Shift가 시작되기까지의 시간, ms단위
    pub arr: u32, // ARR: Auto Repeat Rate: Auto Shift가 활성화되었을 때 이동이 반복되는 사이클타임, ms단위

    pub on_left_move: Option<Instant>,  // left move 클릭한 시작시간
    pub on_right_move: Option<Instant>, // right move 클릭한 시작시간
    pub on_down_move: Option<Instant>,  // down move 클릭한 시작시간
}

impl GameInfo {
    pub fn with_option(option: GameOption) -> Self {
        let column_count = option.column_count;
        let hidden_row_count = 4;
        let row_count = option.row_count + hidden_row_count;
        let board_height = option.board_height;
        let board_width = option.board_width;
        let bag_mode = option.bag_mode;
        let board = Board {
            cells: vec![
                vec![Cell::Empty; column_count as usize];
                row_count as usize + hidden_row_count as usize
            ],
            column_count,
            row_count,
            board_height,
            board_width,
            hidden_row_count,
        };

        let block_list = vec![
            BlockShape::I,
            BlockShape::L,
            BlockShape::J,
            BlockShape::S,
            BlockShape::Z,
            BlockShape::O,
            BlockShape::T,
        ];

        Self {
            record: Default::default(),
            render_interval: 200,
            gravity_drop_interval: 1000,
            current_position: Default::default(),
            current_block: None,
            freezed: false,
            next_count: 5,
            bag: VecDeque::new(),
            garbage_queue: VecDeque::new(),
            board,
            game_mode: GameMode::NORMAL,
            game_state: GameState::IDLE,
            lose: false,
            bag_mode,
            block_list,
            garbage_gauge_count: 0,
            hold: None,
            hold_used: false,
            back2back: None,
            combo: None,
            message: None,
            in_spin: SpinType::None,
            lock_delay: 500,
            das: 300, // 좌우 DAS DEFAULT VALUE
            sdf: 5,   //FIXME: 미사용
            arr: 0,   //FIXME: 미사용
            start_time: Date::new_0(),
            running_time: 0,
            lock_delay_count: 0,
            on_left_move: None,
            on_right_move: None,
            on_down_move: None,
            // closure: Closure::<Box<dyn FnMut() + 'static>>::new(Box::new(|| {
            //     log::info!("ㅅㅅㅅㅅ");
            // })),
        }
    }

    // 가방에서 블럭를 새로 가져옴.
    pub fn get_block(&mut self) -> BlockShape {
        // 현재 가방이 비어있거나, 개수가 모자란다면 충전
        self.manage_bag();
        let block = self.bag.pop_front().unwrap();
        self.manage_bag();
        block
    }

    // 가방이 비어있을 경우 충전
    pub fn manage_bag(&mut self) {
        if self.bag.len() <= self.next_count as usize {
            self.fill_bag();
        }
    }

    // 현재 가방 채움
    fn fill_bag(&mut self) -> Option<()> {
        match self.bag_mode {
            BagType::SevenBag => {
                let mut new_bag = random::shuffle(&self.block_list).collect();
                self.bag.append(&mut new_bag);
            }
            BagType::NoBag => {
                let mut new_bag = (0..self.block_list.len())
                    .map(|_| random::random_select(&self.block_list))
                    .collect();
                self.bag.append(&mut new_bag);
            }
        }

        Some(())
    }

    pub fn add_garbage_line(&mut self, hole_loc: usize) {
        let board_height = self.board.cells.len();

        self.garbage_gauge_count -= 1 as u64;
        for row in 0..(board_height - 1) {
            self.board.cells[row] = self.board.cells[row + 1].clone();
        }

        for (i, cell) in self.board.cells[board_height - 1].iter_mut().enumerate() {
            *cell = if i == hole_loc {
                Cell::Empty
            } else {
                Cell::Garbage
            };
        }
    }

    pub fn trigger_garbage_line(&mut self) {
        while self.garbage_gauge_count > 0 {
            let current_pop = self.garbage_queue.pop_front().unwrap();
            if current_pop > 0 {
                self.add_garbage_line((current_pop - 1 as u8) as usize)
            } else {
                break;
            }
        }
    }

    // 지울 줄이 있을 경우 줄을 지움
    fn clear_line(&mut self) -> ClearInfo {
        let mut line = 0;
        // 스핀 여부 반환
        // 지운 줄 수 반환
        for y in (0..self.board.row_count).into_iter() {
            let row: &Vec<Cell> = &self.board.cells[y as usize];

            if row.iter().all(|e| e != &Cell::Empty) {
                line += 1;
                for e in (0..=y).into_iter().rev() {
                    if e == 0 {
                        for cell in &mut self.board.cells[e as usize] {
                            *cell = Cell::Empty
                        }
                    } else {
                        self.board.cells[e as usize] = self.board.cells[(e - 1) as usize].clone()
                    }
                }
            }
        }

        let is_perfect = self.board.unfold().iter().all(|e| e == &0);

        if line > 0 {
            self.record.line_clear_count += line as u32;

            let mut is_back2back = false;

            match self.combo {
                Some(combo) => {
                    self.combo = Some(combo + 1);

                    match line {
                        1..=3 => {
                            self.message = None;
                        }
                        4 => {
                            self.message = Some("Quad".into());
                        }
                        _ => {}
                    }
                }
                None => {
                    self.combo = Some(0);

                    match line {
                        1..=3 => {
                            self.message = None;
                        }
                        4 => {
                            self.message = Some("Quad".into());
                            self.record.quad_count += 1;
                            is_back2back = true
                        }
                        _ => {}
                    }
                }
            }

            match self.in_spin.clone() {
                SpinType::TSpin => {
                    is_back2back = true;

                    match line {
                        1 => self.message = Some("T-Spin Single".into()),
                        2 => self.message = Some("T-Spin Double".into()),
                        3 => self.message = Some("T-Spin Triple".into()),
                        _ => {}
                    }
                }
                SpinType::Spin => {}
                SpinType::Mini => {
                    is_back2back = true;

                    match line {
                        1 => self.message = Some("T-Spin Single mini".into()),
                        2 => self.message = Some("T-Spin Double mini".into()),
                        3 => self.message = Some("T-Spin Triple mini".into()),
                        _ => {}
                    }
                }
                SpinType::None => {}
            }

            if is_back2back {
                match self.back2back {
                    Some(back2back) => {
                        self.back2back = Some(back2back + 1);
                    }
                    None => {
                        self.back2back = Some(0);
                    }
                }
            } else {
                self.back2back = None;
            }

            if is_perfect {
                self.record.perfect_clear_count += 1;
                self.message = Some("Perfect Clear".into())
            }
        } else {
            self.combo = None;
            self.trigger_garbage_line();
        }

        let score = calculate_score(
            line,
            is_perfect,
            self.combo,
            self.in_spin.clone(),
            self.back2back,
        );
        self.record.score += score;

        self.after_clear();

        ClearInfo {
            line,
            spin: SpinType::None,
            is_perfect,
        }
    }

    // 현재 블럭를 고정
    fn fix_current_block(&mut self) {
        if let Some(current_block) = self.current_block {
            // 블럭 고정 후 현재 블럭에서 제거
            self.board
                .write_current_block(current_block.cells, self.current_position);
            self.current_block = None;
            self.lock_delay_count = 0;
            self.hold_used = false;
        }
    }

    // clear 처리 후에 트리거 (줄이 지워지는지 여부와 별개)
    fn after_clear(&mut self) {
        self.in_spin = SpinType::None;
        write_text(
            "lineclearcount",
            format!("{}", self.record.line_clear_count),
        );

        //NOTE: Garbage Line Queue 채우기
        if random() > 0.5 && self.game_mode == GameMode::NORMAL {
            let hole_loc = floor(random() * self.board.column_count as f64) as usize;
            let height = floor(random() * 3 as f64) as usize;
            for _i in 0..height {
                self.garbage_queue.push_back(hole_loc as u8 + 1 as u8);
                self.garbage_gauge_count += 1;
            }
            self.garbage_queue.push_back(0);
        }
    }

    // 한칸 내려간 후에 트리거
    fn after_down(&mut self) {
        self.in_spin = SpinType::None;
    }

    // 한칸씩 아래로 내려가는 중력 동작
    pub fn gravity_drop(&mut self) {
        if self.game_state != GameState::PLAYING {
            return;
        }

        let current_block = self.current_block;

        match current_block {
            Some(current_block) => {
                let current_position = self.current_position;
                let next_position = current_position.add_y(1);

                if !valid_block(&self.board, &current_block.cells, next_position) {
                    self.fix_current_block();
                    self.clear_line();
                } else {
                    self.current_position = next_position;
                    self.after_down();
                }
            }
            None => {
                let block = self.get_block();
                self.current_block = Some(block);

                let point = Point::start_point(self.board.column_count);
                self.current_position = point;

                if !valid_block(&self.board, &block.cells, point) {
                    // 패배 처리
                    self.game_over();
                }
            }
        }

        // Handle 40-line sprint finish condition
        // FIXME: Parametrizaiton (i.e., instead of hard-coding 40)
        // FIXME: Recude delay. Maybe we can check # erased lines in clear_line
        if self.game_mode == GameMode::SPRINT && self.record.line_clear_count > 40 {
            /* FIXME: call something like `clear` instead of `game_over` */
            self.game_over();
        }
    }

    // 왼쪽 이동
    pub fn left_move(&mut self) {
        if let Some(current_block) = self.current_block {
            let next_position = self.current_position.clone().add_x(-1);

            if valid_block(&self.board, &current_block.cells, next_position) {
                self.current_position = next_position;
                if !valid_block(
                    &self.board,
                    &current_block.cells,
                    self.current_position.add_y(1),
                ) {
                    self.lock_delay_count += 1;
                }
            }
        }
    }

    // 왼쪽 끝까지 이동
    pub fn left_move_end(&mut self) {
        if let Some(current_block) = self.current_block {
            loop {
                let next_position = self.current_position.clone().add_x(-1);

                if valid_block(&self.board, &current_block.cells, next_position) {
                    self.current_position = next_position;
                } else {
                    break;
                }
            }
            if !valid_block(
                &self.board,
                &current_block.cells,
                self.current_position.add_y(1),
            ) {
                self.lock_delay_count += 1;
            }
        }
    }

    // 오른쪽 이동
    pub fn right_move(&mut self) {
        if let Some(current_block) = self.current_block {
            let next_position = self.current_position.clone().add_x(1);

            if valid_block(&self.board, &current_block.cells, next_position) {
                self.current_position = next_position;
                if !valid_block(
                    &self.board,
                    &current_block.cells,
                    self.current_position.add_y(1),
                ) {
                    self.lock_delay_count += 1;
                }
            }
        }
    }

    // 오른쪽 끝까지 이동
    pub fn right_move_end(&mut self) {
        if let Some(current_block) = self.current_block {
            loop {
                let next_position = self.current_position.clone().add_x(1);

                if valid_block(&self.board, &current_block.cells, next_position) {
                    self.current_position = next_position;
                } else {
                    break;
                }
            }
            if !valid_block(
                &self.board,
                &current_block.cells,
                self.current_position.add_y(1),
            ) {
                self.lock_delay_count += 1;
            }
        }
    }

    // 왼쪽 회전 (반시계방향)
    pub fn left_rotate(&mut self) {
        if let Some(current_block) = &mut self.current_block {
            if current_block.block == Block::O {
                return;
            }
            let real_length = if current_block.block == Block::I {
                4
            } else {
                3
            };
            let mut next_shape = current_block.cells.clone();

            rotate_left(&mut next_shape, real_length);
            if valid_block(&self.board, &next_shape, self.current_position) {
                current_block.rotation_count = (current_block.rotation_count + 3) % 4;
                current_block.cells = next_shape;
                if !valid_block(
                    &self.board,
                    &current_block.cells,
                    self.current_position.add_y(1),
                ) {
                    self.lock_delay_count += 1;
                }

                if current_block.block == Block::T {
                    self.in_spin =
                        valid_tspin(&self.board, &current_block, self.current_position, 0);
                }
            } else {
                for i in 0..4 {
                    let mut next_position = self.current_position.clone();
                    if real_length == 3 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_3BY3[4 + current_block.rotation_count][i][0],
                            -KICK_INDEX_3BY3[4 + current_block.rotation_count][i][1],
                        ); // 4, 5, 6, 7 => 03, 10, 21, 32
                    } else if real_length == 4 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_I[4 + current_block.rotation_count][i][0],
                            -KICK_INDEX_I[4 + current_block.rotation_count][i][1],
                        );
                    }
                    if valid_block(&self.board, &next_shape, next_position) {
                        current_block.rotation_count = (current_block.rotation_count + 3) % 4;
                        self.current_position = next_position;
                        current_block.cells = next_shape;
                        if !valid_block(
                            &self.board,
                            &current_block.cells,
                            self.current_position.add_y(1),
                        ) {
                            self.lock_delay_count += 1;
                        }

                        if current_block.block == Block::T {
                            self.in_spin =
                                valid_tspin(&self.board, &current_block, next_position, i);
                        }

                        break;
                    }
                }
            }
        }
    }

    // 오른쪽 회전 (시계방향)
    pub fn right_rotate(&mut self) {
        if let Some(current_block) = &mut self.current_block {
            if current_block.block == Block::O {
                return;
            }

            let real_length = if current_block.block == Block::I {
                4
            } else {
                3
            };

            let mut next_shape = current_block.cells.clone();
            rotate_right(&mut next_shape, real_length);
            if valid_block(&self.board, &next_shape, self.current_position) {
                current_block.rotation_count = (current_block.rotation_count + 1) % 4;
                current_block.cells = next_shape;
                if !valid_block(
                    &self.board,
                    &current_block.cells,
                    self.current_position.add_y(1),
                ) {
                    self.lock_delay_count += 1;
                }

                if current_block.block == Block::T {
                    self.in_spin =
                        valid_tspin(&self.board, &current_block, self.current_position, 0);
                }
            } else {
                for i in 0..4 {
                    let mut next_position = self.current_position.clone();
                    if real_length == 3 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_3BY3[0 + current_block.rotation_count][i][0],
                            -KICK_INDEX_3BY3[0 + current_block.rotation_count][i][1],
                        ); // 0, 1, 2, 3 => 01, 12, 23, 30
                    } else if real_length == 4 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_I[0 + current_block.rotation_count][i][0],
                            -KICK_INDEX_I[0 + current_block.rotation_count][i][1],
                        );
                    }
                    if valid_block(&self.board, &next_shape, next_position) {
                        current_block.rotation_count = (current_block.rotation_count + 1) % 4;
                        self.current_position = next_position;
                        current_block.cells = next_shape;
                        if !valid_block(
                            &self.board,
                            &current_block.cells,
                            self.current_position.add_y(1),
                        ) {
                            self.lock_delay_count += 1;
                        }
                        if current_block.block == Block::T {
                            self.in_spin =
                                valid_tspin(&self.board, &current_block, next_position, i);
                        }

                        break;
                    }
                }
            }
        }
    }

    // 소프트드랍
    pub fn soft_drop(&mut self) {
        self.gravity_drop();
    }

    // 하드드랍될 위치 획득
    pub fn get_hard_drop_position(&self) -> Option<Point> {
        match self.current_block {
            Some(current_block) => {
                let current_position = self.current_position;
                let mut next_position = current_position.add_y(1);
                loop {
                    if !valid_block(&self.board, &current_block.cells, next_position) {
                        next_position = next_position.add_y(-1);
                        break;
                    } else {
                        next_position = next_position.add_y(1);
                    }
                }

                Some(next_position)
            }
            None => None,
        }
    }

    // 하드드랍 동작
    pub fn hard_drop(&mut self) {
        let position = self.get_hard_drop_position();

        match position {
            Some(position) => {
                self.current_position = position;
                self.fix_current_block();
                self.clear_line();
                self.gravity_drop();
            }
            None => {}
        }
    }

    // 홀드
    pub fn hold(&mut self) {
        if !self.hold_used {
            match self.hold {
                Some(hold) => {
                    self.current_position = Point::start_point(self.board.column_count);
                    let temp = self.current_block;
                    self.current_block = Some(hold);
                    self.hold = temp;
                }
                None => {
                    self.current_position = Point::start_point(self.board.column_count);
                    self.hold = self.current_block;
                    self.current_block = None;
                    self.fill_bag();
                }
            }

            self.hold_used = true;

            self.gravity_drop();
        }
    }

    // 180도 회전
    pub fn double_rotate(&mut self) {
        self.left_rotate();
        self.left_rotate();
    }

    // 게임오버
    fn game_over(&mut self) {
        self.game_state = GameState::GAMEOVER;
        self.lose = true;
        self.current_block = None;
        write_text("message", "Game Over".into());
    }

    // 보드 초기화
    pub fn init_board(&mut self) -> Option<()> {
        let column_count = self.board.column_count;
        let row_count = self.board.row_count;

        self.board = Board {
            cells: vec![vec![Cell::Empty; column_count as usize]; row_count as usize],
            row_count,
            column_count,
            board_height: self.board.board_height,
            board_width: self.board.board_width,
            hidden_row_count: self.board.hidden_row_count,
        };

        Some(())
    }

    // 컨텍스트 초기화
    pub fn init_context(&mut self) -> Option<()> {
        self.back2back = None;
        self.combo = None;
        self.in_spin = SpinType::None;
        self.message = None;

        Some(())
    }

    // 키 클릭시간 기록 초기화
    pub fn init_key_click_time(&mut self) -> Option<()> {
        self.on_left_move = None;
        self.on_right_move = None;
        self.on_down_move = None;

        Some(())
    }

    // 가방 초기화
    pub fn init_bag(&mut self) -> Option<()> {
        self.bag = VecDeque::new();
        self.current_block = None;
        self.hold_used = false;
        self.hold = None;
        self.garbage_gauge_count = 0;
        self.garbage_queue = VecDeque::new();

        Some(())
    }

    // 점수 초기화
    pub fn init_score(&mut self) -> Option<()> {
        self.record = Default::default();

        Some(())
    }

    pub fn init_runningtime(&mut self) -> Option<()> {
        self.running_time = 0;
        Some(())
    }

    // 게임 초기화
    pub fn init_game(&mut self) -> Option<()> {
        self.init_bag()?;
        self.init_board()?;
        self.init_score()?;
        self.init_context()?;
        self.init_runningtime()?;
        self.init_key_click_time()?;

        Some(())
    }
}
