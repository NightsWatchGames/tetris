[English](https://github.com/NightsWatchGames/tetris/blob/main/README_EN.md)
# tetris
俄罗斯方块游戏。
- [x] 游戏基础玩法（随机四格骨牌、骨牌旋转、骨牌移动、消除行、计分）
- [x] 游戏UI
- [x] 游戏音效
- [x] 支持暂停、恢复和重新开始游戏
- [x] 支持web
- [x] 展示下一个骨牌
- [x] bag7随机算法

在线游玩：[点这里](https://nightswatchgames.github.io/games/tetris/)（电脑版Chrome/Firefox/Edge打开）

## 运行
1. 本地运行
```
cargo run
```
2. WASM运行
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

## 游戏展示
视频演示：[B站](https://www.bilibili.com/video/BV1y44y1R72Z)

![main menu](https://raw.githubusercontent.com/NightsWatchGames/tetris/main/screenshots/main_menu.png)
![game playing](https://raw.githubusercontent.com/NightsWatchGames/tetris/main/screenshots/game_playing.png)
![game paused](https://raw.githubusercontent.com/NightsWatchGames/tetris/main/screenshots/game_paused.png)
![game over](https://raw.githubusercontent.com/NightsWatchGames/tetris/main/screenshots/game_over.png)

## 参考资料
- [Tetris - Wikipedia](https://en.wikipedia.org/wiki/Tetris)
- [俄罗斯方块 - 百度百科](https://baike.baidu.com/item/%E4%BF%84%E7%BD%97%E6%96%AF%E6%96%B9%E5%9D%97/535753)
- [Online tetris example1](https://tetris.com/play-tetris)
- [Online tetris example2](https://www.freetetris.org/game.php)
- [bevy-cheatbook](https://github.com/bevy-cheatbook/bevy-cheatbook)（[中文翻译](https://yiviv.com/bevy-cheatbook/)）
- https://mbuffett.com/posts/bevy-snake-tutorial/