# COBOL_to — 開発メモ / Development notes

**日本語 → English**

## 日本語

### 役割
COBOL(各方言・各版・固定/自由形式)を自動判定し、Python(FastAPI/MFastAPI)または Rust(Poem/RPoem)のプロジェクトへ変換するツール。作業場所は `F:\COBOL_to`、GitHub は `aon-co-jp/COBOL_to`(公開、MIT)。`COBOL2` → `COBOL-to` → `COBOL_to` と改名した経緯があるが、**履歴は新規**(旧名は残っていない)。

### 構成
- `src/detect.rs` 方言・規格・書式の判定(キーワードと桁位置の根拠つき)
- `src/convert.rs` 字句解析 → 共通 IR(`Program`/`Var`/`Stmt`)→ `to_rust` / `to_python`。単体テストは `hello_roundtrip`
- `src/main.rs` フォルダ選択(`rfd`)、変換先の選択、`.cbl/.cob/.cobol/.pco` の再帰走査、雛形生成、`COBOL_to-REPORT.md`
- `installer/` Windows(`cobol_to.iss`)、Linux(`build-deb.sh`)、macOS(`build-pkg.sh`)。`.github/workflows/release.yml` が全 OS を作って試験
- `NOTIFY.md` / `notify.example.toml` 更新通知(version-control の `[notify] email_to` に 2 件)

### 方針(守ること)
- **REST は作らない**。API は Connect(`POST /cobol_to.Program/Run`)。JSON は RS-JSON に寄せる方針(現状は未使用の小さなコード)。
- **未対応の構文は黙って落とさない**(`TODO(COBOL_to)`)。生成物の依存は**固定しない**。
- RPoem 向けには Bootstrap・TypeScript・Next.js・Node.js を**使わない**(RPoem は Rust→WASM)。
- Windows 配布は `.exe`(Inno Setup)。PowerShell スクリプトを利用者向けの入口にしない。
- 走査の起点(`root`/`owner`)は各ユーザーが指定する。`F:/` や `aon-co-jp` は作者の例。
- メールアドレスは `config.toml`(git 管理外)に置き、リポジトリへ入れない。
- 文書は日本語と英語(通知文書のみ日英独露。ドイツ語・ロシア語は AI 翻訳でネイティブ検証前)。

### 環境の落とし穴
- この PC の Bash では `cargo` が PATH に無い。**PowerShell で `$env:PATH="$env:USERPROFILE\.cargo\bin;$env:PATH"`** を前置きする(WSL の Ubuntu には cargo あり)。
- **`cargo test` では `target\debug\cobol_to.exe` は更新されない**。変更後の実行確認は `cargo build` してから。
- Bash の heredoc に `'` を含む長い内容を入れると構文エラーになる。ファイルは Write ツールで作る。
- PowerShell 5.1 は `git push` の進行表示を赤い「エラー」で出す(終了コードと `git status -sb` で判断)。日本語を含むファイルを PowerShell で書かない(文字化け)。
- Windows で `Select-String` を一時出力フォルダのディレクトリに向けるとアクセス拒否になる(ファイルを個別に指定する)。

### 状態(2026-11-18)
- 手元で確認済み: テスト成功、Windows x64 のインストール/実行/アンインストール、Linux amd64 の `dpkg -i`/実行/`-r`(WSL)。
- **現在: `v0.1.2` の Release 公開済み、CI 全6ジョブ成功**(v0.1.1 は Linux 試験スクリプトの `command -v` の hash 残りと macOS arm64 の runner `macos-14` 未対応で失敗 → 552c5cb で修正)。以下は経緯:
- **v0.1.0 は失敗(Release 無し・タグだけ残る)**: Linux は CI に wayland 開発ライブラリが無くビルド失敗(実行時も `libwayland-client.so.0` に依存)、macOS x64 の runner `macos-13` は廃止で待機のまま。Windows x64/ARM64 は成功。修正(b2180c8・da0cf50)して **`v0.1.1` を push、リリース CI(run 44166948447)を実行中**。Linux arm64・macOS は v0.1.1 が初めての確認。タグは付け直さず次は `v0.1.2` で出す。
- 教訓: 手元(WSL)に開発ライブラリが入っていると CI の欠落に気づけない。`ldd` で実行時依存を調べて `.deb` の `Depends` に入れる。
- version-control は `root = "F:/"` なので `F:\COBOL_to` を自動で走査する(動作確認は走査が長く未了)。通知メールは `email_to` の 2 件(手元と VPS の両方の `config.toml`)。VPS で `version-control.service` を実行して `e-mail: sent` を確認、受信箱への到達は未確認、LINE は未設定。**2 件とも届いていない(利用者報告)**: SMTP は 3 回とも受理(`SMTP send failed` 0 件)でコードのバグではない。送信元は `smtp.lolipop.jp` 経由の `aon.co.jp` だが、`aon.co.jp` には **TXT(SPF)も `_dmarc` も無い**(DNS は value-domain)ため、Gmail に迷惑メール扱い/破棄されている疑い。迷惑メール欄・送信元受信箱の返送を確認し、SPF(Lolipop 指定値を確認)・DMARC を追加する(DNS は利用者の操作)。

## English

### Role
A tool that auto-detects COBOL (dialects, standards, fixed/free format) and converts to Python (FastAPI/MFastAPI) or Rust (Poem/RPoem) projects. Working copy `F:\COBOL_to`; GitHub `aon-co-jp/COBOL_to` (public, MIT). It was renamed `COBOL2` → `COBOL-to` → `COBOL_to`, but the **history is fresh** (no old names remain).

### Layout
- `src/detect.rs` dialect/standard/format detection with evidence
- `src/convert.rs` tokenizer → shared IR (`Program`/`Var`/`Stmt`) → `to_rust` / `to_python`; unit test `hello_roundtrip`
- `src/main.rs` folder picker (`rfd`), target choice, recursive scan of `.cbl/.cob/.cobol/.pco`, scaffolding, `COBOL_to-REPORT.md`
- `installer/` Windows (`cobol_to.iss`), Linux (`build-deb.sh`), macOS (`build-pkg.sh`); `.github/workflows/release.yml` builds and tests every OS
- `NOTIFY.md` / `notify.example.toml` update notifications (two addresses in version-control's `[notify] email_to`)

### Rules
- **No REST.** The API is Connect (`POST /cobol_to.Program/Run`). JSON should move to RS-JSON (currently not used by this small code).
- **Never drop unsupported syntax silently** (`TODO(COBOL_to)`). Do **not pin** dependencies of generated projects.
- No Bootstrap/TypeScript/Next.js/Node.js for the RPoem target (RPoem is Rust→WASM).
- Windows ships as `.exe` (Inno Setup); PowerShell scripts are not a user entry point.
- The scan start points (`root`/`owner`) are specified by each user; `F:/` and `aon-co-jp` are the author's examples.
- E-mail addresses live in `config.toml` (git-ignored), never in the repository.
- Docs are Japanese and English (only the notification doc is JA/EN/DE/RU; German and Russian are AI translations, not native-checked).

### Environment pitfalls
- `cargo` is not on PATH in this PC's Bash. Use **PowerShell with `$env:PATH="$env:USERPROFILE\.cargo\bin;$env:PATH"`** (WSL Ubuntu has cargo).
- **`cargo test` does not refresh `target\debug\cobol_to.exe`**; run `cargo build` before checking behaviour.
- Long Bash heredocs containing `'` fail to parse; create files with the Write tool.
- PowerShell 5.1 shows `git push` progress as a red "error" (judge by exit code and `git status -sb`); do not write Japanese files from PowerShell (mojibake).
- `Select-String` on a directory in a temp output folder gives access-denied (name files individually).

### Status (2026-11-18)
- Verified locally: tests pass, Windows x64 install/run/uninstall, Linux amd64 `dpkg -i`/run/`-r` (WSL).
- **Now: Release `v0.1.2` is published and all 6 CI jobs passed** (v0.1.1 failed on a stale bash hash in the Linux smoke test's `command -v` and on the unsupported macOS arm64 runner `macos-14`; fixed in 552c5cb). History:
- **v0.1.0 failed (no Release; only the tag remains)**: Linux failed to build because CI lacked the wayland dev libraries (the binary also needs `libwayland-client.so.0` at run time), and the macOS x64 runner `macos-13` is retired and stayed queued. Windows x64/ARM64 passed. Fixed (b2180c8, da0cf50), **`v0.1.1` is pushed and its release CI (run 44166948447) is running**. Linux arm64 and macOS are first verified by v0.1.1. Do not move tags; ship the next fix as `v0.1.2`.
- Lesson: dev libraries present locally (WSL) hide gaps in CI. Check run-time deps with `ldd` and list them in the `.deb` `Depends`.
- version-control uses `root = "F:/"`, so `F:\COBOL_to` is scanned automatically (a live check was not finished; the scan is long). Notification mail goes to the two `email_to` addresses (both the local and the VPS `config.toml`). Running `version-control.service` on the VPS logged `e-mail: sent`; delivery to the inboxes is unconfirmed; LINE is not configured. **The user reports neither mail arrived**: SMTP accepted all three sends (0 `SMTP send failed`), so it is not a code bug. The sender is `aon.co.jp` via `smtp.lolipop.jp`, but `aon.co.jp` has **no TXT (SPF) record and no `_dmarc`** (DNS at value-domain), so Gmail probably files it as spam or drops it. Check the spam folder and bounces in the sender mailbox, then add SPF (confirm Lolipop's value) and DMARC (DNS is the user's action).

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

## 2026-11-21 更新: INSPECT の BEFORE/AFTER INITIAL に対応 / Update: INSPECT BEFORE/AFTER INITIAL is supported

**日本語**: `INSPECT ... TALLYING / REPLACING / CONVERTING` に `AFTER INITIAL a` と `BEFORE INITIAL b`(両方でも可)を付けた形を変換できます。対象は「a が最初に現れた直後から、b が最初に現れる直前まで」で、a が見つからなければ範囲は空、b が見つからなければ末尾までです。`"AXBXCXDXE"` に `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` を実行すると `AXB-C-DXE` になり、`TALLYING ... AFTER INITIAL "B"` の数は 3 で、Python と Rust の出力が一致することを実際に動かして確認しました。単体試験は 13 件成功です。**まだ**: `FOR CHARACTERS` / `LEADING` / `REPLACING LEADING` などは `TODO(COBOL_to)` のままです。残りの完成予定: SORT/MERGE/SEARCH、索引・相対ファイル、COMP-3、EXEC SQL/CICS。

**English**: `INSPECT ... TALLYING / REPLACING / CONVERTING` with `AFTER INITIAL a` and `BEFORE INITIAL b` (either or both) is now converted. The affected region runs from just after the first `a` to just before the first `b`; if `a` is not found the region is empty, and if `b` is not found it runs to the end. `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` on `"AXBXCXDXE"` gives `AXB-C-DXE`, and `TALLYING ... AFTER INITIAL "B"` counts 3; the generated Python and Rust were run and give identical output. 13 unit tests pass. **Not yet**: `FOR CHARACTERS`, `LEADING` and `REPLACING LEADING` still stay as `TODO(COBOL_to)`. Still planned: SORT/MERGE/SEARCH, indexed and relative files, COMP-3, EXEC SQL/CICS.
## 世界の言語での 2026-11-21 更新(ON SIZE ERROR・INSPECT BEFORE/AFTER) / 2026-11-21 update in 10 more languages (AI翻訳+AI確認、人間のネイティブ確認はまだ)

### 简体中文 — 更新 (2026-11-21)

*以下为 AI 翻译,并已由 AI 按母语水平审校;尚未经人类母语者校对。*

现已支持 `ADD`、`SUBTRACT`、`MULTIPLY`、`DIVIDE`、`COMPUTE` 的 `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx`:当结果超出目标 PICTURE 的位数,或除数为 0 时,不会赋值,而是执行错误分支;否则赋值并执行 `NOT ON SIZE ERROR` 分支。同时支持在 `INSPECT ... TALLYING / REPLACING / CONVERTING` 后加 `AFTER INITIAL a` 和 `BEFORE INITIAL b`(可只写其中一个):作用范围从第一个 a 之后开始,到第一个 b 之前结束;找不到 a 时范围为空,找不到 b 时一直到末尾。我们实际运行了生成的 Python 和 Rust:`ADD 5` 得到 `OK1`,`ADD 20`(溢出)得到 `OVER2`,`DIVIDE 10 BY 0` 得到 `ZERO`;对 `"AXBXCXDXE"` 执行 `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` 得到 `AXB-C-DXE`,两种语言的输出完全一致。单元测试 13 项通过。**尚未支持**:有多个目标的形式(如 `ADD a b TO c d`)不做溢出判断,仍保留 `TODO(COBOL_to)`;`FOR CHARACTERS`、`LEADING` 仍为 `TODO(COBOL_to)`。其余计划:SORT/MERGE/SEARCH、索引文件与相对文件、COMP-3、EXEC SQL/CICS。上面各语言章节中的“计划”清单已经过时,将由每日翻译更新来修正。

### 繁體中文(台灣) — 更新 (2026-11-21)

*以下為 AI 翻譯,並已由 AI 以母語水準審校;尚未經人類母語者校對。*

現已支援 `ADD`、`SUBTRACT`、`MULTIPLY`、`DIVIDE`、`COMPUTE` 的 `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx`:當結果超出目標 PICTURE 的位數,或除數為 0 時,不會賦值,而是執行錯誤分支;否則賦值並執行 `NOT ON SIZE ERROR` 分支。同時支援在 `INSPECT ... TALLYING / REPLACING / CONVERTING` 後加上 `AFTER INITIAL a` 與 `BEFORE INITIAL b`(可只寫其中一個):作用範圍從第一個 a 之後開始,到第一個 b 之前結束;找不到 a 時範圍為空,找不到 b 時則一直到結尾。我們實際執行了產生的 Python 與 Rust:`ADD 5` 得到 `OK1`,`ADD 20`(溢位)得到 `OVER2`,`DIVIDE 10 BY 0` 得到 `ZERO`;對 `"AXBXCXDXE"` 執行 `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` 得到 `AXB-C-DXE`,兩種語言的輸出完全相同。單元測試 13 項通過。**尚未支援**:有多個目標的形式(如 `ADD a b TO c d`)不做溢位判斷,仍保留 `TODO(COBOL_to)`;`FOR CHARACTERS`、`LEADING` 仍為 `TODO(COBOL_to)`。其餘計畫:SORT/MERGE/SEARCH、索引檔案與相對檔案、COMP-3、EXEC SQL/CICS。上方各語言章節中的「計畫」清單已經過時,將由每日翻譯更新來修正。

### 한국어 — 업데이트 (2026-11-21)

*아래는 AI가 번역하고 AI가 원어민 수준으로 검수한 것이며, 사람 원어민의 검수는 아직 받지 않았습니다.*

이제 `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`, `COMPUTE`의 `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx`를 변환합니다. 결과가 대상 PICTURE의 자릿수를 넘치거나 0으로 나누면 값을 대입하지 않고 오류 쪽 문장을 실행하며, 그렇지 않으면 값을 대입한 뒤 `NOT ON SIZE ERROR` 쪽을 실행합니다. 또한 `INSPECT ... TALLYING / REPLACING / CONVERTING`에 `AFTER INITIAL a`와 `BEFORE INITIAL b`(한쪽만 써도 됨)를 붙인 형태도 지원합니다. 범위는 처음 나타난 a 바로 뒤부터 처음 나타난 b 바로 앞까지이며, a가 없으면 범위가 비고 b가 없으면 끝까지입니다. 생성된 Python과 Rust를 실제로 실행해 확인했습니다. `ADD 5`는 `OK1`, `ADD 20`(넘침)은 `OVER2`, `DIVIDE 10 BY 0`은 `ZERO`였고, `"AXBXCXDXE"`에 `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"`를 실행하면 `AXB-C-DXE`가 되어 두 언어의 출력이 같았습니다. 단위 테스트 13건이 통과했습니다. **아직 지원하지 않는 것**: `ADD a b TO c d`처럼 대상이 여러 개인 형태는 넘침 판정을 하지 않고 `TODO(COBOL_to)`로 남기며, `FOR CHARACTERS`와 `LEADING`도 `TODO(COBOL_to)`입니다. 남은 계획: SORT/MERGE/SEARCH, 인덱스 파일과 상대 파일, COMP-3, EXEC SQL/CICS. 위의 다른 언어 절에 있는 "계획" 목록은 지금은 오래된 내용이며, 매일의 번역 갱신에서 고칩니다.

### Deutsch — Aktualisierung (2026-11-21)

*Die folgenden Texte wurden von einer KI übersetzt und von einer KI auf muttersprachliche Natürlichkeit geprüft; eine Prüfung durch menschliche Muttersprachler steht noch aus.*

Jetzt werden `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` bei `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` und `COMPUTE` umgewandelt: Läuft das Ergebnis über die PICTURE-Stellen des Ziels hinaus oder ist der Divisor 0, wird nichts zugewiesen und stattdessen der Fehlerzweig ausgeführt; andernfalls wird zugewiesen und der Zweig `NOT ON SIZE ERROR` ausgeführt. Ebenso werden `INSPECT ... TALLYING / REPLACING / CONVERTING` mit `AFTER INITIAL a` und `BEFORE INITIAL b` (einzeln oder zusammen) unterstützt: Der Bereich reicht von direkt nach dem ersten a bis direkt vor dem ersten b; fehlt a, ist der Bereich leer, fehlt b, reicht er bis zum Ende. Wir haben das erzeugte Python und Rust tatsächlich ausgeführt: `ADD 5` ergibt `OK1`, `ADD 20` (Überlauf) ergibt `OVER2`, `DIVIDE 10 BY 0` ergibt `ZERO`; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` auf `"AXBXCXDXE"` ergibt `AXB-C-DXE`, und beide Sprachen liefern dieselbe Ausgabe. 13 Unit-Tests bestehen. **Noch nicht unterstützt**: Formen mit mehreren Zielen wie `ADD a b TO c d` erhalten keine Überlaufprüfung und bleiben als `TODO(COBOL_to)` stehen; `FOR CHARACTERS` und `LEADING` bleiben ebenfalls `TODO(COBOL_to)`. Weiter geplant: SORT/MERGE/SEARCH, indizierte und relative Dateien, COMP-3, EXEC SQL/CICS. Die Listen „Geplant“ in den übrigen Sprachabschnitten oben sind damit veraltet und werden durch die tägliche Übersetzungsaktualisierung korrigiert.

### Français — Mise à jour (2026-11-21)

*Les textes suivants ont été traduits par une IA et relus par une IA pour leur naturel ; ils n'ont pas encore été relus par des locuteurs natifs humains.*

`ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` est désormais converti pour `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` et `COMPUTE` : si le résultat dépasse le nombre de chiffres de la PICTURE de la cible, ou si le diviseur est 0, rien n'est affecté et la branche d'erreur s'exécute ; sinon la valeur est affectée et la branche `NOT ON SIZE ERROR` s'exécute. Les formes `INSPECT ... TALLYING / REPLACING / CONVERTING` avec `AFTER INITIAL a` et `BEFORE INITIAL b` (l'un ou l'autre, ou les deux) sont aussi prises en charge : la zone va juste après le premier a jusqu'à juste avant le premier b ; si a est introuvable, la zone est vide, et si b est introuvable, elle va jusqu'à la fin. Nous avons réellement exécuté le Python et le Rust générés : `ADD 5` donne `OK1`, `ADD 20` (dépassement) donne `OVER2`, `DIVIDE 10 BY 0` donne `ZERO` ; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` sur `"AXBXCXDXE"` donne `AXB-C-DXE`, et les deux langages produisent la même sortie. 13 tests unitaires réussis. **Pas encore pris en charge** : les formes à plusieurs cibles, comme `ADD a b TO c d`, n'ont pas de contrôle de dépassement et restent signalées par `TODO(COBOL_to)` ; `FOR CHARACTERS` et `LEADING` restent aussi `TODO(COBOL_to)`. Reste prévu : SORT/MERGE/SEARCH, fichiers indexés et relatifs, COMP-3, EXEC SQL/CICS. Les listes « Prévu » des autres sections de langue ci-dessus sont donc périmées ; la mise à jour quotidienne des traductions les corrigera.

### Español — Actualización (2026-11-21)

*Los textos siguientes fueron traducidos por una IA y revisados por una IA en cuanto a naturalidad; aún no los ha revisado un hablante nativo humano.*

Ahora se convierte `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` en `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` y `COMPUTE`: si el resultado desborda los dígitos de la PICTURE del destino, o si el divisor es 0, no se asigna nada y se ejecuta la rama de error; en caso contrario se asigna el valor y se ejecuta la rama `NOT ON SIZE ERROR`. También se admiten `INSPECT ... TALLYING / REPLACING / CONVERTING` con `AFTER INITIAL a` y `BEFORE INITIAL b` (uno solo o ambos): la zona va desde justo después de la primera a hasta justo antes de la primera b; si no se encuentra a, la zona queda vacía, y si no se encuentra b, llega hasta el final. Ejecutamos realmente el Python y el Rust generados: `ADD 5` da `OK1`, `ADD 20` (desbordamiento) da `OVER2`, `DIVIDE 10 BY 0` da `ZERO`; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` sobre `"AXBXCXDXE"` da `AXB-C-DXE`, y ambos lenguajes producen la misma salida. Pasan 13 pruebas unitarias. **Aún no admitido**: las formas con varios destinos, como `ADD a b TO c d`, no comprueban el desbordamiento y se quedan como `TODO(COBOL_to)`; `FOR CHARACTERS` y `LEADING` también siguen como `TODO(COBOL_to)`. Sigue previsto: SORT/MERGE/SEARCH, archivos indexados y relativos, COMP-3, EXEC SQL/CICS. Las listas «Previsto» de las demás secciones de idioma de arriba han quedado desactualizadas; la actualización diaria de traducciones las corregirá.

### Português (BR) — Atualização (2026-11-21)

*Os textos a seguir foram traduzidos por uma IA e revisados por uma IA quanto à naturalidade; ainda não foram revisados por falantes nativos humanos.*

Agora `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` é convertido em `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` e `COMPUTE`: se o resultado estourar os dígitos da PICTURE do destino, ou se o divisor for 0, nada é atribuído e o ramo de erro é executado; caso contrário, o valor é atribuído e o ramo `NOT ON SIZE ERROR` é executado. Também são suportados `INSPECT ... TALLYING / REPLACING / CONVERTING` com `AFTER INITIAL a` e `BEFORE INITIAL b` (um deles ou os dois): a região vai de logo depois do primeiro a até logo antes do primeiro b; se a não for encontrado, a região fica vazia, e se b não for encontrado, vai até o fim. Executamos de fato o Python e o Rust gerados: `ADD 5` dá `OK1`, `ADD 20` (estouro) dá `OVER2`, `DIVIDE 10 BY 0` dá `ZERO`; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` sobre `"AXBXCXDXE"` dá `AXB-C-DXE`, e as duas linguagens produzem a mesma saída. 13 testes unitários passam. **Ainda não suportado**: formas com vários destinos, como `ADD a b TO c d`, não têm verificação de estouro e continuam como `TODO(COBOL_to)`; `FOR CHARACTERS` e `LEADING` também continuam como `TODO(COBOL_to)`. Continua planejado: SORT/MERGE/SEARCH, arquivos indexados e relativos, COMP-3, EXEC SQL/CICS. As listas «Planejado» das demais seções de idioma acima estão desatualizadas; a atualização diária das traduções vai corrigi-las.

### Русский — Обновление (2026-11-21)

*Следующие тексты переведены ИИ и проверены ИИ на естественность звучания; носители языка — люди — их пока не проверяли.*

Теперь преобразуется `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` для `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` и `COMPUTE`: если результат не помещается в число разрядов PICTURE получателя или делитель равен 0, значение не присваивается, а выполняется ветка ошибки; иначе значение присваивается и выполняется ветка `NOT ON SIZE ERROR`. Также поддерживаются `INSPECT ... TALLYING / REPLACING / CONVERTING` с `AFTER INITIAL a` и `BEFORE INITIAL b` (с одним из них или с обоими): область начинается сразу после первого a и заканчивается сразу перед первым b; если a не найдено, область пуста, а если не найдено b — идёт до конца. Мы действительно запустили сгенерированные Python и Rust: `ADD 5` даёт `OK1`, `ADD 20` (переполнение) даёт `OVER2`, `DIVIDE 10 BY 0` даёт `ZERO`; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` для `"AXBXCXDXE"` даёт `AXB-C-DXE`, и вывод обоих языков совпадает. Проходят 13 модульных тестов. **Пока не поддерживается**: формы с несколькими получателями, например `ADD a b TO c d`, не проверяются на переполнение и остаются как `TODO(COBOL_to)`; `FOR CHARACTERS` и `LEADING` тоже остаются `TODO(COBOL_to)`. По-прежнему планируется: SORT/MERGE/SEARCH, индексированные и относительные файлы, COMP-3, EXEC SQL/CICS. Списки «Планируется» в остальных языковых разделах выше устарели; их исправит ежедневное обновление переводов.

### العربية — تحديث (2026-11-21)

<div dir="rtl" lang="ar">

*النص التالي ترجمه الذكاء الاصطناعي وراجعه الذكاء الاصطناعي من حيث سلامة اللغة، ولم يراجعه بعدُ متحدث أصلي من البشر.*

أصبح `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` مدعومًا الآن في `ADD` و`SUBTRACT` و`MULTIPLY` و`DIVIDE` و`COMPUTE`: إذا تجاوزت النتيجة عدد خانات PICTURE الخاصة بالهدف، أو كان المقسوم عليه 0، فلا تُسند القيمة ويُنفَّذ فرع الخطأ؛ وإلا تُسند القيمة ويُنفَّذ فرع `NOT ON SIZE ERROR`. ويُدعَم كذلك `INSPECT ... TALLYING / REPLACING / CONVERTING` مع `AFTER INITIAL a` و`BEFORE INITIAL b` (أحدهما أو كلاهما): يبدأ النطاق مباشرةً بعد أول a وينتهي مباشرةً قبل أول b؛ فإن لم يوجد a كان النطاق فارغًا، وإن لم يوجد b امتد إلى النهاية. شغّلنا فعليًا شيفرتَي Python وRust المولَّدتين: `ADD 5` يعطي `OK1`، و`ADD 20` (تجاوز) يعطي `OVER2`، و`DIVIDE 10 BY 0` يعطي `ZERO`؛ وتنفيذ `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` على `"AXBXCXDXE"` يعطي `AXB-C-DXE`، وكان ناتج اللغتين متطابقًا. نجح 13 اختبارًا من اختبارات الوحدة. **غير مدعوم بعدُ**: الصيغ ذات الأهداف المتعددة مثل `ADD a b TO c d` لا يُفحَص فيها التجاوز وتبقى موسومة بـ `TODO(COBOL_to)`؛ كما يبقى `FOR CHARACTERS` و`LEADING` موسومَين بـ `TODO(COBOL_to)`. ولا يزال مخططًا: SORT/MERGE/SEARCH، والملفات المفهرسة والنسبية، وCOMP-3، وEXEC SQL/CICS. وقوائم «المخطَّط له» في أقسام اللغات الأخرى أعلاه صارت قديمة، وسيصحّحها التحديث اليومي للترجمات.

</div>

### فارسی (ایران) — به‌روزرسانی (2026-11-21)

<div dir="rtl" lang="fa">

*متن زیر را هوش مصنوعی ترجمه و از نظر روانی زبان بازبینی کرده است؛ هنوز هیچ فارسی‌زبانِ بومیِ انسانی آن را بررسی نکرده است.*

اکنون `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` در `ADD`، `SUBTRACT`، `MULTIPLY`، `DIVIDE` و `COMPUTE` تبدیل می‌شود: اگر نتیجه از تعداد ارقام PICTURE مقصد بیشتر شود یا مقسوم‌علیه 0 باشد، مقداری انتساب داده نمی‌شود و شاخهٔ خطا اجرا می‌شود؛ در غیر این صورت مقدار انتساب داده می‌شود و شاخهٔ `NOT ON SIZE ERROR` اجرا می‌گردد. همچنین `INSPECT ... TALLYING / REPLACING / CONVERTING` همراه با `AFTER INITIAL a` و `BEFORE INITIAL b` (یکی از آن‌ها یا هر دو) پشتیبانی می‌شود: محدوده درست بعد از اولین a آغاز می‌شود و درست پیش از اولین b پایان می‌یابد؛ اگر a پیدا نشود محدوده خالی است و اگر b پیدا نشود تا انتها ادامه می‌یابد. کدهای تولیدشدهٔ Python و Rust را واقعاً اجرا کردیم: `ADD 5` خروجی `OK1`، `ADD 20` (سرریز) خروجی `OVER2` و `DIVIDE 10 BY 0` خروجی `ZERO` می‌دهد؛ اجرای `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` روی `"AXBXCXDXE"` نتیجهٔ `AXB-C-DXE` را می‌دهد و خروجی هر دو زبان یکسان بود. ۱۳ آزمون واحد موفق است. **هنوز پشتیبانی نمی‌شود**: شکل‌های دارای چند مقصد، مانند `ADD a b TO c d`، بررسی سرریز ندارند و با `TODO(COBOL_to)` باقی می‌مانند؛ `FOR CHARACTERS` و `LEADING` نیز همچنان `TODO(COBOL_to)` هستند. برنامهٔ باقی‌مانده: SORT/MERGE/SEARCH، فایل‌های اندیس‌دار و نسبی، COMP-3 و EXEC SQL/CICS. فهرست‌های «برنامه‌ریزی‌شده» در بخش‌های زبانی دیگرِ بالا اکنون قدیمی است و به‌روزرسانی روزانهٔ ترجمه‌ها آن را اصلاح خواهد کرد.

</div>


## 2026-11-22 現状と完成予定と課題 / Status, plan and issues

### 日本語
**現状**: COBOL を Python(FastAPI/MFastAPI)と Rust(Poem/RPoem)へ変換。数値は10の累乗で拡大した整数、ファイルは固定長テキスト。対応済み: INSPECT(TALLYING/REPLACING/LEADING/CHARACTERS)、SEARCH/SEARCH ALL、SORT/MERGE(USING/GIVING と入出力手続き、RELEASE/RETURN)、COMP-3/COMP の記録、EXEC SQL(SELECT INTO・カーソル・SQLCODE・WHENEVER・指示変数)。Python は実 sqlite3 で、Rust は疑似ハンドラーで実行確認。試験20件成功。
**完成予定**: SQLSTATE/SQLERRM、NEXT 以外の FETCH、動的 SQL(PREPARE/EXECUTE)、表の SORT、COLLATING SEQUENCE、複数項目レコードの READ INTO、真の二分探索 SEARCH ALL、索引/相対ファイル、バイト単位の COMP-3、EXEC CICS。
**課題**: Rust の SQL は実 DB 接続が無く(C 依存の SQLite を使わない方針)疑似ハンドラーでしか検証していない。COMP-3/2進はファイル入出力で桁の文字として扱う制限がある。未対応の文は黙って落とさず TODO として出力する。

### English
**Status**: Converts COBOL to Python (FastAPI/MFastAPI) and Rust (Poem/RPoem). Numbers are integers scaled by powers of ten; files are fixed-width text. Done: INSPECT (TALLYING/REPLACING/LEADING/CHARACTERS), SEARCH/SEARCH ALL, SORT/MERGE (USING/GIVING, input/output procedures, RELEASE/RETURN), COMP-3/COMP recorded, EXEC SQL (SELECT INTO, cursors, SQLCODE, WHENEVER, indicator variables). Python verified with real sqlite3, Rust with a fake handler. 20 tests pass.
**Planned**: SQLSTATE/SQLERRM, FETCH other than NEXT, dynamic SQL (PREPARE/EXECUTE), table SORT, COLLATING SEQUENCE, READ INTO for multi-field records, true binary SEARCH ALL, indexed/relative files, byte-level COMP-3, EXEC CICS.
**Issues**: Rust SQL has no real DB connection (we avoid C-based SQLite) and is verified only with a fake handler. COMP-3/binary fields are handled as digit characters in file I/O. Unsupported statements are emitted as TODO, never silently dropped.

## 2026-11-22 追記: INDICATOR キーワード形 (日本語)
`:変数 INDICATOR :指示変数` の形を、`:変数:指示変数` と同じに扱うようにしました(パラメーターと `INTO` の両方)。`tests/samples/sql4.cbl` で、Python・Rust とも `sql3.cbl` と同じ出力になることを試験しました(21件成功)。固定形式の72桁を超える行は切り捨てられるため、長い EXEC SQL は折り返して書きます。残り: SQLSTATE・動的SQL・EXEC CICS。

## 2026-11-22 追記: SQLSTATE (日本語)
ソースに `SQLSTATE` が出てくるプログラムでは、SQL 文のたびに `SQLSTATE`(PIC X(5)、未宣言なら自動で追加)を SQLCODE から導きます(0→`00000`、100→`02000`、それ以外→`HY000`)。生成した Python を実際に動かし、`00000` と `02000` が出ることを確認しました(`tests/samples/sql5.cbl`、試験22件成功)。DBごとの細かい状態コードの対応は未了です。残り: 動的SQL・EXEC CICS。

## 2026-11-22 追記: EXECUTE IMMEDIATE(動的 SQL・日本語)
- `EXEC SQL EXECUTE IMMEDIATE :変数 END-EXEC` に対応しました。SQL 文を変数の中身(前後の空白は除去)から実行し、SQLCODE を更新します(Python・Rust 両方)。文字列定数を直接渡す形は通常の SQL 文として扱います。
- 試験は23件成功。生成した Python を実行し、CREATE・INSERT・SELECT の結果 `0` と `7` が出ることを確認しました(`tests/samples/sql6.cbl`)。Rust は生成コードの確認のみで、実行は未確認です。
- 未了: PREPARE/EXECUTE(`?` の引数つき)、EXEC CICS。

## 2026-11-23 追記: PREPARE / EXECUTE ... USING(日本語)
- `EXEC SQL PREPARE 名 FROM :変数` で SQL 文を名前つきで保存し、`EXEC SQL EXECUTE 名 [USING :a, :b]` で `?` に値を渡して実行できます(Python・Rust 両方)。未準備の名前を実行すると SQLCODE は -518 です。
- 試験は24件成功。生成した Python を実行し、`INSERT ... VALUES (?)` に 5 を渡して SELECT で 5 が返ることを確認しました(`tests/samples/sql7.cbl`)。Rust は生成コードの確認のみです。
- 未了: EXEC CICS、`DESCRIBE`/SQLDA を使う完全な動的 SQL。

## 2026-11-23 追記: EXEC CICS の最小対応(日本語)
- **調査(英語・日本語のWeb検索とGitHub)**: EXEC CICS を実際に Python/Rust へ変換するオープンソースは見つかりませんでした。多くのツールは CICS 部分に TODO を付けて人手に任せ、`viper-mf` は CICS を変換せずエミュレーターで動かす方式、`cobol-py` は EXEC CICS を抜き出す解析だけです。
- **実装**: `EXEC CICS SEND TEXT FROM(変数 または '文字列')` は表示に、`RETURN` と `ABEND` は終了にします。`READ FILE`・`XCTL`・`SEND MAP` など、ほかの命令は黙って落とさず「未対応」の TODO として残します。
- 試験は25件成功(`exec_cics_minimal`)。生成した Python の文字列確認までで、画面(BMS マップ)の再現はしていません。
- 未了: SEND/RECEIVE MAP(BMS)、READ/WRITE FILE、XCTL/LINK、COMMAREA。

- 2026-11-23 追記: `EXEC CICS LINK PROGRAM('X') [COMMAREA(x)]` を CALL へ、`XCTL` は CALL のあと終了へ移すようにしました(PROGRAM が変数の動的指定、SEND/RECEIVE MAP、READ/WRITE FILE は引き続き TODO)。試験25件成功。

- 2026-11-23 update: `EXEC CICS LINK PROGRAM('X') [COMMAREA(x)]` now becomes a CALL, and `XCTL` becomes a CALL followed by a stop (dynamic PROGRAM names, SEND/RECEIVE MAP and READ/WRITE FILE remain TODO). 25 tests pass.
