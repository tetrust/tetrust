# tetrust

Yew.rs, Canvas, Webassembly로 구현된 웹 퍼즐게임

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

