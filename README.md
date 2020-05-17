# pseudo-mountpoint   
  
Linuxの [mountpoint](https://linux.die.net/man/1/mountpoint) から最小限の機能をMac向けに。

単なる `mount` コマンドのラッパーです。

特定ディレクトリがマウントポイントか確認することしかできません。

ほぼ初めて書いたRustコードなので汚いけど気にしないでください。

## インストール

当然だけどRust環境必須です。

```sh
cargo install --git https://github.com/0g1h4r4/pseudo-mountpoint.git
```

## 使い方

```sh
# hogeがマウントポイントか確認する
$ mountpoint ./hoge
./hoge is a mountpoint
```

### オプション

- `-q` : メッセージを表示しない

### 終了ステータス

- 0 : マウントポイントだった
- 1: マウントポイントじゃない OR なんかエラー(panic出ます)

## ライセンス

WTFPL
