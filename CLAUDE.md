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

