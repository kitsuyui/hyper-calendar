//! The `plural=` expression of a gettext catalogue's `Plural-Forms` header,
//! which is how Python's `gettext.ngettext` — and so `humanize` — chooses a
//! form.
//!
//! These are not CLDR's plural categories. A catalogue numbers its forms
//! from 0 and says which form a count takes in a C expression of `n`:
//! `n != 1` for English and most of Europe, `n > 1` for French,
//! `n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20)
//! ? 1 : 2` for Russian. The expression is evaluated here as written, rather
//! than translated to categories, because the form a catalogue's translator
//! wrote under `msgstr[1]` is the one *its* expression selects, and no two
//! conventions agree on every count (Latvian puts 0 with the many, CLDR
//! with zero).
//!
//! The grammar is gettext's: integers, `n`, parentheses, the operators
//! `!`, `*`, `/`, `%`, `+`, `-`, `<`, `<=`, `>`, `>=`, `==`, `!=`, `&&`
//! and `||`, and `?:`, with C's precedence. Every comparison and logical
//! operator yields 0 or 1.

/// The form `expression` selects for `n`, or `None` when the expression is
/// not one of the grammar or divides by zero.
///
/// ```
/// use hc_humanize::natural::gettext::plural_index;
///
/// let russian = "(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2)";
/// assert_eq!(plural_index(russian, 21), Some(0));
/// assert_eq!(plural_index(russian, 22), Some(1));
/// assert_eq!(plural_index(russian, 25), Some(2));
/// assert_eq!(plural_index("n != 1", 1), Some(0));
/// ```
#[must_use]
pub fn plural_index(expression: &str, n: i64) -> Option<usize> {
    index(expression, Num::Small(n))
}

/// [`plural_index`] for a count of any size, written in decimal digits: the
/// number of a `humanize` `intword` of a googol is a float's whole value, up
/// to about 10²⁰⁸, which Python's integers hold and no machine integer does.
///
/// Only what an expression can ask of such a count is answered: its
/// remainder by a number that fits an `i64`, and its comparison with one. The
/// count is above every such number, whatever it is compared with. An
/// expression that adds to, subtracts from, multiplies or divides it, or
/// compares it with another so large a number, selects nothing.
///
/// ```
/// use hc_humanize::natural::gettext::plural_index_digits;
///
/// let russian = "(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2)";
/// assert_eq!(plural_index_digits(russian, "10000000000000000000000000000000000000000002"), Some(1));
/// assert_eq!(plural_index_digits("n != 1", "100000000000000000000000000000000000000000000"), Some(1));
/// ```
#[must_use]
pub fn plural_index_digits(expression: &str, digits: &str) -> Option<usize> {
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let digits = digits.trim_start_matches('0');
    index(
        expression,
        match digits.parse::<i64>() {
            Ok(small) => Num::Small(small),
            Err(_) if digits.is_empty() => Num::Small(0),
            Err(_) => Num::Huge(digits),
        },
    )
}

fn index(expression: &str, n: Num<'_>) -> Option<usize> {
    let mut parser = Parser {
        text: expression.as_bytes(),
        position: 0,
        n,
    };
    let value = parser.expression()?;
    parser.skip_blanks();
    if parser.position != parser.text.len() {
        return None;
    }
    match value {
        Num::Small(small) => usize::try_from(small).ok(),
        Num::Huge(_) => None,
    }
}

/// A number the expression computes with: a machine integer, or the count
/// itself when it does not fit one.
#[derive(Debug, Clone, Copy)]
enum Num<'a> {
    Small(i64),
    Huge(&'a str),
}

impl Num<'_> {
    fn truth(self) -> bool {
        match self {
            Self::Small(value) => value != 0,
            Self::Huge(_) => true,
        }
    }

    fn flag(value: bool) -> Self {
        Self::Small(i64::from(value))
    }

    /// The comparison of two numbers: `Less`, `Equal`, `Greater`.
    fn compare(self, other: Self) -> Option<core::cmp::Ordering> {
        use core::cmp::Ordering;
        match (self, other) {
            (Self::Small(left), Self::Small(right)) => Some(left.cmp(&right)),
            (Self::Huge(_), Self::Small(_)) => Some(Ordering::Greater),
            (Self::Small(_), Self::Huge(_)) => Some(Ordering::Less),
            (Self::Huge(left), Self::Huge(right)) => {
                Some(left.len().cmp(&right.len()).then_with(|| left.cmp(right)))
            }
        }
    }

    fn remainder(self, divisor: Self) -> Option<Self> {
        match (self, divisor) {
            (Self::Small(left), Self::Small(right)) => left.checked_rem(right).map(Self::Small),
            (Self::Huge(digits), Self::Small(modulus)) if modulus > 0 => {
                let modulus = i128::from(modulus);
                let mut rest = 0i128;
                for byte in digits.bytes() {
                    rest = (rest * 10 + i128::from(byte - b'0')) % modulus;
                }
                i64::try_from(rest).ok().map(Self::Small)
            }
            _ => None,
        }
    }
}

struct Parser<'a> {
    text: &'a [u8],
    position: usize,
    n: Num<'a>,
}

impl<'a> Parser<'a> {
    fn skip_blanks(&mut self) {
        while self
            .text
            .get(self.position)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.position += 1;
        }
    }

    /// Whether the next token is `token`, consuming it if so. A token that
    /// is a prefix of a longer operator (`<` of `<=`, `!` of `!=`) is not
    /// matched in its place.
    fn eat(&mut self, token: &str) -> bool {
        self.skip_blanks();
        let rest = self.text.get(self.position..).unwrap_or(&[]);
        if !rest.starts_with(token.as_bytes()) {
            return false;
        }
        let following = rest.get(token.len()).copied();
        if matches!(token, "<" | ">" | "!") && following == Some(b'=') {
            return false;
        }
        self.position += token.len();
        true
    }

    fn expression(&mut self) -> Option<Num<'a>> {
        let condition = self.or()?;
        if self.eat("?") {
            let when_true = self.expression()?;
            if !self.eat(":") {
                return None;
            }
            let when_false = self.expression()?;
            return Some(if condition.truth() {
                when_true
            } else {
                when_false
            });
        }
        Some(condition)
    }

    fn or(&mut self) -> Option<Num<'a>> {
        let mut value = self.and()?;
        while self.eat("||") {
            let right = self.and()?;
            value = Num::flag(value.truth() || right.truth());
        }
        Some(value)
    }

    fn and(&mut self) -> Option<Num<'a>> {
        let mut value = self.equality()?;
        while self.eat("&&") {
            let right = self.equality()?;
            value = Num::flag(value.truth() && right.truth());
        }
        Some(value)
    }

    fn equality(&mut self) -> Option<Num<'a>> {
        use core::cmp::Ordering;
        let mut value = self.relational()?;
        loop {
            if self.eat("==") {
                value = Num::flag(value.compare(self.relational()?)? == Ordering::Equal);
            } else if self.eat("!=") {
                value = Num::flag(value.compare(self.relational()?)? != Ordering::Equal);
            } else {
                return Some(value);
            }
        }
    }

    fn relational(&mut self) -> Option<Num<'a>> {
        use core::cmp::Ordering;
        let mut value = self.additive()?;
        loop {
            if self.eat("<=") {
                value = Num::flag(value.compare(self.additive()?)? != Ordering::Greater);
            } else if self.eat(">=") {
                value = Num::flag(value.compare(self.additive()?)? != Ordering::Less);
            } else if self.eat("<") {
                value = Num::flag(value.compare(self.additive()?)? == Ordering::Less);
            } else if self.eat(">") {
                value = Num::flag(value.compare(self.additive()?)? == Ordering::Greater);
            } else {
                return Some(value);
            }
        }
    }

    fn additive(&mut self) -> Option<Num<'a>> {
        let mut value = self.multiplicative()?;
        loop {
            if self.eat("+") {
                value = Self::small(value, self.multiplicative()?, i64::checked_add)?;
            } else if self.eat("-") {
                value = Self::small(value, self.multiplicative()?, i64::checked_sub)?;
            } else {
                return Some(value);
            }
        }
    }

    fn multiplicative(&mut self) -> Option<Num<'a>> {
        let mut value = self.unary()?;
        loop {
            if self.eat("*") {
                value = Self::small(value, self.unary()?, i64::checked_mul)?;
            } else if self.eat("/") {
                value = Self::small(value, self.unary()?, i64::checked_div)?;
            } else if self.eat("%") {
                value = value.remainder(self.unary()?)?;
            } else {
                return Some(value);
            }
        }
    }

    /// An operation on two machine integers; one on a count too large for
    /// one is not answered.
    fn small(
        left: Num<'a>,
        right: Num<'a>,
        operation: fn(i64, i64) -> Option<i64>,
    ) -> Option<Num<'a>> {
        match (left, right) {
            (Num::Small(left), Num::Small(right)) => operation(left, right).map(Num::Small),
            _ => None,
        }
    }

    fn unary(&mut self) -> Option<Num<'a>> {
        if self.eat("!") {
            return Some(Num::flag(!self.unary()?.truth()));
        }
        self.primary()
    }

    fn primary(&mut self) -> Option<Num<'a>> {
        self.skip_blanks();
        match self.text.get(self.position).copied()? {
            b'(' => {
                self.position += 1;
                let value = self.expression()?;
                self.eat(")").then_some(value)
            }
            b'n' => {
                self.position += 1;
                Some(self.n)
            }
            digit if digit.is_ascii_digit() => {
                let mut value = 0i64;
                while let Some(byte) = self.text.get(self.position).filter(|b| b.is_ascii_digit()) {
                    value = value.checked_mul(10)?.checked_add(i64::from(byte - b'0'))?;
                    self.position += 1;
                }
                Some(Num::Small(value))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_headers_of_the_catalogues_select_their_forms() {
        // English and most of Europe, and French and Portuguese.
        assert_eq!(plural_index("n != 1", 0), Some(1));
        assert_eq!(plural_index("(n != 1)", 1), Some(0));
        assert_eq!(plural_index("(n > 1)", 0), Some(0));
        assert_eq!(plural_index("(n > 1)", 2), Some(1));
        assert_eq!(plural_index("0", 5), Some(0));
        // Polish.
        let polish = "(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2)";
        assert_eq!(
            [1, 2, 5, 12, 22, 25].map(|n| plural_index(polish, n)),
            [Some(0), Some(1), Some(2), Some(2), Some(1), Some(2)]
        );
        // Arabic: six forms.
        let arabic = "n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5";
        assert_eq!(
            [0, 1, 2, 3, 11, 100, 103, 111].map(|n| plural_index(arabic, n)),
            [
                Some(0),
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(3),
                Some(4)
            ]
        );
        // Without parentheses around a conditional.
        let slovak = "(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2";
        assert_eq!(
            [1, 3, 5].map(|n| plural_index(slovak, n)),
            [Some(0), Some(1), Some(2)]
        );
    }

    /// Python's integers have no end, so `ngettext` of the count a googol
    /// leaves (`math.ceil` of a float up to 10²⁰⁸) is chosen by its last
    /// digits: the remainder is taken from the digits, and the count is
    /// above every constant it is compared with.
    #[test]
    fn a_count_beyond_every_machine_integer_is_chosen_by_its_digits() {
        let russian =
            "(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2)";
        let huge = |last: &str| format!("{}{last}", "9".repeat(60));
        assert_eq!(plural_index_digits(russian, &huge("1")), Some(0));
        assert_eq!(plural_index_digits(russian, &huge("12")), Some(2));
        assert_eq!(plural_index_digits(russian, &huge("22")), Some(1));
        assert_eq!(plural_index_digits(russian, &huge("0")), Some(2));
        assert_eq!(plural_index_digits("n != 1", &huge("1")), Some(1));
        assert_eq!(plural_index_digits("(n > 1)", &huge("1")), Some(1));
        assert_eq!(plural_index_digits("0", &huge("1")), Some(0));
        assert_eq!(plural_index_digits("n + 1", &huge("1")), None);
        assert_eq!(plural_index_digits("n", &huge("1")), None);
        assert_eq!(plural_index_digits("n != 1", "00007"), Some(1));
        assert_eq!(plural_index_digits("n != 1", ""), None);
        assert_eq!(plural_index_digits("n != 1", "1x"), None);
        let arabic = "n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5";
        assert_eq!(plural_index_digits(arabic, &huge("05")), Some(3));
        assert_eq!(plural_index_digits(arabic, &huge("00")), Some(5));
    }

    #[test]
    fn a_malformed_expression_selects_nothing() {
        assert_eq!(plural_index("n %% 2", 1), None);
        assert_eq!(plural_index("n / 0", 1), None);
        assert_eq!(plural_index("(n", 1), None);
        assert_eq!(plural_index("n ? 1", 1), None);
        assert_eq!(plural_index("n == ", 1), None);
        assert_eq!(plural_index("n n", 1), None);
        assert_eq!(plural_index("", 1), None);
        assert_eq!(plural_index("0 - 1", 1), None);
    }
}
