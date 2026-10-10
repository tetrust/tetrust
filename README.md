# tetrust

Yew.rs, Canvas, Webassembly로 구현된 웹 퍼즐게임

## 구조

- `crates/tetrust-engine`: 웹 의존성이 없는 결정적 게임 엔진. 같은 설정, 시드, 입력 로그면 브라우저든 서버든 결과가 같다.
- `src/lib/play`: 브라우저 계층. `requestAnimationFrame` 루프 하나가 게임 시계를 진행시키고, 키 입력은 `event.timeStamp`로 눌린 시각을 기록하며, 캔버스는 색별로 묶어 그린다. 설정 페이지의 DAS·ARR·SDF 값을 다음 판부터 엔진 설정으로 쓴다.
- `src/lib/components`, `src/pages`: Yew 화면.

## 조작

| 키 | 동작 |
|---|---|
| ← → | 이동 |
| ↓ | 소프트 드롭 |
| Space | 하드 드롭 |
| ↑, X | 시계 방향 회전 |
| Z | 반시계 방향 회전 |
| A | 180도 회전 |
| Shift, C | 홀드 |
| R | 다시 시작 |

## 실행법

기본설정

```
rustup target add wasm32-unknown-unknown
cargo install trunk
```

개발모드

```
trunk serve --open
```

배포 빌드

```
trunk build --release
```

테스트

```
cargo test --workspace
```
