# tetris
- [x] Basic gameplay（random piece、piece rotation and move、clear full lines、scores）
- [x] Game UI
- [x] Game Audio
- [x] Support pausing, resuming and restarting game
- [x] Support web
- [x] Display next piece
- [x] Bag7 random algorithm

Play online: [click here](https://nightswatchgames.github.io/games/tetris/)（Open with PC Chrome/Firefox/Edge）

## Get started
1. Native
```
cargo run
```
2. WASM
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```
```
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/tetris.wasm
```

## Screenshots
Game video: [YouTube](https://www.youtube.com/watch?v=ovu1hYk-mn8)

![main menu](https://raw.githubusercontent.com/NightsWatchGames/tetris/main/screenshots/main_menu.png)
![game playing](https://raw.githubusercontent.com/NightsWatchGames/tetris/main/screenshots/game_playing.png)
![game paused](https://raw.githubusercontent.com/NightsWatchGames/tetris/main/screenshots/game_paused.png)
![game over](https://raw.githubusercontent.com/NightsWatchGames/tetris/main/screenshots/game_over.png)

## Reference
- [Tetris - Wikipedia](https://en.wikipedia.org/wiki/Tetris)
- [俄罗斯方块 - 百度百科](https://baike.baidu.com/item/%E4%BF%84%E7%BD%97%E6%96%AF%E6%96%B9%E5%9D%97/535753)
- [Online tetris example1](https://tetris.com/play-tetris)
- [Online tetris example2](https://www.freetetris.org/game.php)
- [bevy-cheatbook](https://github.com/bevy-cheatbook/bevy-cheatbook)（[中文翻译](https://yiviv.com/bevy-cheatbook/)）
- https://mbuffett.com/posts/bevy-snake-tutorial/