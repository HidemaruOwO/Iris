<div align="center">

# 鳶尾花💫

<!-- s;HidemaruOwO/Iris;User/Repository;g -->

[![Test CLI](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml/badge.svg)](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml)![最終コミット](https://img.shields.io/github/last-commit/HidemaruOwO/Iris?style=flat-square)![リポジトリのスター](https://img.shields.io/github/stars/HidemaruOwO/Iris?style=flat-square)![問題](https://img.shields.io/github/issues/HidemaruOwO/Iris?style=flat-square)![オープンな問題](https://img.shields.io/github/issues-raw/HidemaruOwO/Iris?style=flat-square)![バグの問題](https://img.shields.io/github/issues/HidemaruOwO/Iris/bug?style=flat-square)

![image](https://user-images.githubusercontent.com/82384920/269208322-7155e5c7-fc40-40fb-9b1f-1f11d5d78ddd.png)

## 這是什麼？

用 Rust 寫的通用 Discord Bot

</div>

-   選擇語言

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

## 用法💨

## 安裝😊

GitHub[發布](https://github.com/HidemaruOwO/Iris/releases)或行動[人工製品](https://github.com/HidemaruOwO/Iris/actions/workflows/build.yml)請從下載可執行檔。

-   壓縮包

```bash
tar xvf iris_*.tar.gz
chmod 755 iris
```

-   壓縮

```bash
unzip iris_*.zip
chmod 755 iris
```

然後運行這個腳本`start.sh`請使用名稱創建它
另外，請編輯每個變數的內容。

```bash
#!/bin/bash
# ----- START BOT ENV ----- #
export BOT_TOKEN=xxx
export BOT_OWNER_ID=830789490481954856
export BOT_PREFIX="&&"
# ----- STOP BOT ENV ------ #
./iris
```

然後給予執行權限

```bash
chmod 755 start.sh
```

並運行

```bash
./start.sh
```

## 建構🔨

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cd Iris
cargo build --release
# And run script
./start.sh
```

## 發展💡

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cd Iris
./test.sh
```

## 依賴關係🪡

要使用此應用程序，請在您的路徑中註冊以下命令。

### 依賴關係

沒有任何

### 建構依賴關係

-   `rust`

## 儲存庫工具🔧

-   [x] Dependabot 設定
-   [ ] CodeQL 設定

<details>
<summary>メモ</summary>

-   Dependabot 設定
    -   `.github/dependabot.yml`的`package-ecosystem`將值設為（例如 npm、yarn、pip）
-   CodeQL 設定
    -   <https://dev.classmethod.jp/articles/github-code-scanning/>
    -   [支援的語言](https://codeql.github.com/docs/codeql-overview/supported-languages-and-frameworks/)

</details>

## 對於貢獻者🤝

如果您想為這個專案做出貢獻，[投稿指南](docs/README.md)請閱讀。

## 參考✨

-   [doremire/Awesome-自述文件](https://github.com/doremire/Awesome-README)
