//! Checks that `docs/references.bib` is well-formed BibTeX.
//!
//! docs/policy.md §11 keeps one BibTeX entry per shared source because every
//! reference manager reads the format. A reference manager given an entry
//! with an unclosed brace swallows the entries after it, and one given two
//! entries with the same key keeps only one. This test is a minimal parser
//! with no dependencies, run by `cargo test --workspace` in CI, that fails on
//! these faults:
//!
//! - text outside an entry that is neither blank nor a `%` comment;
//! - an entry without a type or without a key;
//! - braces that do not balance inside an entry, including an entry left
//!   open when the next `@` or `%` line begins;
//! - a key used by two entries (BibTeX compares keys without case);
//! - a field that does not read `name = value`, or is not followed by a
//!   comma or the end of the entry;
//! - a field named twice in one entry.
//!
//! It does not check what the fields say. `bibtex` run over every entry
//! (`\citation{*}` in the `.aux` file) checks that, and reports a missing
//! field as a warning.

use std::collections::BTreeMap;

const BIB: &str = include_str!("../../../docs/references.bib");

/// One entry: its key, the line it starts on, and its field names in order.
struct Entry {
    key: String,
    line: usize,
    fields: Vec<String>,
}

fn parse(source: &str) -> Result<Vec<Entry>, String> {
    let mut entries = Vec::new();
    let mut lines = source.lines().enumerate().map(|(i, l)| (i + 1, l));

    while let Some((line_no, line)) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('%') {
            continue;
        }
        let Some(rest) = line.strip_prefix('@') else {
            return Err(format!("line {line_no}: text outside an entry: {line}"));
        };

        let Some((kind, after_brace)) = rest.split_once('{') else {
            return Err(format!("line {line_no}: no `{{` after the entry type"));
        };
        if kind.is_empty() || !kind.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(format!("line {line_no}: bad entry type {kind:?}"));
        }
        let Some((key, _)) = after_brace.split_once(',') else {
            return Err(format!("line {line_no}: no `,` after the entry key"));
        };
        if key.is_empty()
            || key
                .chars()
                .any(|c| c.is_whitespace() || "{}\"#%'(),=".contains(c))
        {
            return Err(format!("line {line_no}: bad entry key {key:?}"));
        }

        // Collect the body from after `@type{key,` to the brace that closes it.
        let mut depth = 1usize;
        let mut body = String::new();
        let mut closed = scan(&after_brace[key.len() + 1..], &mut depth, &mut body);
        while !closed {
            let Some((next_no, next)) = lines.next() else {
                return Err(format!("line {line_no}: entry {key} is never closed"));
            };
            if next.starts_with('@') || next.starts_with('%') {
                return Err(format!(
                    "line {line_no}: entry {key} is still open at line {next_no}"
                ));
            }
            closed = scan(next, &mut depth, &mut body);
        }

        entries.push(Entry {
            key: key.to_owned(),
            line: line_no,
            fields: field_names(&body).map_err(|e| format!("line {line_no}: {key}: {e}"))?,
        });
    }
    Ok(entries)
}

/// Appends one line of an entry to `body`, and reports whether the brace that
/// closes the entry was on it. Text after that brace is ignored.
fn scan(text: &str, depth: &mut usize, body: &mut String) -> bool {
    for c in text.chars() {
        match c {
            '{' => *depth += 1,
            '}' => {
                *depth -= 1;
                if *depth == 0 {
                    return true;
                }
            }
            _ => {}
        }
        body.push(c);
    }
    body.push('\n');
    false
}

/// The names of the fields in an entry body, which must read
/// `name = value, name = value` with an optional trailing comma. A value is
/// a braced group, a quoted string, or a bare word such as `mar` or `1998`.
fn field_names(body: &str) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    let mut chars = body.chars().peekable();
    let skip_space = |chars: &mut core::iter::Peekable<core::str::Chars<'_>>| {
        while chars.next_if(|c| c.is_whitespace()).is_some() {}
    };
    loop {
        skip_space(&mut chars);
        if chars.peek().is_none() {
            return Ok(names);
        }
        let mut name = String::new();
        while let Some(c) = chars.next_if(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-') {
            name.push(c.to_ascii_lowercase());
        }
        skip_space(&mut chars);
        if name.is_empty() || chars.next() != Some('=') {
            return Err(format!("expected `name =` after {:?}", names.last()));
        }
        skip_space(&mut chars);
        match chars.next() {
            Some('{') => {
                let mut depth = 1usize;
                while depth > 0 {
                    match chars.next() {
                        Some('{') => depth += 1,
                        Some('}') => depth -= 1,
                        Some(_) => {}
                        None => return Err(format!("the value of `{name}` is not closed")),
                    }
                }
            }
            Some('"') => {
                let mut depth = 0usize;
                loop {
                    match chars.next() {
                        Some('{') => depth += 1,
                        Some('}') => depth = depth.saturating_sub(1),
                        Some('"') if depth == 0 => break,
                        Some(_) => {}
                        None => return Err(format!("the value of `{name}` is not closed")),
                    }
                }
            }
            Some(c) if c.is_ascii_alphanumeric() => {
                while chars.next_if(char::is_ascii_alphanumeric).is_some() {}
            }
            _ => return Err(format!("`{name}` has no value")),
        }
        names.push(name);
        skip_space(&mut chars);
        match chars.next() {
            Some(',') | None => {}
            Some(c) => {
                return Err(format!(
                    "expected `,` after the value of `{}`, found {c:?}",
                    names[names.len() - 1]
                ));
            }
        }
    }
}

#[test]
fn references_bib_is_well_formed() {
    let entries = match parse(BIB) {
        Ok(entries) => entries,
        Err(error) => panic!("docs/references.bib: {error}"),
    };
    assert!(!entries.is_empty(), "docs/references.bib has no entries");

    let mut problems = Vec::new();
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for entry in &entries {
        if let Some(first) = seen.insert(entry.key.to_lowercase(), entry.line) {
            problems.push(format!(
                "key {} at line {} is also used at line {first}",
                entry.key, entry.line
            ));
        }
        let mut fields: Vec<&str> = entry.fields.iter().map(String::as_str).collect();
        fields.sort_unstable();
        for pair in fields.windows(2) {
            if pair[0] == pair[1] {
                problems.push(format!(
                    "entry {} at line {} has two `{}` fields",
                    entry.key, entry.line, pair[0]
                ));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "docs/references.bib:\n{}",
        problems.join("\n")
    );
}

#[test]
fn the_parser_rejects_each_fault() {
    let good = "% comment\n\n@misc{a,\n  title = {A {B} C},\n  note  = {x}\n}\n";
    assert!(parse(good).is_ok());
    let unclosed = "@misc{a,\n  note = {x}\n\n@misc{b,\n  note = {y}\n}\n";
    assert!(parse(unclosed).is_err());
    let no_key = "@misc{,\n  note = {x}\n}\n";
    assert!(parse(no_key).is_err());
    let no_type = "@{a,\n  note = {x}\n}\n";
    assert!(parse(no_type).is_err());
    let stray = "@misc{a,\n  note = {x}\n}\nstray\n";
    assert!(parse(stray).is_err());
    let missing_comma = "@misc{a,\n  note = {x}\n  note = {y}\n}\n";
    assert!(parse(missing_comma).is_err());
    let Ok(entries) = parse("@misc{a,\n  note = {x},\n  note = {y}\n}\n") else {
        panic!("two fields parse");
    };
    assert_eq!(entries[0].fields, ["note", "note"]);
}
