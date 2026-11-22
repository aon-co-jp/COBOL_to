# COBOL_to — お引越し・再開 / Porting and resume

**日本語 → English**

## 日本語

### 別の PC・別アカウントで再開する手順
1. `git clone https://github.com/aon-co-jp/COBOL_to` して `cd COBOL_to`。
2. Rust(最新の stable)を入れる。`cargo build --release` と `cargo test`。
3. Windows のインストーラーを作るなら Inno Setup 6 を入れ、`ISCC /DAppVersion=0.1.0 /DArch=x64 /DExeDir=..\target\release installer\cobol_to.iss`(出力は `dist\`)。Linux の `.deb` は `installer/build-deb.sh`、macOS の `.pkg` は macOS 上で `installer/build-pkg.sh`。
4. 動作確認: `cobol_to --src tests/samples --target rust-poem --out <出力先>`。`src/programs/hello.rs` ができ、`COBOL_to-REPORT.md` に判定が出る。
5. 更新通知は version-control の `config.toml`(git 管理外)の `[notify] email_to` に自分のメールを 2 件書く。`root` も自分のドライブ/フォルダにする。秘密(トークン等)はリポジトリに入れない。

### 未了(優先順)
1. **(完了)リリース CI**: `v0.1.2`(run 44168310290)が全6ジョブ成功、Release に deb×2・tar.gz×2・pkg×2・exe×2・`SHA256SUMS.txt`。残るのは実機での確認(Linux arm64・macOS・Windows ARM64)と macOS の署名。以下は経緯(旧記述)。**(旧)**タグ `v0.1.1`、run 44166948447。v0.1.0 は Linux のビルド失敗と macOS x64 の廃止 runner で Release 無し(Windows は成功)。v0.1.1 で Linux arm64・macOS の通し試験が初確認。失敗があればタグを付け直さず `v0.1.2` で出す。成功したら Release の添付物と SHA256SUMS を確認し、README の検証状況を更新。
2. **変換できる構文の拡充**: `IF`/`EVALUATE`/`PERFORM`(`UNTIL`・`VARYING`)/`COMPUTE`/`SUBTRACT`/`MULTIPLY`/`DIVIDE`/`STRING`/`INSPECT`/`COPY`・`REPLACING`/ファイル I/O/`CALL`/`EXEC SQL`・`EXEC CICS`、数値の小数・符号・`COMP-3`、CONTINUATION 行。テストは `tests/samples` に実プログラムを足して追加。
3. **生成物の実サーバー接続**: FastAPI/MFastAPI/Poem/RPoem に `POST /cobol_to.Program/Run`(Connect)を実装。Next.js(TypeScript+Bootstrap)の画面の自動生成。RPoem は Rust→WASM の画面。
4. **最新版への自動追従**: 生成物の依存は固定していない。`cargo add` と PyPI/npm の最新取得、version-control の登録。
5. **通知メールが届かない件**: VPS では `e-mail: sent`(SMTP は受理、コードのバグではない)だが、`norukia.jp@gmail.com` と `cloud9slack@gmail.com` には**届いていない**(利用者報告)。`aon.co.jp` に SPF(TXT)も `_dmarc` も無いのが有力な原因。迷惑メール欄と送信元(`inf…@aon.co.jp`)の返送を確認 → value-domain で SPF(Lolipop 指定値を確認)・DMARC を追加(利用者の操作)。直らない場合は別の送信経路を検討。LINE は未設定。
6. **GitHub の owner 配下を起点にした走査**: version-control には未実装(`root` のみ)。
7. **macOS の署名・公証**(Apple のシークレットが必要)、EBCDIC の自動変換、Linux の `.rpm`/AppImage。
8. **version-control による COBOL_to の実走査確認**(`root = "F:/"` で拾われるはず。走査が長いので `--dry-run --no-fetch --no-api` を時間に余裕のあるときに)。

### 注意
- この PC の Bash に `cargo` は無い(PowerShell で PATH を前置き)。`cargo test` は debug の exe を更新しない。
- 履歴は新規。旧名(`COBOL2`/`COBOL-to`)のリポジトリは作っていない。

### 再開用メッセージ(そのまま貼り付け)
> `F:\COBOL_to`(GitHub: aon-co-jp/COBOL_to)を再開してください。まず `README.md`・`CLAUDE.md`・`PORTING.md` を読むこと。COBOL の方言・版・書式を自動判定し、FastAPI / MFastAPI / Rust+Poem / Rust+RPoem(RPoem は Bootstrap・TypeScript・Next.js・Node.js なし)へ変換するツールで、API は REST ではなく Connect(`POST /cobol_to.Program/Run`)。v0.1.2 の Release は公開済みで CI は全6ジョブ成功しています(v0.1.0/v0.1.1 は失敗のため Release 無し。タグは動かさない)。次に直す点があれば v0.1.3 以降で出してください。通知メールが利用者に届いていない件(`aon.co.jp` に SPF/DMARC が無い疑い)も `PORTING.md` の 5 を見て進めてください。次に、`PORTING.md` の未了の優先順(構文の拡充 → 実サーバー接続 → 最新版への自動追従)を進めてください。注意: Bash に cargo は無いので PowerShell で PATH を前置きする、`cargo test` は debug の exe を更新しない、未対応の構文は黙って落とさず `TODO(COBOL_to)` にする、メールアドレスやトークンはリポジトリに入れない、`git pull` 前に `git status -sb` で分岐を確認する。

## English

### Resume on another PC or account
1. `git clone https://github.com/aon-co-jp/COBOL_to` and `cd COBOL_to`.
2. Install the latest stable Rust. `cargo build --release` and `cargo test`.
3. For the Windows installer install Inno Setup 6 and run `ISCC /DAppVersion=0.1.0 /DArch=x64 /DExeDir=..\target\release installer\cobol_to.iss` (output in `dist\`). The Linux `.deb` is `installer/build-deb.sh`; the macOS `.pkg` is `installer/build-pkg.sh` on macOS.
4. Check: `cobol_to --src tests/samples --target rust-poem --out <out>` creates `src/programs/hello.rs` and the detection in `COBOL_to-REPORT.md`.
5. For update notifications put your own two addresses into `[notify] email_to` in version-control's `config.toml` (git-ignored) and set `root` to your drive/folder. Never commit secrets.

### Open items (priority order)
1. **(Done) Release CI**: `v0.1.2` (run 44168310290) passed all 6 jobs; the Release holds 2 debs, 2 tar.gz, 2 pkg, 2 exe and `SHA256SUMS.txt`. Still open: real-machine checks (Linux arm64, macOS, Windows ARM64) and macOS signing. The rest is history (old text). **(Old)** tag `v0.1.1`, run 44166948447. v0.1.0 had no Release: Linux failed to build and the macOS x64 runner was retired (Windows passed). v0.1.1 is the first real check of the Linux arm64 and macOS smoke tests. On failure ship `v0.1.2` rather than moving the tag. On success verify the Release assets and `SHA256SUMS`, then update the README status.
2. **Widen the supported syntax**: `IF`/`EVALUATE`/`PERFORM` (`UNTIL`, `VARYING`)/`COMPUTE`/`SUBTRACT`/`MULTIPLY`/`DIVIDE`/`STRING`/`INSPECT`/`COPY`+`REPLACING`/file I/O/`CALL`/`EXEC SQL`+`EXEC CICS`, decimals/signs/`COMP-3`, continuation lines. Add real programs to `tests/samples` as tests.
3. **Wire the output into real servers**: implement `POST /cobol_to.Program/Run` (Connect) in FastAPI/MFastAPI/Poem/RPoem; generate the Next.js (TypeScript + Bootstrap) UI; a Rust→WASM UI for RPoem.
4. **Automatic latest versions**: generated dependencies are not pinned. Use `cargo add` and the PyPI/npm latest lookups; register in version-control.
5. **Notification mail is not arriving**: the VPS logs `e-mail: sent` (SMTP accepted; not a code bug), yet the user reports nothing reached `norukia.jp@gmail.com` or `cloud9slack@gmail.com`. The likely cause is that `aon.co.jp` has no SPF (TXT) record and no `_dmarc`. Check the spam folder and bounces in the sender mailbox (`inf…@aon.co.jp`), then add SPF (confirm Lolipop's value) and DMARC at value-domain (the user's action). If that does not help, consider another sending path. LINE is not configured.
6. **Scanning from a GitHub owner**: not implemented in version-control (only `root`).
7. **macOS signing/notarization** (Apple secrets needed), EBCDIC auto-conversion, Linux `.rpm`/AppImage.
8. **Live version-control scan of COBOL_to** (should be picked up via `root = "F:/"`; the scan is long, so run `--dry-run --no-fetch --no-api` when there is time).

### Notes
- No `cargo` in this PC's Bash (prepend PATH in PowerShell). `cargo test` does not refresh the debug exe.
- History is fresh. No repositories were created under the old names (`COBOL2`/`COBOL-to`).

### Resume message (paste as is)
> Please resume `F:\COBOL_to` (GitHub: aon-co-jp/COBOL_to). Read `README.md`, `CLAUDE.md` and `PORTING.md` first. It auto-detects the COBOL dialect, standard and source format and converts to FastAPI / MFastAPI / Rust+Poem / Rust+RPoem (RPoem without Bootstrap, TypeScript, Next.js or Node.js); the API is Connect (`POST /cobol_to.Program/Run`), not REST. The Release v0.1.2 is published and all 6 CI jobs passed (v0.1.0/v0.1.1 failed and have no Release; never move a tag). Ship any further fix as v0.1.3 or later. Also work on the notification mail not reaching the user (suspected missing SPF/DMARC for `aon.co.jp`), see item 5 in `PORTING.md`. Then work through the open items in `PORTING.md` in order (syntax coverage → real server wiring → automatic latest versions). Cautions: Bash has no cargo, so prepend PATH in PowerShell; `cargo test` does not refresh the debug exe; never drop unsupported syntax silently, emit `TODO(COBOL_to)`; keep e-mail addresses and tokens out of the repository; check `git status -sb` for divergence before any `git pull`.

## 2026-11-18 追記2: 未対応の構文を一括対応 / Remaining syntax implemented
**日本語**: 未了の2(構文の拡充)の大半を実装した。`GO TO`(DEPENDING ON・段落の状態機械)・`PERFORM THRU`・小数(固定小数点・ROUNDED)・`OCCURS`(添字)・`REDEFINES`(同型は別名、他は別領域+注記)・`88` 条件名・`NUMERIC`/`ALPHABETIC`・`STRING`/`UNSTRING`/`INSPECT`・`INITIALIZE`/`SET`/`ACCEPT`・`DIVIDE REMAINDER`・`FUNCTION`(9種)・継続行・`COPY`(変換前に展開)・`CALL`(`EXTERNAL`/`run_with` に登録する外部呼び出し)・順編成ファイル I/O(SELECT/FD/OPEN/READ/WRITE/CLOSE/FILE STATUS)。`tests/samples/features.cbl`・`fileio.cbl`・`misc.cbl` を Python と Rust の両方で実行し同一出力を確認、`cargo test` 9件成功。**未対応(TODO表示)**: SORT/MERGE/SEARCH、EXEC SQL/CICS、索引・相対ファイル・REWRITE、COMP-3 の表現、参照修飾、多次元添字、STRING POINTER/ON OVERFLOW、ON SIZE ERROR、BEFORE/AFTER、動的 CALL、ACCEPT FROM DATE、メモリ重ね合わせ。**未確認**: 実際のメインフレーム由来の大きなプログラムでの通し(サンプルは自作の小さなもの)、CI(Linux arm64・macOS)上での新サンプル変換。次は v0.1.3 として出すか、上記の TODO を順に。
**English**: Most of open item 2 (syntax coverage) is implemented: `GO TO` (incl. `DEPENDING ON`; paragraphs run on a small state machine), `PERFORM THRU`, decimals (fixed point, `ROUNDED`), `OCCURS` subscripts, `REDEFINES` (aliased when types match, otherwise a separate area plus a note), level-88 condition names, `NUMERIC`/`ALPHABETIC`, `STRING`/`UNSTRING`/`INSPECT`, `INITIALIZE`/`SET`/`ACCEPT`, `DIVIDE REMAINDER`, nine `FUNCTION`s, continuation lines, `COPY` (expanded before parsing), `CALL` (external calls registered in `EXTERNAL` / `run_with`), and sequential file I/O. `tests/samples/features.cbl`, `fileio.cbl`, `misc.cbl` give identical output in generated Python and Rust; 9 unit tests pass. **Not supported (shown as TODO)**: SORT/MERGE/SEARCH, EXEC SQL/CICS, indexed/relative files and REWRITE, COMP-3 representation, reference modification, multi-dimensional subscripts, STRING POINTER / ON OVERFLOW, ON SIZE ERROR, BEFORE/AFTER, dynamic CALL, ACCEPT FROM DATE, true memory overlay. **Unverified**: large real-world mainframe programs (samples are small and hand-written) and converting the new samples on CI (Linux arm64, macOS). Next: ship as v0.1.3, or work through the TODO list above.

## 2026-11-18 追記: 変換を「小さく→多機能」へ / Conversion grown from minimal to richer
**日本語**: `src/convert.rs` を構造化(式・条件の構文解析)。対応: IF/ELSE/END-IF(AND/OR/NOT・比較語)、PERFORM(段落・n TIMES・UNTIL・VARYING・インライン)、段落、COMPUTE、ADD/SUBTRACT/MULTIPLY/DIVIDE(GIVING 含む)、STOP RUN/GOBACK。未対応の文(OPEN/READ/WRITE/CALL/EVALUATE/GO TO 等)は `TODO(COBOL_to)` として残す。数値は整数(小数 PICは近似+TODO)。`tests/samples/calc.cbl` を Python と Rust の両方で実行し、同じ出力(`TOTAL=35 MSG=BIG`)を確認。次: EVALUATE・ファイル I/O・小数(固定小数点)・REDEFINES/OCCURS。
**EVALUATE(WHEN 値・OR・OTHER・TRUE)も対応**。
**English**: EVALUATE is supported too. `convert.rs` now parses expressions/conditions. Supported: IF/ELSE, PERFORM (paragraph, TIMES, UNTIL, VARYING, inline), paragraphs, COMPUTE, ADD/SUBTRACT/MULTIPLY/DIVIDE (GIVING), STOP RUN/GOBACK. Unsupported statements stay as `TODO(COBOL_to)`. Numbers are integers. `calc.cbl` gives identical output in generated Python and Rust. Next: EVALUATE, file I/O, fixed-point decimals, REDEFINES/OCCURS.

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

## 2026-11-20 追記: ACCEPT FROM DATE/TIME に対応 / ACCEPT FROM DATE/TIME supported
**日本語**: `ACCEPT x FROM DATE`(YYMMDD)・`DATE YYYYMMDD`・`DAY`(YYDDD)・`DAY YYYYDDD`・`TIME`(HHMMSSCC)・`DAY-OF-WEEK`(月=1)を、Python と Rust の両方に変換できるようになった(時刻は UTC)。単体試験 11 件成功。まだ TODO のまま: SORT/MERGE/SEARCH、EXEC SQL/CICS、索引・相対ファイル、COMP-3 の表現、参照修飾の書き込み、多次元添字、ON SIZE ERROR、BEFORE/AFTER、動的 CALL。
**English**: `ACCEPT x FROM DATE` (YYMMDD), `DATE YYYYMMDD`, `DAY` (YYDDD), `DAY YYYYDDD`, `TIME` (HHMMSSCC) and `DAY-OF-WEEK` (Monday = 1) now convert to both Python and Rust (UTC). 11 unit tests pass. Still TODO: SORT/MERGE/SEARCH, EXEC SQL/CICS, indexed/relative files, COMP-3 representation, reference-modification writes, multi-dimensional subscripts, ON SIZE ERROR, BEFORE/AFTER, dynamic CALL.

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

## 2026-11-22 追記: INSPECT の LEADING / CHARACTERS / Added INSPECT LEADING and CHARACTERS
**日本語**: `INSPECT ... TALLYING ... FOR LEADING "x"` と `FOR CHARACTERS`、`REPLACING LEADING "x" BY "y"` を変換できるようになりました(`AFTER/BEFORE INITIAL` との併用も可)。`"000A00B00"` と `"AAABAAAAA"` で実際に動かし、Python と Rust がどちらも `3 9 2 3` / `   A00B00` / `---BAAAAA` と同じ出力になることを確認しました(サンプル `tests/samples/inspect2.cbl`、単体試験14件成功)。**まだ**: `REPLACING CHARACTERS`、`FOR` を複数並べる形、`SORT/MERGE/SEARCH`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。pl2pl の `converters/cobol_to` にも同じ変更を反映済みです。他の言語の節の「完成予定」は、毎朝の翻訳更新で順に直します。
**English**: `INSPECT ... TALLYING ... FOR LEADING "x"`, `FOR CHARACTERS` and `REPLACING LEADING "x" BY "y"` are now converted (also together with `AFTER/BEFORE INITIAL`). We ran `"000A00B00"` and `"AAABAAAAA"` and both the generated Python and Rust print the same `3 9 2 3` / `   A00B00` / `---BAAAAA` (sample `tests/samples/inspect2.cbl`, 14 unit tests pass). **Not yet**: `REPLACING CHARACTERS`, several `FOR` clauses in one statement, `SORT/MERGE/SEARCH`, indexed and relative files, COMP-3, EXEC SQL/CICS. The same change is applied to `converters/cobol_to` in pl2pl. The planned lists in the other language sections will be fixed by the daily translation update.

## 2026-11-22 追記: SEARCH / SEARCH ALL / Added SEARCH and SEARCH ALL
**日本語**: `OCCURS ... INDEXED BY ix` の表を `SEARCH 表 [VARYING ix] [AT END ...] WHEN 条件 ... END-SEARCH` で引けるようになりました(`WHEN` は複数可)。`SEARCH ALL` は索引を 1 から始める直線探索として動かします(整列済みの表で一致が1件なら、二分探索と結果は同じです)。表を `PERFORM UNTIL`(範囲外か条件一致まで索引を進める)と `IF` に分解して出すので、Python と Rust の両方で同じ出力になります。サンプル `tests/samples/search1.cbl` で `FOUND 3` / `NONE` / `ALL 5` / `EQ20` を確認、単体試験15件成功。`INDEXED BY` の索引名は変数として自動で宣言します。**まだ**: `SORT`(表・ファイル)、`MERGE`、`SEARCH ALL` の本物の二分探索、複数の `INDEXED BY` 名、`REPLACING CHARACTERS`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。
**English**: Tables declared with `OCCURS ... INDEXED BY ix` can now be searched with `SEARCH table [VARYING ix] [AT END ...] WHEN cond ... END-SEARCH` (several `WHEN` clauses allowed). `SEARCH ALL` runs as a linear search starting at index 1 (identical results on a sorted table with a single match). It is emitted as a `PERFORM UNTIL` (advance the index until out of range or a condition matches) plus an `IF` chain, so Python and Rust print the same output. In `tests/samples/search1.cbl` we confirmed `FOUND 3` / `NONE` / `ALL 5` / `EQ20`; 15 unit tests pass. The `INDEXED BY` name is declared as a variable automatically. **Not yet**: `SORT` (table and file forms), `MERGE`, a true binary `SEARCH ALL`, several `INDEXED BY` names, `REPLACING CHARACTERS`, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-22 追記: SORT(USING / GIVING)/ Added SORT with USING and GIVING
**日本語**: `SORT 整列用ファイル ON ASCENDING|DESCENDING KEY 項目... [WITH DUPLICATES IN ORDER] USING ファイル... GIVING ファイル...` を変換できるようになりました(`SD` も `FD` と同じ扱いです)。USING のファイルの行をすべて読み、キー項目の位置と幅(SD の項目並びから計算)で、キーを後ろから順に安定ソートし、GIVING のファイルへ書き出します(キーは複数可、昇順・降順の混在可、数値キーは数として比べます)。サンプル `tests/samples/sort1.cbl`(年齢の昇順、同じ年齢は名前の降順)で、Python も Rust も `DI 25` / `AL 25` / `CY 30` / `BOB 30` と一致しました。単体試験16件成功。**まだ**: `INPUT PROCEDURE` / `OUTPUT PROCEDURE`(`RELEASE`/`RETURN`)、表の `SORT`、`MERGE`、`COLLATING SEQUENCE`、複数項目をまたぐレコードの `READ INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。
**English**: `SORT sort-file ON ASCENDING|DESCENDING KEY field... [WITH DUPLICATES IN ORDER] USING file... GIVING file...` is now converted (`SD` is handled like `FD`). All records of the USING files are read, then stably sorted from the last key to the first by the key's position and width (taken from the SD layout), then written to the GIVING files (several keys, mixed ascending/descending, numeric keys compared as numbers). In `tests/samples/sort1.cbl` (ascending age, then descending name) Python and Rust both print `DI 25` / `AL 25` / `CY 30` / `BOB 30`; 16 unit tests pass. **Not yet**: `INPUT PROCEDURE` / `OUTPUT PROCEDURE` (`RELEASE`/`RETURN`), table `SORT`, `MERGE`, `COLLATING SEQUENCE`, `READ INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-22 追記: SORT の INPUT / OUTPUT PROCEDURE、RELEASE、RETURN / Added SORT INPUT/OUTPUT PROCEDURE, RELEASE and RETURN
**日本語**: `SORT ... INPUT PROCEDURE IS 段落 [THRU 段落]` と `OUTPUT PROCEDURE IS 段落 [THRU 段落]`、手続きの中の `RELEASE レコード [FROM ...]` と `RETURN 整列用ファイル [INTO ...] AT END ... NOT AT END ... END-RETURN` を変換できるようになりました。`USING` と `OUTPUT PROCEDURE`(またはその逆)の組み合わせも可です。整列用の行は State の中のバッファに溜め、INPUT 側が終わったら安定ソートし、OUTPUT 側の `RETURN` が1行ずつ取り出します。サンプル `tests/samples/sort2.cbl`(20歳の行を INPUT で除き、年齢の降順で取り出す)で、Python も Rust も `CY 31` / `BOB 30` / `AL 25` / `DONE` と一致しました。単体試験17件成功。**まだ**: 表の `SORT`、`MERGE`、`COLLATING SEQUENCE`、複数項目レコードの `READ INTO` と `RELEASE/RETURN INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。
**English**: `SORT ... INPUT PROCEDURE IS para [THRU para]` and `OUTPUT PROCEDURE IS para [THRU para]`, plus `RELEASE record [FROM ...]` and `RETURN sort-file [INTO ...] AT END ... NOT AT END ... END-RETURN` inside those procedures, are now converted. `USING` can be combined with `OUTPUT PROCEDURE` and vice versa. Sort records collect in a buffer inside State; when the INPUT side ends they are stably sorted, and `RETURN` on the OUTPUT side takes them one by one. In `tests/samples/sort2.cbl` (INPUT drops the age-20 row, output in descending age) Python and Rust both print `CY 31` / `BOB 30` / `AL 25` / `DONE`; 17 unit tests pass. **Not yet**: table `SORT`, `MERGE`, `COLLATING SEQUENCE`, `READ INTO` and `RELEASE/RETURN INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-22 追記: MERGE / Added MERGE
**日本語**: `MERGE 整列用ファイル ON ASCENDING|DESCENDING KEY 項目... USING ファイル ファイル... GIVING ファイル...`(`OUTPUT PROCEDURE` でも可)を変換できるようになりました。入力が整列済みなら、USING の全ファイルをつないで安定ソートした結果と同じなので、`SORT` と同じ生成コードを使います(同じキーの行は、USING に書いた順のまま並びます)。COBOL の規則どおり、`USING` が1つだけの `MERGE` や `INPUT PROCEDURE` つきの `MERGE` は変換せず `TODO(COBOL_to)` に残します。サンプル `tests/samples/merge1.cbl`(年齢の昇順)で、Python も Rust も `AL 20` / `BO 20` / `CY 30` / `DI 40` と一致しました。単体試験18件成功。**注意**: 入力が整列済みでないと、本物の MERGE は結果が不定ですが、この変換は整列して出します。**まだ**: 表の `SORT`、`COLLATING SEQUENCE`、複数項目レコードの `READ INTO` と `RELEASE/RETURN INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。
**English**: `MERGE sort-file ON ASCENDING|DESCENDING KEY field... USING file file... GIVING file...` (or `OUTPUT PROCEDURE`) is now converted. When the inputs are already sorted, the result equals a stable sort of all USING files joined together, so it shares the `SORT` generated code (rows with equal keys keep the USING order). As COBOL requires, a `MERGE` with a single `USING` file or with an `INPUT PROCEDURE` is not converted and stays as `TODO(COBOL_to)`. In `tests/samples/merge1.cbl` (ascending age) Python and Rust both print `AL 20` / `BO 20` / `CY 30` / `DI 40`; 18 unit tests pass. **Note**: a real MERGE gives undefined results for unsorted input, while this conversion sorts it. **Not yet**: table `SORT`, `COLLATING SEQUENCE`, `READ INTO` and `RELEASE/RETURN INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-22 追記: COMP-3 / COMP(使用句)の扱い / USAGE COMP-3 and COMP handling
**日本語**: `COMP-3`・`COMPUTATIONAL-3`・`PACKED-DECIMAL`(パック10進)と、`COMP`・`COMP-4`・`COMP-5`・`BINARY`(2進)を、`USAGE [IS]` の有無にかかわらず読み取り、変数ごとに記録するようになりました(`Var.usage`)。算術は桁数(PICTURE)で決まるので、元から正しく動いていました。サンプル `tests/samples/comp3.cbl`(`ADD`・`COMPUTE`・`MULTIPLY`・`ON SIZE ERROR` を含む)で、Python も Rust も `OVER` / `12347.00 -5 9999 95 185 3.0 -3.25` と一致しました。単体試験19件成功。**できないこと(明記)**: ファイルのレコードに入った COMP-3 / COMP の項目は、この変換器のファイルが「固定長テキストの行」なので、パックや2進のバイト列ではなく**桁の文字**として入出力します(その旨を変換結果の注意書きに出します)。実際のメインフレームの2進ファイルとは互換がありません。パックのバイト列を扱う機能(2進の順編成ファイル、`REDEFINES` や群項目の `MOVE` でのバイト単位の重ね合わせ)は未実装です。また、`COMP-5` を含む2進の桁あふれは、PICTURE の桁数で判定します(本物の2進の幅は使いません)。**まだ**: 上のバイト列対応、表の `SORT`、`COLLATING SEQUENCE`、複数項目レコードの `READ INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、EXEC SQL/CICS。
**English**: `COMP-3`, `COMPUTATIONAL-3`, `PACKED-DECIMAL` (packed decimal) and `COMP`, `COMP-4`, `COMP-5`, `BINARY` (binary) are now read, with or without `USAGE [IS]`, and recorded per variable (`Var.usage`). Arithmetic is driven by the PICTURE digits, so it already worked. In `tests/samples/comp3.cbl` (with `ADD`, `COMPUTE`, `MULTIPLY` and `ON SIZE ERROR`) Python and Rust both print `OVER` / `12347.00 -5 9999 95 185 3.0 -3.25`; 19 unit tests pass. **Limits, stated plainly**: files in this converter are fixed-width text lines, so a COMP-3 / COMP item inside a file record is read and written as **digit characters**, not as packed or binary bytes (the output notes say so); it is not compatible with real mainframe binary files. Byte-level packed handling (binary sequential files, byte overlay through `REDEFINES` or group `MOVE`) is not implemented. Overflow of binary items, `COMP-5` included, is judged by the PICTURE digits, not by the real binary width. **Not yet**: the byte-level support above, table `SORT`, `COLLATING SEQUENCE`, `READ INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, EXEC SQL/CICS.

## 2026-11-22 追記: EXEC SQL(埋め込み SQL)/ Added EXEC SQL (embedded SQL)
**日本語**: `EXEC SQL ... END-EXEC` を変換できるようになりました。対応: `INCLUDE SQLCA`、`DECLARE カーソル CURSOR FOR SELECT ...`(WORKING-STORAGE に書いても手続き部に書いても可)、`OPEN` / `FETCH ... INTO :a, :b` / `CLOSE`、`SELECT ... INTO :a, :b FROM ...`(先頭の1行を取る)、そのほかの文(`INSERT` / `UPDATE` / `DELETE` / `CREATE` / `COMMIT` / `ROLLBACK` など)。`:変数` は `?` に置き換えて、変数の値を文字列にして渡します。結果は `SQLCODE`(自動で宣言)に入れます: 成功 0、行なし 100、カーソルが無い -501、実行の失敗 -1。**実行先**: Python は標準の `sqlite3`(環境変数 `COBOL_SQL_DB` で DB ファイル、無ければメモリ)で、`SQL_HANDLER` を差し替えれば別の DB にもできます。Rust は DB 部品に依存せず(C 依存の SQLite をやめた方針と同じ)、`set_sql_handler(fn(&str, Vec<String>) -> (SQLCODE, 行のリスト))` で実行先を登録します。未登録なら `SQLCODE` は -1 です。サンプル `tests/samples/sql1.cbl` で、Python(実際の sqlite3)の出力 `COUNT=3 SQLCODE=0` / `BOB 20.25 SQLCODE=0` / `NOROW SQLCODE=100` / `ROW 1 ALICE` / `ROW 2 BOB` / `END SQLCODE=100` を確認し、Rust(試験用の疑似ハンドラー)も同じ出力になりました。単体試験20件成功。**確認していないこと**: Rust は本物の DB につないでいません(受け渡しだけを疑似ハンドラーで確認)。数値の列は文字列で渡すので、DB 側の型変換に任せます。**まだ**: `WHENEVER`、指示子変数(`:変数:指示子`)、`SQLSTATE`・`SQLERRM`、`FETCH` の位置指定(`NEXT` 以外)、動的 SQL(`PREPARE` / `EXECUTE`)、`EXEC CICS`、表の `SORT`、`COLLATING SEQUENCE`、複数項目レコードの `READ INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、COMP-3 のバイト列。
**English**: `EXEC SQL ... END-EXEC` is now converted. Supported: `INCLUDE SQLCA`; `DECLARE cursor CURSOR FOR SELECT ...` (in WORKING-STORAGE or the procedure division); `OPEN` / `FETCH ... INTO :a, :b` / `CLOSE`; `SELECT ... INTO :a, :b FROM ...` (takes the first row); and other statements (`INSERT` / `UPDATE` / `DELETE` / `CREATE` / `COMMIT` / `ROLLBACK` ...). `:host-vars` become `?` and their values are passed as strings. Results go to `SQLCODE` (declared automatically): 0 on success, 100 for no row, -501 for an unknown cursor, -1 when execution fails. **Where it runs**: Python uses the standard `sqlite3` (env `COBOL_SQL_DB` for a DB file, in-memory otherwise) and `SQL_HANDLER` can be replaced to use another database. Rust depends on no database crate (same policy as dropping C-based SQLite); register the target with `set_sql_handler(fn(&str, Vec<String>) -> (SQLCODE, rows))`. Without one, `SQLCODE` is -1. With `tests/samples/sql1.cbl`, Python (real sqlite3) prints `COUNT=3 SQLCODE=0` / `BOB 20.25 SQLCODE=0` / `NOROW SQLCODE=100` / `ROW 1 ALICE` / `ROW 2 BOB` / `END SQLCODE=100`, and Rust (with a test fake handler) prints the same; 20 unit tests pass. **Not verified**: Rust was not connected to a real database (only the hand-over was checked with the fake handler). Numeric columns are passed as strings and left to the database's type conversion. **Not yet**: `WHENEVER`, indicator variables (`:var:ind`), `SQLSTATE` / `SQLERRM`, positioned `FETCH` other than `NEXT`, dynamic SQL (`PREPARE` / `EXECUTE`), `EXEC CICS`, table `SORT`, `COLLATING SEQUENCE`, `READ INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, byte-level COMP-3.
