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
