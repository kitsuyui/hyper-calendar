//! The Holy Years of the Catholic Church: the jubilees a Pope proclaims by
//! a bull of indiction, each with the days the bull fixes.
//!
//! A Holy Year is a set the Holy See defines (`docs/policy.md` §10): an
//! ordinary jubilee every twenty-five years, the rule since Paul II's
//! decree of 1470, and extraordinary ones when a Pope proclaims them. The
//! intervals changed — a hundred years under Boniface VIII, fifty from
//! 1343, thirty-three under Urban VI — and the days of each jubilee are
//! whatever its bull says, so the table is a list of bulls rather than a
//! cycle. Its criterion is the bull: a jubilee is in the table when a Pope
//! indicted it by a bull of indiction, ordinary or extraordinary. It holds
//! the jubilees whose bulls were read, from Paul VI's of 1975 to Francis's
//! of 2025:
//!
//! | Jubilee | Bull | Opens | Closes |
//! | --- | --- | --- | --- |
//! | 1975, ordinary | *Apostolorum limina*, 23 May 1974 | 24 December 1974 | 25 December 1975 |
//! | 1983–84, extraordinary, of the Redemption | *Aperite portas Redemptori*, 6 January 1983 | 25 March 1983 | 22 April 1984 |
//! | 2000, ordinary | *Incarnationis mysterium*, 29 November 1998 | 24 December 1999 | 6 January 2001 |
//! | 2015–16, extraordinary, of Mercy | *Misericordiae vultus*, 11 April 2015 | 8 December 2015 | 20 November 2016 |
//! | 2025, ordinary | *Spes non confundit*, 9 May 2024 | 24 December 2024 | 6 January 2026 |
//!
//! "Opens" and "Closes" are the days at St Peter's in the Vatican; the
//! 2025 bull also dates the jubilee in the dioceses, 29 December 2024 to
//! 28 December 2025, and [`Jubilee::particular_churches`] carries that.
//! The close of 1975 is the rite Paul VI celebrated on Christmas Day 1975,
//! as the Holy See's text of his homily heads it, the bull naming no
//! closing day.
//!
//! Sources, keyed in `docs/references.bib`: the bulls on vatican.va,
//! *Spes non confundit* no. 6 (`spes-non-confundit`), *Misericordiae
//! vultus* nos. 3 and 5 (`misericordiae-vultus`), *Incarnationis
//! mysterium* (`incarnationis-mysterium`), and in Italian *Aperite portas
//! Redemptori* no. 2 (`aperite-portas-redemptori`) and *Apostolorum
//! limina* (`apostolorum-limina`); Paul VI's homily of 25 December 1975,
//! headed as the closing rite of the Holy Year (`paul-vi-closing-1975`);
//! and Wikipedia's "Jubilee (Christianity)"
//! (`wikipedia-jubilee-christianity`, secondary), for the intervals and the
//! jubilee of 2033. All retrieved 2026-09-27.
//!
//! Not carried: the jubilees before 1975, whose bulls were not read; the
//! jubilee of 2033, announced and not yet indicted; and the special
//! jubilee years the Apostolic Penitentiary grants by decree. One of these
//! runs from 10 January 2026 to 10 January 2027, under the "Decree of the
//! Apostolic Penitentiary for the Eighth Centenary of the Death of Saint
//! Francis of Assisi, Proclaiming a Special Jubilee Year with the
//! Concession of Plenary Indulgences" of 10 January 2026
//! (`penitentiary-st-francis-2026`), which names no bull of indiction and
//! no Holy Door. It is a jubilee in the Holy See's own word, and it is
//! left out by the table's criterion, not for want of that word: the
//! Penitentiary grants such years by decree, for a centenary or a shrine
//! as well as for the whole Church, and no list of them was found, so a
//! table that carried them could not say when it was complete. [`holy_year_on`] answers only between the first
//! opening carried and the day the sources were checked, and refuses
//! outside it.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;

/// Whether a jubilee is the ordinary one of its twenty-five years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JubileeKind {
    /// The ordinary jubilee.
    Ordinary,
    /// An extraordinary jubilee, proclaimed for an occasion.
    Extraordinary,
}

/// A Gregorian date as a table writes it: year, month, day.
pub type TableDate = (i64, u8, u8);

/// A Holy Year, as its bull of indiction fixes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jubilee {
    /// The jubilee's name.
    pub title: &'static str,
    /// Ordinary or extraordinary.
    pub kind: JubileeKind,
    /// The Pope who proclaimed it.
    pub pope: &'static str,
    /// The bull of indiction, by its opening words.
    pub bull: &'static str,
    /// The day the bull was given.
    pub given: TableDate,
    /// The jubilee's first day in Rome: the opening of the Holy Door of
    /// St Peter's, or in 1983 the day the jubilee began.
    pub opens: TableDate,
    /// Its last day in Rome: the closing of the Holy Door of St Peter's.
    pub closes: TableDate,
    /// The first and last days in the dioceses, where the bull dates them.
    pub particular_churches: Option<(TableDate, TableDate)>,
}

impl Jubilee {
    /// Whether `day` is within the jubilee in Rome, from its opening to its
    /// closing inclusive.
    #[must_use]
    pub fn contains(&self, day: Rd) -> bool {
        match (fixed(self.opens), fixed(self.closes)) {
            (Some(opens), Some(closes)) => (opens..=closes).contains(&day),
            _ => false,
        }
    }
}

/// The fixed day of a table date.
fn fixed((year, month, day): TableDate) -> Option<Rd> {
    gregorian::to_fixed(year, month, day).ok()
}

/// The jubilees whose bulls were read, in order.
pub static JUBILEES: &[Jubilee] = &[
    Jubilee {
        title: "Holy Year 1975",
        kind: JubileeKind::Ordinary,
        pope: "Paul VI",
        bull: "Apostolorum limina",
        given: (1974, 5, 23),
        opens: (1974, 12, 24),
        closes: (1975, 12, 25),
        particular_churches: None,
    },
    Jubilee {
        title: "Jubilee of the Redemption",
        kind: JubileeKind::Extraordinary,
        pope: "John Paul II",
        bull: "Aperite portas Redemptori",
        given: (1983, 1, 6),
        opens: (1983, 3, 25),
        closes: (1984, 4, 22),
        particular_churches: None,
    },
    Jubilee {
        title: "Great Jubilee of the Year 2000",
        kind: JubileeKind::Ordinary,
        pope: "John Paul II",
        bull: "Incarnationis mysterium",
        given: (1998, 11, 29),
        opens: (1999, 12, 24),
        closes: (2001, 1, 6),
        particular_churches: None,
    },
    Jubilee {
        title: "Extraordinary Jubilee of Mercy",
        kind: JubileeKind::Extraordinary,
        pope: "Francis",
        bull: "Misericordiae vultus",
        given: (2015, 4, 11),
        opens: (2015, 12, 8),
        closes: (2016, 11, 20),
        particular_churches: None,
    },
    Jubilee {
        title: "Ordinary Jubilee of the Year 2025",
        kind: JubileeKind::Ordinary,
        pope: "Francis",
        bull: "Spes non confundit",
        given: (2024, 5, 9),
        opens: (2024, 12, 24),
        closes: (2026, 1, 6),
        particular_churches: Some(((2024, 12, 29), (2025, 12, 28))),
    },
];

/// The day the sources of [`JUBILEES`] were checked: a jubilee proclaimed
/// after it is not in the table.
pub const SOURCES_CHECKED: TableDate = (2026, 9, 27);

/// What the table says of a day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolyYearOn {
    /// The day is within this jubilee in Rome.
    Within(&'static Jubilee),
    /// The day is within no jubilee.
    Outside,
    /// The table does not reach the day: it is before the first jubilee
    /// carried, or after the day the sources were checked.
    NotCarried,
}

/// Whether `day` is within a Holy Year in Rome.
#[must_use]
pub fn holy_year_on(day: Rd) -> HolyYearOn {
    let first = JUBILEES.first().and_then(|jubilee| fixed(jubilee.opens));
    let (Some(first), Some(checked)) = (first, fixed(SOURCES_CHECKED)) else {
        return HolyYearOn::NotCarried;
    };
    if day < first || day > checked {
        return HolyYearOn::NotCarried;
    }
    JUBILEES
        .iter()
        .find(|jubilee| jubilee.contains(day))
        .map_or(HolyYearOn::Outside, HolyYearOn::Within)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_jubilee_of_2025_runs_as_spes_non_confundit_fixes_it() {
        // Spes non confundit, 6: the Holy Door of St Peter's opened on
        // 24 December 2024 and closed on 6 January 2026; in the particular
        // Churches from Sunday 29 December 2024 to Sunday 28 December 2025.
        let jubilee = &JUBILEES[4];
        assert_eq!(holy_year_on(ymd(2024, 12, 23)), HolyYearOn::Outside);
        assert_eq!(holy_year_on(ymd(2024, 12, 24)), HolyYearOn::Within(jubilee));
        assert_eq!(holy_year_on(ymd(2026, 1, 6)), HolyYearOn::Within(jubilee));
        assert_eq!(holy_year_on(ymd(2026, 1, 7)), HolyYearOn::Outside);
        assert_eq!(
            jubilee.particular_churches,
            Some(((2024, 12, 29), (2025, 12, 28)))
        );
    }

    #[test]
    fn each_jubilee_opens_and_closes_on_the_days_its_bull_names() {
        for (opens, closes, bull) in [
            (ymd(1974, 12, 24), ymd(1975, 12, 25), "Apostolorum limina"),
            (
                ymd(1983, 3, 25),
                ymd(1984, 4, 22),
                "Aperite portas Redemptori",
            ),
            (
                ymd(1999, 12, 24),
                ymd(2001, 1, 6),
                "Incarnationis mysterium",
            ),
            (ymd(2015, 12, 8), ymd(2016, 11, 20), "Misericordiae vultus"),
        ] {
            let HolyYearOn::Within(jubilee) = holy_year_on(opens) else {
                panic!("{bull} should open on its day");
            };
            assert_eq!(jubilee.bull, bull);
            assert_eq!(holy_year_on(closes), HolyYearOn::Within(jubilee));
            // The eve of 1975's opening is before the table.
            assert!(!matches!(
                holy_year_on(Rd(opens.0 - 1)),
                HolyYearOn::Within(_)
            ));
            assert_eq!(holy_year_on(Rd(closes.0 + 1)), HolyYearOn::Outside);
        }
    }

    #[test]
    fn the_table_refuses_the_days_it_does_not_reach() {
        // 1950 was a Holy Year; its bull was not read.
        assert_eq!(holy_year_on(ymd(1950, 6, 1)), HolyYearOn::NotCarried);
        assert_eq!(holy_year_on(ymd(1974, 12, 23)), HolyYearOn::NotCarried);
        assert_eq!(holy_year_on(ymd(2026, 9, 27)), HolyYearOn::Outside);
        assert_eq!(holy_year_on(ymd(2033, 6, 1)), HolyYearOn::NotCarried);
    }

    #[test]
    fn the_jubilees_are_in_order_and_do_not_overlap() {
        for pair in JUBILEES.windows(2) {
            assert!(fixed(pair[0].closes) < fixed(pair[1].opens));
        }
        for jubilee in JUBILEES {
            assert!(
                fixed(jubilee.given) < fixed(jubilee.opens),
                "{}",
                jubilee.title
            );
        }
        // The ordinary jubilees open twenty-five years apart, on Christmas Eve.
        let ordinary: Vec<TableDate> = JUBILEES
            .iter()
            .filter(|jubilee| jubilee.kind == JubileeKind::Ordinary)
            .map(|jubilee| jubilee.opens)
            .collect();
        assert_eq!(ordinary, [(1974, 12, 24), (1999, 12, 24), (2024, 12, 24)]);
    }
}
