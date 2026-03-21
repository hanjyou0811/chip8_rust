# CHIP-8 Emulator in Rust

Rust と SDL2 で実装したシンプルな CHIP-8 エミュレータです。  
ROM を読み込み、64x32 のモノクロ画面と 16 キー入力を再現します。

## Features

- Rust 製の CHIP-8 エミュレータ
- SDL2 による描画とキーボード入力

## Requirements

- Rust
- Cargo
- SDL2 開発ライブラリ

Linux 系環境では、SDL2 の開発パッケージが必要です。例えば Debian / Ubuntu 系なら次を使えます。

```bash
sudo apt install libsdl2-dev
```

## Run

```bash
cargo run <path to ROM>
```

## Controls

CHIP-8 の 16 キーは次のように PC キーボードへ割り当てています。

```text
Chip-8: 1 2 3 C      Keyboard: 1 2 3 4
        4 5 6 D                Q W E R
        7 8 9 E                A S D F
        A 0 B F                Z X C V
```

## Notes
- サウンド再生はまだ未実装です

![Screenshot0](/IBM_logo.png)
![Screenshot1](/15PUZZLE.png)
![Screenshot2](/BRIX.png)