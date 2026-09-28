//! The lists a document gives in code, for the tests that hold a written
//! list of identifiers to the table it lists.

/// The lists a text gives in code: each a run of code words (between
/// backticks) that only a comma, "and", "or", ", and" or ", or" separate,
/// so that `` `a`, `b` or `c` `` is one list and `` `a` for `b` `` two.
pub fn code_lists(text: &str) -> Vec<Vec<&str>> {
    let mut lists: Vec<Vec<&str>> = Vec::new();
    let mut joined = false;
    for (index, piece) in text.split('`').enumerate() {
        if index % 2 == 1 {
            match lists.last_mut() {
                Some(list) if joined => list.push(piece),
                _ => lists.push(vec![piece]),
            }
        } else {
            joined = index > 0 && matches!(piece.trim(), "," | "and" | "or" | ", and" | ", or");
        }
    }
    lists
}
