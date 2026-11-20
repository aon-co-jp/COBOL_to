# COBOL_to

**日本語 → English**

COBOL の方言・版・書式を**自動判定**し、選んだスタックのプロジェクトへ変換するツールです。
*Detects the COBOL dialect, standard and source format automatically and converts to the stack you choose.*

- リポジトリ / Repository: https://github.com/aon-co-jp/COBOL_to
- ライセンス / License: MIT

## 日本語

### できること
- **自動判定**: IBM Enterprise(メインフレーム)/ Micro Focus / GnuCOBOL / ACUCOBOL-GT / Fujitsu NetCOBOL / 日立 COBOL、規格は COBOL-68・74・85・2002/2014、書式は固定・自由。根拠つきで `COBOL_to-REPORT.md` に出力します。
- **変換先を選択**(4種):
  1. 最新 Python + 最新 FastAPI + Next.js/Node.js + HTML5/CSS3/TypeScript/Bootstrap
  2. 最新 Python + 最新 MFastAPI + 同上
  3. 最新 Rust + 最新 Poem
  4. 最新 Rust + 最新 RPoem(Bootstrap・TypeScript・Next.js・Node.js は使わない。RPoem は Rust→WASM 方針)
- **フォルダ選択**: 引数なしで起動すると、エクスプローラー(ファイル選択ダイアログ)が開き、プログラムや複数プロジェクトの上位フォルダを選べます。出力先は既存フォルダの選択も、新しい名前の入力(空欄は `<元>-converted`)もできます。
- **API**: Connect 形式 `POST /cobol_to.Program/Run`(REST は使いません)。

### 使い方
```
cobol_to                                  # フォルダ選択ダイアログが開く
cobol_to --src <dir> --target <fastapi|mfastapi|rust-poem|rpoem> --out <dir>
```

### 現在の制限(正直な状態)
- 変換できる構文: `PROGRAM-ID` / `01〜49・77・88`(PIC・VALUE・OCCURS・REDEFINES〈同型のみ別名、他は別領域+注記〉)/ `DISPLAY`(`WITH NO ADVANCING`)・`ACCEPT` / `MOVE`・`ADD`・`SUBTRACT`・`MULTIPLY`・`DIVIDE`(`REMAINDER`)・`COMPUTE`(小数=固定小数点、`ROUNDED`)/ `IF`(条件名・`NUMERIC`/`ALPHABETIC`)・`EVALUATE`(`THRU`)/ `PERFORM`(段落・`THRU`・`TIMES`・`UNTIL`・`VARYING`・インライン)・`GO TO`(`DEPENDING ON`)/ `STRING`・`UNSTRING`・`INSPECT`(`TALLYING FOR ALL`・`REPLACING ALL/FIRST`・`CONVERTING`)/ `INITIALIZE`・`SET` / `CALL "名前" USING`(生成物の `EXTERNAL`/`run_with` に登録)/ `COPY`(変換前に展開、`REPLACING ==a== BY ==b==` 可)/ 順編成ファイル(`SELECT`・`FD`・`OPEN`・`READ`・`WRITE`・`CLOSE`・`FILE STATUS`)/ `FUNCTION`(`LENGTH`・`UPPER-CASE`・`LOWER-CASE`・`REVERSE`・`MAX`・`MIN`・`ABS`・`MOD`・`NUMVAL`)。`samples/features.cbl`・`fileio.cbl`・`misc.cbl` を Python と Rust の両方で実行し、同じ出力になることを確認済み。
- **未対応(`TODO(COBOL_to)` として表示、黙って落とさない)**: `SORT`/`MERGE`/`SEARCH`、`EXEC SQL`/`EXEC CICS`、索引ファイル・相対ファイル(`INDEXED`/`RELATIVE`)・`REWRITE`/`START`/`DELETE`、`COMP-3` 等のバイナリ表現(桁だけ扱う)、参照修飾の書き込み側(`MOVE ... TO X(a:b)`)、多次元の添字、`STRING ... WITH POINTER`・`ON OVERFLOW`、`ON SIZE ERROR`、`SET ... TO FALSE`、`INSPECT ... BEFORE/AFTER`、動的 `CALL`、`WRITE ... ADVANCING`(改行のみ)、`REDEFINES` のメモリ重ね合わせ。`DISPLAY` は小数を小数点つきで表示(COBOL の暗黙小数点とは異なる)。`CALL` の引数は値渡しで、呼び先が変えた値は戻らない。
- メインフレームの EBCDIC ファイルは変換せず、レポートに載せるだけです(UTF-8/ASCII へ変換してから実行)。
- 生成物は「プログラムの登録表 + 雛形」までで、FastAPI/MFastAPI/RPoem 実サーバーへの接続、Next.js 画面の自動生成は未実装です。
- 生成プロジェクトの依存は**固定しません**(最新版は version-control の通知に従う)。

### インストーラー
| OS | 形式 | 作り方 |
|---|---|---|
| Windows x64 / ARM64 | `COBOL_to-Setup-x64.exe` / `-arm64.exe`(Inno Setup、ユーザー単位、PATH 追加は任意) | `installer/cobol_to.iss` |
| Linux amd64 / arm64 | `cobol-to_<版>_<arch>.deb`、`.tar.gz` | `installer/build-deb.sh` |
| macOS arm64 / x64 | `COBOL_to-<版>-macos-<arch>.pkg`(未署名) | `installer/build-pkg.sh` |

タグ `v*` の push で `.github/workflows/release.yml` が全 OS 分を作り、各 OS で install→実行→uninstall の通し試験をしてから Release に添付します。
検証状況: **v0.1.2 の CI で全6ジョブ成功**(Windows x64/ARM64・Linux amd64/arm64・macOS arm64/x64 で install→実行→uninstall の通し試験を通過)し、[Release v0.1.2](https://github.com/aon-co-jp/COBOL_to/releases/tag/v0.1.2) に 8 個のインストーラーと `SHA256SUMS.txt` を添付。実機での確認は Windows x64 と Linux amd64(WSL)のみで、他は CI のランナー上。macOS の署名・公証は未対応(未署名の .pkg は警告が出る)。

### 更新通知
自動更新しきれないものは、登録した 2 つのメールへ Claude 向けメッセージとして送ります → [NOTIFY.md](NOTIFY.md)(日英独露)。

## English

### What it does
- **Auto-detection**: IBM Enterprise (mainframe) / Micro Focus / GnuCOBOL / ACUCOBOL-GT / Fujitsu NetCOBOL / Hitachi COBOL; standards COBOL-68, 74, 85, 2002/2014; fixed or free format. Evidence goes to `COBOL_to-REPORT.md`.
- **Choose the target** (4 stacks): (1) latest Python + latest FastAPI + Next.js/Node.js + HTML5/CSS3/TypeScript/Bootstrap; (2) latest Python + latest MFastAPI + the same; (3) latest Rust + latest Poem; (4) latest Rust + latest RPoem (no Bootstrap, TypeScript, Next.js or Node.js; RPoem is Rust→WASM).
- **Folder picker**: started without arguments, a native file dialog opens so you can pick the top folder of a program or a multi-project tree. For the output you can pick an existing folder or type a new name (empty = `<source>-converted`).
- **API**: Connect style `POST /cobol_to.Program/Run` (no REST).

### Usage
```
cobol_to                                  # opens the folder picker
cobol_to --src <dir> --target <fastapi|mfastapi|rust-poem|rpoem> --out <dir>
```

### Current limits (honest status)
- Converted: `PROGRAM-ID`; `01–49/77/88` (PIC, VALUE, OCCURS, REDEFINES — aliased only when the types match, otherwise a separate area plus a note); `DISPLAY` (`WITH NO ADVANCING`), `ACCEPT`; `MOVE`, `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` (`REMAINDER`), `COMPUTE` (decimals as fixed point, `ROUNDED`); `IF` (condition names, `NUMERIC`/`ALPHABETIC`), `EVALUATE` (`THRU`); `PERFORM` (paragraph, `THRU`, `TIMES`, `UNTIL`, `VARYING`, inline), `GO TO` (`DEPENDING ON`); `STRING`, `UNSTRING`, `INSPECT` (`TALLYING FOR ALL`, `REPLACING ALL/FIRST`, `CONVERTING`); `INITIALIZE`, `SET`; `CALL "name" USING` (register the callee in the generated `EXTERNAL` / `run_with`); `COPY` (expanded before conversion, `REPLACING ==a== BY ==b==`); sequential files (`SELECT`, `FD`, `OPEN`, `READ`, `WRITE`, `CLOSE`, `FILE STATUS`); `FUNCTION` (`LENGTH`, `UPPER-CASE`, `LOWER-CASE`, `REVERSE`, `MAX`, `MIN`, `ABS`, `MOD`, `NUMVAL`). `samples/features.cbl`, `fileio.cbl` and `misc.cbl` run in both generated Python and Rust with identical output.
- **Not supported (shown as `TODO(COBOL_to)`, never dropped silently)**: `SORT`/`MERGE`/`SEARCH`, `EXEC SQL`/`EXEC CICS`, indexed/relative files (`INDEXED`/`RELATIVE`) and `REWRITE`/`START`/`DELETE`, binary representations such as `COMP-3` (only the digit count is used), reference modification as a target (`MOVE ... TO X(a:b)`; reads work), multi-dimensional subscripts, `STRING ... WITH POINTER` / `ON OVERFLOW`, `ON SIZE ERROR`, `SET ... TO FALSE`, `INSPECT ... BEFORE/AFTER`, dynamic `CALL`, `WRITE ... ADVANCING` (newline only), true memory overlay for `REDEFINES`. `DISPLAY` prints decimals with a decimal point (COBOL's implied point is not shown). `CALL` arguments are passed by value; changes made by the callee are not returned.
- EBCDIC mainframe files are not converted, only listed in the report (convert to UTF-8/ASCII first).
- Output is a program registry plus a skeleton; wiring into a real FastAPI/MFastAPI/RPoem server and generating the Next.js UI are not implemented.
- Dependencies of generated projects are **not pinned** (latest versions follow version-control notifications).

### Installers
See the table above. Tagging `v*` builds every installer in `.github/workflows/release.yml`, smoke-tests install → run → uninstall on each OS, then attaches them to the Release. Status: **all 6 jobs of the v0.1.2 CI passed** (install → run → uninstall on Windows x64/ARM64, Linux amd64/arm64, macOS arm64/x64) and [Release v0.1.2](https://github.com/aon-co-jp/COBOL_to/releases/tag/v0.1.2) carries 8 installers plus `SHA256SUMS.txt`. Real-machine checks exist only for Windows x64 and Linux amd64 (WSL); the rest ran on CI runners. macOS signing/notarization is not done (the unsigned .pkg shows a warning).

### Update notifications
What cannot be updated automatically is mailed to two registered addresses as a Claude message → [NOTIFY.md](NOTIFY.md) (JA/EN/DE/RU).

> このリポジトリの変換機能は `pl2pl` に統合され、組み合わせ選択は pl2pl が最優先で進めます。/ The conversion features of this repository are being merged into `pl2pl`, which now carries the selectable-stack feature as top priority. https://github.com/aon-co-jp/pl2pl

## 最優先機能 / Top-priority feature: 組み合わせを選べる変換 (`pl2pl web --backend ...`) (2026-11-19, 10 languages)

```
pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea>
                [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>
```
COBOL は変換器で、C / C++ / Fortran は「そのままビルドして呼ぶ橋渡し」(`legacy/build.sh`)で、選んだ組み合わせのアプリの骨組みになります。Axum 版は実際に起動して呼び出しの成功を確認済みです(COBOL と C/C++)。RPoem / RPoRea は Bootstrap・TypeScript なし(Rust→WASM)。

### 日本語
やさしく言うと、「古いプログラム(COBOL・C・C++・Fortran)を、今のウェブのしくみに乗せる」機能です。サーバーは Axum・Poem・FastAPI・RPoem・RPoRea から、画面は HTML5・CSS3・TypeScript・Bootstrap から、データベースは MySQL・MariaDB・PostgreSQL・SQLite から、好きな組み合わせを選べます。COBOL は書き換えて変換します。C・C++・Fortran は元のまま動かし、ウェブから呼べるようにします(まだ Rust への書き換えはしません)。メインフレーム・UNIX・Linux・Windows・Mac の COBOL を別の OS の COBOL へ直す機能は `pl2pl cobol`(IBM→GnuCOBOL)にあります。
### English
In simple words: it puts old programs (COBOL, C, C++, Fortran) onto today's web. You pick the server (Axum, Poem, FastAPI, RPoem or RPoRea), the screen (HTML5, CSS3, TypeScript, Bootstrap) and the database (MySQL, MariaDB, PostgreSQL or SQLite) in any allowed mix. COBOL is rewritten. C, C++ and Fortran keep running as they are, and the web calls them (they are not rewritten into Rust yet). Turning COBOL from one OS into COBOL for another OS is `pl2pl cobol` (IBM to GnuCOBOL).
### 简体中文
简单地说:把旧程序(COBOL、C、C++、Fortran)放到今天的网页上。服务器可选 Axum、Poem、FastAPI、RPoem、RPoRea;界面可选 HTML5、CSS3、TypeScript、Bootstrap;数据库可选 MySQL、MariaDB、PostgreSQL、SQLite,自由组合。COBOL 会被改写;C、C++、Fortran 保持原样运行,由网页调用(暂不改写成 Rust)。把 COBOL 从一个系统转到另一个系统用 `pl2pl cobol`。
### 繁體中文
簡單地說:把舊程式(COBOL、C、C++、Fortran)放到現在的網頁上。伺服器可選 Axum、Poem、FastAPI、RPoem、RPoRea;畫面可選 HTML5、CSS3、TypeScript、Bootstrap;資料庫可選 MySQL、MariaDB、PostgreSQL、SQLite,自由搭配。COBOL 會被改寫;C、C++、Fortran 維持原樣執行,由網頁呼叫(暫不改寫成 Rust)。把 COBOL 從一個系統轉到另一個系統用 `pl2pl cobol`。
### 한국어
쉽게 말하면, 오래된 프로그램(COBOL, C, C++, Fortran)을 지금의 웹에 올려 주는 기능입니다. 서버는 Axum, Poem, FastAPI, RPoem, RPoRea 중에서, 화면은 HTML5, CSS3, TypeScript, Bootstrap 중에서, 데이터베이스는 MySQL, MariaDB, PostgreSQL, SQLite 중에서 마음대로 조합할 수 있습니다. COBOL은 다시 써서 변환하고, C·C++·Fortran은 그대로 실행하면서 웹에서 부릅니다(아직 Rust로 다시 쓰지는 않습니다). COBOL을 다른 OS의 COBOL로 바꾸는 일은 `pl2pl cobol`이 합니다.
### Deutsch
Einfach gesagt: Alte Programme (COBOL, C, C++, Fortran) werden ins heutige Web gebracht. Man wählt Server (Axum, Poem, FastAPI, RPoem oder RPoRea), Oberfläche (HTML5, CSS3, TypeScript, Bootstrap) und Datenbank (MySQL, MariaDB, PostgreSQL oder SQLite) frei im erlaubten Rahmen. COBOL wird umgeschrieben; C, C++ und Fortran laufen unverändert und werden vom Web aus aufgerufen (noch nicht nach Rust umgeschrieben). COBOL für ein anderes Betriebssystem erzeugt `pl2pl cobol`.
### Français
En termes simples : on met de vieux programmes (COBOL, C, C++, Fortran) sur le web d'aujourd'hui. On choisit le serveur (Axum, Poem, FastAPI, RPoem ou RPoRea), l'écran (HTML5, CSS3, TypeScript, Bootstrap) et la base de données (MySQL, MariaDB, PostgreSQL ou SQLite) dans les combinaisons permises. Le COBOL est réécrit ; C, C++ et Fortran tournent tels quels et sont appelés par le web (pas encore réécrits en Rust). Pour du COBOL d'un autre système, voir `pl2pl cobol`.
### Español
En palabras sencillas: lleva programas antiguos (COBOL, C, C++, Fortran) a la web de hoy. Eliges el servidor (Axum, Poem, FastAPI, RPoem o RPoRea), la pantalla (HTML5, CSS3, TypeScript, Bootstrap) y la base de datos (MySQL, MariaDB, PostgreSQL o SQLite) con las combinaciones permitidas. El COBOL se reescribe; C, C++ y Fortran siguen funcionando tal cual y la web los llama (todavía no se reescriben en Rust). Para COBOL de otro sistema operativo, usa `pl2pl cobol`.
### Português
Em palavras simples: leva programas antigos (COBOL, C, C++, Fortran) para a web de hoje. Você escolhe o servidor (Axum, Poem, FastAPI, RPoem ou RPoRea), a tela (HTML5, CSS3, TypeScript, Bootstrap) e o banco de dados (MySQL, MariaDB, PostgreSQL ou SQLite) nas combinações permitidas. O COBOL é reescrito; C, C++ e Fortran continuam rodando como estão e a web os chama (ainda não são reescritos em Rust). Para COBOL de outro sistema, use `pl2pl cobol`.
### Русский
Простыми словами: старые программы (COBOL, C, C++, Fortran) переносятся в сегодняшний веб. Сервер выбирается из Axum, Poem, FastAPI, RPoem, RPoRea; интерфейс — из HTML5, CSS3, TypeScript, Bootstrap; база данных — из MySQL, MariaDB, PostgreSQL, SQLite, в допустимых сочетаниях. COBOL переписывается; C, C++ и Fortran работают как есть, а веб их вызывает (в Rust пока не переписываются). COBOL для другой ОС делает `pl2pl cobol`.

**確認したこと / Verified**: Axum(COBOL 7本・C・C++)を起動して `POST /cobol_to.Program/Run` が正しい出力を返す。全組み合わせの検査は単体試験。**まだ / Not yet**: gfortran の実機ビルド(この PC の WSL に無い)、C/C++/Fortran→Rust の書き換え、COBOL→別 OS の COBOL の拡張、Axum 以外の起動確認。**進め方 / How we work**: 未了は確認せず自動で進める(元のリポジトリの削除だけは最後に確認)。 / Unfinished items go on automatically without asking (only deleting the original repos is confirmed first).

## 現状と完成予定 (2026-11-20) / Status and plan
### 日本語
**現状(確認済み)**: COBOL の方言・書式を自動判定し、FastAPI/MFastAPI/Rust+Poem/Rust+RPoem へ変換(Connect)。IF/EVALUATE/PERFORM/GO TO/小数/OCCURS/REDEFINES/88/STRING/UNSTRING/INSPECT(一部)/COPY/CALL/順編成ファイル I/O/ACCEPT FROM DATE・TIME・DAY・DAY-OF-WEEK。単体試験 11 件、Python と Rust の出力一致。
**完成予定(未了・この順で確認なしに進める)**: ①ON SIZE ERROR(PICTURE の桁数情報が要る) ②INSPECT BEFORE/AFTER ③SORT/MERGE/SEARCH ④索引・相対ファイルと REWRITE/START/DELETE ⑤COMP-3 などのバイナリ表現 ⑥参照修飾の書き込み・多次元添字 ⑦実際のメインフレーム由来の大きなプログラムでの通し ⑧EXEC SQL/CICS(未対応のまま TODO 表示)。
**約束**: 実機で確かめてから「できた」と書く/未対応は黙って落とさず明示する/秘密は書かない/新しい REST は作らない(Connect のみ)。
### English
**Current state (verified)**: Auto-detects the COBOL dialect and format and converts to FastAPI/MFastAPI/Rust+Poem/Rust+RPoem (Connect). Covers IF/EVALUATE/PERFORM/GO TO, decimals, OCCURS, REDEFINES, level 88, STRING/UNSTRING, part of INSPECT, COPY, CALL, sequential file I/O and ACCEPT FROM DATE/TIME/DAY/DAY-OF-WEEK. 11 unit tests; Python and Rust outputs match.
**Planned (open, continued automatically in this order)**: (1) ON SIZE ERROR (needs per-field PICTURE digit info), (2) INSPECT BEFORE/AFTER, (3) SORT/MERGE/SEARCH, (4) indexed/relative files with REWRITE/START/DELETE, (5) COMP-3 and other binary representations, (6) reference-modification writes and multi-dimensional subscripts, (7) an end-to-end run on large real mainframe programs, (8) EXEC SQL/CICS (stays flagged as TODO).
**Rules**: say "done" only after checking on a real system; never drop unsupported items silently, always flag them; keep secrets out; no new REST (Connect only).


### 追記 (2026-11-20) 紹介ページ / Intro pages
**日本語**: 紹介ページ https://easy-web.tokyo/COBOL_to/ とデモ https://easy-web.tokyo/COBOL_to/demo/(実際に変換・実行した入出力の記録)を公開した。VPS の `/root/easy-web-intros/COBOL_to/`(静的、ポート 8141)と `domains.toml` の `path_prefix` で配信。生成は runo の `scripts/gen-converters-intro.py`。日英のみで、他言語は準備中。
**English**: The intro page https://easy-web.tokyo/COBOL_to/ and the demo https://easy-web.tokyo/COBOL_to/demo/ (a record of real conversion/run input and output) are live. They are served statically from `/root/easy-web-intros/COBOL_to/` on the VPS (port 8141) through a `path_prefix` entry in `domains.toml`; generated by `scripts/gen-converters-intro.py` in runo. Japanese and English only; other languages are in preparation.

## 多言語: 現状と完成予定 / Status and plan in 10 languages (ja, en above)

### 简体中文 — 现状与计划 (2026-11-20)
*以下为 AI 翻译,并已由 AI 按母语水平审校;尚未经人类母语者校对。*

**现状(已验证)**: 自动识别 COBOL 的方言、版本和书写格式,并转换为 FastAPI / MFastAPI / Rust+Poem / Rust+RPoem(API 为 Connect)。支持 IF / EVALUATE / PERFORM / GO TO、定点小数、OCCURS、REDEFINES、88 级条件名、STRING / UNSTRING / INSPECT(部分)、COPY、CALL、顺序文件 I/O、ACCEPT FROM DATE/TIME。不支持的语法会标为 TODO(COBOL_to)。单元测试 11 项通过,Python 与 Rust 输出一致。

**计划(未完成,按此顺序自动推进)**: ON SIZE ERROR、INSPECT BEFORE/AFTER、SORT / SEARCH、索引文件、COMP-3、EXEC SQL/CICS,以及在大型真实主机程序上的完整验证。

原则:先在真实环境确认再说“完成”;不支持的内容绝不悄悄丢弃,而是明确标出;不写入机密;不新增 REST(仅 Connect)。

### 繁體中文(台灣) — 現況與計畫 (2026-11-20)
*以下為 AI 翻譯,並已由 AI 以母語水準審校;尚未經人類母語者校對。*

**現況(已驗證)**: 自動辨識 COBOL 的方言、版本與書寫格式,並轉換為 FastAPI / MFastAPI / Rust+Poem / Rust+RPoem(API 為 Connect)。支援 IF / EVALUATE / PERFORM / GO TO、定點小數、OCCURS、REDEFINES、88 層條件名、STRING / UNSTRING / INSPECT(部分)、COPY、CALL、循序檔案 I/O、ACCEPT FROM DATE/TIME。不支援的語法會標為 TODO(COBOL_to)。單元測試 11 項通過,Python 與 Rust 輸出一致。

**計畫(未完成,依此順序自動推進)**: ON SIZE ERROR、INSPECT BEFORE/AFTER、SORT / SEARCH、索引檔案、COMP-3、EXEC SQL/CICS,以及在大型真實主機程式上的完整驗證。

原則:先在真實環境確認才說「完成」;不支援的內容絕不悄悄丟棄,而是明確標出;不寫入機密;不新增 REST(僅 Connect)。

### 한국어 — 현황과 계획 (2026-11-20)
*아래는 AI가 번역하고 AI가 원어민 수준으로 검수한 것이며, 사람 원어민의 검수는 아직 받지 않았습니다.*

**현황(검증 완료)**: COBOL의 방언·버전·소스 형식을 자동 판별하여 FastAPI / MFastAPI / Rust+Poem / Rust+RPoem으로 변환합니다(API는 Connect). IF / EVALUATE / PERFORM / GO TO, 고정 소수점, OCCURS, REDEFINES, 88 조건명, STRING / UNSTRING / INSPECT(일부), COPY, CALL, 순차 파일 I/O, ACCEPT FROM DATE/TIME를 지원합니다. 지원하지 않는 구문은 TODO(COBOL_to)로 표시합니다. 단위 테스트 11건 통과, Python과 Rust의 출력이 같습니다.

**계획(미완료, 이 순서대로 자동 진행)**: ON SIZE ERROR, INSPECT BEFORE/AFTER, SORT / SEARCH, 인덱스 파일, COMP-3, EXEC SQL/CICS, 그리고 대규모 실제 메인프레임 프로그램에서의 전체 검증.

원칙: 실제 환경에서 확인한 뒤에만 "완료"라고 씁니다. 지원하지 않는 항목은 조용히 버리지 않고 명확히 표시합니다. 비밀 정보는 쓰지 않으며 새 REST는 만들지 않습니다(Connect만).

### Deutsch — Stand und Plan (2026-11-20)
*Die folgenden Texte wurden von einer KI übersetzt und von einer KI auf muttersprachliche Natürlichkeit geprüft; eine Prüfung durch menschliche Muttersprachler steht noch aus.*

**Aktueller Stand (geprüft)**: Erkennt Dialekt, Version und Quellformat von COBOL automatisch und wandelt nach FastAPI / MFastAPI / Rust+Poem / Rust+RPoem um (API: Connect). Unterstützt: IF / EVALUATE / PERFORM / GO TO, Festkommazahlen, OCCURS, REDEFINES, Bedingungsnamen der Stufe 88, STRING / UNSTRING / INSPECT (teilweise), COPY, CALL, sequenzielle Datei-E/A, ACCEPT FROM DATE/TIME. Nicht Unterstütztes wird als TODO(COBOL_to) markiert. 11 Unit-Tests bestehen; Python und Rust liefern dieselbe Ausgabe.

**Geplant (offen, wird in dieser Reihenfolge automatisch fortgesetzt)**: ON SIZE ERROR, INSPECT BEFORE/AFTER, SORT / SEARCH, indizierte Dateien, COMP-3, EXEC SQL/CICS sowie ein vollständiger Durchlauf mit großen echten Mainframe-Programmen.

Regeln: „Fertig“ erst nach Prüfung auf einem echten System; Nicht-Unterstütztes nie stillschweigend weglassen, sondern klar kennzeichnen; keine Geheimnisse ablegen; keine neue REST-API (nur Connect).

### Français — État et plan (2026-11-20)
*Les textes suivants ont été traduits par une IA et relus par une IA pour leur naturel ; ils n'ont pas encore été relus par des locuteurs natifs humains.*

**État actuel (vérifié)**: Détecte automatiquement le dialecte, la version et le format source de COBOL et convertit vers FastAPI / MFastAPI / Rust+Poem / Rust+RPoem (API : Connect). Prend en charge IF / EVALUATE / PERFORM / GO TO, décimaux en virgule fixe, OCCURS, REDEFINES, noms de condition de niveau 88, STRING / UNSTRING / INSPECT (en partie), COPY, CALL, E/S de fichiers séquentiels, ACCEPT FROM DATE/TIME. Ce qui n'est pas pris en charge est signalé par TODO(COBOL_to). 11 tests unitaires réussis ; Python et Rust donnent la même sortie.

**Prévu (ouvert, poursuivi automatiquement dans cet ordre)**: ON SIZE ERROR, INSPECT BEFORE/AFTER, SORT / SEARCH, fichiers indexés, COMP-3, EXEC SQL/CICS, ainsi qu'un essai complet sur de grands programmes mainframe réels.

Règles : n'écrire « terminé » qu'après vérification sur un système réel ; ne jamais abandonner en silence ce qui n'est pas pris en charge, mais le signaler clairement ; aucun secret ; pas de nouvelle API REST (Connect uniquement).

### Español — Estado y plan (2026-11-20)
*Los textos siguientes fueron traducidos por una IA y revisados por una IA en cuanto a naturalidad; aún no los ha revisado un hablante nativo humano.*

**Estado actual (verificado)**: Detecta automáticamente el dialecto, la versión y el formato fuente de COBOL y convierte a FastAPI / MFastAPI / Rust+Poem / Rust+RPoem (API: Connect). Admite IF / EVALUATE / PERFORM / GO TO, decimales de coma fija, OCCURS, REDEFINES, nombres de condición de nivel 88, STRING / UNSTRING / INSPECT (en parte), COPY, CALL, E/S de archivos secuenciales y ACCEPT FROM DATE/TIME. Lo que no se admite se marca con TODO(COBOL_to). 11 pruebas unitarias correctas; Python y Rust dan la misma salida.

**Previsto (pendiente, se continúa automáticamente en este orden)**: ON SIZE ERROR, INSPECT BEFORE/AFTER, SORT / SEARCH, archivos indexados, COMP-3, EXEC SQL/CICS y una prueba completa con programas grandes reales de mainframe.

Reglas: decir «terminado» solo tras comprobarlo en un sistema real; nunca descartar en silencio lo no admitido, sino señalarlo claramente; sin secretos; sin nuevas API REST (solo Connect).

### Português (BR) — Estado e plano (2026-11-20)
*Os textos a seguir foram traduzidos por uma IA e revisados por uma IA quanto à naturalidade; ainda não foram revisados por falantes nativos humanos.*

**Estado atual (verificado)**: Detecta automaticamente o dialeto, a versão e o formato-fonte do COBOL e converte para FastAPI / MFastAPI / Rust+Poem / Rust+RPoem (API: Connect). Suporta IF / EVALUATE / PERFORM / GO TO, decimais de ponto fixo, OCCURS, REDEFINES, nomes de condição de nível 88, STRING / UNSTRING / INSPECT (em parte), COPY, CALL, E/S de arquivos sequenciais e ACCEPT FROM DATE/TIME. O que não é suportado é marcado como TODO(COBOL_to). 11 testes unitários passam; Python e Rust dão a mesma saída.

**Planejado (em aberto, seguido automaticamente nesta ordem)**: ON SIZE ERROR, INSPECT BEFORE/AFTER, SORT / SEARCH, arquivos indexados, COMP-3, EXEC SQL/CICS e um teste completo com programas grandes reais de mainframe.

Regras: só dizer «concluído» depois de verificar em um sistema real; nunca descartar em silêncio o que não é suportado, mas sinalizar claramente; sem segredos; sem novas APIs REST (somente Connect).

### Русский — Состояние и план (2026-11-20)
*Следующие тексты переведены ИИ и проверены ИИ на естественность звучания; носители языка — люди — их пока не проверяли.*

**Текущее состояние (проверено)**: Автоматически определяет диалект, версию и формат исходника COBOL и преобразует в FastAPI / MFastAPI / Rust+Poem / Rust+RPoem (API — Connect). Поддерживает IF / EVALUATE / PERFORM / GO TO, десятичные числа с фиксированной точкой, OCCURS, REDEFINES, имена условий уровня 88, STRING / UNSTRING / INSPECT (частично), COPY, CALL, ввод-вывод последовательных файлов, ACCEPT FROM DATE/TIME. Неподдерживаемые конструкции помечаются как TODO(COBOL_to). 11 модульных тестов проходят; Python и Rust выдают одинаковый вывод.

**Планируется (не завершено, продолжается автоматически в таком порядке)**: ON SIZE ERROR, INSPECT BEFORE/AFTER, SORT / SEARCH, индексированные файлы, COMP-3, EXEC SQL/CICS, а также полная проверка на больших реальных мейнфрейм-программах.

Правила: писать «готово» только после проверки на реальной системе; неподдерживаемое никогда не отбрасывать молча, а явно помечать; не хранить секреты; не создавать новый REST (только Connect).

## アラビア語・ペルシャ語(イラン) / Arabic and Persian (Iran) — AI翻訳+AIネイティブ確認、人間のネイティブ確認はまだ

### العربية — الحالة والخطة (2026-11-20)

<div dir="rtl" lang="ar">

*النص التالي ترجمه الذكاء الاصطناعي وراجعه الذكاء الاصطناعي من حيث سلامة اللغة، ولم يراجعه بعدُ متحدث أصلي من البشر.*

**الحالة الحالية (تم التحقق منها)**: يكتشف تلقائيًا لهجة COBOL وإصدارها وتنسيق المصدر، ويحوّل البرنامج إلى FastAPI / MFastAPI / Rust+Poem / Rust+RPoem (الواجهة: Connect). يدعم IF / EVALUATE / PERFORM / GO TO، والأعداد العشرية ذات الفاصلة الثابتة، وOCCURS، وREDEFINES، وأسماء الشروط من المستوى 88، وSTRING / UNSTRING / INSPECT (جزئيًا)، وCOPY، وCALL، وإدخال/إخراج الملفات التسلسلية، وACCEPT FROM DATE/TIME. تُوسَم العبارات غير المدعومة بـ TODO(COBOL_to). نجح 11 اختبارًا من اختبارات الوحدة، ويعطي Python وRust المخرجات نفسها.

**المخطَّط له (غير مكتمل، يُتابَع تلقائيًا بهذا الترتيب)**: ON SIZE ERROR وINSPECT BEFORE/AFTER وSORT / SEARCH والملفات المفهرسة وCOMP-3 وEXEC SQL/CICS، إضافةً إلى تجربة كاملة على برامج حواسيب مركزية (mainframe) حقيقية كبيرة.

المبادئ: لا نكتب «تم» إلا بعد التحقق على نظام حقيقي؛ لا نُسقِط ما لا يُدعَم بصمت بل نضع عليه علامة واضحة؛ لا نكتب أسرارًا؛ لا نضيف واجهات REST جديدة (Connect فقط).

</div>

### فارسی (ایران) — وضعیت و برنامه (2026-11-20)

<div dir="rtl" lang="fa">

*متن زیر را هوش مصنوعی ترجمه و از نظر روانی زبان بازبینی کرده است؛ هنوز هیچ فارسی‌زبانِ بومیِ انسانی آن را بررسی نکرده است.*

**وضعیت کنونی (بررسی‌شده)**: گویش، نسخه و قالب کد منبع COBOL را خودکار تشخیص می‌دهد و برنامه را به FastAPI / MFastAPI / Rust+Poem / Rust+RPoem تبدیل می‌کند (API از نوع Connect است). از IF / EVALUATE / PERFORM / GO TO، اعداد اعشاری با ممیز ثابت، OCCURS، REDEFINES، نام شرط سطح ۸۸، STRING / UNSTRING / INSPECT (بخشی)، COPY، CALL، ورودی/خروجی فایل ترتیبی و ACCEPT FROM DATE/TIME پشتیبانی می‌کند. نحو پشتیبانی‌نشده با TODO(COBOL_to) علامت می‌خورد. ۱۱ آزمون واحد موفق است و خروجی Python و Rust یکسان است.

**برنامه‌ریزی‌شده (ناتمام؛ به‌صورت خودکار و با همین ترتیب ادامه می‌یابد)**: ON SIZE ERROR، INSPECT BEFORE/AFTER، SORT / SEARCH، فایل‌های اندیس‌دار، COMP-3، EXEC SQL/CICS و نیز آزمایش کامل روی برنامه‌های بزرگ و واقعی رایانهٔ مرکزی (مین‌فریم).

اصول: فقط پس از بررسی روی یک سامانهٔ واقعی می‌نویسیم «انجام شد»؛ آنچه پشتیبانی نمی‌شود هرگز بی‌صدا حذف نمی‌شود و آشکارا علامت می‌خورد؛ رازی (مانند گذرواژه و کلید) نمی‌نویسیم؛ REST جدید نمی‌سازیم (فقط Connect).

</div>


## 2026-11-21 更新: ON SIZE ERROR に対応 / Update: ON SIZE ERROR is supported

**日本語**: `ADD`・`SUBTRACT`・`MULTIPLY`・`DIVIDE`・`COMPUTE` の `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` を変換できるようになりました。結果が代入先の PICTURE の桁数をあふれる、または 0 で割るときは、代入せずエラー側の文を実行します(そうでなければ代入して `NOT ON SIZE ERROR` 側を実行)。Python と Rust の両方で、`ADD 5`→`OK1`、`ADD 20`(あふれ)→`OVER2`、`DIVIDE 10 BY 0`→`ZERO`、最終値 `95` と同じ出力になることを実際に動かして確認しました。単体試験は 12 件成功です。**まだ**: `ADD a b TO c d` のように対象が複数ある形は、あふれの判定をせず `TODO(COBOL_to)` を残します。他の言語の節にある「完成予定」の一覧は、毎朝の翻訳更新で順に直します(それまでは古い記述が残ります)。残りの完成予定: INSPECT BEFORE/AFTER、SORT/MERGE/SEARCH、索引・相対ファイル、COMP-3、EXEC SQL/CICS。

**English**: `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` on `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` and `COMPUTE` is now converted. When the result overflows the target's PICTURE digits, or the divisor is zero, the target is left unchanged and the error branch runs (otherwise the value is stored and the `NOT ON SIZE ERROR` branch runs). We ran the generated Python and Rust: `ADD 5` -> `OK1`, `ADD 20` (overflow) -> `OVER2`, `DIVIDE 10 BY 0` -> `ZERO`, final value `95`, identical in both. 12 unit tests pass. **Not yet**: forms with several targets, such as `ADD a b TO c d`, get no overflow check and keep a `TODO(COBOL_to)`. The "planned" lists in the other language sections will be corrected by the daily translation update (until then they are stale). Still planned: INSPECT BEFORE/AFTER, SORT/MERGE/SEARCH, indexed and relative files, COMP-3, EXEC SQL/CICS.