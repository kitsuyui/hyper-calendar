//! The stable identifier of a holiday within its table.
//!
//! A holiday's English name is what a line shows; its identifier is what a
//! line is joined on. A name may be reworded, may carry an apostrophe or a
//! macron, and may be shared by two entries that are not the same day; an
//! identifier is lower-case ASCII, hyphenated, and one to a holiday within
//! its table, so that a page can join the lines of `hc_holidays_on` with
//! those of `hc_common_worship_on`, or a year's lines with a locale's
//! names, without comparing titles.
//!
//! Every rule has one. A rule that sets none is identified by the
//! kebab-case of its English name — `New Year's Day` is `new-years-day`,
//! `Tōkanya` is `tokanya` — so names that differ only in case,
//! punctuation or diacritics are one holiday: the United States' states
//! spell `Mothers' Day` and `Mother's Day` of one observance, and share
//! its identifier. A rule sets one only where that would not do: where two
//! days of a table would fold to the same identifier, or where two
//! readings of one day are registered as separate conventions
//! (`docs/policy.md` §5). Two rules of one table with the same English
//! name are the same holiday — its successive statutory forms, its
//! nationwide and regional shapes — and share the identifier.
//!
//! The rule is held by a test in `crates/hc-holiday/tests/ids.rs`: every
//! identifier is well formed, and one a rule sets itself is no other
//! name's.

use core::fmt::{self, Write as _};
use core::hash::{Hash, Hasher};

/// The identifier of a holiday: the one its rule sets, or the kebab-case
/// of its English name.
///
/// Compared, hashed and written by the identifier it renders to, so that
/// an explicit `new-years-day` and a derived one are equal.
#[derive(Debug, Clone, Copy)]
pub struct HolidayId {
    /// The identifier the rule set, or `""`.
    explicit: &'static str,
    /// The English name it is derived from otherwise.
    name: &'static str,
}

impl HolidayId {
    /// The identifier a rule with English name `name` has when it sets
    /// `explicit`, which may be `""` for none.
    #[must_use]
    pub const fn new(explicit: &'static str, name: &'static str) -> Self {
        Self { explicit, name }
    }

    /// The identifier derived from an English name alone.
    #[must_use]
    pub const fn of_name(name: &'static str) -> Self {
        Self { explicit: "", name }
    }

    /// An identifier set outright.
    #[must_use]
    pub const fn explicit(id: &'static str) -> Self {
        Self {
            explicit: id,
            name: "",
        }
    }

    /// Whether the rule set the identifier, rather than leaving it to the
    /// name.
    #[must_use]
    pub const fn is_explicit(&self) -> bool {
        !self.explicit.is_empty()
    }

    /// The characters of the identifier, in order.
    fn chars(&self) -> Chars<'_> {
        Chars {
            explicit: self.explicit.chars(),
            derived: Kebab::new(self.name),
            use_explicit: self.is_explicit(),
        }
    }

    /// Whether `given` names this identifier, matched as every identifier
    /// is ([`hc_core::catalogue::matches`]): white space around `given`
    /// ignored, ASCII letters in either case.
    #[must_use]
    pub fn matches(&self, given: &str) -> bool {
        let given = given.trim().chars().map(|c| c.to_ascii_lowercase());
        let mine = self.chars();
        given.eq(mine)
    }

    /// Whether `text`, already trimmed and in lower case, is exactly this
    /// identifier.
    #[must_use]
    pub fn is(&self, text: &str) -> bool {
        text.chars().eq(self.chars())
    }
}

impl fmt::Display for HolidayId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.chars() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl PartialEq for HolidayId {
    fn eq(&self, other: &Self) -> bool {
        self.chars().eq(other.chars())
    }
}

impl Eq for HolidayId {}

impl Hash for HolidayId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.chars() {
            c.hash(state);
        }
    }
}

impl PartialOrd for HolidayId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HolidayId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.chars().cmp(other.chars())
    }
}

/// The characters of a [`HolidayId`].
struct Chars<'a> {
    explicit: core::str::Chars<'a>,
    derived: Kebab<'a>,
    use_explicit: bool,
}

impl Iterator for Chars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.use_explicit {
            self.explicit.next()
        } else {
            self.derived.next()
        }
    }
}

/// The kebab-case of a name: its ASCII letters and digits in lower case,
/// each Latin letter with a diacritic replaced by its base letter, and
/// every other run of characters by one hyphen, with no hyphen at either
/// end. `Washington's Birthday` is `washingtons-birthday` — an apostrophe
/// within a word is dropped, as it is in a possessive — and
/// `Kurban Bayramı` is `kurban-bayrami`.
#[derive(Debug, Clone)]
pub struct Kebab<'a> {
    rest: core::str::Chars<'a>,
    /// Characters folded and not yet given out, first at the front.
    queue: [Option<char>; 3],
    /// Whether a letter or digit has been given out, so that a hyphen may
    /// follow.
    started: bool,
    /// Whether a separator is owed before the next letter or digit.
    separator_owed: bool,
}

impl<'a> Kebab<'a> {
    /// The kebab-case of `name`, character by character.
    #[must_use]
    pub fn new(name: &'a str) -> Self {
        Self {
            rest: name.chars(),
            queue: [None; 3],
            started: false,
            separator_owed: false,
        }
    }

    /// Queues the identifier's characters for one folded letter: the
    /// separator owed, then the letter or letters.
    fn push(&mut self, first: char, second: Option<char>) {
        let mut queue = [None; 3];
        let mut at = 0;
        if self.separator_owed {
            queue[at] = Some('-');
            at += 1;
        }
        queue[at] = Some(first);
        at += 1;
        queue[at] = second;
        self.queue = queue;
        self.separator_owed = false;
        self.started = true;
    }

    /// The next queued character, if any.
    fn pop(&mut self) -> Option<char> {
        let next = self.queue[0].take()?;
        self.queue.rotate_left(1);
        Some(next)
    }
}

impl Iterator for Kebab<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            if let Some(c) = self.pop() {
                return Some(c);
            }
            let c = self.rest.next()?;
            match fold(c) {
                Folded::Keep(kept) => self.push(kept, None),
                Folded::Two(first, second) => self.push(first, Some(second)),
                // An apostrophe within a word, as in a possessive, is
                // dropped; at a word's edge it separates as a space does.
                Folded::Apostrophe => {
                    let within = self.started
                        && !self.separator_owed
                        && self.rest.clone().next().is_some_and(char::is_alphanumeric);
                    if !within {
                        self.separator_owed = self.started;
                    }
                }
                Folded::Separator => self.separator_owed = self.started,
            }
        }
    }
}

/// How one character of a name folds into an identifier.
enum Folded {
    /// One character of the identifier.
    Keep(char),
    /// Two characters: `ß` is `ss`, `æ` is `ae`.
    Two(char, char),
    /// An apostrophe, which is dropped within a word.
    Apostrophe,
    /// Anything else, which separates.
    Separator,
}

/// Folds one character: ASCII letters and digits to lower case, Latin
/// letters with diacritics to their base letters, apostrophes to
/// [`Folded::Apostrophe`], everything else to a separator.
const fn fold(c: char) -> Folded {
    if c.is_ascii_alphanumeric() {
        return Folded::Keep(c.to_ascii_lowercase());
    }
    match c {
        '\'' | '\u{2019}' => Folded::Apostrophe,
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å'
        | 'Ā' | 'Ă' | 'Ą' => Folded::Keep('a'),
        'ç' | 'ć' | 'č' | 'ĉ' | 'ċ' | 'Ç' | 'Ć' | 'Č' | 'Ĉ' | 'Ċ' => Folded::Keep('c'),
        'ď' | 'đ' | 'Ď' | 'Đ' => Folded::Keep('d'),
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' | 'È' | 'É' | 'Ê' | 'Ë' | 'Ē' | 'Ĕ'
        | 'Ė' | 'Ę' | 'Ě' => Folded::Keep('e'),
        'ğ' | 'ĝ' | 'ġ' | 'ģ' | 'Ğ' | 'Ĝ' | 'Ġ' | 'Ģ' => Folded::Keep('g'),
        'ĥ' | 'ħ' | 'Ĥ' | 'Ħ' | 'ḥ' | 'Ḥ' => Folded::Keep('h'),
        'ì' | 'í' | 'î' | 'ï' | 'ī' | 'ĭ' | 'į' | 'ı' | 'Ì' | 'Í' | 'Î' | 'Ï' | 'Ī' | 'Ĭ' | 'Į'
        | 'İ' => Folded::Keep('i'),
        'ĵ' | 'Ĵ' => Folded::Keep('j'),
        'ķ' | 'Ķ' => Folded::Keep('k'),
        'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' | 'Ĺ' | 'Ļ' | 'Ľ' | 'Ŀ' | 'Ł' => Folded::Keep('l'),
        'ñ' | 'ń' | 'ņ' | 'ň' | 'Ñ' | 'Ń' | 'Ņ' | 'Ň' | 'ṅ' | 'Ṅ' | 'ṇ' | 'Ṇ' => {
            Folded::Keep('n')
        }
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø'
        | 'Ō' | 'Ŏ' | 'Ő' => Folded::Keep('o'),
        'ŕ' | 'ŗ' | 'ř' | 'Ŕ' | 'Ŗ' | 'Ř' | 'ṛ' | 'Ṛ' => Folded::Keep('r'),
        'ś' | 'ŝ' | 'ş' | 'š' | 'ș' | 'Ś' | 'Ŝ' | 'Ş' | 'Š' | 'Ș' | 'ṣ' | 'Ṣ' => {
            Folded::Keep('s')
        }
        'ţ' | 'ť' | 'ŧ' | 'ț' | 'Ţ' | 'Ť' | 'Ŧ' | 'Ț' | 'ṭ' | 'Ṭ' => Folded::Keep('t'),
        'ù' | 'ú' | 'û' | 'ü' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' | 'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ū' | 'Ŭ'
        | 'Ů' | 'Ű' | 'Ų' => Folded::Keep('u'),
        'ŵ' | 'Ŵ' => Folded::Keep('w'),
        'ý' | 'ÿ' | 'ŷ' | 'Ý' | 'Ÿ' | 'Ŷ' => Folded::Keep('y'),
        'ź' | 'ż' | 'ž' | 'Ź' | 'Ż' | 'Ž' => Folded::Keep('z'),
        'ß' => Folded::Two('s', 's'),
        'æ' | 'Æ' => Folded::Two('a', 'e'),
        'œ' | 'Œ' => Folded::Two('o', 'e'),
        _ => Folded::Separator,
    }
}
