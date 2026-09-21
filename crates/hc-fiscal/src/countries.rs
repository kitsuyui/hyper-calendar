//! The national tables.
//!
//! Every country here is a [`FiscalProfile`] — a code, a name, a list of
//! [`YearSystem`] values, the date the sources were last checked and the
//! statute or ministry they came from. None of them contributes a line of
//! logic; adding a country means adding a value.
//!
//! # What is and is not claimed
//!
//! * A country with **no offset** is listed anyway. "Does Germany have a
//!   fiscal year that differs from the calendar year?" is a question people
//!   ask, and "no, and here is the statute that says so" is the answer, not
//!   a missing row.
//! * A country with **more than one** year is listed with all of them. The
//!   United Kingdom has a government financial year starting 1 April and a
//!   personal tax year starting 6 April; New Zealand has a Crown financial
//!   year starting 1 July and a tax year starting 1 April *labelled the
//!   other way round*. Choosing one and calling it "the" fiscal year would
//!   be wrong about half the questions.
//! * A country that **changed** its fiscal year carries both systems with
//!   their validity ranges, not only the current one. The United States and
//!   Thailand are the worked examples.
//! * Every entry carries a **`sources_checked`** date, for the reason
//!   `docs/observances.md` gives: a fiscal year without a source is a
//!   rumour, and one without a date is a rumour about when it was true.
//! * Where the author could not read the primary source first-hand, the
//!   entry's `note` says so rather than implying a citation it does not
//!   have.
//!
//! # Validity ranges are labels, not Gregorian years
//!
//! [`YearSystem::valid_from`] and [`YearSystem::valid_until`] are expressed
//! in the system's own labels. For [`IRAN`] that means Solar Hijri years and
//! for [`ETHIOPIA`] Ethiopic ones, because the label is the number a caller
//! has in hand.

use hc_calendar::Rd;

use crate::year_system::{
    Authority, LabelConvention, SourceDate, StartCalendar, SystemKind, YearStart, YearSystem,
};

/// A country's fiscal, tax and (where nationally fixed) academic years.
///
/// The same type whether the country has one system or four; a country is
/// not special, it is just the profile people ask for most.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FiscalProfile {
    /// The ISO 3166-1 alpha-2 code.
    pub code: &'static str,
    /// The English name of the country.
    pub english_name: &'static str,
    /// Every year system this table knows about, current and historical.
    pub systems: &'static [YearSystem],
    /// When the sources behind this table were last checked.
    pub sources_checked: SourceDate,
    /// The statute, ministry or official publication the table came from.
    pub sources: &'static str,
}

impl FiscalProfile {
    /// Every system of a given kind, newest entry last.
    pub fn of_kind(&self, kind: SystemKind) -> impl Iterator<Item = &'static YearSystem> + '_ {
        self.systems
            .iter()
            .filter(move |system| system.kind == kind)
    }

    /// The system of `kind` that covers the year labelled `label`.
    ///
    /// Returns `None` when the country has no system of that kind, or none
    /// that reached that year. The latter is a real answer: the United
    /// States federal government had no October fiscal year in 1970.
    #[must_use]
    pub fn in_force(&self, kind: SystemKind, label: i64) -> Option<&'static YearSystem> {
        self.of_kind(kind).find(|system| system.covers(label))
    }

    /// The government financial year in force for `label`.
    #[must_use]
    pub fn government(&self, label: i64) -> Option<&'static YearSystem> {
        self.in_force(SystemKind::Government, label)
    }

    /// The system of `kind` that contains `rd`, and the label it gives it.
    ///
    /// This is the lookup to use when what you have is a date rather than a
    /// year number, and it is the one that makes the United States'
    /// transition quarter visible: on 1 August 1976 the July system had
    /// already expired and the October system had not begun, so this returns
    /// `None` — which is the correct answer and not a lookup failure.
    #[must_use]
    pub fn at(&self, kind: SystemKind, rd: Rd) -> Option<(&'static YearSystem, i64)> {
        self.of_kind(kind)
            .find_map(|system| system.label_at(rd).ok().map(|label| (system, label)))
    }

    /// Whether every system this country has is the plain **Gregorian**
    /// calendar year.
    ///
    /// Gregorian, because that is what the question means. Iran's fiscal
    /// year coincides exactly with the Iranian calendar year and so returns
    /// `false` here, which is right: it has a very large offset from
    /// 1 January and none at all from 1 Farvardin.
    #[must_use]
    pub fn has_no_offset(&self) -> bool {
        !self.systems.is_empty()
            && self
                .systems
                .iter()
                .all(YearSystem::is_gregorian_calendar_year)
    }
}

/// A fiscal year this crate knows about and deliberately does not compute.
///
/// `docs/policy.md` §4: the library refuses to guess. A gap recorded here is
/// a claim that the answer is known and the machinery to express it is not
/// present — which is a different statement from silence, and a different
/// statement again from an approximation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentedGap {
    /// The ISO 3166-1 alpha-2 code.
    pub code: &'static str,
    /// The English name of the country.
    pub english_name: &'static str,
    /// What the fiscal year actually is, in words.
    pub description: &'static str,
    /// Why this crate cannot express it.
    pub reason: &'static str,
    /// Where the description came from.
    pub source: &'static str,
}

/// Fiscal years that are known and not implemented.
pub static GAPS: &[DocumentedGap] = &[DocumentedGap {
    code: "NP",
    english_name: "Nepal",
    description: "The Nepali fiscal year runs from 1 Shrawan to the end of Ashadh in the Bikram \
                  Sambat calendar, which places its start in mid-July of the Gregorian year.",
    reason: "This workspace has no Bikram Sambat calendar. Bikram Sambat month lengths are \
             published annually by the Nepal Panchanga Nirnayak Samiti rather than derived from \
             a rule, so the start cannot be computed and a fixed Gregorian date would be an \
             approximation dressed as an answer. `hc-holiday` reaches the same conclusion about \
             Nepal's holidays.",
    source: "Ministry of Finance, Government of Nepal, budget speeches (fiscal year 2081/82)",
}];

// ── Asia and the Pacific ────────────────────────────────────────────────

/// Japan 🇯🇵 — 会計年度 and 学年度, both 1 April.
///
/// 財政法（昭和二十二年法律第三十四号）第十一条:
/// 「国の会計年度は、毎年四月一日に始まり、翌年三月三十一日に終るものとする。」
/// Local authorities have the parallel rule in 地方自治法第208条第1項. The
/// school year is 学校教育法施行規則第五十九条, carried to the other school
/// types by 準用.
///
/// The label convention is the start year without ambiguity: 令和6年度 and
/// 2024年度 are the same year and both begin in April 2024. This is the
/// crate's counter-example to the United States.
pub static JAPAN: FiscalProfile = FiscalProfile {
    code: "JP",
    english_name: "Japan",
    systems: &[
        YearSystem {
            name: "Japanese national fiscal year",
            local_name: "会計年度",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1947),
            valid_until: None,
            note: "April has been the start of the Japanese fiscal year since the Meiji period, \
                   but the statute cited here is the 1947 財政法, so the range begins there \
                   rather than claiming a source this table does not have.",
        },
        crate::academic::JAPAN_SCHOOL_YEAR,
    ],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "財政法（昭和22年法律第34号）第11条; 地方自治法第208条; \
              学校教育法施行規則（昭和22年文部省令第11号）第59条 — e-Gov 法令検索",
};

/// India 🇮🇳 — 1 April, and the switch to a calendar year that never came.
///
/// The General Clauses Act, 1897, s. 3(21) defines "financial year" as the
/// year commencing on the first day of April. The official form is a span
/// that leads with the start year — "FY 2024-25" is 1 April 2024 to
/// 31 March 2025 — and the assessment year is the one after.
///
/// Two committees recommended moving to a January start: L. K. Jha's in 1984
/// and Shankar Acharya's, which reported in 2017. **Neither was enacted**,
/// which is why this table has one system and not two.
pub static INDIA: FiscalProfile = FiscalProfile {
    code: "IN",
    english_name: "India",
    systems: &[YearSystem {
        name: "Indian financial year",
        local_name: "",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(4, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "Written officially as a span leading with the start year (FY 2024-25). Indian \
               market shorthand \"FY25\" means the same year but names it by its end, so the \
               short form and the official form disagree by one; this entry follows the \
               official form.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "General Clauses Act, 1897, s. 3(21); Arthapedia (Indian Economic Service, \
              Ministry of Finance)",
};

/// Hong Kong 🇭🇰 — 1 April.
///
/// The government financial year and the Inland Revenue Department's year of
/// assessment both run 1 April to 31 March, written "2025/26" after the year
/// they begin in.
pub static HONG_KONG: FiscalProfile = FiscalProfile {
    code: "HK",
    english_name: "Hong Kong",
    systems: &[YearSystem {
        name: "Hong Kong financial year",
        local_name: "財政年度",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(4, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "The statutory definition is in the Public Finance Ordinance (Cap. 2) s. 2, which \
               the author could not retrieve first-hand; the dates are taken from the Inland \
               Revenue Department's published year of assessment and from budget practice.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Inland Revenue Department, \"Year of Assessment\"; Public Finance Ordinance \
              (Cap. 2) s. 2 (not read first-hand)",
};

/// Singapore 🇸🇬 — 1 April for the state, the calendar year for personal tax.
///
/// The Financial Procedure Act 1966 defines the financial year as the twelve
/// months ending on 31 March. Personal income tax is assessed on the
/// calendar year, in the Year of Assessment that follows it — a genuinely
/// different year in the same country, which is why both are listed.
pub static SINGAPORE: FiscalProfile = FiscalProfile {
    code: "SG",
    english_name: "Singapore",
    systems: &[
        YearSystem {
            name: "Singapore government financial year",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            note: "The Financial Procedure Act 1966 s. 2 wording was read from a search result \
                   of the official Singapore Statutes Online text rather than the page itself.",
        },
        YearSystem {
            name: "Singapore basis period for personal income tax",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Statute,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            note: "Income of calendar year Y is assessed in Year of Assessment Y+1. This entry \
                   is the basis period, not the Year of Assessment.",
        },
    ],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Financial Procedure Act 1966 s. 2; Constitution of the Republic of Singapore \
              Art. 147; Accountant-General's Department",
};

/// Thailand 🇹🇭 — 1 October, named after the Buddhist Era year it **ends**
/// in, and moved three times.
///
/// The naming rule is in the statute itself rather than in custom:
/// พระราชบัญญัติวิธีการงบประมาณ พ.ศ. 2561 s. 4 defines the budget year as
/// 1 October to 30 September and directs that the *following* B.E. year be
/// used as its name. So ปีงบประมาณ 2568 ran 1 October 2024 to
/// 30 September 2025.
///
/// The history is not the one usually told. The year began on 1 April under
/// the older Thai new year; it moved to 1 October in B.E. 2481 (1938), to
/// the calendar year in B.E. 2483 (1940) alongside Phibunsongkhram's move of
/// the civil new year to 1 January, and back to 1 October from fiscal year
/// B.E. 2505, which began on 1 October 1961.
///
/// Labels here are **Buddhist Era** years, as the statute writes them.
pub static THAILAND: FiscalProfile = FiscalProfile {
    code: "TH",
    english_name: "Thailand",
    systems: &[
        YearSystem {
            name: "Thai budget year",
            local_name: "ปีงบประมาณ",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::new(StartCalendar::ThaiBuddhist, 10, 1),
            label: LabelConvention::LabelledByEndYear,
            valid_from: Some(2505),
            valid_until: None,
            note: "Labels are Buddhist Era years (B.E. = CE + 543). The statute names the year \
                   after the B.E. year it ends in, so ปีงบประมาณ 2568 began 1 October 2024.",
        },
        YearSystem {
            name: "Thai budget year, calendar-year period",
            local_name: "ปีงบประมาณ",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::new(StartCalendar::ThaiBuddhist, 1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(2484),
            valid_until: Some(2504),
            note: "B.E. 2483 itself was a nine-month transitional period (April to December \
                   1940) when the civil new year moved to 1 January; this entry covers the full \
                   calendar years that followed it, and the transitional year is not modelled.",
        },
    ],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "พระราชบัญญัติวิธีการงบประมาณ พ.ศ. 2561 s. 4 (naming clause read from a \
              Thai-language search result, not the Budget Bureau PDF); Pridi Banomyong \
              Institute, \"วันขึ้นปีใหม่\"; National Assembly Library of Thailand",
};

/// Australia 🇦🇺 — 1 July, for the budget and the income year alike.
///
/// The Income Tax Assessment Act 1997 s. 995-1 defines a financial year as
/// the twelve months commencing on 1 July, and the Commonwealth budget uses
/// the same year. Written "2024-25" after the year it begins in; the
/// shorthand "FY25" names the same year by its end, which is a habit rather
/// than a definition.
pub static AUSTRALIA: FiscalProfile = FiscalProfile {
    code: "AU",
    english_name: "Australia",
    systems: &[YearSystem {
        name: "Australian financial year",
        local_name: "",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(7, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "The budget year and the income year coincide, so one entry covers both.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Income Tax Assessment Act 1997 s. 995-1; Australian Taxation Office; \
              Commonwealth Treasury",
};

/// New Zealand 🇳🇿 — two years, and they are labelled in opposite
/// directions.
///
/// The Crown financial year runs 1 July to 30 June (Public Finance Act 1989
/// s. 2) and is written "2024/25" after the year it begins in. The tax year
/// for individuals runs 1 April to 31 March and Inland Revenue names it
/// after the year it **ends** in: "the 2025 tax year" is 1 April 2024 to
/// 31 March 2025.
///
/// This is the crate's tidiest demonstration that the labelling convention
/// is a property of a *system*, not of a country: one country, two systems,
/// two conventions.
pub static NEW_ZEALAND: FiscalProfile = FiscalProfile {
    code: "NZ",
    english_name: "New Zealand",
    systems: &[
        YearSystem {
            name: "New Zealand Crown financial year",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(7, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            note: "The move to 1 July is attributed to the Public Finance Act 1989, but the \
                   author found only secondary sources for the date of the change, so no \
                   valid_from is asserted.",
        },
        YearSystem {
            name: "New Zealand tax year",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByEndYear,
            valid_from: None,
            valid_until: None,
            note: "Inland Revenue names the tax year after the calendar year it ends in, the \
                   opposite of the Crown financial year in the same country.",
        },
    ],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Public Finance Act 1989 s. 2 (wording from a search result of the official \
              legislation.govt.nz page); Inland Revenue, IR3G",
};

/// Pakistan 🇵🇰 — 1 July.
pub static PAKISTAN: FiscalProfile = FiscalProfile {
    code: "PK",
    english_name: "Pakistan",
    systems: &[YearSystem {
        name: "Pakistani financial year",
        local_name: "",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(7, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "A change from an April start around 1959 is sometimes asserted; the author found \
               no source for it either way, so no earlier system is listed and no validity \
               range is claimed.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Finance Division, Government of Pakistan, Budget in Brief 2025-26",
};

/// China 🇨🇳 — the calendar year, by statute.
///
/// Budget Law of the People's Republic of China, art. 18: the budgetary year
/// begins on 1 January and ends on 31 December of the Gregorian calendar.
/// Listed because "does China's fiscal year follow the lunar new year?" is a
/// question people ask, and the answer is no.
pub static CHINA: FiscalProfile = FiscalProfile {
    code: "CN",
    english_name: "China",
    systems: &[YearSystem {
        name: "Chinese budgetary year",
        local_name: "预算年度",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(1, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "The Budget Law names the Gregorian calendar explicitly (公历), so the lunisolar \
               new year has no bearing on it.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "中华人民共和国预算法 art. 18 (National People's Congress)",
};

// ── The Americas ────────────────────────────────────────────────────────

/// United States 🇺🇸 — 1 October, labelled by the year it **ends** in, and
/// 1 July before that.
///
/// 31 U.S.C. § 1102: "The fiscal year of the Treasury begins on October 1 of
/// each year and ends on September 30 of the following year." The
/// Congressional Budget Office states the labelling rule as plainly as one
/// could wish: federal fiscal years "are designated by the calendar year in
/// which they end", so FY 2021 began on 1 October 2020.
///
/// # The 1976 change, and the quarter that belongs to no year
///
/// Before fiscal year 1977 the federal year ran 1 July to 30 June — a basis
/// that goes back to the Act of 26 August 1842. The Congressional Budget and
/// Impoundment Control Act of 1974 (Pub. L. 93-344), Title V § 501, moved it
/// to October, first effective for FY1977 beginning 1 October 1976.
///
/// That leaves **1 July to 30 September 1976** in no fiscal year at all:
/// FY1976 had ended on 30 June and FY1977 had not begun. Congress legislated
/// for it separately, as the *transition quarter*, under the Fiscal Year
/// Transition Act (Pub. L. 94-274). This table reproduces the hole rather
/// than papering over it — [`FiscalProfile::at`] returns `None` for any day
/// in it, which is the correct answer and the reason validity ranges are
/// part of [`YearSystem`] rather than a footnote.
pub static UNITED_STATES: FiscalProfile = FiscalProfile {
    code: "US",
    english_name: "United States",
    systems: &[
        YearSystem {
            name: "United States federal fiscal year",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(10, 1),
            label: LabelConvention::LabelledByEndYear,
            valid_from: Some(1977),
            valid_until: None,
            note: "FY1977 began 1 October 1976, the first year on the October basis.",
        },
        YearSystem {
            name: "United States federal fiscal year, July basis",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(7, 1),
            label: LabelConvention::LabelledByEndYear,
            valid_from: None,
            valid_until: Some(1976),
            note: "The July basis dates from the Act of 26 August 1842, which also gave the \
                   fiscal year its first statutory definition; the author could not confirm \
                   which label the first July year carried, so no valid_from is asserted. \
                   1 July to 30 September 1976 — the transition quarter — is covered by \
                   neither system, deliberately.",
        },
    ],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "31 U.S.C. § 1102; Congressional Budget and Impoundment Control Act of 1974, \
              Pub. L. 93-344 tit. V § 501; Fiscal Year Transition Act, Pub. L. 94-274; \
              GAO-05-734SP, A Glossary of Terms Used in the Federal Budget Process; \
              CBO, Common Budgetary Terms Explained",
};

/// Canada 🇨🇦 — 1 April, and not internally consistent about the label.
///
/// Financial Administration Act, RSC 1985, c. F-11, s. 2: "*fiscal year*
/// means the period beginning on April 1 in one year and ending on March 31
/// in the next year."
///
/// The Estimates and the Budget write the year as a span leading with the
/// start year — "2025–26" — while the Public Accounts are titled by the year
/// they end in: the volume for the year ended 31 March 2025 is *Public
/// Accounts of Canada 2025*. This table follows the Estimates, because that
/// is the form the statute's "beginning on April 1 in one year" matches, and
/// says so here rather than leaving the reader to be surprised by a Public
/// Accounts title.
pub static CANADA: FiscalProfile = FiscalProfile {
    code: "CA",
    english_name: "Canada",
    systems: &[YearSystem {
        name: "Canadian federal fiscal year",
        local_name: "exercice",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(4, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "Estimates and Budget name the year by its start (\"2025-26\"); the Public \
               Accounts name the same year by its end (\"Public Accounts of Canada 2025\"). \
               One country, one fiscal year, two published labels.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Financial Administration Act, RSC 1985, c. F-11, ss. 2 and 64; Treasury Board of \
              Canada Secretariat, Main Estimates; Receiver General, Public Accounts of Canada",
};

/// Brazil 🇧🇷 — the calendar year, by statute.
///
/// Lei nº 4.320 de 17 de março de 1964, art. 34: "O exercício financeiro
/// coincidirá com o ano civil."
pub static BRAZIL: FiscalProfile = FiscalProfile {
    code: "BR",
    english_name: "Brazil",
    systems: &[YearSystem {
        name: "Brazilian financial year",
        local_name: "exercício financeiro",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(1, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Lei nº 4.320/1964 art. 34 (Presidência da República)",
};

// ── Europe ──────────────────────────────────────────────────────────────

/// United Kingdom 🇬🇧 — 1 April for the state, 6 April for the taxpayer, and
/// the reason is a calendar reform.
///
/// The two years are five days apart and both are called "the UK fiscal
/// year" in ordinary speech. Both are statutory:
///
/// * **Government financial year** — Interpretation Act 1978, Schedule 1:
///   "the twelve months ending with 31st March".
/// * **Personal tax year** — Income Tax Act 2007, s. 4(3): "A tax year
///   begins on 6 April and ends on the following 5 April", and s. 4(4):
///   "'the tax year 2007-08' means the tax year beginning on 6 April 2007",
///   which fixes the labelling convention in the statute itself.
/// * **Corporation tax "financial year"** — Corporation Tax Act 2010,
///   s. 1119: "the financial year 2010" means the financial year *beginning*
///   with April 2010. Same dates as the government year, named the same way,
///   different tax.
///
/// # Why 6 April: the clearest case in the library of a calendar reform
/// still shaping present-day law
///
/// The English legal and fiscal year began on **25 March**, Lady Day, until
/// 1752. The Calendar (New Style) Act 1750 (24 Geo. 2 c. 23) then made
/// 2 September 1752 be followed by 14 September 1752, dropping eleven days
/// to bring Britain onto the Gregorian calendar — the reform
/// `hc_calendars_solar::julian_gregorian` implements as the `gb` adoption.
///
/// The Act's section 6 is the part that matters here. It provided that the
/// times of payment of rents and annuities, the running of leases, and the
/// attaining of full age were **not** altered by the renaming of the days.
/// Obligations therefore moved eleven natural days later, and the Lady Day
/// boundary landed on 5 April. That is a provision of the statute, not a
/// discretionary Treasury decision, and it is a better account than the
/// familiar "the Treasury did not want to lose eleven days of revenue" —
/// which is a fair gloss on the motive but not the mechanism.
///
/// # The 1800 leap day: repeated everywhere, evidenced nowhere
///
/// The usual telling adds a further step: that a twelfth day was added in
/// 1800, because the Julian calendar would have had a leap day that year and
/// the Gregorian did not, moving the year start from 5 April to 6 April.
/// **This crate does not assert that**, for two reasons. No statute or
/// contemporary record for such an adjustment has been produced, in the
/// author's search or in the popular accounts that repeat it. And Pitt's
/// first income tax, under the Income Tax Act 1799, already ran to 5 April
/// **1800** — so the 5 April / 6 April boundary existed *before* the
/// adjustment is supposed to have happened.
///
/// A better-evidenced explanation, argued at length by Alan O'Brien and
/// summarised by Paul Lewis, is that under the older legal sense of "from",
/// a year running "from 25 March" began on **26 March**; twenty-six March
/// plus eleven days is 6 April, with no 1800 step required. That account is
/// not yet established scholarship — O'Brien's study is self-published — so
/// this entry states the 1752 chain as fact, records the 1800 story as
/// unsupported, and names the alternative without endorsing it.
pub static UNITED_KINGDOM: FiscalProfile = FiscalProfile {
    code: "GB",
    english_name: "United Kingdom",
    systems: &[
        YearSystem {
            name: "United Kingdom government financial year",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1753),
            valid_until: None,
            note: "HM Treasury writes it as a span (\"2024-25\"); the Corporation Tax Act 2010 \
                   s. 1119 names the same dates by the starting year alone. The table does not \
                   know which year the 1 April government year began — the statute cited is the \
                   Interpretation Act 1978 — and bounds it at 1753 only so that it cannot \
                   overlap the Lady Day year listed below; 1751 and 1752 were transitional and \
                   belong to neither.",
        },
        YearSystem {
            name: "United Kingdom personal tax year",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 6),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            note: "The 6 April start descends from the 1752 calendar reform; see this profile's \
                   documentation for what is and is not evidenced. No valid_from is asserted \
                   because the boundary predates the modern income tax and this table cannot \
                   cite the year it settled.",
        },
        YearSystem {
            name: "England and Wales legal and fiscal year, before 1752",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(3, 25),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: Some(1750),
            note: "Lady Day, 25 March. The dates here are proleptic Gregorian, which is not \
                   what anyone wrote at the time: for the Julian dates actually used in \
                   Britain before September 1752, use hc_calendars_solar::julian_gregorian \
                   with the `gb` adoption. The range stops at 1750 because 1751 and 1752 were \
                   both transitional — the 1750 Act moved the legal new year to 1 January, so \
                   the English year 1751 ran only from 25 March to 31 December 1751, and 1752 \
                   then lost eleven days in September. Neither transitional year is modelled. \
                   Listed to show where 6 April came from, not as a practical calendar.",
        },
    ],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Interpretation Act 1978 Sch. 1; Income Tax Act 2007 s. 4; Corporation Tax Act \
              2010 s. 1119; Calendar (New Style) Act 1750 (24 Geo. 2 c. 23) ss. 1 and 6",
};

/// Germany 🇩🇪 — the calendar year, by statute, with an escape hatch.
///
/// Bundeshaushaltsordnung § 4: "Rechnungsjahr (Haushaltsjahr) ist das
/// Kalenderjahr. Das Bundesministerium der Finanzen kann für einzelne
/// Bereiche etwas anderes bestimmen." The same wording is in § 4 HGrG for
/// the Länder.
pub static GERMANY: FiscalProfile = FiscalProfile {
    code: "DE",
    english_name: "Germany",
    systems: &[YearSystem {
        name: "German budget year",
        local_name: "Haushaltsjahr",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(1, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "§ 4 BHO lets the Federal Ministry of Finance set a different year for individual \
               areas; this entry is the general rule, not a claim that no exception exists.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Bundeshaushaltsordnung § 4; Haushaltsgrundsätzegesetz § 4",
};

/// France 🇫🇷 — the calendar year, plus up to twenty days to write it down.
///
/// LOLF (loi organique n° 2001-692 du 1er août 2001), art. 1: "L'exercice
/// s'étend sur une année civile." A *période complémentaire* of at most
/// twenty days may be used to record operations against the closed year; it
/// is an accounting window, not a longer year, so it is noted rather than
/// modelled.
pub static FRANCE: FiscalProfile = FiscalProfile {
    code: "FR",
    english_name: "France",
    systems: &[YearSystem {
        name: "French budget year",
        local_name: "exercice budgétaire",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(1, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "A période complémentaire of at most twenty days allows operations to be booked \
               to the closed year. It does not extend the year and is not modelled.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Loi organique n° 2001-692 du 1er août 2001 (LOLF) art. 1 (Légifrance)",
};

/// Russia 🇷🇺 — the calendar year, by the Budget Code.
///
/// Бюджетный кодекс РФ art. 12: the financial year runs from 1 January to
/// 31 December. The federal budget is enacted for three years at a time —
/// the year plus a two-year planning period — which changes how budgets are
/// *passed*, not what a financial year is.
pub static RUSSIA: FiscalProfile = FiscalProfile {
    code: "RU",
    english_name: "Russia",
    systems: &[YearSystem {
        name: "Russian financial year",
        local_name: "финансовый год",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(1, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "The three-year budget horizon is a budgeting practice; the financial year itself \
               is the calendar year.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Бюджетный кодекс Российской Федерации (145-ФЗ) art. 12",
};

/// Sweden 🇸🇪 — the calendar year for the state now, 1 July for seventy
/// years, and the calendar year before that.
///
/// Sweden is the crate's best demonstration that a fiscal year is a
/// historical object. The state budget year was the calendar year, moved to
/// 1 July – 30 June with budget year 1923/24, and moved back to the calendar
/// year with budget year **1997** as part of the *rambeslutsmodell* budget
/// reform. The change was made through an eighteen-month transitional budget
/// year 1995/96, running 1 July 1995 to 31 December 1996.
///
/// This table carries the two full systems and deliberately leaves the two
/// transitional periods — the stub half-year of 1923 and the eighteen-month
/// 1995/96 — outside both, so a caller asking about 1996 gets `None` rather
/// than a confident wrong answer.
///
/// Companies are separate: Bokföringslag (1999:1078) 3 kap. 1 § makes the
/// calendar year the default *räkenskapsår* and requires it outright for
/// natural persons, while permitting a *brutet räkenskapsår* — which must be
/// one of 1 May, 1 July or 1 September — for other undertakings.
pub static SWEDEN: FiscalProfile = FiscalProfile {
    code: "SE",
    english_name: "Sweden",
    systems: &[
        YearSystem {
            name: "Swedish state budget year",
            local_name: "budgetår",
            kind: SystemKind::Government,
            authority: Authority::Convention,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1997),
            valid_until: None,
            note: "Neither Budgetlagen (2011:203) nor Regeringsformen ch. 9 defines the budget \
                   year in a single clause, so this rests on the 1990s reform decisions and \
                   practice rather than on a statutory definition — hence Convention, not \
                   Statute.",
        },
        YearSystem {
            name: "Swedish state budget year, July basis",
            local_name: "budgetår",
            kind: SystemKind::Government,
            authority: Authority::Convention,
            start: YearStart::gregorian(7, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1923),
            valid_until: Some(1995),
            note: "Budget year 1995/96 was itself an eighteen-month transitional period \
                   (1 July 1995 to 31 December 1996) and is not modelled; nor is the \
                   six-month stub of 1 January to 30 June 1923. 1996 therefore falls in no \
                   Swedish budget year here, which is the honest answer.",
        },
        YearSystem {
            name: "Swedish company accounting year",
            local_name: "räkenskapsår",
            kind: SystemKind::CorporateDefault,
            authority: Authority::Statute,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            note: "Mandatory for natural persons and for partnerships taxed through one. Other \
                   undertakings may choose a brutet räkenskapsår beginning 1 May, 1 July or \
                   1 September, which this entry does not enumerate as systems of their own.",
        },
    ],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Kommittédirektiv 1999:23; Prop. 1994/95:100; Prop. 1922:1; \
              Bokföringslag (1999:1078) 3 kap. 1-2 §§",
};

// ── Africa and the Middle East ──────────────────────────────────────────

/// Egypt 🇪🇬 — 1 July.
pub static EGYPT: FiscalProfile = FiscalProfile {
    code: "EG",
    english_name: "Egypt",
    systems: &[YearSystem {
        name: "Egyptian fiscal year",
        local_name: "السنة المالية",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(7, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "Written as a span (\"FY 2025/2026\"). Alignment with the calendar year has been \
               discussed publicly more than once; no enacted change was found.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Ministry of Finance of Egypt, budget and borrowing-plan publications",
};

/// South Africa 🇿🇦 — 1 April for the state, 1 March for the individual.
///
/// The National Treasury's budget year runs 1 April to 31 March. The South
/// African Revenue Service's year of assessment for individuals runs
/// 1 March to the end of February and is named after the calendar year it
/// **ends** in: the 2026 tax year ran 1 March 2025 to 28 February 2026. Two
/// systems, one month apart, labelled in opposite directions.
pub static SOUTH_AFRICA: FiscalProfile = FiscalProfile {
    code: "ZA",
    english_name: "South Africa",
    systems: &[
        YearSystem {
            name: "South African government financial year",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            note: "The Public Finance Management Act 1 of 1999 s. 1 definition could not be \
                   read first-hand; the dates are from National Treasury budget documents.",
        },
        YearSystem {
            name: "South African year of assessment for individuals",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Statute,
            start: YearStart::gregorian(3, 1),
            label: LabelConvention::LabelledByEndYear,
            valid_from: None,
            valid_until: None,
            note: "Ends on the last day of February, so the year is 365 or 366 days and its \
                   final month is the one the leap day belongs to.",
        },
    ],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "National Treasury, Estimates of National Expenditure; Public Finance Management \
              Act 1 of 1999 (definition not read first-hand); South African Revenue Service, \
              Personal Income Tax",
};

// ── Fiscal years that are not Gregorian ─────────────────────────────────

/// Iran 🇮🇷 — the Solar Hijri year itself, beginning at Nowruz.
///
/// قانون محاسبات عمومی کشور (Public Accounting Act), art. 6: the fiscal year
/// is one Solar Hijri year, beginning on the first of Farvardin and ending
/// at the end of Esfand. The statute says "the end of Esfand" rather than a
/// numbered day, which neatly sidesteps the fact that Esfand has 29 days in
/// a common year and 30 in a leap one.
///
/// So the Iranian fiscal year is not an offset from a Gregorian year at all:
/// it *is* the Solar Hijri calendar year, and its Gregorian start moves
/// between 20 and 21 March. This entry is the crate's proof that the
/// calendar abstraction carries — the start is expressed as 1 Farvardin in
/// [`StartCalendar::SolarHijriArithmetic`] and converted by the calendar,
/// not hard-coded as "about 21 March".
///
/// # What is approximated, and by how much
///
/// The official rule is astronomical: 1 Farvardin is the day on which the
/// March equinox falls before noon, true time, at the 52.5°E meridian.
/// `hc-calendars-solar` has no ephemeris, so this entry goes through the
/// 2 820-year arithmetic cycle associated with Ahmad Birashk, which is very
/// nearly but not exactly that calendar.
///
/// Measured against the published Solar Hijri years, the cycle agrees for
/// 1400, 1401, 1402, 1403 and 1405 and is **one day early for 1404**: it
/// starts that year on 20 March 2025 where the official calendar starts it
/// on 21 March. Claus Tøndering's survey names 1404 and 1437 as the only two
/// such disagreements between AP 1244 and 1531 (AD 1865 and 2152), so the
/// claim this entry makes is: correct to the day for every Iranian fiscal
/// year in that window except two, and this crate's tests name both.
///
/// That is a stated precision, not a hidden one. A caller who needs the
/// official date needs an ephemeris, which is `hc-astro`'s job and a
/// documented gap in `hc-calendars-solar` rather than here.
pub static IRAN: FiscalProfile = FiscalProfile {
    code: "IR",
    english_name: "Iran",
    systems: &[YearSystem {
        name: "Iranian fiscal year",
        local_name: "سال مالی",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::new(StartCalendar::SolarHijriArithmetic, 1, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: Some(1366),
        valid_until: None,
        note: "Labels are Solar Hijri years. Computed through the arithmetic Birashk cycle, \
               which is one day early for AP 1404 and AP 1437 against the official \
               equinox-based calendar; every other year between AP 1244 and AP 1531 agrees. \
               valid_from is the year the cited Public Accounting Act was enacted, not the \
               year Iran began using this fiscal year.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "قانون محاسبات عمومی کشور art. 6 (enacted 1/6/1366 SH); Claus Tøndering, \
              \"The Persian Calendar\", for the arithmetic approximation's error",
};

/// Ethiopia 🇪🇹 — Hamle 1 to Sene 30, labelled by the Ethiopic year it
/// **ends** in.
///
/// The Federal Government of Ethiopia Financial Administration Proclamation
/// No. 648/2009 defines the fiscal year as running from Hamle 1 to Sene 30
/// of the Ethiopian calendar; the Ministry of Finance publishes the same
/// thing as "from July 08 to July 7". Hamle is the eleventh Ethiopic month
/// and Sene the tenth, so the year straddles two Ethiopic years.
///
/// # The label is the trap again, in a second calendar
///
/// EFY 2016 ran from Hamle 1 of **2015** EC to Sene 30 of 2016 EC —
/// 8 July 2023 to 7 July 2024. The fiscal year is named after the Ethiopic
/// year it ends in, exactly as the United States federal year is named after
/// the Gregorian year it ends in, and a table that assumed the start year
/// would be wrong by one for every Ethiopian fiscal year. The budget
/// proclamation numbering makes the point vividly: EFY 2016's budget was
/// enacted by Proclamation No. 1297/**2015**.
///
/// # Why the Gregorian equivalent does not drift
///
/// Pagumen, the five or six intercalary days that make the Ethiopic year
/// long, falls at the *end* of the Ethiopic year — in September — and so
/// never lands between Sene 30 and the following Hamle 1. Over the modern
/// era the Ethiopic and Gregorian leap cycles keep step and the fiscal year
/// begins on 8 July every year. This crate computes it through the calendar
/// anyway, because the stability is a consequence and not a rule, and it
/// ends when the Gregorian century rule next diverges.
///
/// Quarters are not available: see [`StartCalendar::months_of_equal_standing`].
pub static ETHIOPIA: FiscalProfile = FiscalProfile {
    code: "ET",
    english_name: "Ethiopia",
    systems: &[YearSystem {
        name: "Ethiopian fiscal year",
        local_name: "የበጀት ዓመት",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::new(StartCalendar::Ethiopic, 11, 1),
        label: LabelConvention::LabelledByEndYear,
        valid_from: Some(2002),
        valid_until: None,
        note: "Labels are Ethiopic years. The Proclamation 648/2009 definition was read from a \
               search index rather than the ministry's own PDF; the Ministry of Finance's \
               \"Hamle 01 to Sene 30\" wording is independent confirmation. valid_from is the \
               Ethiopic year of that proclamation, not the year the fiscal year was adopted.",
    }],
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Federal Government of Ethiopia Financial Administration Proclamation No. 648/2009 \
              (648/2001 EC); Ministry of Finance, Citizens' Budget EFY 2017; \
              Budget Proclamation No. 1297/2015",
};

/// Every country table in this crate, in ISO 3166-1 alpha-2 order.
pub static ALL: &[&FiscalProfile] = &[
    &AUSTRALIA,
    &BRAZIL,
    &CANADA,
    &CHINA,
    &GERMANY,
    &EGYPT,
    &ETHIOPIA,
    &FRANCE,
    &UNITED_KINGDOM,
    &HONG_KONG,
    &INDIA,
    &IRAN,
    &JAPAN,
    &NEW_ZEALAND,
    &PAKISTAN,
    &RUSSIA,
    &SWEDEN,
    &SINGAPORE,
    &THAILAND,
    &UNITED_STATES,
    &SOUTH_AFRICA,
];

/// The profile for an ISO 3166-1 alpha-2 code, if this crate has one.
#[must_use]
pub fn by_code(code: &str) -> Option<&'static FiscalProfile> {
    ALL.iter().copied().find(|profile| profile.code == code)
}

/// The documented gap for an ISO 3166-1 alpha-2 code, if there is one.
///
/// Checked separately from [`by_code`] on purpose: "this crate has no data"
/// and "this crate knows the answer and declines to compute it" are
/// different replies and should not both arrive as `None`.
#[must_use]
pub fn gap_for(code: &str) -> Option<&'static DocumentedGap> {
    GAPS.iter().find(|gap| gap.code == code)
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::{buddhist, ethiopic, gregorian, persian};

    use super::*;
    use crate::error::FiscalError;
    use crate::quarters::Quarter;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    // ── Japan ───────────────────────────────────────────────────────────

    #[test]
    fn japans_fiscal_year_2024_runs_from_april_2024_to_march_2025() {
        let system = JAPAN.government(2024).unwrap();
        let span = system.span(2024).unwrap();
        assert_eq!(span.first, greg(2024, 4, 1));
        assert_eq!(span.last, greg(2025, 3, 31));
    }

    #[test]
    fn japans_fiscal_year_boundary_is_tested_on_both_sides() {
        let system = JAPAN.government(2024).unwrap();
        assert_eq!(system.label_at(greg(2024, 3, 31)).unwrap(), 2023);
        assert_eq!(system.label_at(greg(2024, 4, 1)).unwrap(), 2024);
        assert_eq!(system.label_at(greg(2025, 3, 31)).unwrap(), 2024);
        assert_eq!(system.label_at(greg(2025, 4, 1)).unwrap(), 2025);
        // And the day before and after the whole span, from the span side.
        let span = system.span(2024).unwrap();
        assert!(!span.contains(Rd(span.first.0 - 1)));
        assert!(!span.contains(Rd(span.last.0 + 1)));
    }

    #[test]
    fn japans_school_year_and_fiscal_year_are_the_same_year() {
        let fiscal = JAPAN.in_force(SystemKind::Government, 2024).unwrap();
        let school = JAPAN.in_force(SystemKind::Academic, 2024).unwrap();
        assert_eq!(fiscal.span(2024).unwrap(), school.span(2024).unwrap());
        // They are still two entries, with different authorities behind them.
        assert_eq!(fiscal.authority, Authority::Statute);
        assert_eq!(school.authority, Authority::Regulation);
    }

    #[test]
    fn japans_first_quarter_is_april_to_june_not_january_to_march() {
        let system = JAPAN.government(2024).unwrap();
        assert_eq!(
            system.calendar_months_of_quarter(Quarter::First).unwrap(),
            [4, 5, 6]
        );
        let q1 = system.quarter_span(2024, Quarter::First).unwrap();
        assert_eq!(q1.first, greg(2024, 4, 1));
        assert_eq!(q1.last, greg(2024, 6, 30));
    }

    // ── The label convention, country against country ───────────────────

    #[test]
    fn japan_and_the_united_states_label_the_same_day_a_year_apart() {
        // 1 November 2023 is in Japan's 2023年度 and in the United States'
        // FY2024. Both are written "FY".
        let day = greg(2023, 11, 1);
        let (_, japan) = JAPAN.at(SystemKind::Government, day).unwrap();
        let (_, us) = UNITED_STATES.at(SystemKind::Government, day).unwrap();
        assert_eq!(japan, 2023);
        assert_eq!(us, 2024);
    }

    #[test]
    fn the_two_years_both_called_fy2024_start_six_months_apart() {
        let japan = JAPAN.government(2024).unwrap().span(2024).unwrap();
        let us = UNITED_STATES.government(2024).unwrap().span(2024).unwrap();
        assert_eq!(japan.first, greg(2024, 4, 1));
        assert_eq!(us.first, greg(2023, 10, 1));
        assert_eq!(japan.first.0 - us.first.0, 183);
    }

    #[test]
    fn new_zealand_labels_its_two_years_in_opposite_directions() {
        // The Crown financial year 2024/25 begins 1 July 2024; the 2025 tax
        // year begins 1 April 2024. One country, two conventions.
        let crown = NEW_ZEALAND.in_force(SystemKind::Government, 2024).unwrap();
        let tax = NEW_ZEALAND.in_force(SystemKind::PersonalTax, 2025).unwrap();
        assert_eq!(crown.label, LabelConvention::LabelledByStartYear);
        assert_eq!(tax.label, LabelConvention::LabelledByEndYear);
        assert_eq!(crown.span(2024).unwrap().first, greg(2024, 7, 1));
        assert_eq!(tax.span(2025).unwrap().first, greg(2024, 4, 1));
        assert_eq!(tax.span(2025).unwrap().last, greg(2025, 3, 31));
    }

    #[test]
    fn south_africa_labels_its_two_years_in_opposite_directions_too() {
        let government = SOUTH_AFRICA.government(2025).unwrap();
        let tax = SOUTH_AFRICA
            .in_force(SystemKind::PersonalTax, 2026)
            .unwrap();
        assert_eq!(government.span(2025).unwrap().first, greg(2025, 4, 1));
        // SARS: the 2026 year of assessment ran 1 March 2025 to 28 February 2026.
        assert_eq!(tax.span(2026).unwrap().first, greg(2025, 3, 1));
        assert_eq!(tax.span(2026).unwrap().last, greg(2026, 2, 28));
    }

    #[test]
    fn thailands_budget_year_is_named_by_the_buddhist_era_year_it_ends_in() {
        // ปีงบประมาณ 2568 ran 1 October 2024 to 30 September 2025.
        let system = THAILAND.government(2568).unwrap();
        let span = system.span(2568).unwrap();
        assert_eq!(span.first, greg(2024, 10, 1));
        assert_eq!(span.last, greg(2025, 9, 30));
    }

    #[test]
    fn thailands_earlier_calendar_year_period_is_a_separate_system() {
        // B.E. 2500 (1957) fell in the calendar-year period.
        let system = THAILAND.government(2500).unwrap();
        assert!(system.is_calendar_year());
        assert_eq!(system.span(2500).unwrap().first, greg(1957, 1, 1));
        // And the October system refuses that year outright.
        let october = &THAILAND.systems[0];
        assert_eq!(october.span(2500), Err(FiscalError::OutsideValidity));
    }

    // ── The United States, including the hole in 1976 ────────────────────

    #[test]
    fn the_us_fiscal_year_1977_began_on_the_first_of_october_1976() {
        let system = UNITED_STATES.government(1977).unwrap();
        let span = system.span(1977).unwrap();
        assert_eq!(span.first, greg(1976, 10, 1));
        assert_eq!(span.last, greg(1977, 9, 30));
    }

    #[test]
    fn the_us_fiscal_year_1976_ended_on_the_thirtieth_of_june() {
        let system = UNITED_STATES.government(1976).unwrap();
        assert_eq!(system.start, YearStart::gregorian(7, 1));
        let span = system.span(1976).unwrap();
        assert_eq!(span.first, greg(1975, 7, 1));
        assert_eq!(span.last, greg(1976, 6, 30));
    }

    #[test]
    fn the_transition_quarter_of_1976_belongs_to_no_fiscal_year() {
        // 1 July to 30 September 1976: FY1976 had ended and FY1977 had not
        // begun. Congress legislated for the quarter separately.
        for day in greg(1976, 7, 1).0..=greg(1976, 9, 30).0 {
            assert_eq!(
                UNITED_STATES.at(SystemKind::Government, Rd(day)),
                None,
                "{day} should be in the transition quarter"
            );
        }
        // The days on either side of it do belong to a year.
        assert!(
            UNITED_STATES
                .at(SystemKind::Government, greg(1976, 6, 30))
                .is_some()
        );
        assert!(
            UNITED_STATES
                .at(SystemKind::Government, greg(1976, 10, 1))
                .is_some()
        );
    }

    #[test]
    fn every_other_day_of_the_twentieth_century_has_a_us_fiscal_year() {
        let mut missing = 0;
        for day in greg(1900, 1, 1).0..=greg(2000, 12, 31).0 {
            if UNITED_STATES.at(SystemKind::Government, Rd(day)).is_none() {
                missing += 1;
            }
        }
        // Exactly the 92 days of the transition quarter.
        assert_eq!(missing, 92);
    }

    // ── The United Kingdom ──────────────────────────────────────────────

    #[test]
    fn the_uk_tax_year_starts_on_the_sixth_of_april() {
        let tax = UNITED_KINGDOM
            .in_force(SystemKind::PersonalTax, 2024)
            .unwrap();
        let span = tax.span(2024).unwrap();
        assert_eq!(span.first, greg(2024, 4, 6));
        assert_eq!(span.last, greg(2025, 4, 5));
        // Income Tax Act 2007 s. 4(4): "the tax year 2007-08" is the one
        // beginning 6 April 2007.
        assert_eq!(tax.span(2007).unwrap().first, greg(2007, 4, 6));
    }

    #[test]
    fn the_uk_government_and_tax_years_are_five_days_apart() {
        let government = UNITED_KINGDOM.government(2024).unwrap();
        let tax = UNITED_KINGDOM
            .in_force(SystemKind::PersonalTax, 2024)
            .unwrap();
        assert_eq!(
            tax.span(2024).unwrap().first.0 - government.span(2024).unwrap().first.0,
            5
        );
        // And the five days between them belong to different years in the
        // two systems.
        for day in 1..=5 {
            let rd = greg(2024, 4, day);
            assert_eq!(government.label_at(rd).unwrap(), 2024);
            assert_eq!(tax.label_at(rd).unwrap(), 2023);
        }
    }

    #[test]
    fn the_pre_1752_english_year_began_on_lady_day() {
        let old = UNITED_KINGDOM
            .of_kind(SystemKind::Government)
            .find(|system| system.start == YearStart::gregorian(3, 25))
            .unwrap();
        assert_eq!(old.span(1700).unwrap().first, greg(1700, 3, 25));
        assert!(!old.covers(1752));
        assert_eq!(old.span(1752), Err(FiscalError::OutsideValidity));
    }

    // ── Non-Gregorian fiscal years ──────────────────────────────────────

    #[test]
    fn the_iranian_fiscal_year_is_the_solar_hijri_year_itself() {
        let system = IRAN.government(1403).unwrap();
        assert_eq!(system.start.calendar, StartCalendar::SolarHijriArithmetic);
        let span = system.span(1403).unwrap();
        assert_eq!(span.first, persian::to_fixed(1403, 1, 1).unwrap());
        assert_eq!(span.last, Rd(persian::to_fixed(1404, 1, 1).unwrap().0 - 1));
    }

    #[test]
    fn the_iranian_fiscal_year_matches_the_published_gregorian_starts() {
        // Published Solar Hijri year starts. 1404 is deliberately absent:
        // see the next test.
        let published = [
            (1400, (2021, 3, 21)),
            (1401, (2022, 3, 21)),
            (1402, (2023, 3, 21)),
            (1403, (2024, 3, 20)),
            (1405, (2026, 3, 21)),
        ];
        let system = IRAN.government(1403).unwrap();
        for (label, (year, month, day)) in published {
            assert_eq!(
                system.span(label).unwrap().first,
                greg(year, month, day),
                "Iranian fiscal year {label}"
            );
        }
        // 1402 ended on 19 March 2024, the least obvious value in the set.
        assert_eq!(system.span(1402).unwrap().last, greg(2024, 3, 19));
    }

    #[test]
    fn the_arithmetic_approximation_is_one_day_early_for_ap_1404() {
        // The official, equinox-based calendar starts AP 1404 on 21 March
        // 2025. The Birashk cycle this crate uses starts it on 20 March.
        // Tøndering names AP 1404 and AP 1437 as the only two disagreements
        // between AP 1244 and AP 1531. Asserting the wrong answer here is
        // deliberate: the test exists so that the day the crate gains an
        // astronomical Solar Hijri calendar, it fails and says so.
        let system = IRAN.government(1404).unwrap();
        assert_eq!(system.span(1404).unwrap().first, greg(2025, 3, 20));
        assert_ne!(system.span(1404).unwrap().first, greg(2025, 3, 21));
        assert!(system.start.calendar.is_approximate());
    }

    #[test]
    fn the_ethiopian_fiscal_year_runs_hamle_1_to_sene_30() {
        let system = ETHIOPIA.government(2016).unwrap();
        assert_eq!(system.start.calendar, StartCalendar::Ethiopic);
        let span = system.span(2016).unwrap();
        // Hamle 1 of 2015 EC to Sene 30 of 2016 EC.
        assert_eq!(span.first, ethiopic::to_fixed(2015, 11, 1).unwrap());
        assert_eq!(span.last, ethiopic::to_fixed(2016, 10, 30).unwrap());
    }

    #[test]
    fn the_ethiopian_fiscal_year_is_named_by_the_year_it_ends_in() {
        // EFY 2016 ran 8 July 2023 to 7 July 2024; EFY 2017, 8 July 2024 to
        // 7 July 2025. If the label were the starting Ethiopic year, every
        // one of these would be out by one.
        let system = ETHIOPIA.government(2016).unwrap();
        assert_eq!(system.label, LabelConvention::LabelledByEndYear);
        for (label, start, end) in [
            (2016, (2023, 7, 8), (2024, 7, 7)),
            (2017, (2024, 7, 8), (2025, 7, 7)),
            (2018, (2025, 7, 8), (2026, 7, 7)),
        ] {
            let span = system.span(label).unwrap();
            assert_eq!(span.first, greg(start.0, start.1, start.2), "EFY {label}");
            assert_eq!(span.last, greg(end.0, end.1, end.2), "EFY {label}");
        }
    }

    #[test]
    fn the_ethiopian_fiscal_year_does_not_drift_against_the_gregorian_one() {
        // Pagumen falls in September, outside the Sene-to-Hamle boundary, so
        // the two leap cycles keep step across the modern era.
        let system = ETHIOPIA.government(2016).unwrap();
        for label in 2005..=2090 {
            let (_, month, day) = gregorian::from_fixed(system.span(label).unwrap().first).unwrap();
            assert_eq!((month, day), (7, 8), "EFY {label}");
        }
    }

    #[test]
    fn the_ethiopian_fiscal_year_has_no_quarters() {
        let system = ETHIOPIA.government(2016).unwrap();
        assert_eq!(
            system.quarter(greg(2024, 1, 1)),
            Err(FiscalError::PeriodsNotDefined)
        );
        assert!(system.span(2016).is_ok());
    }

    // ── Sweden, and the years that fall between systems ─────────────────

    #[test]
    fn swedens_budget_year_was_july_based_until_the_1997_reform() {
        let july = SWEDEN.government(1990).unwrap();
        assert_eq!(july.span(1990).unwrap().first, greg(1990, 7, 1));
        assert_eq!(july.span(1990).unwrap().last, greg(1991, 6, 30));
        let calendar = SWEDEN.government(1997).unwrap();
        assert!(calendar.is_calendar_year());
        assert_eq!(calendar.span(1997).unwrap().first, greg(1997, 1, 1));
    }

    #[test]
    fn the_eighteen_month_swedish_transition_is_left_out_of_both_systems() {
        // Budget year 1995/96 ran 1 July 1995 to 31 December 1996 and is not
        // modelled, so 1996 belongs to no Swedish budget year here.
        assert_eq!(SWEDEN.government(1996), None);
        assert!(SWEDEN.government(1995).is_some());
        assert!(SWEDEN.government(1997).is_some());
    }

    #[test]
    fn swedish_companies_keep_the_calendar_year_by_default() {
        let corporate = SWEDEN.in_force(SystemKind::CorporateDefault, 2024).unwrap();
        assert!(corporate.is_calendar_year());
    }

    // ── Countries with no offset at all ─────────────────────────────────

    #[test]
    fn the_calendar_year_countries_report_no_offset() {
        for profile in [&CHINA, &GERMANY, &FRANCE, &RUSSIA, &BRAZIL] {
            assert!(profile.has_no_offset(), "{}", profile.english_name);
            let system = profile.government(2024).unwrap();
            assert_eq!(system.span(2024).unwrap().first, greg(2024, 1, 1));
            assert_eq!(system.span(2024).unwrap().last, greg(2024, 12, 31));
        }
    }

    #[test]
    fn countries_with_an_offset_do_not_report_none() {
        for profile in [&JAPAN, &UNITED_STATES, &UNITED_KINGDOM, &IRAN, &ETHIOPIA] {
            assert!(!profile.has_no_offset(), "{}", profile.english_name);
        }
    }

    // ── The April club, and the July club ───────────────────────────────

    #[test]
    fn the_april_countries_all_start_on_the_first_of_april() {
        for profile in [&INDIA, &CANADA, &HONG_KONG, &SINGAPORE, &SOUTH_AFRICA] {
            let system = profile.government(2024).unwrap();
            assert_eq!(
                system.start,
                YearStart::gregorian(4, 1),
                "{}",
                profile.english_name
            );
            assert_eq!(system.span(2024).unwrap().first, greg(2024, 4, 1));
            assert_eq!(system.span(2024).unwrap().last, greg(2025, 3, 31));
        }
    }

    #[test]
    fn the_july_countries_all_start_on_the_first_of_july() {
        for profile in [&AUSTRALIA, &NEW_ZEALAND, &EGYPT, &PAKISTAN] {
            let system = profile.government(2024).unwrap();
            assert_eq!(
                system.start,
                YearStart::gregorian(7, 1),
                "{}",
                profile.english_name
            );
            assert_eq!(system.span(2024).unwrap().last, greg(2025, 6, 30));
        }
    }

    #[test]
    fn the_october_countries_put_q1_in_october() {
        for profile in [&UNITED_STATES, &THAILAND] {
            let system = profile.of_kind(SystemKind::Government).next().unwrap();
            assert_eq!(system.start.month, 10);
            assert_eq!(
                system.calendar_months_of_quarter(Quarter::First).unwrap(),
                [10, 11, 12],
                "{}",
                profile.english_name
            );
        }
    }

    // ── Table hygiene ───────────────────────────────────────────────────

    #[test]
    fn every_profile_has_a_source_a_check_date_and_at_least_one_system() {
        for profile in ALL {
            assert_eq!(profile.code.len(), 2, "{}", profile.english_name);
            assert!(!profile.english_name.is_empty());
            assert!(!profile.systems.is_empty(), "{}", profile.english_name);
            assert!(!profile.sources.is_empty(), "{}", profile.english_name);
            assert!(profile.sources_checked.year >= 2026, "{}", profile.code);
            assert!((1..=12).contains(&profile.sources_checked.month));
            assert!((1..=31).contains(&profile.sources_checked.day));
            for system in profile.systems {
                assert!(!system.name.is_empty(), "{}", profile.code);
                assert!((1..=13).contains(&system.start.month), "{}", profile.code);
                assert!((1..=31).contains(&system.start.day), "{}", profile.code);
            }
        }
    }

    #[test]
    fn every_country_code_appears_exactly_once() {
        for (index, profile) in ALL.iter().enumerate() {
            for other in &ALL[index + 1..] {
                assert_ne!(profile.code, other.code, "duplicate {}", profile.code);
            }
        }
        assert_eq!(ALL.len(), 21);
    }

    #[test]
    fn every_system_of_one_kind_within_a_country_covers_distinct_years() {
        // Two systems of the same kind must not both claim a label, or
        // `in_force` would return whichever came first by accident.
        for profile in ALL {
            for kind in [
                SystemKind::Government,
                SystemKind::PersonalTax,
                SystemKind::CorporateDefault,
                SystemKind::Academic,
            ] {
                for label in -100..3000 {
                    let claimants = profile
                        .of_kind(kind)
                        .filter(|system| system.covers(label))
                        .count();
                    assert!(claimants <= 1, "{} claims {label} twice", profile.code);
                }
            }
        }
    }

    #[test]
    fn a_profile_can_be_found_by_its_code() {
        assert_eq!(by_code("JP").unwrap().english_name, "Japan");
        assert_eq!(by_code("ET").unwrap().code, "ET");
        assert!(by_code("ZZ").is_none());
        // Nepal is not a profile; it is a documented gap, and the two
        // lookups are deliberately separate.
        assert!(by_code("NP").is_none());
        assert_eq!(gap_for("NP").unwrap().english_name, "Nepal");
        assert!(gap_for("JP").is_none());
    }

    #[test]
    fn every_government_system_round_trips_a_long_span_of_days() {
        for profile in ALL {
            for system in profile.of_kind(SystemKind::Government) {
                let first = system.valid_from.unwrap_or(1950).max(1950);
                let last = system.valid_until.unwrap_or(2050).min(2050);
                for label in first..=last {
                    let span = system.projected_span(label).unwrap();
                    assert!(span.days() >= 353 && span.days() <= 367, "{}", profile.code);
                    assert_eq!(system.projected_label_at(span.first).unwrap(), label);
                    assert_eq!(system.projected_label_at(span.last).unwrap(), label);
                    let position = span.position_of(span.last).unwrap();
                    assert_eq!(i64::from(position.day_of_year), span.days());
                    assert_eq!(position.days_remaining(), 0);
                }
            }
        }
    }

    #[test]
    fn thailands_budget_year_is_expressed_in_the_thai_calendar_not_offset_by_hand() {
        // The label 2568 is a Buddhist Era year, so the start is a date in
        // the Thai solar calendar rather than a Gregorian date with 543
        // added after the fact.
        let system = THAILAND.government(2568).unwrap();
        assert_eq!(system.start.calendar, StartCalendar::ThaiBuddhist);
        assert_eq!(system.start.calendar.id().as_str(), "buddhist");
        assert_eq!(
            system.span(2568).unwrap().first,
            buddhist::to_fixed(2567, 10, 1).unwrap()
        );
    }

    #[test]
    fn thailands_quarters_are_named_in_thai_calendar_months() {
        // Same months as the United States' — the Thai calendar shares the
        // Gregorian month structure — but reached through a different
        // calendar and a different era.
        let system = THAILAND.government(2568).unwrap();
        assert_eq!(
            system.calendar_months_of_quarter(Quarter::First).unwrap(),
            [10, 11, 12]
        );
        let q1 = system.quarter_span(2568, Quarter::First).unwrap();
        assert_eq!(q1.first, greg(2024, 10, 1));
        assert_eq!(q1.last, greg(2024, 12, 31));
    }

    #[test]
    fn the_iranian_year_is_its_own_calendar_year_but_not_the_gregorian_one() {
        // The distinction the two predicates exist to draw.
        let system = IRAN.government(1403).unwrap();
        assert!(system.is_calendar_year());
        assert!(!system.is_gregorian_calendar_year());
        assert!(!IRAN.has_no_offset());
        // Germany is the other way about: both are true.
        let germany = GERMANY.government(2024).unwrap();
        assert!(germany.is_calendar_year());
        assert!(germany.is_gregorian_calendar_year());
    }

    #[test]
    fn a_documented_gap_says_what_it_knows_and_why_it_declines() {
        let nepal = gap_for("NP").unwrap();
        assert!(nepal.description.contains("Shrawan"));
        assert!(nepal.reason.contains("Bikram Sambat"));
        assert!(!nepal.source.is_empty());
        for gap in GAPS {
            assert_eq!(gap.code.len(), 2);
            assert!(by_code(gap.code).is_none(), "{} is both", gap.code);
        }
    }

    #[test]
    fn the_hong_kong_and_singapore_years_are_the_april_shape_too() {
        for profile in [&HONG_KONG, &SINGAPORE] {
            let system = profile.government(2025).unwrap();
            assert_eq!(system.label, LabelConvention::LabelledByStartYear);
            assert_eq!(system.span(2025).unwrap().first, greg(2025, 4, 1));
        }
        // Singapore's personal tax basis period is the calendar year, which
        // is a different year in the same country.
        let tax = SINGAPORE.in_force(SystemKind::PersonalTax, 2025).unwrap();
        assert!(tax.is_gregorian_calendar_year());
    }

    #[test]
    fn the_second_half_of_a_japanese_fiscal_year_begins_in_october() {
        use crate::quarters::Half;

        let system = JAPAN.government(2024).unwrap();
        let second = system.half_span(2024, Half::Second).unwrap();
        assert_eq!(second.first, greg(2024, 10, 1));
        assert_eq!(second.last, greg(2025, 3, 31));
        assert_eq!(system.half(greg(2024, 9, 30)).unwrap(), Half::First);
        assert_eq!(system.half(greg(2024, 10, 1)).unwrap(), Half::Second);
    }

    #[test]
    fn every_system_in_the_tables_converts_a_day_in_its_own_range() {
        // A sweep that touches every entry, including the historical ones,
        // to prove no table row is unreachable.
        for profile in ALL {
            for system in profile.systems {
                // Pick a label the system actually covers: its first, or
                // 2024 clamped into its range for the open-ended ones.
                let label = system
                    .valid_from
                    .unwrap_or(2024)
                    .min(system.valid_until.unwrap_or(i64::MAX));
                let span = system.span(label).unwrap_or_else(|error| {
                    panic!("{} {} at {label}: {error:?}", profile.code, system.name)
                });
                assert!(span.days() > 300, "{} {}", profile.code, system.name);
                assert_eq!(system.label_at(span.first).unwrap(), label);
            }
        }
    }
}
