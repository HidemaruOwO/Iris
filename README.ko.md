<div align="center">

# 아이리스 🎃

<!-- s;HidemaruOwO/Iris;User/Repository;g -->

[![Test CLI](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml/badge.svg)](https://github.com/HidemaruOwO/Iris/actions/workflows/test.yml)![最終コミット](https://img.shields.io/github/last-commit/HidemaruOwO/Iris?style=flat-square)![リポジトリのスター](https://img.shields.io/github/stars/HidemaruOwO/Iris?style=flat-square)![問題](https://img.shields.io/github/issues/HidemaruOwO/Iris?style=flat-square)![オープンな問題](https://img.shields.io/github/issues-raw/HidemaruOwO/Iris?style=flat-square)![バグの問題](https://img.shields.io/github/issues/HidemaruOwO/Iris/bug?style=flat-square)

![image](https://user-images.githubusercontent.com/82384920/269208322-7155e5c7-fc40-40fb-9b1f-1f11d5d78ddd.png)

## 이게 뭐야?

Rust로 작성된 범용 Discord Bot

</div>

-   언어 선택

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

## 사용법 💨

## 설치 😊

### 로컬 설치 🏠

GitHub의[풀어 주다](https://github.com/HidemaruOwO/Iris/releases)또는 Actions[인공물](https://github.com/HidemaruOwO/Iris/actions/workflows/build.yml)에서 실행 파일을 다운로드합니다.

-   tar.gz

```bash
tar xvf iris_*.tar.gz
chmod 755 iris
```

-   지퍼

```bash
unzip iris_*.zip
chmod 755 iris
```

그런 다음이 스크립트를`start.sh`라는 이름으로 만드십시오.
또한 변수의 내용을 각각 편집하십시오.

```bash
#!/bin/bash
# ----- START BOT ENV ----- #
export BOT_OWNER_ID=830789490481954856
export BOT_TOKEN="xxx"
export BOT_PREFIX="&&"
# ----- STOP BOT ENV ------ #
./iris
```

그러면 실행 권한을 부여합니다.

```bash
chmod 755 start.sh
```

그리고 실행

```bash
./start.sh
```

### 도커 사용 🐋

리포지토리 복제 및 시작 스크립트를 복제합니다.

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cp start.example.sh start.sh
```

시작 스크립트를 TOKEN과 같은 환경 변수를 설정하기 위해 모든 편집기에서 엽니다.  
`BOT_OWNER_ID`그리고`BOT_TOKEN`그리고`BOT_PREFIX`편집한 후 저장하고 편집기를 종료합니다.

```bash
# Edit start.sh to set environment value
vim start.sh
```

시작합니다.

```bash
sudo docker compose up -d
```

## 빌드🔨

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cd Iris
cargo build --release
# And run script
mv start.example.sh start.sh
./start.sh
```

## 개발 💡

```bash
git clone https://github.com/HidemaruOwO/Iris.git
cd Iris
mv test.example.sh test.sh
./test.sh
```

## 종속성 🪡

이 앱을 사용하려면 다음 명령을 경로에 등록하십시오.

### 종속성

없음

### Build Dependencies

-   `rust`

## 저장소 도구 🔧

-   [x] Dependabot 설정
-   [ ] CodeQL 설정

<details>
<summary>メモ</summary>

-   Dependabot 설정
    -   `.github/dependabot.yml`의`package-ecosystem`에 값 설정(예: npm,yarn,pip)
-   CodeQL 설정
    -   <https://dev.classmethod.jp/articles/github-code-scanning/>
    -   [대응 언어](https://codeql.github.com/docs/codeql-overview/supported-languages-and-frameworks/)

</details>

## 기여자용 🤝

이 프로젝트에 컨트리뷰트하는 경우는[컨트리뷰트 가이드](docs/README.md)를 읽으십시오.

## 참고 ✨

-   [doremire/Awesome-README](https://github.com/doremire/Awesome-README)
