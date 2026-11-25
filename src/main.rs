mod convert;
mod detect;

use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Target {
    FastApi,
    MFastApi,
    RustPoem,
    RPoem,
    Java,
}
impl Target {
    const ALL: [Target; 5] = [Target::FastApi, Target::MFastApi, Target::RustPoem, Target::RPoem, Target::Java];
    fn label(self) -> &'static str {
        match self {
            Target::FastApi => "最新Python + 最新FastAPI + HTML5/CSS3(Bootstrap・TypeScript・Node.js は任意で、推薦はしません)",
            Target::MFastApi => "最新Python + 最新MFastAPI + HTML5/CSS3(Bootstrap・TypeScript・Node.js は任意で、推薦はしません)",
            Target::RustPoem => "最新Rust + 最新Poem",
            Target::RPoem => "最新Rust + 最新RPoem (Rust→WASM。Bootstrap・TypeScript・Node.js は使わない設計。詳しくは pl2pl info nojs)",
            Target::Java => "Java 17 以上(JDK 標準のみ。ファイル入出力・SORT・EXEC SQL・範囲つき INSPECT は未対応で TODO を残す)",
        }
    }
    fn slug(self) -> &'static str {
        match self {
            Target::FastApi => "fastapi",
            Target::MFastApi => "mfastapi",
            Target::RustPoem => "rust-poem",
            Target::RPoem => "rpoem",
            Target::Java => "java",
        }
    }
    fn is_python(self) -> bool {
        matches!(self, Target::FastApi | Target::MFastApi)
    }
}

fn ask(q: &str) -> String {
    print!("{q}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    s.trim().to_string()
}

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
}

fn is_cobol(p: &Path) -> bool {
    matches!(
        p.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()).as_deref(),
        Some("cbl" | "cob" | "cobol" | "pco" | "cob85" | "cbx")
    )
}

fn read_text(p: &Path) -> Option<String> {
    let b = std::fs::read(p).ok()?;
    // EBCDICらしい(メインフレームの転送ファイル)ものは明示的に拒否して報告する
    if b.iter().filter(|c| **c == 0x40).count() > b.len() / 4 {
        return None;
    }
    Some(String::from_utf8_lossy(&b).into_owned())
}

fn write(p: &Path, s: &str) {
    if let Some(d) = p.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    std::fs::write(p, s).unwrap_or_else(|e| eprintln!("書き込み失敗 {}: {e}", p.display()));
}

/// RPoem 向けの起動用サーバー(Connect 単純呼び出し、REST なし)。
const SERVER_RS: &str = r#"//! COBOL_to が生成した RPoem サーバー。POST /cobol_to.Program/Run {"program":"名前"}
use open_runo_poem_compat::{post, Json, Params, Request, Response, Route, Server, StatusCode, TcpListener};

#[open_runo_poem_compat_macro::handler]
async fn run(req: Request, _params: Params) -> Response {
    let body = match open_runo_poem_compat::body_limit::read_body_limited(req, 64 * 1024).await {
        Ok(b) => b,
        Err(r) => return r,
    };
    let v: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => return Json(serde_json::json!({"code": "invalid_argument"})).into_response(StatusCode::BAD_REQUEST),
    };
    let name = v.get("program").and_then(|x| x.as_str()).unwrap_or("");
    match cobol_to_out::PROGRAMS.iter().find(|(n, _)| *n == name) {
        Some((_, f)) => Json(serde_json::json!({"output": f()})).into_response(StatusCode::OK),
        None => Json(serde_json::json!({"code": "not_found"})).into_response(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3001);
    let app = Route::new().at("/cobol_to.Program/Run", post(run()));
    let (addr, handle) = Server::new(TcpListener::bind(([127, 0, 0, 1], port))).run(app).await.expect("bind");
    println!("listening on {addr}");
    handle.await.ok();
}
"#;

/// 出力名 → Java のクラス名(英数字のみ、各語の先頭を大文字、先頭に P)
fn java_class(n: &str) -> String {
    let mut s = String::from("P");
    let mut up = true;
    for c in n.chars() {
        if c.is_ascii_alphanumeric() {
            s.push(if up { c.to_ascii_uppercase() } else { c });
            up = false;
        } else {
            up = true;
        }
    }
    s
}

/// Java 向けの起動用サーバー(JDK 標準 com.sun.net.httpserver、Connect 単純呼び出し、REST なし)
const SERVER_JAVA: &str = include_str!("server_java.txt");

fn scaffold(t: Target, out: &Path, mods: &[String]) {
    if t == Target::Java {
        let entries: String = mods.iter().map(|m| format!("        Map.entry({m:?}, {}::run)", java_class(m))).collect::<Vec<_>>().join(",\n");
        write(&out.join("src/cobolto/Server.java"), &SERVER_JAVA.replace("__ENTRIES__", &entries));
        write(&out.join("README.md"), &format!("# COBOL_to 変換結果 (java)\n\n{}\n\n```\njavac -d build src/cobolto/*.java\njava -cp build cobolto.Server\n```\n\nPOST /cobol_to.Program/Run {{\"program\":\"名前\"}}(Connect 単純呼び出し、REST なし)。各クラスは単独でも `java -cp build cobolto.<クラス名>` で動く。\n", t.label()));
        return;
    }
    let calls: String = mods.iter().map(|m| format!("    (\"{m}\", programs::{m}::run),\n")).collect();
    if t.is_python() {
        let fw = if t == Target::FastApi { "fastapi" } else { "mfastapi" };
        write(&out.join("programs/__init__.py"), "");
        let reg: String = mods.iter().map(|m| format!("from programs import {m}\n")).collect();
        let tbl: String = mods.iter().map(|m| format!("    \"{m}\": {m}.run,\n")).collect();
        write(&out.join("app.py"), &format!(
            "{reg}\nPROGRAMS = {{\n{tbl}}}\n\n# Connect形式: POST /cobol_to.Program/Run  {{\"program\": \"名前\"}}\n# {fw} のアプリへ登録すること(下の run_program を呼ぶ)\ndef run_program(body: dict) -> dict:\n    f = PROGRAMS.get(body.get(\"program\"))\n    if f is None:\n        return {{\"code\": \"not_found\"}}\n    return {{\"output\": f()}}\n"));
        write(&out.join("requirements.txt"), &format!("{fw}\nuvicorn\n"));
        write(&out.join("frontend/README.md"), "画面は HTML5/CSS3 で十分です(静的ファイルを FastAPI が配信)。Bootstrap・TypeScript・Node.js を使うかは自由ですが、推薦はしません。Connect(POST /cobol_to.Program/Run)を fetch で呼びます。\n");
    } else {
        if t == Target::RPoem {
            // 起動できるサーバー: POST /cobol_to.Program/Run {"program":"名前"} -> {"output":..}
            let rp = std::env::var("RPOEM_DIR").unwrap_or_else(|_| "F:/RPoem".into()).replace('\\', "/");
            write(&out.join("Cargo.toml"), &format!("[package]\nname = \"cobol_to-out\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\nname = \"cobol_to_out\"\npath = \"src/lib.rs\"\n\n[[bin]]\nname = \"cobol_to-server\"\npath = \"src/main.rs\"\n\n[dependencies]\nopen-runo-poem-compat = {{ path = \"{rp}/crates/open-runo-poem-compat\" }}\nopen-runo-poem-compat-macro = {{ path = \"{rp}/crates/open-runo-poem-compat-macro\" }}\nserde_json = \"1\"\ntokio = {{ version = \"1\", features = [\"rt-multi-thread\", \"macros\", \"net\"] }}\n"));
            write(&out.join("src/main.rs"), SERVER_RS);
        } else {
            write(&out.join("Cargo.toml"), "[package]\nname = \"cobol_to-out\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n# 最新版を入れる: cargo add tokio --features full ; cargo add poem (RPoem は F:\\RPoem を path 依存で追加)\n");
        }
        write(&out.join("src/programs/mod.rs"), &mods.iter().map(|m| format!("pub mod {m};\n")).collect::<String>());
        write(&out.join("src/lib.rs"), &format!(
            "pub mod programs;\n\n/// Connect形式 POST /cobol_to.Program/Run から呼ぶ登録表\npub const PROGRAMS: &[(&str, fn() -> String)] = &[\n{calls}];\n"));
    }
    let note = if t == Target::RPoem { "RPoemのAdmin系Connectハンドラへ PROGRAMS を接続する。画面は Rust→WASM(Bootstrap等は使わない)。" } else { "" };
    write(&out.join("README.md"), &format!("# COBOL_to 変換結果 ({})\n\n{}\n\n依存は固定していません。最新版は version-control の通知に従って更新します。{note}\n", t.slug(), t.label()));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("COBOL_to - COBOL 方言自動判定・変換");
    let src: PathBuf = match arg(&args, "--src") {
        Some(s) => s.into(),
        None => match rfd::FileDialog::new().set_title("COBOLのプログラム/プロジェクトの上位フォルダを選択").pick_folder() {
            Some(p) => p,
            None => PathBuf::from(ask("フォルダ選択が使えません。パスを入力: ")),
        },
    };
    let target = match arg(&args, "--target") {
        Some(s) => Target::ALL.iter().copied().find(|t| t.slug() == s).unwrap_or_else(|| { eprintln!("不明なtarget: {s}"); std::process::exit(2) }),
        None => {
            for (i, t) in Target::ALL.iter().enumerate() {
                println!("  {}) {}", i + 1, t.label());
            }
            let n: usize = ask("変換先の番号: ").parse().unwrap_or(0);
            *Target::ALL.get(n.wrapping_sub(1)).unwrap_or_else(|| { eprintln!("番号が不正です"); std::process::exit(2) })
        }
    };
    let out: PathBuf = match arg(&args, "--out") {
        Some(s) => s.into(),
        None => {
            let parent = rfd::FileDialog::new().set_title("変換後の出力先の親フォルダを選択(新規作成は名前入力)").set_directory(src.parent().unwrap_or(&src)).pick_folder().unwrap_or_else(|| src.parent().unwrap_or(&src).to_path_buf());
            let def = format!("{}-converted", src.file_name().and_then(|n| n.to_str()).unwrap_or("cobol"));
            let n = ask(&format!("出力フォルダ名(空=既定 {def}): "));
            parent.join(if n.is_empty() { def } else { n })
        }
    };

    let mut mods = vec![];
    let mut report = String::from("# 判定レポート\n\n| ファイル | 方言 | 規格 | 書式 | 根拠 |\n|---|---|---|---|---|\n");
    for e in walkdir::WalkDir::new(&src).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file() && is_cobol(e.path())) {
        let rel = e.path().strip_prefix(&src).unwrap_or(e.path());
        let Some(text) = read_text(e.path()) else {
            report += &format!("| {} | (EBCDICらしく未変換。UTF-8/ASCIIへ変換して再実行) | | | |\n", rel.display());
            continue;
        };
        let d = detect::detect(&text);
        report += &format!("| {} | {} | {} | {} | {} |\n", rel.display(), d.dialect, d.standard, if d.free_format { "自由" } else { "固定" }, d.evidence.join("; "));
        let mut dirs: Vec<PathBuf> = vec![];
        if let Some(par) = e.path().parent() {
            for sub in ["", "copy", "copybook", "COPY", "COPYBOOK", "cpy"] {
                dirs.push(par.join(sub));
            }
        }
        dirs.push(src.clone());
        let text = convert::expand_copy(&text, d.free_format, &dirs);
        let prog = convert::parse(&text, d.free_format);
        let name: String = rel.with_extension("").to_string_lossy().chars().map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' }).collect();
        let (code, ext, dir) = if target.is_python() {
            (convert::to_python(&prog), "py", "programs")
        } else if target == Target::Java {
            (format!("package cobolto;\n\n{}", convert::to_java(&prog, &java_class(&name))), "java", "src/cobolto")
        } else {
            (convert::to_rust(&prog), "rs", "src/programs")
        };
        let file = if target == Target::Java { java_class(&name) } else { name.clone() };
        write(&out.join(dir).join(format!("{file}.{ext}")), &code);
        mods.push(name);
    }
    if mods.is_empty() {
        eprintln!("COBOLファイル(.cbl/.cob/.cobol/.pco)が見つかりません: {}", src.display());
        std::process::exit(1);
    }
    scaffold(target, &out, &mods);
    write(&out.join("COBOL_to-REPORT.md"), &report);
    println!("{} 本を変換 -> {} ({})", mods.len(), out.display(), target.slug());
}
