<div align="center">

# Iris 💫

<!-- s;HidemaruOwO/Iris;User/Repository;g -->

[![Test CLI](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml/badge.svg)](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml)![最終コミット](https://img.shields.io/github/last-commit/HidemaruOwO/Iris?style=flat-square)![リポジトリのスター](https://img.shields.io/github/stars/HidemaruOwO/Iris?style=flat-square)![問題](https://img.shields.io/github/issues/HidemaruOwO/Iris?style=flat-square)![オープンな問題](https://img.shields.io/github/issues-raw/HidemaruOwO/Iris?style=flat-square)![バグの問題](https://img.shields.io/github/issues/HidemaruOwO/Iris/bug?style=flat-square)

![image](https://user-images.githubusercontent.com/82384920/269208322-7155e5c7-fc40-40fb-9b1f-1f11d5d78ddd.png)

## What is this?

General-purpose Discord Bot written in Rust

</div>

-   Select Language

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

### Local Install 🏠

GitHub[Release](https://github.com/HidemaruOwO/Iris/releases)Or Actions[Artifact](https://github.com/HidemaruOwO/Iris/actions/workflows/build.yml)Please download the executable file from.

-   tar.gz

```bash
tar xvf iris_*.tar.gz
chmod 755 iris
```

-   zip

```bash
unzip iris_*.zip
chmod 755 iris
```

Then run this script`start.sh`Please create it with the name
Also, please edit the contents of each variable.

```bash
#!/bin/bash
# ----- START BOT ENV ----- #
export BOT_OWNER_ID=830789490481954856
export BOT_TOKEN="xxx"
export BOT_PREFIX="&&"
# ----- STOP BOT ENV ------ #
./iris
```

Then give execute permission

```bash
chmod 755 start.sh
```

and run

```bash
./start.sh
```

### Use Docker 🐋

Clone the repository and duplicate the startup script.

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cp start.example.sh start.sh
```

Open the startup script in any editor to set environment variables such as TOKEN.  
`BOT_OWNER_ID`and`BOT_TOKEN`and`BOT_PREFIX`After editing, save and exit the editor.

```bash
# Edit start.sh to set environment value
vim start.sh
```

start up.

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

To use this app, please register the following command in your path.

### Dependencies

none

### Build Dependencies

-   `rust`

## Repository Tools 🔧

-   [x] Dependabot setup
-   [ ] CodeQL setup

<details>
<summary>メモ</summary>

-   Dependabot setup
    -   `.github/dependabot.yml`of`package-ecosystem`Set the value to (e.g. npm,yarn,pip)
-   CodeQL setup
    -   <https://dev.classmethod.jp/articles/github-code-scanning/>
    -   [supported language](https://codeql.github.com/docs/codeql-overview/supported-languages-and-frameworks/)

</details>

## For Contributor 🤝

If you would like to contribute to this project,[Contribution guide](docs/README.md)Please read.

## Reference ✨

-   [doremire/Awesome-README](https://github.com/doremire/Awesome-README)
