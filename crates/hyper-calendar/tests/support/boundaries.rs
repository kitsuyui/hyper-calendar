//! The exports of the C library and the WebAssembly module, read from
//! their sources and from the facade's table of shared exports, for the
//! tests that hold the READMEs and the docs to them.
//!
//! A boundary's exports are, in order: the `#[unsafe(no_mangle)]`
//! functions of its `src/lib.rs`, in every build; then, for each layer
//! module `lib.rs` declares behind `#[cfg(feature = "...")]`, that
//! module's own `#[unsafe(no_mangle)]` functions and, where it expands
//! `hc::exports!("<layer>", ...)`, the rows of that layer in
//! `crates/hyper-calendar/src/exports.rs`, each with the boundary's
//! rustdoc and the signature its argument kinds give on that boundary.

#![allow(dead_code)]

use std::path::Path;

/// Which boundary.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    /// `hyper-calendar-ffi`.
    C,
    /// `hyper-calendar-wasm`.
    Wasm,
}

/// One exported function.
#[derive(Debug)]
pub struct Export {
    pub name: String,
    /// Parameter names and their Rust types, in order.
    pub params: Vec<(String, String)>,
    /// The Rust return type, if any.
    pub returns: Option<String>,
    /// The doc comment, one line each, without the `///`.
    pub docs: Vec<String>,
    /// The Cargo feature the export is behind, or `always`.
    pub feature: String,
    /// Whether the function is `unsafe`.
    pub is_unsafe: bool,
}

impl Export {
    /// The first paragraph of the doc comment, as one line, with intra-doc
    /// links made plain code.
    pub fn summary(&self) -> String {
        self.docs
            .iter()
            .take_while(|line| !line.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
            .replace("[`", "`")
            .replace("`]", "`")
    }
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", path.display()))
}

/// The doc lines immediately above `index`, oldest first, skipping
/// attributes.
fn docs_above(lines: &[&str], index: usize) -> Vec<String> {
    let mut docs = Vec::new();
    let mut cursor = index;
    while cursor > 0 {
        let line = lines[cursor - 1].trim_start();
        if let Some(doc) = line.strip_prefix("///") {
            docs.push(doc.strip_prefix(' ').unwrap_or(doc).trim_end().to_owned());
        } else if !line.starts_with("#[") {
            break;
        }
        cursor -= 1;
    }
    docs.reverse();
    docs
}

/// `pub [unsafe] extern "C" fn NAME(PARAMS) [-> RET] {` into its parts.
fn parse_signature(signature: &str, docs: Vec<String>, feature: &str) -> Export {
    let is_unsafe = signature.contains("unsafe extern");
    let after_fn = signature
        .split(" fn ")
        .nth(1)
        .unwrap_or_else(|| panic!("not a function signature: {signature}"));
    let (Some(open), Some(close)) = (after_fn.find('('), after_fn.rfind(')')) else {
        panic!("no parameter list in signature: {signature}")
    };
    let name = after_fn[..open].to_owned();
    let params = after_fn[open + 1..close]
        .split(',')
        .map(str::trim)
        .filter(|param| !param.is_empty())
        .map(|param| {
            let (name, ty) = param
                .split_once(':')
                .unwrap_or_else(|| panic!("{name}: parameter without a type: {param}"));
            (name.trim().to_owned(), ty.trim().to_owned())
        })
        .collect();
    let tail = after_fn[close + 1..].trim().trim_end_matches('{').trim();
    let returns = tail.strip_prefix("->").map(|ret| ret.trim().to_owned());
    Export {
        name,
        params,
        returns,
        docs,
        feature: feature.to_owned(),
        is_unsafe,
    }
}

/// The hand-written exports of one source file.
fn written_exports(source: &str, feature: &str, out: &mut Vec<Export>, table: &str, side: Side) {
    let lines: Vec<&str> = source.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        if let Some(layer) = line
            .trim()
            .strip_prefix("hc::exports!(\"")
            .and_then(|rest| rest.split('"').next())
        {
            out.extend(rows(table, layer, feature, side));
            continue;
        }
        if line.trim() != "#[unsafe(no_mangle)]" {
            continue;
        }
        let docs = docs_above(&lines, index);
        assert!(
            docs.first().is_some_and(|line| !line.is_empty()),
            "the export after line {} has no doc summary",
            index + 1
        );
        // The signature runs from the next line to the line that opens the
        // body; rustfmt may spread it over several.
        let mut signature = String::new();
        let mut cursor = index + 1;
        loop {
            let piece = lines[cursor].trim();
            if !piece.starts_with("#[") {
                signature.push_str(piece);
                signature.push(' ');
            }
            if piece.ends_with('{') {
                break;
            }
            cursor += 1;
        }
        out.push(parse_signature(&signature, docs, feature));
    }
}

/// What `#[cfg(feature = "...")]` names, if the line is one.
fn feature_gate(line: &str) -> Option<&str> {
    line.strip_prefix("#[cfg(feature = \"")?
        .strip_suffix("\")]")
}

/// Every export of a boundary, in order.
pub fn exports(crate_dir: &Path, facade_dir: &Path, side: Side) -> Vec<Export> {
    let table = read(&facade_dir.join("src/exports.rs"));
    let lib = read(&crate_dir.join("src/lib.rs"));
    let mut out = Vec::new();
    written_exports(&lib, "always", &mut out, &table, side);
    let lines: Vec<&str> = lib.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        let Some(feature) = feature_gate(line) else {
            continue;
        };
        let Some(module) = lines
            .get(index + 1)
            .and_then(|next| next.strip_prefix("mod "))
            .and_then(|rest| rest.strip_suffix(';'))
        else {
            continue;
        };
        let source = read(&crate_dir.join(format!("src/{module}.rs")));
        written_exports(&source, feature, &mut out, &table, side);
    }
    assert!(!out.is_empty(), "no exports found");
    out
}

/// The Rust type of an argument kind of the table on a boundary.
fn kind_type(kind: &str, side: Side) -> &str {
    match (kind, side) {
        ("name" | "text" | "opt", Side::C) => "*const c_char",
        ("name" | "text" | "opt", Side::Wasm) => "*const u8",
        ("flag" | "int", Side::C) => "c_int",
        ("flag" | "int", Side::Wasm) => "i32",
        (ty, _) => ty,
    }
}

/// The rows of one layer of the table, as exports of a boundary.
fn rows(table: &str, layer: &str, feature: &str, side: Side) -> Vec<Export> {
    let start = format!("(\"{layer}\", $backend:ident) => {{ $backend! {{");
    let at = table
        .find(&start)
        .unwrap_or_else(|| panic!("the table has no layer {layer}"));
    let body = &table[at + start.len()..];
    let body = &body[..body
        .find("\n    } };")
        .unwrap_or_else(|| panic!("layer {layer} does not end"))];
    let mut out = Vec::new();
    let mut docs = (Vec::new(), Vec::new());
    let mut block: Option<bool> = None;
    // A row may be spread over several lines, as rustfmt would spread a
    // signature; it ends at the `;` that closes it.
    let mut row = String::new();
    for line in body.lines().map(str::trim) {
        if !row.is_empty() || line.starts_with("fn ") {
            row.push_str(line);
            row.push(' ');
            if !line.ends_with(';') {
                continue;
            }
        }
        match line {
            "c {" => block = Some(true),
            "wasm {" => block = Some(false),
            "}" => block = None,
            _ => {}
        }
        if let (Some(c), Some(doc)) = (block, line.strip_prefix("///")) {
            let doc = doc.strip_prefix(' ').unwrap_or(doc).trim_end().to_owned();
            if c {
                docs.0.push(doc)
            } else {
                docs.1.push(doc)
            }
            continue;
        }
        if row.is_empty() {
            continue;
        }
        let joined = std::mem::take(&mut row)
            .replace("( ", "(")
            .replace(", )", ")")
            .replace(",)", ")");
        let (docs_c, docs_wasm) = std::mem::take(&mut docs);
        out.push(row_export(
            joined.trim_start_matches("fn "),
            if side == Side::C { docs_c } else { docs_wasm },
            feature,
            side,
        ));
    }
    assert!(!out.is_empty(), "layer {layer} has no rows");
    out
}

/// One row, `NAME(ARGS) -> SHAPE = LINE;`, as an export of a boundary.
fn row_export(row: &str, docs: Vec<String>, feature: &str, side: Side) -> Export {
    let open = row.find('(').unwrap_or_else(|| panic!("a row's arguments"));
    let name = row[..open].to_owned();
    let mut depth = 0;
    let close = open
        + row[open..]
            .find(|c: char| {
                depth += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                depth == 0
            })
            .unwrap_or_else(|| panic!("a row's arguments end"));
    let mut params = Vec::new();
    let mut is_unsafe = false;
    for arg in row[open + 1..close]
        .split(", ")
        .filter(|arg| !arg.is_empty())
    {
        let (arg, kind) = arg
            .split_once(": ")
            .unwrap_or_else(|| panic!("an argument's kind"));
        let (kind, len) = match kind.split_once('(') {
            Some((kind, len)) => (kind, Some(len.trim_end_matches(')'))),
            None => (kind, None),
        };
        params.push((arg.to_owned(), kind_type(kind, side).to_owned()));
        if let Some(len) = len {
            is_unsafe = true;
            if side == Side::Wasm {
                params.push((len.to_owned(), "usize".to_owned()));
            }
        }
    }
    let shape = row[close + 1..]
        .trim_start()
        .strip_prefix("-> ")
        .unwrap_or_else(|| panic!("a row's shape"));
    let returns = if shape.starts_with("line") {
        is_unsafe = true;
        match side {
            Side::C => {
                params.push(("buffer".into(), "*mut c_char".into()));
                params.push(("capacity".into(), "usize".into()));
                params.push(("written".into(), "*mut usize".into()));
                "HcStatus"
            }
            Side::Wasm => {
                params.push(("buffer".into(), "*mut u8".into()));
                params.push(("capacity".into(), "usize".into()));
                "i64"
            }
        }
    } else {
        let out = shape
            .strip_prefix("value(")
            .and_then(|rest| rest.split(')').next())
            .unwrap_or_else(|| panic!("{name}: a shape is `line` or `value(out: kind)`"));
        let (out, kind) = out
            .split_once(": ")
            .unwrap_or_else(|| panic!("an out-parameter's kind"));
        match side {
            Side::C => {
                is_unsafe = true;
                params.push((out.to_owned(), format!("*mut {}", kind_type(kind, side))));
                "HcStatus"
            }
            Side::Wasm => "i64",
        }
    };
    Export {
        name,
        params,
        returns: Some(returns.to_owned()),
        docs,
        feature: feature.to_owned(),
        is_unsafe,
    }
}

/// A source text of `///` blocks and signatures, one per export, for a
/// test that reads rustdoc the way it reads a source file.
pub fn as_source(exports: &[Export]) -> String {
    let mut out = String::new();
    for export in exports {
        for doc in &export.docs {
            out.push_str("/// ");
            out.push_str(doc);
            out.push('\n');
        }
        let unsafety = if export.is_unsafe { "unsafe " } else { "" };
        out.push_str(&format!("pub {unsafety}extern \"C\" fn {}(\n", export.name));
    }
    out
}
