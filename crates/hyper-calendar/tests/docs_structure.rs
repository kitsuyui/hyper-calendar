//! Guards the shape of the Markdown documents.
//!
//! Several agents edit the roadmap, the system documents and the crates'
//! READMEs side by side, and a merge of two branches that each add the same
//! row to a table keeps both copies without a conflict. Duplicated rows have
//! been merged by hand before. This test fails on these faults instead:
//!
//! - a table in any Markdown file of the repository — `docs/`, every
//!   crate's README, the top-level README and CONTRIBUTING — that repeats a
//!   whole row within that table;
//! - a table in `docs/calendars.md`, `docs/observances.md` or
//!   `docs/systems/README.md` that repeats the first cell of a row within
//!   that table (the same first cell in two different tables, such as a
//!   country listed in two regions, is not a fault, and elsewhere a first
//!   cell may repeat: a Ranges table has a row per range, not per kind);
//! - a `docs/systems/*.md` file that repeats a heading at the same level;
//! - a `docs/systems/*.md` file without exactly one H1.
//!
//! The parser knows only what these files use: pipe tables of consecutive
//! lines that begin with `|`, whose first two lines are the header and the
//! delimiter row; ATX headings; and fenced code blocks, which are skipped.
//! Cells are split on `|` not escaped as `\|`, as GitHub splits them.

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fs;
use std::path::{Path, PathBuf};

/// The files whose tables may not repeat a first cell either, relative to
/// the repository root: the roadmap and the index, keyed by their first
/// column.
const KEYED_FILES: [&str; 3] = [
    "docs/calendars.md",
    "docs/observances.md",
    "docs/systems/README.md",
];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// The lines outside fenced code blocks, numbered from 1.
fn prose_lines(source: &str) -> Vec<(usize, &str)> {
    let mut fence: Option<&str> = None;
    let mut lines = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        let marker = ["```", "~~~"]
            .into_iter()
            .find(|marker| trimmed.starts_with(marker));
        match (fence, marker) {
            (None, Some(marker)) => fence = Some(marker),
            (Some(open), Some(marker)) if open == marker => fence = None,
            (None, None) => lines.push((index + 1, line)),
            _ => {}
        }
    }
    lines
}

/// A table's body rows: the line number and the trimmed cells of each.
type Table = Vec<(usize, Vec<String>)>;

fn cells(line: &str) -> Vec<String> {
    let line = line.trim();
    let line = line.strip_prefix('|').unwrap_or(line);
    let line = match line.strip_suffix('|') {
        Some(rest) if !rest.ends_with('\\') => rest,
        _ => line,
    };
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut escaped = false;
    for c in line.chars() {
        if c == '|' && !escaped {
            cells.push(cell.trim().to_owned());
            cell.clear();
        } else {
            cell.push(c);
        }
        escaped = c == '\\' && !escaped;
    }
    cells.push(cell.trim().to_owned());
    cells
}

/// Every table in `source`, without its header and delimiter rows.
fn tables(source: &str) -> Vec<Table> {
    let mut tables = Vec::new();
    let mut current: Option<Table> = None;
    let mut previous_line = 0;
    for (line_no, line) in prose_lines(source) {
        let is_row = line.trim_start().starts_with('|');
        let continues = line_no == previous_line + 1;
        previous_line = line_no;
        if !is_row || !continues {
            tables.extend(current.take());
        }
        if is_row {
            current
                .get_or_insert_with(Vec::new)
                .push((line_no, cells(line)));
        }
    }
    tables.extend(current);
    tables
        .into_iter()
        .map(|table| table.into_iter().skip(2).collect())
        .collect()
}

/// The line `key` was first seen on, or `None` after recording `line_no` as
/// its first.
fn first_seen<K: Ord>(seen: &mut BTreeMap<K, usize>, key: K, line_no: usize) -> Option<usize> {
    match seen.entry(key) {
        Entry::Occupied(first) => Some(*first.get()),
        Entry::Vacant(slot) => {
            slot.insert(line_no);
            None
        }
    }
}

/// Every Markdown file of the repository, relative to its root, in order:
/// all but those under a directory whose name starts with `.` or is
/// `target` or `node_modules`.
fn markdown_files() -> Vec<String> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
        let entries =
            fs::read_dir(dir).unwrap_or_else(|error| panic!("{}: {error}", dir.display()));
        for entry in entries {
            let path = entry
                .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
                .path();
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if path.is_dir() {
                if !(name.starts_with('.') || name == "target" || name == "node_modules") {
                    walk(root, &path, out);
                }
            } else if path.extension().is_some_and(|ext| ext == "md") {
                let relative = path.strip_prefix(root).unwrap_or(&path);
                out.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    let root = repository_root();
    let mut files = Vec::new();
    walk(&root, &root, &mut files);
    files.sort();
    files
}

/// The whole rows one file's tables repeat, as `path:line: message`.
fn row_faults(path: &str, source: &str) -> Vec<String> {
    let mut faults = Vec::new();
    for table in tables(source) {
        let mut rows: BTreeMap<&[String], usize> = BTreeMap::new();
        for (line_no, cells) in &table {
            if let Some(first) = first_seen(&mut rows, cells, *line_no) {
                faults.push(format!(
                    "{path}:{line_no}: repeats the row at line {first} of the same table"
                ));
            }
        }
    }
    faults
}

/// The faults in one keyed file's tables, a repeated row or a repeated
/// first cell, as `path:line: message`.
fn table_faults(path: &str, source: &str) -> Vec<String> {
    let mut faults = Vec::new();
    for table in tables(source) {
        let mut rows: BTreeMap<&[String], usize> = BTreeMap::new();
        let mut first_cells: BTreeMap<&str, usize> = BTreeMap::new();
        for (line_no, cells) in &table {
            if let Some(first) = first_seen(&mut rows, cells, *line_no) {
                faults.push(format!(
                    "{path}:{line_no}: repeats the row at line {first} of the same table"
                ));
                continue;
            }
            let key = cells[0].as_str();
            if let Some(first) = first_seen(&mut first_cells, key, *line_no) {
                faults.push(format!(
                    "{path}:{line_no}: repeats the first cell {key:?} of line {first} in the same table"
                ));
            }
        }
    }
    faults
}

/// An ATX heading's level and text, or `None` for any other line.
fn heading(line: &str) -> Option<(usize, &str)> {
    let level = line.bytes().take_while(|&b| b == b'#').count();
    if !(1..=6).contains(&level) {
        return None;
    }
    let rest = &line[level..];
    if !(rest.is_empty() || rest.starts_with([' ', '\t'])) {
        return None;
    }
    let text = rest.trim();
    // A closing run of `#` counts only after a space, as in `## Notes ##`.
    let without_closing = text.trim_end_matches('#');
    let text = if without_closing.is_empty() || without_closing.ends_with([' ', '\t']) {
        without_closing.trim_end()
    } else {
        text
    };
    Some((level, text))
}

/// The faults in one system document's headings.
fn heading_faults(path: &str, source: &str) -> Vec<String> {
    let mut faults = Vec::new();
    let mut seen: BTreeMap<(usize, &str), usize> = BTreeMap::new();
    let mut h1s = Vec::new();
    for (line_no, line) in prose_lines(source) {
        let Some((level, text)) = heading(line) else {
            continue;
        };
        if level == 1 {
            h1s.push(line_no);
        }
        if let Some(first) = first_seen(&mut seen, (level, text), line_no) {
            faults.push(format!(
                "{path}:{line_no}: repeats the level-{level} heading {text:?} of line {first}"
            ));
        }
    }
    match h1s.as_slice() {
        [_] => {}
        [] => faults.push(format!("{path}: has no H1")),
        [_, rest @ ..] => {
            for line_no in rest {
                faults.push(format!("{path}:{line_no}: a second H1"));
            }
        }
    }
    faults
}

fn system_documents() -> Vec<PathBuf> {
    let dir = repository_root().join("docs/systems");
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
        .map(|entry| {
            entry
                .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
                .path()
        })
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect();
    paths.sort();
    paths
}

#[test]
fn no_table_in_any_markdown_file_repeats_a_row() {
    let root = repository_root();
    let files = markdown_files();
    for expected in [
        "README.md",
        "CONTRIBUTING.md",
        "docs/policy.md",
        "docs/systems/spreadsheet-dates.md",
        "crates/hyper-calendar-wasm/README.md",
        "crates/hyper-calendar-ffi/README.md",
        "crates/hc-tz/README.md",
    ] {
        assert!(
            files.iter().any(|file| file == expected),
            "{expected} not walked"
        );
    }
    let mut faults = Vec::new();
    let mut checked = 0;
    for path in &files {
        let source = read(&root.join(path));
        checked += tables(&source).len();
        faults.extend(row_faults(path, &source));
    }
    assert!(
        checked >= 100,
        "found only {checked} tables; is the parser broken?"
    );
    assert!(faults.is_empty(), "\n{}", faults.join("\n"));
}

#[test]
fn no_keyed_table_repeats_a_row_or_a_first_cell() {
    let root = repository_root();
    let mut faults = Vec::new();
    let mut checked = 0;
    for path in KEYED_FILES {
        let source = read(&root.join(path));
        checked += tables(&source).len();
        faults.extend(table_faults(path, &source));
    }
    assert!(
        checked >= 3,
        "found only {checked} tables; is the parser broken?"
    );
    assert!(faults.is_empty(), "\n{}", faults.join("\n"));
}

#[test]
fn no_system_document_repeats_a_heading_or_has_two_h1s() {
    let paths = system_documents();
    assert!(paths.len() > 1, "docs/systems/ holds no documents");
    let mut faults = Vec::new();
    for path in paths {
        let name = format!(
            "docs/systems/{}",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        faults.extend(heading_faults(&name, &read(&path)));
    }
    assert!(faults.is_empty(), "\n{}", faults.join("\n"));
}

/// The six sections of a system document, in order (docs/policy.md §12;
/// docs/systems/README.md gives the headings).
const SECTIONS: [&str; 6] = [
    "What it is",
    "How it works",
    "What is carried",
    "Accuracy",
    "Sources",
    "Code",
];

/// The fault in one system document's H2 headings, if they are not the six.
fn section_faults(path: &str, source: &str) -> Vec<String> {
    let h2s: Vec<&str> = prose_lines(source)
        .into_iter()
        .filter_map(|(_, line)| heading(line))
        .filter(|(level, _)| *level == 2)
        .map(|(_, text)| text)
        .collect();
    if h2s == SECTIONS {
        Vec::new()
    } else {
        vec![format!(
            "{path}: the H2 headings are {h2s:?}, not the six sections {SECTIONS:?}"
        )]
    }
}

#[test]
fn every_system_document_has_the_six_sections_in_order() {
    let mut faults = Vec::new();
    for path in system_documents() {
        if path.file_name().is_some_and(|name| name == "README.md") {
            continue;
        }
        let name = format!(
            "docs/systems/{}",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        faults.extend(section_faults(&name, &read(&path)));
    }
    assert!(faults.is_empty(), "\n{}", faults.join("\n"));
}

// The checks on made-up input, so that a parser that finds nothing does not
// pass the real documents by accident.

#[test]
fn an_extra_or_misplaced_section_is_found() {
    let six = "# T\n\n## What it is\n\n## How it works\n\n## What is carried\n\n## Accuracy\n\n## Sources\n\n## Code\n";
    assert!(section_faults("t.md", six).is_empty());
    let extra = six.replace("## Accuracy", "## Extra\n\n## Accuracy");
    assert_eq!(section_faults("t.md", &extra).len(), 1);
    let swapped = six.replace("## Sources\n\n## Code", "## Code\n\n## Sources");
    assert_eq!(section_faults("t.md", &swapped).len(), 1);
    let fenced = six.replace("## Accuracy", "```\n## Fenced\n```\n\n## Accuracy");
    assert!(section_faults("t.md", &fenced).is_empty());
}

#[test]
fn a_repeated_row_is_found() {
    let source = "| A | B |\n| --- | --- |\n| x | 1 |\n| y | 2 |\n| x | 1 |\n|x|1|\n";
    assert_eq!(
        table_faults("t.md", source),
        [
            "t.md:5: repeats the row at line 3 of the same table",
            "t.md:6: repeats the row at line 3 of the same table",
        ]
    );
}

#[test]
fn a_repeated_row_is_found_and_a_repeated_first_cell_passes_outside_the_keyed_files() {
    let source = "| A | B |\n| --- | --- |\n| x | 1 |\n| x | 2 |\n| x | 1 |\n";
    assert_eq!(
        row_faults("t.md", source),
        ["t.md:5: repeats the row at line 3 of the same table"]
    );
}

#[test]
fn a_repeated_first_cell_is_found() {
    let source = "| A | B |\n| --- | --- |\n| x | 1 |\n|x|2|\n";
    assert_eq!(
        table_faults("t.md", source),
        ["t.md:4: repeats the first cell \"x\" of line 3 in the same table"]
    );
}

#[test]
fn the_same_first_cell_in_two_tables_passes() {
    let source = "| A | B |\n| --- | --- |\n| x | 1 |\n\n| A | B |\n| --- | --- |\n| x | 2 |\n";
    assert_eq!(table_faults("t.md", source), Vec::<String>::new());
}

#[test]
fn an_escaped_pipe_does_not_split_a_cell() {
    assert_eq!(cells(r"| a \| b | c |"), [r"a \| b", "c"]);
    let source = "| A | B |\n| --- | --- |\n| a \\| b | 1 |\n| a \\| c | 1 |\n";
    assert_eq!(table_faults("t.md", source), Vec::<String>::new());
}

#[test]
fn a_fenced_table_is_not_checked() {
    let source = "```text\n| A |\n| --- |\n| x |\n| x |\n```\n";
    assert_eq!(table_faults("t.md", source), Vec::<String>::new());
}

#[test]
fn a_repeated_heading_and_a_second_h1_are_found() {
    let source = "# One\n\n## Sources\n\n### Sources\n\n## Sources ##\n\n## C#\n\n## C\n\n```\n# not a heading\n```\n\n# Two\n";
    assert_eq!(
        heading_faults("s.md", source),
        [
            "s.md:7: repeats the level-2 heading \"Sources\" of line 3",
            "s.md:17: a second H1",
        ]
    );
    assert_eq!(heading_faults("s.md", "## Only\n"), ["s.md: has no H1"]);
}
