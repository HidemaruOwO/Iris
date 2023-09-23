<div align="center">

# 鸢尾花💫

<!-- s;HidemaruOwO/Iris;User/Repository;g -->

[![Test CLI](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml/badge.svg)](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml)![最終コミット](https://img.shields.io/github/last-commit/HidemaruOwO/Iris?style=flat-square)![リポジトリのスター](https://img.shields.io/github/stars/HidemaruOwO/Iris?style=flat-square)![問題](https://img.shields.io/github/issues/HidemaruOwO/Iris?style=flat-square)![オープンな問題](https://img.shields.io/github/issues-raw/HidemaruOwO/Iris?style=flat-square)![バグの問題](https://img.shields.io/github/issues/HidemaruOwO/Iris/bug?style=flat-square)

![image](https://user-images.githubusercontent.com/82384920/269208322-7155e5c7-fc40-40fb-9b1f-1f11d5d78ddd.png)

## 这是什么？

用 Rust 编写的通用 Discord Bot

</div>

-   选择语言

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

## 安装😊

GitHub[发布](https://github.com/HidemaruOwO/Iris/releases)或行动[人工制品](https://github.com/HidemaruOwO/Iris/actions/workflows/build.yml)请从下载可执行文件。

-   压缩包

```bash
tar xvf iris_*.tar.gz
chmod 755 iris
```

-   压缩

```bash
unzip iris_*.zip
chmod 755 iris
```

然后运行这个脚本`start.sh`请使用名称创建它
另外，请编辑每个变量的内容。

```bash
#!/bin/bash
# ----- START BOT ENV ----- #
export BOT_TOKEN=xxx
export BOT_OWNER_ID=830789490481954856
export BOT_PREFIX="&&"
# ----- STOP BOT ENV ------ #
./iris
```

然后给予执行权限

```bash
chmod 755 start.sh
```

并运行

```bash
./start.sh
```

## 构建🔨

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cd Iris
cargo build --release
# And run script
./start.sh
```

## 发展💡

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cd Iris
./test.sh
```

## 依赖关系🪡

要使用此应用程序，请在您的路径中注册以下命令。

### 依赖关系

没有任何

### 构建依赖关系

-   `rust`

## 存储库工具🔧

-   [x] Dependabot 设置
-   [ ] CodeQL 设置

<details>
<summary>メモ</summary>

-   Dependabot 设置
    -   `.github/dependabot.yml`的`package-ecosystem`将值设置为（例如 npm、yarn、pip）
-   CodeQL 设置
    -   [HTTPS://Dev.class method.键盘/articles/GitHub-code-scanning/](https://dev.classmethod.jp/articles/github-code-scanning/)
    -   [支持的语言](https://codeql.github.com/docs/codeql-overview/supported-languages-and-frameworks/)

</details>

## 对于贡献者🤝

如果您想为这个项目做出贡献，[投稿指南](docs/README.md)请阅读。

## 参考✨

-   [doremire/Awesome-自述文件](https://github.com/doremire/Awesome-README)
