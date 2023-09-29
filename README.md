<div align="center">

# Iris 💫

<!-- s;HidemaruOwO/Iris;User/Repository;g -->

[![Test CLI](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml/badge.svg)](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml)
![最終コミット](https://img.shields.io/github/last-commit/HidemaruOwO/Iris?style=flat-square)
![リポジトリのスター](https://img.shields.io/github/stars/HidemaruOwO/Iris?style=flat-square)
![問題](https://img.shields.io/github/issues/HidemaruOwO/Iris?style=flat-square)
![オープンな問題](https://img.shields.io/github/issues-raw/HidemaruOwO/Iris?style=flat-square)
![バグの問題](https://img.shields.io/github/issues/HidemaruOwO/Iris/bug?style=flat-square)

![image](https://user-images.githubusercontent.com/82384920/269208322-7155e5c7-fc40-40fb-9b1f-1f11d5d78ddd.png)

## なんだこれは？

Rustで書かれた汎用Discord Bot

</div>

- Select Language

<table>
  <thead>
    <tr>
      <th style="text-align:center"><a href="README.md">🎌日本語</a></th>
      <th style="text-align:center"><a href="README.en.md">🤡English</a></th>
      <th style="text-align:center"><a href="README.zh-CN.md">🐉简体中文</a></th>
      <th style="text-align:center"><a href="README.zh-TW.md">🍜繁体中文</a></th>
      <th style="text-align:center"><a href="README.ko.md">🌸한국어</a></th>
    </tr>
  </thead>
</table>

## Usage 💨

## Install 😊

### Local Intall 🏠

GitHubの[Release](https://github.com/HidemaruOwO/Iris/releases)もしくは、Actionsの[Artifact](https://github.com/HidemaruOwO/Iris/actions/workflows/build.yml)から実行ファイルをダウンロードしてください。

- tar.gz

```bash
tar xvf iris_*.tar.gz
chmod 755 iris
```

- zip

```bash
unzip iris_*.zip
chmod 755 iris
```

次にこのスクリプトを`start.sh`という名前で作成してください
また、変数の中身を各々編集してください

```bash
#!/bin/bash
# ----- START BOT ENV ----- #
export BOT_OWNER_ID=830789490481954856
export BOT_TOKEN="xxx"
export BOT_PREFIX="&&"
# ----- STOP BOT ENV ------ #
./iris
```

そうしたら実行権限を付与します

```bash
chmod 755 start.sh
```

そして実行します

```bash
./start.sh
```

### Use Docker 🐋

リポジトリのクローンと起動スクリプトを複製します。

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cp start.example.sh start.sh
```

起動スクリプトをTOKENなどの環境変数を設定するために、任意のエディターで開きます。
`BOT_OWNER_ID`と`BOT_TOKEN`と`BOT_PREFIX`を編集したら、保存してエディターを終了します。

```bash
# Edit start.sh to set environment value
vim start.sh
```

起動します。

```bash
sudo docker compose up -d
```

## Build 🔨

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cd Iris
cargo build --release
# And run script
mv start.example.sh start.sh
./start.sh
```

## Development 💡

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cd Iris
mv test.example.sh test.sh
./test.sh
```

## Dependencies 🪡

このアプリを使用するには、以下のコマンドをパスに登録してください。

### Dependencies

none

### Build Dependencies

- `rust`

## Repository Tools 🔧

- [x] Depandabotのセットアップ
- [ ] CodeQLのセットアップ

<details>
<summary>メモ</summary>

- Depandabotのセットアップ
  - `.github/dependabot.yml`の`package-ecosystem`に値を設定 (例: npm,yarn,pip)
- CodeQLのセットアップ
  - https://dev.classmethod.jp/articles/github-code-scanning/
  - [対応言語](https://codeql.github.com/docs/codeql-overview/supported-languages-and-frameworks/)

</details>

## For Contributor 🤝

本プロジェクトにコントリービュートする場合は[コントリービュートガイド](docs/README.md)をお読みください。

## Reference ✨

- [doremire/Awesome-README](https://github.com/doremire/Awesome-README)
