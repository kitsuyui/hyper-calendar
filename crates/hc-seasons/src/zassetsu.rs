//! 雑節 — the "miscellaneous seasonal days" of the Japanese almanac.
//!
//! The 24 terms and the 72 pentads are Chinese imports describing the
//! climate of the Yellow River valley. The 雑節 are what Japan added on top
//! of them for its own agricultural year: when to expect the last frost, when
//! the rainy season starts, when the typhoons come.
//!
//! They are a ragbag of rule shapes, and that is the interesting part. Some
//! are pinned to a solar longitude, some count days from 立春, some are an
//! offset from a term, and 社日 is pinned to the sexagenary day cycle — a
//! rule with no astronomy in it at all. [`ZassetsuRule`] makes each one
//! explicit, so that [`day_of`] is a single `match` over data rather than
//! twenty special cases.
//!
//! # Which definitions these are
//!
//! Where a rule has both a classical and a modern form, the modern one — the
//! one the National Astronomical Observatory of Japan publishes in the
//! 暦要項 — is what [`ZassetsuRule`] carries, because that is what a Japanese
//! calendar prints today. 入梅 and 半夏生 are the two that changed, and both
//! classical forms are available separately as [`classical_nyubai`] and
//! [`classical_hangesho`].
//!
//! These are Japanese observances. China and Korea have their own 雜節 and
//! they are not the same list; nothing here should be read as describing
//! them.

use hc_calendar::Rd;
use hc_calendar::cycle::sexagenary_day;

use crate::meridian::Meridian;
use crate::seasons::Season;
use crate::solar_terms::{SolarTerm, term_day};

/// The heavenly stem 戊 (tsuchinoe), the fifth, which 社日 is pinned to.
const STEM_TSUCHINOE: u8 = 4;

/// The heavenly stem 壬 (mizunoe), the ninth, which the classical 入梅 rule
/// is pinned to.
const STEM_MIZUNOE: u8 = 8;

/// The earthly branch 丑 (ushi, the ox), the second, whose days inside 土用
/// are the ones eel is eaten on.
const BRANCH_USHI: u8 = 1;

/// How the day of a 雑節 is derived.
///
/// Four shapes cover all twenty-one of them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZassetsuRule {
    /// The day on which the Sun reaches a given apparent longitude.
    ///
    /// This is how the four 土用 entries, 入梅 and 半夏生 are defined in the
    /// modern almanac.
    SolarLongitude(f64),
    /// A whole-day offset from the day a solar term falls on.
    ///
    /// 節分 is −1 from a 立 term; the 彼岸 days are −3, 0 and +3 from an
    /// equinox.
    OffsetFromTerm {
        /// The term counted from.
        term: SolarTerm,
        /// Days to add to that term's day; may be negative.
        days: i64,
    },
    /// A count of days from 立春, in the traditional inclusive reckoning
    /// where 立春 itself is day 1.
    ///
    /// 八十八夜 is day 88, 二百十日 is day 210, 二百二十日 is day 220.
    NightsFromBeginningOfSpring(i64),
    /// The day nearest a term whose sexagenary stem is a given one, ties
    /// going to the earlier day.
    ///
    /// Only 社日 uses this, and only with 戊.
    NearestStemDay {
        /// The term the search is centred on.
        term: SolarTerm,
        /// The heavenly stem, 0 for 甲 through 9 for 癸.
        stem: u8,
    },
}

/// One of the 雑節.
///
/// Each variant is a single day. The multi-day observances — 彼岸 is a week,
/// 土用 is about eighteen days — appear here as their marked days and are
/// also available as periods through [`higan`] and [`doyo`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Zassetsu {
    /// 冬土用入り, the start of the winter 土用, around 17 January.
    WinterDoyoEntry,
    /// 節分 before 立春, around 3 February: the one everyone means.
    SpringSetsubun,
    /// 彼岸入り in spring, three days before the equinox.
    SpringHiganEntry,
    /// 春分, the middle day of the spring 彼岸.
    SpringHiganMiddle,
    /// 彼岸明け in spring, three days after the equinox.
    SpringHiganExit,
    /// 春社, the 戊 day nearest the spring equinox.
    SpringShanichi,
    /// 春土用入り, the start of the spring 土用, around 17 April.
    SpringDoyoEntry,
    /// 八十八夜, the eighty-eighth night from 立春, around 2 May.
    Hachijuhachiya,
    /// 節分 before 立夏, around 5 May.
    SummerSetsubun,
    /// 入梅, the nominal start of the rainy season, around 11 June.
    Nyubai,
    /// 半夏生, around 2 July: the day rice planting had to be finished by.
    Hangesho,
    /// 夏土用入り, the start of the summer 土用, around 20 July.
    SummerDoyoEntry,
    /// 節分 before 立秋, around 6 August.
    AutumnSetsubun,
    /// 二百十日, the two hundred and tenth day from 立春, around 1 September.
    Nihyakutoka,
    /// 二百二十日, the two hundred and twentieth day, around 11 September.
    Nihyakuhatsuka,
    /// 彼岸入り in autumn, three days before the equinox.
    AutumnHiganEntry,
    /// 秋分, the middle day of the autumn 彼岸.
    AutumnHiganMiddle,
    /// 彼岸明け in autumn, three days after the equinox.
    AutumnHiganExit,
    /// 秋社, the 戊 day nearest the autumn equinox.
    AutumnShanichi,
    /// 秋土用入り, the start of the autumn 土用, around 20 October.
    AutumnDoyoEntry,
    /// 節分 before 立冬, around 7 November.
    WinterSetsubun,
}

impl Zassetsu {
    /// All twenty-one 雑節, in the order they usually fall in a Gregorian
    /// year.
    ///
    /// "Usually" is doing work: 社日 can fall either side of 彼岸入り, so this
    /// is a conventional listing order and not a guarantee about dates. Sort
    /// the days yourself if you need a strict one.
    pub const ALL: [Self; 21] = [
        Self::WinterDoyoEntry,
        Self::SpringSetsubun,
        Self::SpringHiganEntry,
        Self::SpringHiganMiddle,
        Self::SpringHiganExit,
        Self::SpringShanichi,
        Self::SpringDoyoEntry,
        Self::Hachijuhachiya,
        Self::SummerSetsubun,
        Self::Nyubai,
        Self::Hangesho,
        Self::SummerDoyoEntry,
        Self::AutumnSetsubun,
        Self::Nihyakutoka,
        Self::Nihyakuhatsuka,
        Self::AutumnHiganEntry,
        Self::AutumnHiganMiddle,
        Self::AutumnHiganExit,
        Self::AutumnShanichi,
        Self::AutumnDoyoEntry,
        Self::WinterSetsubun,
    ];

    /// The rule that fixes this day.
    ///
    /// The four 土用 entries sit 18° of solar longitude before their 立 term,
    /// which is where the "eighteen days" in every description of 土用 comes
    /// from; the arc is fixed, the number of days is not.
    #[must_use]
    pub const fn rule(self) -> ZassetsuRule {
        match self {
            Self::SpringSetsubun => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::BEGINNING_OF_SPRING,
                days: -1,
            },
            Self::SummerSetsubun => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::BEGINNING_OF_SUMMER,
                days: -1,
            },
            Self::AutumnSetsubun => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::BEGINNING_OF_AUTUMN,
                days: -1,
            },
            Self::WinterSetsubun => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::BEGINNING_OF_WINTER,
                days: -1,
            },
            Self::SpringHiganEntry => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::SPRING_EQUINOX,
                days: -3,
            },
            Self::SpringHiganMiddle => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::SPRING_EQUINOX,
                days: 0,
            },
            Self::SpringHiganExit => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::SPRING_EQUINOX,
                days: 3,
            },
            Self::AutumnHiganEntry => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::AUTUMN_EQUINOX,
                days: -3,
            },
            Self::AutumnHiganMiddle => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::AUTUMN_EQUINOX,
                days: 0,
            },
            Self::AutumnHiganExit => ZassetsuRule::OffsetFromTerm {
                term: SolarTerm::AUTUMN_EQUINOX,
                days: 3,
            },
            Self::SpringShanichi => ZassetsuRule::NearestStemDay {
                term: SolarTerm::SPRING_EQUINOX,
                stem: STEM_TSUCHINOE,
            },
            Self::AutumnShanichi => ZassetsuRule::NearestStemDay {
                term: SolarTerm::AUTUMN_EQUINOX,
                stem: STEM_TSUCHINOE,
            },
            Self::Hachijuhachiya => ZassetsuRule::NightsFromBeginningOfSpring(88),
            Self::Nihyakutoka => ZassetsuRule::NightsFromBeginningOfSpring(210),
            Self::Nihyakuhatsuka => ZassetsuRule::NightsFromBeginningOfSpring(220),
            Self::Nyubai => ZassetsuRule::SolarLongitude(80.0),
            Self::Hangesho => ZassetsuRule::SolarLongitude(100.0),
            Self::SpringDoyoEntry => ZassetsuRule::SolarLongitude(27.0),
            Self::SummerDoyoEntry => ZassetsuRule::SolarLongitude(117.0),
            Self::AutumnDoyoEntry => ZassetsuRule::SolarLongitude(207.0),
            Self::WinterDoyoEntry => ZassetsuRule::SolarLongitude(297.0),
        }
    }

    /// The name in Japanese characters, e.g. `"八十八夜"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::SpringSetsubun
            | Self::SummerSetsubun
            | Self::AutumnSetsubun
            | Self::WinterSetsubun => "節分",
            Self::SpringHiganEntry | Self::AutumnHiganEntry => "彼岸入り",
            Self::SpringHiganMiddle | Self::AutumnHiganMiddle => "彼岸の中日",
            Self::SpringHiganExit | Self::AutumnHiganExit => "彼岸明け",
            Self::SpringShanichi => "春社",
            Self::AutumnShanichi => "秋社",
            Self::Hachijuhachiya => "八十八夜",
            Self::Nyubai => "入梅",
            Self::Hangesho => "半夏生",
            Self::SpringDoyoEntry
            | Self::SummerDoyoEntry
            | Self::AutumnDoyoEntry
            | Self::WinterDoyoEntry => "土用の入り",
            Self::Nihyakutoka => "二百十日",
            Self::Nihyakuhatsuka => "二百二十日",
        }
    }

    /// The name in Hepburn romaji, e.g. `"hachijūhachiya"`.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        match self {
            Self::SpringSetsubun
            | Self::SummerSetsubun
            | Self::AutumnSetsubun
            | Self::WinterSetsubun => "setsubun",
            Self::SpringHiganEntry | Self::AutumnHiganEntry => "higan-iri",
            Self::SpringHiganMiddle | Self::AutumnHiganMiddle => "higan no chūnichi",
            Self::SpringHiganExit | Self::AutumnHiganExit => "higan-ake",
            Self::SpringShanichi => "haru-shanichi",
            Self::AutumnShanichi => "aki-shanichi",
            Self::Hachijuhachiya => "hachijūhachiya",
            Self::Nyubai => "nyūbai",
            Self::Hangesho => "hangeshō",
            Self::SpringDoyoEntry
            | Self::SummerDoyoEntry
            | Self::AutumnDoyoEntry
            | Self::WinterDoyoEntry => "doyō no iri",
            Self::Nihyakutoka => "nihyakutōka",
            Self::Nihyakuhatsuka => "nihyakuhatsuka",
        }
    }

    /// A short English description.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::SpringSetsubun => "eve of the beginning of spring",
            Self::SummerSetsubun => "eve of the beginning of summer",
            Self::AutumnSetsubun => "eve of the beginning of autumn",
            Self::WinterSetsubun => "eve of the beginning of winter",
            Self::SpringHiganEntry => "first day of the spring equinoctial week",
            Self::SpringHiganMiddle => "middle day of the spring equinoctial week",
            Self::SpringHiganExit => "last day of the spring equinoctial week",
            Self::AutumnHiganEntry => "first day of the autumn equinoctial week",
            Self::AutumnHiganMiddle => "middle day of the autumn equinoctial week",
            Self::AutumnHiganExit => "last day of the autumn equinoctial week",
            Self::SpringShanichi => "spring day of the god of the soil",
            Self::AutumnShanichi => "autumn day of the god of the soil",
            Self::Hachijuhachiya => "eighty-eighth night from the beginning of spring",
            Self::Nyubai => "nominal start of the rainy season",
            Self::Hangesho => "the crow-dipper sprouts; rice planting must be done",
            Self::SpringDoyoEntry => "start of the spring earth period",
            Self::SummerDoyoEntry => "start of the summer earth period",
            Self::AutumnDoyoEntry => "start of the autumn earth period",
            Self::WinterDoyoEntry => "start of the winter earth period",
            Self::Nihyakutoka => "two hundred and tenth day; the typhoon day",
            Self::Nihyakuhatsuka => "two hundred and twentieth day",
        }
    }
}

/// The day a 雑節 falls on in a Gregorian year, at a given meridian.
///
/// ```
/// use hc_seasons::{Meridian, zassetsu::{Zassetsu, day_of}};
///
/// // 節分 2024 was 3 February, RD 738919.
/// assert_eq!(
///     day_of(Zassetsu::SpringSetsubun, 2024, Meridian::JAPAN),
///     hc_calendar::Rd(738_919)
/// );
/// ```
#[must_use]
pub fn day_of(kind: Zassetsu, year: i64, meridian: Meridian) -> Rd {
    match kind.rule() {
        ZassetsuRule::SolarLongitude(degrees) => {
            meridian.day_of(hc_astro::solar::seasonal_event(year, degrees))
        }
        ZassetsuRule::OffsetFromTerm { term, days } => Rd(term_day(year, term, meridian).0 + days),
        ZassetsuRule::NightsFromBeginningOfSpring(count) => {
            Rd(term_day(year, SolarTerm::BEGINNING_OF_SPRING, meridian).0 + count - 1)
        }
        ZassetsuRule::NearestStemDay { term, stem } => {
            nearest_stem_day(term_day(year, term, meridian), stem)
        }
    }
}

/// A 雑節 with the day it falls on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZassetsuEvent {
    /// Which observance.
    pub kind: Zassetsu,
    /// The day it falls on.
    pub day: Rd,
}

/// Every 雑節 of a Gregorian year, in [`Zassetsu::ALL`] order.
#[must_use]
pub fn zassetsu_in_year(year: i64, meridian: Meridian) -> ZassetsuInYear {
    ZassetsuInYear {
        year,
        meridian,
        position: 0,
    }
}

/// The iterator returned by [`zassetsu_in_year`].
#[derive(Debug, Clone, Copy)]
pub struct ZassetsuInYear {
    year: i64,
    meridian: Meridian,
    position: usize,
}

impl Iterator for ZassetsuInYear {
    type Item = ZassetsuEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = *Zassetsu::ALL.get(self.position)?;
        self.position += 1;
        Some(ZassetsuEvent {
            kind,
            day: day_of(kind, self.year, self.meridian),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = Zassetsu::ALL.len() - self.position;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ZassetsuInYear {}

/// The day nearest `centre` whose sexagenary stem is `stem`.
///
/// The stems repeat every ten days, so there is always one within five days
/// either side. When the centre is exactly five days from both — which
/// happens when the equinox falls on a 癸 day — this returns the earlier,
/// which is the reading the Japanese almanacs use. Sources do differ on that
/// tie, and this is the one place in the module where a caller might
/// legitimately want the other answer.
fn nearest_stem_day(centre: Rd, stem: u8) -> Rd {
    let here = sexagenary_day(centre).stem_index();
    let forward = i64::from((stem + 10 - here) % 10);
    let backward = i64::from((here + 10 - stem) % 10);
    if forward == 0 {
        centre
    } else if backward <= forward {
        Rd(centre.0 - backward)
    } else {
        Rd(centre.0 + forward)
    }
}

/// The first day at or after `start` whose sexagenary stem is `stem`.
fn stem_day_at_or_after(start: Rd, stem: u8) -> Rd {
    let here = sexagenary_day(start).stem_index();
    Rd(start.0 + i64::from((stem + 10 - here) % 10))
}

/// 彼岸, the seven days centred on an equinox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HiganPeriod {
    /// 彼岸入り, three days before the equinox.
    pub entry: Rd,
    /// 中日, the equinox itself.
    pub middle: Rd,
    /// 彼岸明け, three days after the equinox.
    pub exit: Rd,
}

impl HiganPeriod {
    /// 彼岸 is always exactly seven days.
    pub const LENGTH_DAYS: i64 = 7;

    /// Whether a day falls inside the week, endpoints included.
    #[must_use]
    pub const fn contains(self, day: Rd) -> bool {
        day.0 >= self.entry.0 && day.0 <= self.exit.0
    }

    /// Which day of the week a day is, 1 for 彼岸入り through 7 for 彼岸明け.
    #[must_use]
    pub const fn position_of(self, day: Rd) -> Option<u8> {
        if self.contains(day) {
            Some((day.0 - self.entry.0 + 1) as u8)
        } else {
            None
        }
    }
}

/// Which equinox a 彼岸 is centred on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HiganSeason {
    /// The spring 彼岸, centred on 春分.
    Spring,
    /// The autumn 彼岸, centred on 秋分.
    Autumn,
}

/// The 彼岸 week of a Gregorian year.
#[must_use]
pub fn higan(year: i64, season: HiganSeason, meridian: Meridian) -> HiganPeriod {
    let middle = match season {
        HiganSeason::Spring => term_day(year, SolarTerm::SPRING_EQUINOX, meridian),
        HiganSeason::Autumn => term_day(year, SolarTerm::AUTUMN_EQUINOX, meridian),
    };
    HiganPeriod {
        entry: Rd(middle.0 - 3),
        middle,
        exit: Rd(middle.0 + 3),
    }
}

/// The 雑節 that marks the first day of a season's 土用.
#[must_use]
pub const fn doyo_entry(season: Season) -> Zassetsu {
    match season {
        Season::Spring => Zassetsu::SpringDoyoEntry,
        Season::Summer => Zassetsu::SummerDoyoEntry,
        Season::Autumn => Zassetsu::AutumnDoyoEntry,
        Season::Winter => Zassetsu::WinterDoyoEntry,
    }
}

/// The 立 term a season's 土用 runs up to; the period ends the day before it.
///
/// 土用 closes a season, so the term is the *next* season's opening one: the
/// spring 土用 ends the day before 立夏.
#[must_use]
pub const fn doyo_closing_term(season: Season) -> SolarTerm {
    season.next().east_asian_opening_term()
}

/// The 節分 that falls on the eve of a season's opening 立 term.
///
/// [`Zassetsu::SpringSetsubun`] is the eve of 立春, so it opens spring; that
/// is the one in February with the roasted soybeans.
#[must_use]
pub const fn setsubun_opening(season: Season) -> Zassetsu {
    match season {
        Season::Spring => Zassetsu::SpringSetsubun,
        Season::Summer => Zassetsu::SummerSetsubun,
        Season::Autumn => Zassetsu::AutumnSetsubun,
        Season::Winter => Zassetsu::WinterSetsubun,
    }
}

/// 土用, one of the four periods of about eighteen days that close each
/// season.
///
/// In five-phase cosmology the four seasons are wood, fire, metal and water;
/// earth has no season of its own, so it is given the last eighteen days of
/// each. The modern rule makes the arc exact — 18° of solar longitude — and
/// lets the number of days vary from 17 to 19 with the Earth's orbital speed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoyoPeriod {
    /// Which season this period closes.
    pub season: Season,
    /// 土用の入り, the first day.
    pub start: Rd,
    /// The last day, which is the day before the closing 立 term.
    pub end: Rd,
}

impl DoyoPeriod {
    /// How many days the period runs, endpoints included.
    #[must_use]
    pub const fn length_days(self) -> i64 {
        self.end.0 - self.start.0 + 1
    }

    /// Whether a day falls inside the period.
    #[must_use]
    pub const fn contains(self, day: Rd) -> bool {
        day.0 >= self.start.0 && day.0 <= self.end.0
    }

    /// The 丑の日 — the days of the ox — inside this period.
    ///
    /// The branches repeat every twelve days and the period is seventeen to
    /// nineteen, so there are always one or two. When the summer 土用 has
    /// two, the second is 二の丑, and both are eel days.
    #[must_use]
    pub fn ox_days(self) -> OxDays {
        let mut days = [Rd(0); 2];
        let mut count = 0;
        let here = sexagenary_day(self.start).branch_index();
        let first = Rd(self.start.0 + i64::from((BRANCH_USHI + 12 - here) % 12));
        if self.contains(first) {
            days[0] = first;
            count = 1;
            let second = Rd(first.0 + 12);
            if self.contains(second) {
                days[1] = second;
                count = 2;
            }
        }
        OxDays { days, count }
    }

    /// The first 丑の日 of the period, if it has one.
    #[must_use]
    pub fn first_ox_day(self) -> Option<Rd> {
        self.ox_days().into_iter().next()
    }

    /// 二の丑, the second 丑の日, when the period has two.
    #[must_use]
    pub fn second_ox_day(self) -> Option<Rd> {
        self.ox_days().into_iter().nth(1)
    }
}

/// The one or two 丑の日 of a [`DoyoPeriod`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OxDays {
    days: [Rd; 2],
    count: u8,
}

impl OxDays {
    /// How many there are: one or two.
    #[must_use]
    pub const fn len(self) -> usize {
        self.count as usize
    }

    /// Whether there are none, which should not happen for a real 土用.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.count == 0
    }

    /// The days as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[Rd] {
        &self.days[..self.count as usize]
    }
}

impl IntoIterator for OxDays {
    type Item = Rd;
    type IntoIter = core::iter::Take<core::array::IntoIter<Rd, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        self.days.into_iter().take(self.count as usize)
    }
}

/// A 土用 period of a Gregorian year.
///
/// The winter 土用 of year *y* runs from mid-January to the day before that
/// year's 立春, i.e. it sits at the start of the Gregorian year rather than
/// the end.
#[must_use]
pub fn doyo(year: i64, season: Season, meridian: Meridian) -> DoyoPeriod {
    let start = day_of(doyo_entry(season), year, meridian);
    let end = Rd(term_day(year, doyo_closing_term(season), meridian).0 - 1);
    DoyoPeriod { season, start, end }
}

/// 節分, the eve of the 立 term that opens a season.
///
/// All four exist. `setsubun(year, Season::Spring, ..)` is the February one.
#[must_use]
pub fn setsubun(year: i64, season: Season, meridian: Meridian) -> Rd {
    day_of(setsubun_opening(season), year, meridian)
}

/// 社日 under the classical rule, for whichever equinox is nearer.
///
/// Exposed separately from [`day_of`] because the tie-breaking matters and
/// the doc comment on [`nearest_stem_day`] is where it is explained.
#[must_use]
pub fn shanichi(year: i64, season: HiganSeason, meridian: Meridian) -> Rd {
    let kind = match season {
        HiganSeason::Spring => Zassetsu::SpringShanichi,
        HiganSeason::Autumn => Zassetsu::AutumnShanichi,
    };
    day_of(kind, year, meridian)
}

/// 入梅 under the pre-modern rule: the first 壬 day on or after 芒種.
///
/// Japanese almanacs used this until the Meiji reform and some regional
/// calendars still print it. It can differ from the modern 80° rule by
/// several days.
#[must_use]
pub fn classical_nyubai(year: i64, meridian: Meridian) -> Rd {
    stem_day_at_or_after(
        term_day(year, SolarTerm::GRAIN_IN_EAR, meridian),
        STEM_MIZUNOE,
    )
}

/// 半夏生 under the pre-modern rule: the eleventh day counting 夏至 as the
/// first, i.e. ten days after the solstice.
///
/// The count is inclusive, as 「夏至から数えて11日目」 is in Japanese. The
/// modern rule puts 半夏生 at 100° of solar longitude instead; the Sun is at
/// its slowest near the June solstice and covers those ten degrees in about
/// 10.5 days, so the two rules land on the same day rather more than half
/// the time and never differ by more than one.
#[must_use]
pub fn classical_hangesho(year: i64, meridian: Meridian) -> Rd {
    Rd(term_day(year, SolarTerm::SUMMER_SOLSTICE, meridian).0 + 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::from_year_month_day;

    const JAPAN: Meridian = Meridian::JAPAN;

    /// The National Astronomical Observatory of Japan's 暦要項 for 2024.
    #[test]
    fn the_zassetsu_of_2024_fall_where_the_japanese_almanac_puts_them() {
        let expected = [
            (Zassetsu::SpringSetsubun, (2024, 2, 3)),
            (Zassetsu::SpringHiganEntry, (2024, 3, 17)),
            (Zassetsu::SpringHiganMiddle, (2024, 3, 20)),
            (Zassetsu::SpringHiganExit, (2024, 3, 23)),
            (Zassetsu::Hachijuhachiya, (2024, 5, 1)),
            (Zassetsu::Nyubai, (2024, 6, 10)),
            (Zassetsu::Hangesho, (2024, 7, 1)),
            (Zassetsu::Nihyakutoka, (2024, 8, 31)),
            (Zassetsu::Nihyakuhatsuka, (2024, 9, 10)),
            (Zassetsu::AutumnHiganEntry, (2024, 9, 19)),
            (Zassetsu::AutumnHiganMiddle, (2024, 9, 22)),
            (Zassetsu::AutumnHiganExit, (2024, 9, 25)),
        ];
        for (kind, (year, month, day)) in expected {
            assert_eq!(
                day_of(kind, 2024, JAPAN),
                from_year_month_day(year, month, day),
                "{} ({})",
                kind.japanese_name(),
                kind.english_name()
            );
        }
    }

    /// 2024 was a leap year, so 八十八夜 and 二百十日 came a day earlier than
    /// the "2 May, 1 September" that every description of them gives. That is
    /// the point of counting from 立春 rather than from 1 January.
    #[test]
    fn the_counted_days_shift_with_the_leap_year() {
        assert_eq!(
            day_of(Zassetsu::Hachijuhachiya, 2023, JAPAN),
            from_year_month_day(2023, 5, 2)
        );
        assert_eq!(
            day_of(Zassetsu::Hachijuhachiya, 2024, JAPAN),
            from_year_month_day(2024, 5, 1)
        );
        assert_eq!(
            day_of(Zassetsu::Nihyakutoka, 2023, JAPAN),
            from_year_month_day(2023, 9, 1)
        );
        assert_eq!(
            day_of(Zassetsu::Nihyakutoka, 2024, JAPAN),
            from_year_month_day(2024, 8, 31)
        );
    }

    #[test]
    fn the_counted_days_are_exactly_the_counts_they_are_named_for() {
        for year in 1950..2050 {
            let risshun = term_day(year, SolarTerm::BEGINNING_OF_SPRING, JAPAN);
            assert_eq!(
                day_of(Zassetsu::Hachijuhachiya, year, JAPAN).0 - risshun.0,
                87
            );
            assert_eq!(
                day_of(Zassetsu::Nihyakutoka, year, JAPAN).0 - risshun.0,
                209
            );
            assert_eq!(
                day_of(Zassetsu::Nihyakuhatsuka, year, JAPAN).0 - risshun.0,
                219
            );
            assert_eq!(
                day_of(Zassetsu::Nihyakuhatsuka, year, JAPAN).0
                    - day_of(Zassetsu::Nihyakutoka, year, JAPAN).0,
                10
            );
        }
    }

    #[test]
    fn setsubun_is_always_the_day_before_the_term_that_opens_a_season() {
        for year in 1900..2100 {
            for season in Season::ALL {
                let eve = setsubun(year, season, JAPAN);
                let term = term_day(year, season.east_asian_opening_term(), JAPAN);
                assert_eq!(eve.0 + 1, term.0, "{} in {year}", season.english_name());
            }
        }
    }

    /// 節分 has not always been 3 February. It was 4 February from 1985 to
    /// 2020 in a quarter of years, and 2021 was the first 2 February since
    /// 1897 — a fact worth pinning because it is the kind of thing a
    /// hard-coded "3 February" gets wrong.
    #[test]
    fn setsubun_was_the_second_of_february_in_2021() {
        assert_eq!(
            day_of(Zassetsu::SpringSetsubun, 2021, JAPAN),
            from_year_month_day(2021, 2, 2)
        );
        assert_eq!(
            day_of(Zassetsu::SpringSetsubun, 2020, JAPAN),
            from_year_month_day(2020, 2, 3)
        );
        assert_eq!(
            day_of(Zassetsu::SpringSetsubun, 1984, JAPAN),
            from_year_month_day(1984, 2, 4)
        );
    }

    #[test]
    fn higan_is_seven_days_centred_on_the_equinox() {
        for year in 1950..2050 {
            for season in [HiganSeason::Spring, HiganSeason::Autumn] {
                let week = higan(year, season, JAPAN);
                assert_eq!(week.exit.0 - week.entry.0 + 1, HiganPeriod::LENGTH_DAYS);
                assert_eq!(week.middle.0 - week.entry.0, 3);
                assert!(week.contains(week.entry));
                assert!(week.contains(week.middle));
                assert!(week.contains(week.exit));
                assert!(!week.contains(Rd(week.entry.0 - 1)));
                assert!(!week.contains(Rd(week.exit.0 + 1)));
                assert_eq!(week.position_of(week.middle), Some(4));
                assert_eq!(week.position_of(Rd(week.exit.0 + 1)), None);
            }
        }
    }

    #[test]
    fn the_two_higan_weeks_do_not_overlap() {
        for year in 1900..2100 {
            let spring = higan(year, HiganSeason::Spring, JAPAN);
            let autumn = higan(year, HiganSeason::Autumn, JAPAN);
            assert!(spring.exit.0 < autumn.entry.0);
            assert!(autumn.entry.0 - spring.exit.0 > 170);
        }
    }

    /// 社日 is the 戊 day nearest the equinox, so it is never more than five
    /// days away and always has 戊 as its stem.
    #[test]
    fn shanichi_is_always_a_tsuchinoe_day_near_the_equinox() {
        for year in 1900..2100 {
            for season in [HiganSeason::Spring, HiganSeason::Autumn] {
                let day = shanichi(year, season, JAPAN);
                assert_eq!(
                    sexagenary_day(day).stem_index(),
                    STEM_TSUCHINOE,
                    "{day} in {year} was not a 戊 day"
                );
                assert_eq!(sexagenary_day(day).stem_name(), "wu");
                let equinox = higan(year, season, JAPAN).middle;
                assert!(
                    (day.0 - equinox.0).abs() <= 5,
                    "{} days from the equinox in {year}",
                    day.0 - equinox.0
                );
            }
        }
    }

    /// The tie-break is a real case, not a hypothetical: over two centuries
    /// the equinox lands on a 癸 day often enough to exercise it, and when it
    /// does this crate takes the earlier 戊.
    #[test]
    fn the_shanichi_tie_break_takes_the_earlier_day() {
        let mut ties = 0;
        for year in 1800..2200 {
            for season in [HiganSeason::Spring, HiganSeason::Autumn] {
                let equinox = higan(year, season, JAPAN).middle;
                if sexagenary_day(equinox).stem_index() == (STEM_TSUCHINOE + 5) % 10 {
                    ties += 1;
                    assert_eq!(shanichi(year, season, JAPAN).0, equinox.0 - 5);
                }
            }
        }
        assert!(ties > 20, "only {ties} ties in four centuries");
    }

    #[test]
    fn a_stem_day_search_lands_on_the_stem_it_asked_for() {
        for offset in 0..200 {
            let start = Rd(739_000 + offset);
            let found = nearest_stem_day(start, STEM_TSUCHINOE);
            assert_eq!(sexagenary_day(found).stem_index(), STEM_TSUCHINOE);
            assert!((found.0 - start.0).abs() <= 5);
            let after = stem_day_at_or_after(start, STEM_MIZUNOE);
            assert_eq!(sexagenary_day(after).stem_index(), STEM_MIZUNOE);
            assert!((0..10).contains(&(after.0 - start.0)));
        }
    }

    #[test]
    fn each_doyo_runs_from_its_entry_to_the_eve_of_its_term() {
        for year in 1950..2050 {
            for season in Season::ALL {
                let period = doyo(year, season, JAPAN);
                assert_eq!(period.start, day_of(doyo_entry(season), year, JAPAN));
                assert_eq!(
                    period.end.0 + 1,
                    term_day(year, doyo_closing_term(season), JAPAN).0
                );
                assert!(
                    (17..=19).contains(&period.length_days()),
                    "{} of {year} ran {} days",
                    season.english_name(),
                    period.length_days()
                );
                assert!(period.contains(period.start));
                assert!(period.contains(period.end));
                assert!(!period.contains(Rd(period.start.0 - 1)));
            }
        }
    }

    /// The four 土用 cover the eighteen degrees before each 立 term, so they
    /// never overlap and together they take up about a fifth of the year.
    #[test]
    fn the_four_doyo_of_a_year_are_disjoint() {
        for year in 1990..2030 {
            let mut total = 0;
            let mut periods = Season::ALL.map(|season| doyo(year, season, JAPAN));
            periods.sort_by_key(|period| period.start.0);
            for window in periods.windows(2) {
                assert!(
                    window[0].end.0 < window[1].start.0,
                    "two 土用 overlapped in {year}"
                );
            }
            for period in periods {
                total += period.length_days();
            }
            assert!((68..=76).contains(&total), "{total} days of 土用 in {year}");
        }
    }

    /// 2024's summer 土用 had two 丑の日, 24 July and 5 August.
    #[test]
    fn the_summer_doyo_of_2024_had_two_days_of_the_ox() {
        let period = doyo(2024, Season::Summer, JAPAN);
        let days = period.ox_days();
        assert_eq!(days.len(), 2);
        assert_eq!(days.as_slice()[0], from_year_month_day(2024, 7, 24));
        assert_eq!(days.as_slice()[1], from_year_month_day(2024, 8, 5));
        assert_eq!(
            period.first_ox_day(),
            Some(from_year_month_day(2024, 7, 24))
        );
        assert_eq!(
            period.second_ox_day(),
            Some(from_year_month_day(2024, 8, 5))
        );
    }

    /// 2023's summer 土用 had only one, 30 July: the period opened on a 寅
    /// day, so the first 丑 did not come round until eleven days in and the
    /// second would have fallen a day past 立秋.
    #[test]
    fn the_summer_doyo_of_2023_had_one_day_of_the_ox() {
        let period = doyo(2023, Season::Summer, JAPAN);
        assert_eq!(period.ox_days().len(), 1);
        assert_eq!(
            period.first_ox_day(),
            Some(from_year_month_day(2023, 7, 30))
        );
        assert_eq!(period.second_ox_day(), None);
    }

    /// 2025's, by contrast, opened on a 丑 day itself, so 19 July was the
    /// first and 31 July the 二の丑.
    #[test]
    fn the_summer_doyo_of_2025_opened_on_a_day_of_the_ox() {
        let period = doyo(2025, Season::Summer, JAPAN);
        assert_eq!(period.start, from_year_month_day(2025, 7, 19));
        assert_eq!(period.ox_days().len(), 2);
        assert_eq!(
            period.first_ox_day(),
            Some(from_year_month_day(2025, 7, 19))
        );
        assert_eq!(
            period.second_ox_day(),
            Some(from_year_month_day(2025, 7, 31))
        );
    }

    #[test]
    fn every_doyo_has_one_or_two_days_of_the_ox_and_they_are_inside_it() {
        for year in 1950..2050 {
            for season in Season::ALL {
                let period = doyo(year, season, JAPAN);
                let days = period.ox_days();
                assert!(!days.is_empty());
                assert!((1..=2).contains(&days.len()));
                for day in days {
                    assert!(period.contains(day));
                    assert_eq!(sexagenary_day(day).branch_index(), BRANCH_USHI);
                    assert_eq!(sexagenary_day(day).zodiac_animal(), "ox");
                }
            }
        }
    }

    #[test]
    fn a_year_lists_twenty_one_zassetsu_without_repeating_a_kind() {
        let mut iterator = zassetsu_in_year(2024, JAPAN);
        assert_eq!(iterator.len(), 21);
        let mut seen = [false; 21];
        let mut count = 0;
        for event in &mut iterator {
            let index = Zassetsu::ALL
                .iter()
                .position(|kind| *kind == event.kind)
                .unwrap();
            assert!(!seen[index]);
            seen[index] = true;
            count += 1;
        }
        assert_eq!(count, 21);
        assert!(seen.iter().all(|&flag| flag));
    }

    #[test]
    fn every_zassetsu_of_a_year_falls_inside_that_year() {
        for year in [1900i64, 2000, 2024, 2100] {
            let start = crate::gregorian::new_year(year);
            let end = crate::gregorian::new_year(year + 1);
            for event in zassetsu_in_year(year, JAPAN) {
                assert!(
                    event.day >= start && event.day < end,
                    "{} of {year} escaped its year",
                    event.kind.japanese_name()
                );
            }
        }
    }

    #[test]
    fn every_zassetsu_has_a_rule_a_name_and_a_gloss() {
        for kind in Zassetsu::ALL {
            assert!(!kind.japanese_name().is_empty());
            assert!(!kind.romaji().is_empty());
            assert!(!kind.english_name().is_empty());
            assert!(kind.english_name().is_ascii());
            // Every rule is one of the four shapes, and the longitudes are
            // all in range.
            if let ZassetsuRule::SolarLongitude(degrees) = kind.rule() {
                assert!((0.0..360.0).contains(&degrees));
            }
        }
    }

    /// The four 土用 entries sit exactly 18° before their closing term, which
    /// is the rule stated as data rather than as four magic numbers.
    #[test]
    fn the_doyo_entries_are_eighteen_degrees_before_their_terms() {
        for season in Season::ALL {
            let ZassetsuRule::SolarLongitude(entry) = doyo_entry(season).rule() else {
                panic!("a 土用 entry should be a longitude rule");
            };
            let closing = doyo_closing_term(season).solar_longitude_degrees();
            let gap = (closing - entry + 360.0) % 360.0;
            assert!((gap - 18.0).abs() < 1e-9, "gap of {gap} degrees");
        }
    }

    /// The pre-modern and modern 入梅 rules are different rules and give
    /// different days; the classical one is always a 壬 day, the modern one
    /// is whatever day 80° falls on.
    #[test]
    fn the_classical_and_modern_rainy_season_rules_disagree() {
        let mut disagreements = 0;
        for year in 1950..2050 {
            let classical = classical_nyubai(year, JAPAN);
            assert_eq!(sexagenary_day(classical).stem_index(), STEM_MIZUNOE);
            if classical != day_of(Zassetsu::Nyubai, year, JAPAN) {
                disagreements += 1;
            }
        }
        assert!(
            disagreements > 80,
            "only {disagreements} disagreements in a century, which cannot be right"
        );
    }

    /// The two 半夏生 rules land on the same day about half the time and are
    /// never more than a day apart. The Sun covers the ten degrees from 90°
    /// to 100° in about 10.5 days, so a whole-day count cannot do better.
    #[test]
    fn the_two_hangesho_rules_never_differ_by_more_than_a_day() {
        let mut agreements = 0;
        for year in 1950..2050 {
            let classical = classical_hangesho(year, JAPAN);
            let modern = day_of(Zassetsu::Hangesho, year, JAPAN);
            assert!(
                (modern.0 - classical.0).abs() <= 1,
                "{year}: the two rules were {} days apart",
                modern.0 - classical.0
            );
            if classical == modern {
                agreements += 1;
            }
        }
        assert!(
            (30..=80).contains(&agreements),
            "{agreements} agreements in a century is not the expected rate"
        );
    }

    /// The meridian matters here too: Beijing is an hour behind Tokyo, so a
    /// 雑節 computed at the wrong meridian lands on the wrong day now and
    /// then.
    #[test]
    fn the_meridian_changes_some_zassetsu_dates() {
        let mut disagreements = 0;
        for year in 1950..2050 {
            for kind in Zassetsu::ALL {
                if day_of(kind, year, Meridian::JAPAN) != day_of(kind, year, Meridian::CHINA) {
                    disagreements += 1;
                }
            }
        }
        assert!(disagreements > 0, "the meridian never mattered");
    }
}
