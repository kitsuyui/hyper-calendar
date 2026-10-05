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

mod more;

pub use more::*;

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
///
/// # Before 1947
///
/// The National Archives of Japan's account of the 会計年度 puts the April
/// year at 明治19年度 (1886): 明治8年度 (1875) to 明治17年度 (1884) ran from
/// July, 明治18年度 was the nine months of 1 July 1885 to 31 March 1886, and
/// 明治19年度 began on 1 April 1886. The 会計法 of 1889 states the April year
/// and the 財政法 of 1947 does again; the 会計法 revised in 1921 was not
/// read, so 1921 to 1946 is a gap, not a year without a fiscal year. The July
/// system is carried as a system of its own; the calendar-year and October
/// years of 明治2 to 明治7 are not (the page does not say how the year of
/// 明治7 passed into the July year).
pub static JAPAN: FiscalProfile = FiscalProfile {
    code: "JP",
    english_name: "Japan",
    systems: &[
        YearSystem {
            name: "Japanese national fiscal year, July basis",
            local_name: "会計年度",
            kind: SystemKind::Government,
            authority: Authority::Unread,
            start: YearStart::gregorian(7, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1875),
            valid_until: Some(1884),
            read_from: 1875,
            unread: &[],
            note: "明治8年度 (1875) to 明治17年度 (1884), the year matched to the times land tax was \
                   paid, from the National Archives of Japan's account (the instrument was not \
                   read). 明治18年度 was the nine months of 1 July 1885 to 31 March 1886 and is in \
                   no system, so label 1885 is in neither this one nor the April system. \
                   The years labelled 明治 N are carried by the Gregorian year they begin in, \
                   1867 + N.",
        },
        YearSystem {
            name: "Japanese national fiscal year",
            local_name: "会計年度",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1886),
            valid_until: None,
            read_from: 1886,
            unread: &[(1921, 1946)],
            note: "The April year began with 明治19年度 (1 April 1886), per the National Archives \
                   of Japan; the instrument that set it (a 公文類聚 item 会計年度ヲ改定ス, and an \
                   1884 太政官布達 per one reference answer) was not read. 1886 to 1889 rest on \
                   that account, 1890 to 1920 on the 会計法 of 1889 (明治22年法律第4号, read in \
                   Nagoya University's database), and from 1947 on the 財政法 (in force \
                   1 April 1947, read in e-Gov). The 会計法 was wholly revised in 1921 and the \
                   revision was not read, so 1921 to 1946 are not carried.",
        },
        crate::academic::JAPAN_SCHOOL_YEAR,
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "財政法（昭和22年法律第34号）第11条 and 附則 (e-Gov 法令検索, API); 会計法（明治22年法律第4号）\
              (Nagoya University 法令データベース); 国立公文書館「あの日の公文書」(会計年度の変遷); \
              地方自治法第208条; 学校教育法施行規則（昭和22年文部省令第11号）第59条 (e-Gov), all \
              read 2026-10-04",
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
///
/// The Ministry of Finance's own Arthapedia says the April year was fixed in
/// 1867 to conform to English practice, and a reference work says the year
/// before that ran from 1 May to 30 April. No Act or order of 1867 was
/// found, nor the dates of the first April year, so the system is
/// established in 1867 and read from 1868, and the May year is not carried.
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
        valid_from: Some(1867),
        valid_until: None,
        read_from: 1868,
        unread: &[],
        note: "Written officially as a span leading with the start year (FY 2024-25). Indian \
               market shorthand \"FY25\" means the same year but names it by its end, so the \
               short form and the official form disagree by one; this entry follows the \
               official form. Arthapedia (Ministry of Finance) dates the April year to 1867 \
               and gives no instrument; whether a short year preceded the first April year is \
               not stated, so 1867 is a gap and the first whole year read is 1868. The earlier \
               1 May to 30 April year (a reference work's statement, labels not given) is not \
               carried.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "General Clauses Act, 1897, s. 3(21) (IBC Laws and Casemine transcriptions; the \
              Act's date is inconsistent across them); Arthapedia (Indian Economic Service, \
              Ministry of Finance), \"Financial Year (FY) or Fiscal Year\"; Wikipedia, \"Fiscal \
              year\", for the earlier May year (secondary), all read 2026-10-04",
};

/// Hong Kong 🇭🇰 — 1 April.
///
/// The government financial year and the Inland Revenue Department's year of
/// assessment both run 1 April to 31 March, written "2025/26" after the year
/// they begin in. The year of assessment is the older of the two on the
/// pages read: the Inland Revenue Ordinance of 1947 fixes it from 1 April 1947.
pub static HONG_KONG: FiscalProfile = FiscalProfile {
    code: "HK",
    english_name: "Hong Kong",
    systems: &[
        YearSystem {
            name: "Hong Kong financial year",
            local_name: "財政年度",
            kind: SystemKind::Government,
            authority: Authority::Unread,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1841),
            valid_until: None,
            read_from: 2025,
            unread: &[],
            note: "The statutory definition is in the Public Finance Ordinance (Cap. 2) s. 2, \
                   which could not be read (commenced 1 April 1983, per the Historical Laws \
                   of Hong Kong Online); the Budget's own page for 2025-26 and Wikipedia's \
                   \"Fiscal year\" give the dates, and nothing read says what the year was \
                   before 2025. Britain occupied the island on 25 January 1841 (Wikipedia, \
                   \"History of Hong Kong\"), the earliest government of Hong Kong, so every \
                   label from 1841 to 2024 is a gap and the years before 1841 are absent.",
        },
        YearSystem {
            name: "Hong Kong year of assessment",
            local_name: "課稅年度",
            kind: SystemKind::PersonalTax,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1841),
            valid_until: None,
            read_from: 1947,
            unread: &[],
            note: "The Inland Revenue Ordinance, 1947 (assented 2 May 1947) takes the year of \
                   assessment as the twelve months commencing on 1 April 1947 and each \
                   following 1 April, read as Wikisource transcribes it; the Inland Revenue \
                   Department names the year \"2026/27\" after the year it begins in. Whether \
                   the text has been amended since was not read. No page read says when the \
                   tax was first assessed, only that the island was occupied from 1841, so \
                   every label from 1841 to 1946 is a gap and the years before 1841 are \
                   absent.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Inland Revenue Ordinance, 1947 (Wikisource); Inland Revenue Department, \"2026-27 \
              Budget\"; The 2025-26 Budget, Public Finance (budget.gov.hk); Public Finance \
              Ordinance (Cap. 2) s. 2 (not read; commencement from Historical Laws of Hong \
              Kong Online); Wikipedia, \"Fiscal year\" and \"History of Hong Kong\" (secondary), all read \
              2026-10-04",
};

/// Singapore 🇸🇬 — 1 April for the state, the calendar year for personal tax.
///
/// The Financial Procedure Act 1966 defines the financial year as the twelve
/// months ending on 31 March. Personal income tax is assessed on the
/// calendar year, in the Year of Assessment that follows it — a genuinely
/// different year in the same country, which is why both are listed.
///
/// The Act's text could not be read (the statute site refused it), so the
/// two years rest on secondary pages of 2026 and are read from 2026.
pub static SINGAPORE: FiscalProfile = FiscalProfile {
    code: "SG",
    english_name: "Singapore",
    systems: &[
        YearSystem {
            name: "Singapore government financial year",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Unread,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            read_from: 2026,
            unread: &[],
            note: "Financial Procedure Act 1966 s. 2 (a legal-wires page confirms that it \
                   defines the term and shows a commencement note of 9 August 1965; Wikipedia \
                   gives the dates). The Act's wording was not read, nor how the year was \
                   named, so the start-year label is the table's, and nothing earlier than \
                   2026 is carried.",
        },
        YearSystem {
            name: "Singapore basis period for personal income tax",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Unread,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            read_from: 2026,
            unread: &[],
            note: "Income of calendar year Y is assessed in Year of Assessment Y+1. This entry \
                   is the basis period, not the Year of Assessment. Read from Wikipedia's \
                   \"Income tax in Singapore\" only; no page of the Inland Revenue Authority \
                   could be read, and no source read gives the first Year of Assessment.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Financial Procedure Act 1966 s. 2 (legal-wires.com transcription, wording not \
              shown; the Singapore Statutes Online page was refused); Wikipedia, \"Income tax \
              in Singapore\" and \"Fiscal year\" (secondary), read 2026-10-04",
};

/// Thailand 🇹🇭 — 1 October, named after the Buddhist Era year it **ends**
/// in, and moved three times.
///
/// The Pridi Banomyong Institute's account of the budget year, read for
/// this table, gives the history: the year began on 1 April before B.E. 2481;
/// the Budget Procedure Act (4th amendment) was approved on 27 January
/// B.E. 2481 for a year from 1 October, begun in B.E. 2482 by that page and
/// in B.E. 2481 by the Thai Wikipedia; the civil new year moved to
/// 1 January from B.E. 2484 and a 6th amendment made the budget year the
/// calendar year; and the Budget Procedure Act B.E. 2502 brought back
/// 1 October, with ปีงบประมาณ 2505 beginning on 1 October B.E. 2504 (1961).
/// The Budget Procedure Act B.E. 2561 (2018) carries the October year today;
/// its wording and date in force are in a Royal Gazette PDF that was not
/// read, so the table does not quote its naming rule. The page read names
/// 2505 for the year from 1 October 2504, which is the year it ends in.
///
/// Labels here are **Buddhist Era** years.
///
/// The two systems carried leave the years in between in none: B.E. 2504
/// can only have run from 1 January to 30 September, nine months, if 2505
/// began on 1 October 2504 and the calendar year came before it (two stated
/// facts; no page gives the length), and B.E. 2483's budget year is not
/// dated by any page read (the civil year ran 1 April to 31 December 1940).
pub static THAILAND: FiscalProfile = FiscalProfile {
    code: "TH",
    english_name: "Thailand",
    systems: &[
        YearSystem {
            name: "Thai budget year",
            local_name: "ปีงบประมาณ",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::new(StartCalendar::THAI_BUDDHIST, 10, 1),
            label: LabelConvention::LabelledByEndYear,
            valid_from: Some(2505),
            valid_until: None,
            read_from: 2505,
            unread: &[],
            note: "Labels are Buddhist Era years (B.E. = CE + 543). ปีงบประมาณ 2505 began on \
                   1 October 2504 under the Budget Procedure Act B.E. 2502 (Pridi Banomyong \
                   Institute); the year is named for the B.E. year it ends in, so ปีงบประมาณ \
                   2568 began 1 October 2024. The 2561 Act's text was not read. An earlier \
                   October year (B.E. 2481 or 2482, the pages disagree) is not carried: neither \
                   page gives its labels or dates.",
        },
        YearSystem {
            name: "Thai budget year, calendar-year period",
            local_name: "ปีงบประมาณ",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::new(StartCalendar::THAI_BUDDHIST, 1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(2484),
            valid_until: Some(2503),
            read_from: 2484,
            unread: &[],
            note: "The 6th amendment of the Budget Procedure Act made the year 1 January to \
                   31 December from B.E. 2484 (1941), when the civil new year moved to \
                   1 January. B.E. 2504 is in neither this system nor the October one: it can \
                   only have been the nine months from 1 January to 30 September 2504, and the \
                   table does not carry a nine-month year. B.E. 2483 (the civil year ran 1 \
                   April to 31 December 1940) is not carried: no page read dates its budget \
                   year.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Pridi Banomyong Institute, \"วันขึ้นปีใหม่ : วันเริ่มต้นปีงบประมาณในอดีต\" (2022); \
              Thai Wikipedia, \"ปีงบประมาณ\" and English Wikipedia, \"Thai solar calendar\" \
              (secondary), read 2026-10-04; พระราชบัญญัติวิธีการงบประมาณ พ.ศ. 2561 s. 4 (not read: \
              a Royal Gazette PDF)",
};

/// Australia 🇦🇺 — 1 July, for the budget and the income year alike.
///
/// The Treasury's reporting-periods page gives a standard income year from
/// 1 July to 30 June, and the Commonwealth budget uses the same year.
/// Written "2024-25" after the year it begins in; the shorthand "FY25"
/// names the same year by its end, which is a habit rather than a
/// definition. The Income Tax Assessment Act 1997 s. 995-1, which defines a
/// financial year, could not be read (its pages gave only a table of
/// contents).
///
/// Wikipedia's account, citing Arndt (1990, not read), is that the
/// Commonwealth has used the 30 June year-end since its inception in 1901
/// and that the colonies moved to it from a calendar year in Victoria 1870,
/// South Australia 1874, Queensland 1875, Western Australia 1892, New South
/// Wales 1895 and Tasmania 1904. The table carries the Commonwealth's year
/// from the first whole year after 1901; the colonies' own years are
/// subnational and are not carried.
pub static AUSTRALIA: FiscalProfile = FiscalProfile {
    code: "AU",
    english_name: "Australia",
    systems: &[YearSystem {
        name: "Australian financial year",
        local_name: "",
        kind: SystemKind::Government,
        authority: Authority::Unread,
        start: YearStart::gregorian(7, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: Some(1901),
        valid_until: None,
        read_from: 1902,
        unread: &[],
        note: "The budget year and the income year coincide, so one entry covers both. No \
               instrument was read: the Treasury's page and Wikipedia (secondary, citing \
               Arndt 1990) state the 1 July year, and Wikipedia says the Commonwealth has used \
               it since 1901. The first Commonwealth year's dates are not given, so 1901 is \
               not carried and the first whole year read is 1902 (the one beginning \
               1 July 1902; label 1902). The Commonwealth began in 1901, so 1901 is a gap \
               and every label before it is absent.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Commonwealth Treasury, \"Reporting periods\" (treasury.gov.au); Wikipedia, \
              \"Australian financial year\" and \"Fiscal year\" (secondary, citing Arndt, \"The \
              financial year\", 1990, not read); Income Tax Assessment Act 1997 s. 995-1 (not \
              read), all read 2026-10-04",
};

/// New Zealand 🇳🇿 — two years, and they are labelled in opposite
/// directions.
///
/// The Crown financial year runs 1 July to 30 June (Public Finance Act 1989
/// s. 2, which could not be read; Wikipedia's "Fiscal year" gives the dates)
/// and is written "2024/25" after the year it begins in. The tax year for
/// individuals runs 1 April to 31 March and Inland Revenue names it after the
/// year it **ends** in: "the 2025 tax year" is 1 April 2024 to 31 March 2025
/// (the page of Inland Revenue read gives these dates and not the naming,
/// which rests on a search result of its pages).
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
            authority: Authority::Unread,
            start: YearStart::gregorian(7, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            read_from: 2026,
            unread: &[],
            note: "The move to 1 July is attributed to the Public Finance Act 1989, but no \
                   page read gives the date of the change, the year it replaced or the label \
                   of the first July year, and the Act could not be read: the table does not \
                   say when it began and answers from 2026, the year of the one page that \
                   states it.",
        },
        YearSystem {
            name: "New Zealand tax year",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Unread,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByEndYear,
            valid_from: None,
            valid_until: None,
            read_from: 2025,
            unread: &[],
            note: "Inland Revenue names the tax year after the calendar year it ends in, the \
                   opposite of the Crown financial year in the same country: that naming rests \
                   on a search result of Inland Revenue's pages, since the page read, of \
                   2026, gives the dates (a bill for the \"1 April 2024 to 31 March 2025 tax \
                   year\") and does not say how the year is named. The earliest year it \
                   shows is 2025; the Income Tax Act's own first year was not found.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Inland Revenue, \"Timelines at the end of the tax year\" (ird.govt.nz); Wikipedia, \
              \"Fiscal year\" (secondary), for the Crown year; Public Finance Act 1989 s. 2 \
              (not read: legislation.govt.nz refused it), read 2026-10-04",
};

/// Pakistan 🇵🇰 — 1 July.
pub static PAKISTAN: FiscalProfile = FiscalProfile {
    code: "PK",
    english_name: "Pakistan",
    systems: &[YearSystem {
        name: "Pakistani financial year",
        local_name: "",
        kind: SystemKind::Government,
        authority: Authority::Unread,
        start: YearStart::gregorian(7, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        read_from: 2026,
        unread: &[],
        note: "A change from an April start around 1959 is sometimes asserted; no page read \
               says it either way, so no earlier system is listed. The dates come from \
               Wikipedia's \"Fiscal year\" (secondary) and the Finance Division's site shows \
               the budgets of 2024-25 to 2026-27 in the span form that leads with the start \
               year; Constitution art. 260 could not be read.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Finance Division, Government of Pakistan (finance.gov.pk), budget titles; \
              Wikipedia, \"Fiscal year\" (secondary), read 2026-10-04",
};

/// China 🇨🇳 — the calendar year, by statute.
///
/// Budget Law of the People's Republic of China: the budgetary year begins
/// on 1 January and ends on 31 December of the Gregorian calendar — article
/// 10 in the law of 22 March 1994 (in force 1 January 1995) and article 18
/// since the amendment of 2014. The State Council's regulation on the
/// management of the state budget of 1991 (Order No. 90, article 10, in
/// force 1 January 1992) said the same, so the first year read is 1992.
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
        read_from: 1992,
        unread: &[],
        note: "The Budget Law names the Gregorian calendar explicitly (公历), so the lunisolar \
               new year has no bearing on it. The regulation of 1991 (in force 1 January 1992) \
               and the Budget Law of 1994 (1 January 1995) were read in Wikisource's \
               transcriptions; the regulation of 1951 that the 1991 one repealed was not \
               found, and nothing read says what the year was before 1992.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "中华人民共和国预算法 (1994, arts. 10 and 79; as amended 2014 and 2018, art. 18); \
              国家预算管理条例 (1991, arts. 10 and 78), as zh.wikisource.org transcribes them, \
              read 2026-10-04",
};

// ── The Americas ────────────────────────────────────────────────────────

/// United States 🇺🇸 — 1 October, labelled by the year it **ends** in, 1 July
/// before that, and the calendar year before that.
///
/// 31 U.S.C. § 1102: "The fiscal year of the Treasury begins on October 1 of
/// each year and ends on September 30 of the following year." The
/// Congressional Budget Office states the labelling rule as plainly as one
/// could wish: federal fiscal years "are designated by the calendar year in
/// which they end", so FY 2021 began on 1 October 2020.
///
/// # The 1976 change, and the quarter that belongs to no year
///
/// Before fiscal year 1977 the federal year ran 1 July to 30 June. The
/// Congressional Budget and Impoundment Control Act of 1974 (Pub. L. 93-344),
/// Title V § 501, moved it to October, first effective for FY1977 beginning
/// 1 October 1976.
///
/// That leaves **1 July to 30 September 1976** in no fiscal year at all:
/// FY1976 had ended on 30 June and FY1977 had not begun. Congress legislated
/// for it separately, as the *transition quarter*, under the Fiscal Year
/// Transition Act (Pub. L. 94-274). This table reproduces the hole rather
/// than papering over it — [`FiscalProfile::at`] returns `None` for any day
/// in it, which is the correct answer and the reason validity ranges are
/// part of [`YearSystem`] rather than a footnote.
///
/// # The 1842 change, and the half year before it
///
/// The year was the calendar year until the Act of 26 August 1842, which
/// began the July year. The Treasury's letter of 15 December 1842 asks for
/// estimates for "the half calendar year ending 30th June, 1843" and "the
/// fiscal year ending 30th June, 1844": so the first July year is FY1844,
/// 1 July 1843 to 30 June 1844, and 1 January to 30 June 1843 belongs to
/// neither system. The Act's own text was not read. The calendar-year system
/// is carried to 1842 and read only for 1842 (a Treasury report on "the
/// first half of the year 1842"): the label of its first year, and of every
/// year before 1842, is a gap.
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
            read_from: 1977,
            unread: &[],
            note: "FY1977 began 1 October 1976, the first year on the October basis.",
        },
        YearSystem {
            name: "United States federal fiscal year, July basis",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(7, 1),
            label: LabelConvention::LabelledByEndYear,
            valid_from: Some(1844),
            valid_until: Some(1976),
            read_from: 1844,
            unread: &[],
            note: "The July basis dates from the Act of 26 August 1842 (not read; the \
                   Treasury's reports cite it). The first July year is FY1844, 1 July 1843 to \
                   30 June 1844, by the Treasury's letter of 15 December 1842; 1 January to \
                   30 June 1843 is a \"half calendar year\" in neither system. 1 July to \
                   30 September 1976 — the transition quarter — is covered by neither this \
                   system nor the October one (not carried; the Fiscal Year Transition \
                   Act, Pub. L. 94-274, gives it no label).",
        },
        YearSystem {
            name: "United States federal fiscal year, calendar-year basis",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Unread,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1789),
            valid_until: Some(1842),
            read_from: 1842,
            unread: &[],
            note: "The year before the Act of 26 August 1842 was the calendar year (the \
                   Congressional Research Service: \"initially aligned with the calendar \
                   year\"). No page read gives the year it began or names a year before 1842: \
                   1842 is read from a Treasury report on \"the first half of the year 1842\", \
                   and every year from 1789, the year the Treasury Department was \
                   established (the Act of 2 September 1789, per Wikipedia), is a gap; the \
                   years before it are absent. The label is the calendar year, which \
                   start and end convention share.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "31 U.S.C. § 1102 and its source notes (law.cornell.edu); Congressional Research \
              Service, 98-325, The Federal Fiscal Year; Treasury, H. Doc. 27-16 (15 Dec 1842) \
              and S. Doc. 27-371 (govinfo metadata pages); Budget and Impoundment Control Act \
              of 1974, Pub. L. 93-344 tit. V § 501; Fiscal Year Transition Act, Pub. L. 94-274; \
              GAO-05-734SP, A Glossary of Terms Used in the Federal Budget Process; CBO, \
              Common Budgetary Terms Explained; Wikipedia, \"Fiscal year\" and \"United States \
              Department of the Treasury\" (secondary, for 1789), read 2026-10-05",
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
/// Accounts title. No page read this time shows either title.
///
/// The year ran 1 July to 30 June until 1906: *House of Commons Procedure
/// and Practice* says "Until 1906, the fiscal year ran from July 1 to
/// June 30", and the Annotated Standing Orders say the beginning of the
/// fiscal year "was changed from July 1 to April 1" in 1906. A search
/// result's summary of a Library of Parliament page that could not be opened
/// gives a nine-month year from 1 July 1906 to 31 March 1907; the table does
/// not rest on it, and begins at the first whole April year, 1907-08. The
/// July years are not carried: no page read gives the labels the Public
/// Accounts used for them, or the year the July year began.
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
        valid_from: Some(1907),
        valid_until: None,
        read_from: 1907,
        unread: &[],
        note: "Estimates and Budget name the year by its start (\"2025-26\"); the Public \
               Accounts name the same year by its end (\"Public Accounts of Canada 2025\"). \
               One country, one fiscal year, two published labels. The year began on 1 April \
               in 1906 (House of Commons Procedure and Practice; Annotated Standing Orders); \
               the first whole year is 1907-08, label 1907, and 1906 was the transition.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Financial Administration Act, RSC 1985, c. F-11, s. 2 (laws-lois.justice.gc.ca); \
              House of Commons Procedure and Practice, 3rd ed., ch. 18 n. 9 and Annotated \
              Standing Orders n. 10 to S.O. 28(2) (ourcommons.ca); Treasury Board of Canada \
              Secretariat, Main Estimates; Receiver General, Public Accounts of Canada, read \
              2026-10-04 where they could be",
};

/// Brazil 🇧🇷 — the calendar year, by statute.
///
/// Lei nº 4.320 de 17 de março de 1964, art. 34: "O exercício financeiro
/// coincidirá com o ano civil." The Senate's text counts the law's effects
/// from 1 January 1964 (art. 114, as a summarising read of the page gave
/// it), although it was published only on 23 March 1964 and the vetoes were
/// settled in May, so the first exercise wholly after publication, 1965, is
/// the first year read.
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
        read_from: 1965,
        unread: &[],
        note: "The exercise before 1964 is not carried: the Código de Contabilidade da União \
               (Decreto 4.536 of 1922) is the likely earlier instrument and was not read.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Lei nº 4.320/1964 art. 34 (Senado Federal), with its publication data from \
              LexML, read 2026-10-04",
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
            valid_from: Some(1854),
            valid_until: None,
            read_from: 1855,
            unread: &[],
            note: "HM Treasury writes it as a span (\"2024-25\"); the Corporation Tax Act 2010 \
                   s. 1119 names the same dates by the starting year alone. The year ending on \
                   31 March is established by the Public Revenue and Consolidated Fund Charges \
                   Act 1854 (17 & 18 Vict. c. 94, assented 10 August 1854), which from \
                   1 April 1854 lets the Treasury, \"if they shall see fit\", have the annual \
                   accounts made up to 31 March instead of 5 January. The Act obliges nothing, \
                   and the first year a page read reports as ending on 31 March is 1855-56 \
                   (Hansard, 19 May 1856), so 1854 is a gap and the table is read from 1855; \
                   the Exchequer and Audit Departments Act 1866 (in force 1 April 1867) \
                   restates the year. The accounts of the year to 5 April that Gladstone's \
                   budget speeches of 1854 use are not carried: no page read gives the year \
                   they began or its instrument.",
        },
        YearSystem {
            name: "United Kingdom personal tax year, Pitt's income tax",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Unread,
            start: YearStart::gregorian(4, 6),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1799),
            valid_until: Some(1801),
            read_from: 1802,
            unread: &[],
            note: "Wikipedia (secondary; the Acts' own texts were PDF only): income tax was \
                   announced in December 1798, introduced in 1799 and levied from 1799 to \
                   1802, when Addington abolished it. The first Act's year ran to 5 April \
                   1800, but no page read gives the dates of each year of this tax, so every \
                   label from 1799 to 1801 is a gap. The tax lapsed in 1802-03: that year, and \
                   every one before 1799, is absent.",
        },
        YearSystem {
            name: "United Kingdom personal tax year, Addington's income tax",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Unread,
            start: YearStart::gregorian(4, 6),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1803),
            valid_until: Some(1815),
            read_from: 1816,
            unread: &[],
            note: "Wikipedia (secondary): Addington reintroduced the tax in 1803 when \
                   hostilities recommenced, and it was abolished again in 1816, one year after \
                   Waterloo. No page read gives the dates of its years, so every label from \
                   1803 to 1815 is a gap; from 1816 to 1841 the tax lapsed and the labels are \
                   absent.",
        },
        YearSystem {
            name: "United Kingdom personal tax year",
            local_name: "",
            kind: SystemKind::PersonalTax,
            authority: Authority::Statute,
            start: YearStart::gregorian(4, 6),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1842),
            valid_until: None,
            read_from: 2007,
            unread: &[],
            note: "Income Tax Act 2007 s. 4, read as revised on legislation.gov.uk: the tax \
                   year 2007-08 is the one beginning 6 April 2007. The 6 April start descends \
                   from the 1752 calendar reform; see this profile's documentation for what \
                   is and is not evidenced. Wikipedia (secondary) says Peel reintroduced the \
                   income tax in the Income Tax Act 1842, the beginning of the permanent \
                   tax, so 1842 is the first label of this system and every year from 1842 to \
                   2006 is a gap: no earlier text of the tax year's definition was read.",
        },
        YearSystem {
            name: "England and Wales legal and fiscal year, before 1752",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Unread,
            start: YearStart::gregorian(3, 25),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1155),
            valid_until: Some(1750),
            read_from: 1155,
            unread: &[],
            note: "Lady Day, 25 March. Wikipedia's articles on the Old Style and New Style \
                   dates and on Lady Day (secondary; the Catholic Encyclopedia and the 1750 \
                   Act stand behind them) say the English legal year began on 25 March from \
                   1155 to 1752; no primary page was read for 1155 or for a fiscal year of \
                   that date. The dates here are proleptic Gregorian, which is not what \
                   anyone wrote at the time: for the Julian dates actually used in Britain \
                   before September 1752, use hc_calendars_solar::julian_gregorian with the \
                   `gb` adoption. The range stops at 1750 because 1751 and 1752 were both \
                   transitional — the Calendar (New Style) Act 1750 moved the legal new year \
                   to 1 January from 1752, so the English year 1751 ran only from 25 March to \
                   31 December 1751 (282 days), and 1752 then lost eleven days in \
                   September. Neither transitional year is modelled. Listed to show where \
                   6 April came from, not as a practical calendar.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Public Revenue and Consolidated Fund Charges Act 1854 ss. 1 and 2 and Exchequer \
              and Audit Departments Act 1866 (legislation.gov.uk, as enacted); Interpretation \
              Act 1978 Sch. 1; Income Tax Act 2007 s. 4 (legislation.gov.uk); Corporation Tax \
              Act 2010 s. 1119; Calendar (New Style) Act 1750 (24 Geo. 2 c. 23) ss. 1 and 6 \
              (Wikisource); Wikipedia, \"Old Style and New Style dates\" and \"Lady Day\" \
              (secondary), read 2026-10-04, and \"Income tax in the United Kingdom\" \
              (secondary, for 1799, 1802, 1803, 1816 and 1842), read 2026-10-05",
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
        read_from: 1970,
        unread: &[],
        note: "§ 4 BHO lets the Federal Ministry of Finance set a different year for individual \
               areas; this entry is the general rule, not a claim that no exception exists. \
               The BHO and the HGrG are both of 19 August 1969 and in force from 1 January \
               1970; what the year was before is not carried: no page read gives it (the \
               Reichshaushaltsordnung of 1922, in force 1923 to 1969, was not found).",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Bundeshaushaltsordnung § 4; Haushaltsgrundsätzegesetz § 4 (gesetze-im-internet.de), \
              read 2026-10-04",
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
        read_from: 2006,
        unread: &[],
        note: "A période complémentaire of at most twenty days allows operations to be booked \
               to the closed year. It does not extend the year and is not modelled. The LOLF \
               repealed the ordonnance n° 59-2 of 2 January 1959 from 1 January 2005 \
               (Conseil constitutionnel, decision 2001-448 DC) and applied to the budget of \
               2006 in full; the ordonnance's own year was not read, so earlier years are not \
               carried.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Loi organique n° 2001-692 du 1er août 2001 (LOLF) art. 1 (Légifrance); Conseil \
              constitutionnel, décision n° 2001-448 DC; Wikipedia, \"Loi organique relative \
              aux lois de finances\" (secondary), read 2026-10-04",
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
        read_from: 2000,
        unread: &[],
        note: "The three-year budget horizon is a budgeting practice; the financial year itself \
               is the calendar year. The Budget Code (145-FZ of 31 July 1998) is in force from \
               1 January 2000; the page's wording of article 12 differed between two reads, so \
               it is not quoted. Russian Wikipedia says the Soviet economic year ran from \
               1 October until a decree of 1930 and that the calendar year has applied since \
               1931, and an encyclopaedia says the Empire's budgets were calendar-year from \
               1862; both are secondary and give no year labels, so neither is carried.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Бюджетный кодекс Российской Федерации (145-ФЗ) art. 12 (consultant.ru); Russian \
              Wikipedia, \"Финансовый год\" and \"Бюджетный кодекс\" (secondary), read 2026-10-04",
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
/// The budget year is fixed by the Riksdag Act, which is a statute, not a
/// convention: the Constitutional Committee's report of 1993/94 (KU 18)
/// quotes "3 kap. 2 § RO" as "börjar budgetåret den 1 juli" and proposes the
/// calendar year from 1997, and the Riksdag Act of 2014 says "Budgetåret
/// sammanfaller med kalenderåret". The instrument that moved the year in 1923
/// was not read: Proposition 1922:1 is read as a title only, which shows
/// that the Riksdag treated 1 January to 30 June 1923 as a period of its
/// own.
///
/// This table carries the three full systems. Not carried: the two
/// transitional periods, the half year of 1923 and the eighteen-month
/// 1995/96 (not yet done; no page read gives the label either takes). They
/// lie outside all three systems, so a caller asking about 1996 gets `None`
/// rather than a confident wrong answer.
///
/// Companies are separate: Bokföringslag (1999:1078) 3 kap. 1 § makes the
/// calendar year the *räkenskapsår* for natural persons and lets other
/// undertakings keep a *brutet räkenskapsår*. It names no months. The months
/// 1 May, 1 July and 1 September are in the earlier Bokföringslag (1976:125)
/// § 12, in force from 1 January 1977, which the 1999 law replaced.
pub static SWEDEN: FiscalProfile = FiscalProfile {
    code: "SE",
    english_name: "Sweden",
    systems: &[
        YearSystem {
            name: "Swedish state budget year, calendar basis",
            local_name: "budgetår",
            kind: SystemKind::Government,
            authority: Authority::Unread,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: Some(1922),
            read_from: 1921,
            unread: &[],
            note: "Before 1923 the state budget was for a calendar year: the Riksdag's \
                   propositions are titled \"under år 1921\" (Prop. 1920:1), as the data.riksdagen.se \
                   listing shows. 1922 is the year before the half year of 1 January to \
                   30 June 1923 (Prop. 1922:1); neither proposition was read, only their \
                   titles. When the calendar-year budget began is not stated in anything \
                   read, so no year before 1921 is carried.",
        },
        YearSystem {
            name: "Swedish state budget year, July basis",
            local_name: "budgetår",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(7, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1923),
            valid_until: Some(1994),
            read_from: 1923,
            unread: &[],
            note: "Riksdagsordningen (1974:153) 3 kap. 2 § has the budget year begin on \
                   1 July (quoted in KU 1993/94:18). Budget year 1923/24, 1 July 1923 to \
                   30 June 1924, is the first, by the Riksgäldskontoret's report for \
                   \"budgetåret 1923—1924\". Budget year 1995/96 was itself an eighteen-month \
                   transitional period (1 July 1995 to 31 December 1996) and is not \
                   modelled; nor is the six-month period of 1 January to 30 June 1923. \
                   1 July 1995 to 31 December 1996 therefore falls in no Swedish budget year \
                   here, which is the honest answer. The last twelve-month July year is \
                   1994/95, label 1994; the instrument of 1923 was not read.",
        },
        YearSystem {
            name: "Swedish state budget year",
            local_name: "budgetår",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1997),
            valid_until: None,
            read_from: 1997,
            unread: &[],
            note: "Riksdagsordningen (2014:801): \"Budgetåret sammanfaller med kalenderåret\"; \
                   before it Riksdagsordningen (1974:153) 3 kap. 2 §. Proposition 1994/95:100 \
                   puts the budget for 1995/96 over an eighteen-month period, and the \
                   Constitutional Committee's reports of 1993/94 and 1995/96 have the calendar \
                   year from 1997. The amendment that moved it was not found by number.",
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
            read_from: 1977,
            unread: &[],
            note: "Mandatory for natural persons and for partnerships taxed through one. Other \
                   undertakings may choose a brutet räkenskapsår, which this entry does not \
                   enumerate as systems of its own. Read from Bokföringslag (1976:125) § 12 \
                   (in force 1 January 1977), which names 1 May, 1 July and 1 September as the \
                   brutna years, and from Bokföringslag (1999:1078) 3 kap. 1 § (from \
                   1 January 2000), which names none. The law of 1929 it replaced was not read.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Riksdagsordning (1974:153) 3 kap. 2 § and Riksdagsordning (2014:801), via the \
              Riksdag's open data; Konstitutionsutskottet 1993/94:KU18 and 1995/96:KU21; Prop. \
              1994/95:100; Prop. 1922:1 (title); Riksgäldskontoret, report of 1925; \
              Bokföringslag (1976:125) § 12; Bokföringslag (1999:1078) 3 kap. 1-2 §§, read \
              2026-10-04",
};

// ── Africa and the Middle East ──────────────────────────────────────────

/// Egypt 🇪🇬 — 1 July since 1980, the calendar year before it.
///
/// Law No. 6 of 2022 (the Unified Public Finance Law), art. 1, as two news
/// pages quote it, defines the fiscal year as a calendar year beginning on
/// 1 July and ending on 30 June. The year was the calendar year before
/// 1980: the original article 2 of Law 53 of 1973 (issued 29 July 1973)
/// reads 1 January to 31 December, and Law 104 of 1980, in force on 30 May
/// 1980 (the page was refused; two search summaries and two pages read agree
/// that the change was made in 1980), replaced it with 1 July to 30 June.
/// So the table's July year is established in 1980, and the law of 1973 is
/// the calendar-year system's. The six months of 1 January to 30 June 1980 were a period of their own (the
/// 1980 budget ended on 30 June) and are in neither system.
pub static EGYPT: FiscalProfile = FiscalProfile {
    code: "EG",
    english_name: "Egypt",
    systems: &[
        YearSystem {
            name: "Egyptian fiscal year, calendar basis",
            local_name: "السنة المالية",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: Some(1979),
            read_from: 1974,
            unread: &[],
            note: "Law 53 of 1973 art. 2 as first issued (read on a legal aggregator's site, \
                   which does not show later amendments): the fiscal year is 1 January to \
                   31 December. The first whole year under it is 1974. A think tank's account \
                   of 1913 (1 April), 1926 (1 May) and 1946 (1 March) rests on no instrument \
                   read and is not carried. 1 January to 30 June 1980 was the last period of \
                   this system and is in neither it nor the July one.",
        },
        YearSystem {
            name: "Egyptian fiscal year",
            local_name: "السنة المالية",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(7, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: Some(1980),
            valid_until: None,
            read_from: 1980,
            unread: &[],
            note: "Written as a span (\"FY 2025/2026\"); no page read names the year by one \
                   number, so the start-year label is the span's. The July year began with \
                   1980/81, 1 July 1980 (Library of Congress country study and Al-Bawaba: the \
                   fiscal year changed in 1980 from the calendar year; Law 104 of 1980 itself \
                   was seen only in search summaries). Alignment with the calendar year has \
                   been discussed publicly more than once; no enacted change was found.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Law No. 6 of 2022 art. 1, as quoted by Al-Bawaba and Youm7; Law No. 53 of 1973 on \
              the State General Budget (lawyeregypt.net); Library of Congress country study, \
              Egypt, \"Public Finance\" (countrystudies.us); Al-Bawaba, \"Why does Egypt's fiscal \
              year start on 1 July?\"; Law No. 104 of 1980 (search summaries only), read \
              2026-10-04",
};

/// South Africa 🇿🇦 — 1 April for the state, 1 March for the individual.
///
/// The National Treasury's budget year runs 1 April to 31 March. The South
/// African Revenue Service's year of assessment for individuals runs
/// 1 March to the end of February and is named after the calendar year it
/// **ends** in: the 2026 tax year ran 1 March 2025 to 28 February 2026, as
/// SARS's page says in terms. Two systems, one month apart, labelled in
/// opposite directions.
///
/// The Public Finance Management Act 1 of 1999 took effect on 1 April 2000
/// (the National Treasury's page), but no page read quotes its definition of
/// a financial year, and the Exchequer Act of 1975 it replaced was not read;
/// the government year is read from 2026 only. The Income Tax Act 94 of 1983
/// (in force 13 July 1983, read on gov.za) fixes rates for individuals'
/// years of assessment ending on 29 February 1984 and 28 February 1985 while
/// others ended on 30 June 1984, so the individuals' year is read from
/// 1985, the first whole year the Act names.
pub static SOUTH_AFRICA: FiscalProfile = FiscalProfile {
    code: "ZA",
    english_name: "South Africa",
    systems: &[
        YearSystem {
            name: "South African government financial year",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Unread,
            start: YearStart::gregorian(4, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            read_from: 2026,
            unread: &[],
            note: "The Public Finance Management Act 1 of 1999 s. 1 definition could not be \
                   read (only a search result quoted it); the dates are from Wikipedia's \
                   \"Fiscal year\" and the National Treasury's Budget 2026 page, which names \
                   the budget by one year and does not state the label convention. The \
                   year before 2026 is not carried.",
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
            read_from: 1985,
            unread: &[],
            note: "Ends on the last day of February, so the year is 365 or 366 days and its \
                   final month is the one the leap day belongs to. The Income Tax Act 94 of \
                   1983 names the years of assessment ending 29 February 1984 and 28 February \
                   1985; whether the year to 29 February 1984 was a full year, and when \
                   individuals moved from a 30 June year-end, was not found.",
        },
    ],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "South African Revenue Service, \"Personal Income Tax\" (updated 2026-08-19); \
              Income Tax Act 94 of 1983 (gov.za); National Treasury, PFMA page and Budget \
              2026; Wikipedia, \"Fiscal year\" (secondary); Public Finance Management Act 1 of \
              1999 s. 1 (not read), read 2026-10-04",
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
/// [`StartCalendar::SOLAR_HIJRI_33`] and converted by the calendar,
/// not hard-coded as "about 21 March".
///
/// # What is approximated, and by how much
///
/// The official rule is astronomical: 1 Farvardin is the day on which the
/// March equinox falls before noon at the 52.5°E meridian. This entry goes
/// through the 33-year arithmetic rule, `persian-arithmetic-33`, which agrees
/// with that calendar on every Nowruz from AP 1178 to 1634 (AD 1799 to 2255):
/// the span Borkowski gives, as Heydari-Malayeri reports him, and one that
/// `hc-calendars-equinox` tests against its own equinox calendar `persian`.
/// So every year the statute covers (from AP 1366) up to AP 1634 starts on
/// the official day. After 1634 the rule is an extrapolation and the entry
/// claims nothing; `is_approximate` stays true for that reason.
///
/// The entry is not Birashk's 2 820-year cycle, `persian-arithmetic`. That
/// cycle starts AP 1404 on 20 March 2025 where Iran began it on 21 March
/// (the Solar Hijri calendar's correspondence table on Wikipedia,
/// `wikipedia-solar-hijri-calendar`). Tøndering names 1404 and 1437 as its
/// only two disagreements with an astronomical calendar between AP 1244 and
/// 1531, without saying which rule that calendar used; against `persian` of
/// `hc-calendars-equinox` the cycle differs in four years of that span:
/// 1404, 1437, 1470 and 1503.
///
/// A caller who needs the official date of a year after 1634 needs the
/// equinox calendar, `persian` of `hc-calendars-equinox`.
pub static IRAN: FiscalProfile = FiscalProfile {
    code: "IR",
    english_name: "Iran",
    systems: &[YearSystem {
        name: "Iranian fiscal year",
        local_name: "سال مالی",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::new(StartCalendar::SOLAR_HIJRI_33, 1, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        read_from: 1350,
        unread: &[],
        note: "Labels are Solar Hijri years. Computed through the 33-year arithmetic rule, \
               which agrees with the official equinox-based calendar on every Nowruz from \
               AP 1178 to AP 1634 and is an extrapolation after it. The Public Accounting Act \
               approved on 1366/06/01 (art. 6, read on three transcription sites) restates \
               the rule of the Act approved on 1349/10/15 (art. 5, in force from \
               1 Farvardin 1350, read on one site), which repealed the Act of 1312; the first \
               year read is therefore 1350. Neither is the year Iran began using this fiscal \
               year: the Acts of 1312 and 1289 (1911) say the year is \"one solar year\" in what \
               was read and name no start month, so 1349 and earlier are a gap.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "قانون محاسبات عمومی کشور art. 6 (approved 1366/06/01) and art. 5 and 98 of the \
              law approved 1349/10/15, as nezamat.ir, davoudabadi.ir and vindad.com transcribe \
              them; Wikipedia, \"Solar Hijri calendar\", correspondence table, for the year \
              starts; M. Heydari-Malayeri, \"A concise review of the Iranian calendar\", for the \
              33-year rule's span, read 2026-10-04",
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
        authority: Authority::Unread,
        start: YearStart::new(StartCalendar::ETHIOPIC, 11, 1),
        label: LabelConvention::LabelledByEndYear,
        valid_from: None,
        valid_until: None,
        read_from: 2014,
        unread: &[],
        note: "Labels are Ethiopic years. The draft budget proclamation for EFY 2014 (posted \
               14 June 2021) appropriates for the year \"commencing on Hamle 1, 2013 E.C. and \
               ending on Sene 30, 2014 E.C.\"; the Ministry of Finance's page gives the same \
               dates. Proclamation 648/2009 (648/2001 EC) is a PDF and was not read, so \
               nothing earlier than EFY 2014 is carried and no page says when Hamle 1 was \
               adopted.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "2014 (2021-22) Fiscal Year Federal Government Budget Draft Proclamation, as \
              chilot.wordpress.com posts it; Ministry of Finance (mofed.gov.et); Financial \
              Administration Proclamation No. 648/2009 (a PDF, not read), read 2026-10-04",
};

/// Nepal 🇳🇵 — 1 Shrawan to the last day of Asar, in the Bikram Sambat,
/// labelled by the year it **starts** in.
///
/// The Financial Procedures and Fiscal Responsibility Act, 2076, section
/// 2(e), as the Nepal Law Commission's English translation is quoted (the
/// translation is a PDF and was not opened for this table): the fiscal
/// year "starts from the first day of Shrawan of the current year and ends
/// on the last day of Ashad of the following year of Vikram Samvat". Shrawan
/// is the fourth month, so the year straddles two Bikram Sambat years and is
/// written with both — the budget of 2081/82 is for the year from Shrawan
/// 2081 — and it is labelled here by the first, as the United Kingdom's
/// 2024/25 is.
///
/// Its Gregorian start moves with the length of the Bikram Sambat months:
/// 16 July in 2024, 17 July in 2025 and 2026, from the months the
/// Government of Nepal gazettes. Outside the gazetted years 2080–2083 the
/// start is computed, and [`StartCalendar::BIKRAM_SAMBAT`] says by how
/// much that can be trusted.
///
/// With the `indic` feature, which brings the Bikram Sambat.
#[cfg(feature = "indic")]
pub static NEPAL: FiscalProfile = FiscalProfile {
    code: "NP",
    english_name: "Nepal",
    systems: &[YearSystem {
        name: "Nepali fiscal year",
        local_name: "आर्थिक वर्ष",
        kind: SystemKind::Government,
        authority: Authority::Unread,
        start: YearStart::new(StartCalendar::BIKRAM_SAMBAT, 4, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        read_from: 2083,
        unread: &[],
        note: "Labels are Bikram Sambat years, the first of the two the year is written with. \
               The months are the gazette's for 2080–2083 BS and computed otherwise. The Act \
               2076 s. 2(e) is a PDF and was not opened; \
               Wikipedia's \"Fiscal year\" and \"Economy of Nepal\" give 16 July to 15 July, so \
               the table is read from the year in progress when they were read, 2083/84, and \
               carries nothing earlier. Nepal's first annual budget, presented at the end of \
               2007 BS, covered March 1951 to February 1952, so the Shrawan year was not \
               always the year; when it began is not stated in anything read.",
    }],
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Wikipedia, \"Fiscal year\" and \"Economy of Nepal\" (secondary); myRepublica, \"Nepal's \
              budget history dates back to 1951\"; The Financial Procedures and Fiscal \
              Responsibility Act, 2076, section 2(e) (Nepal Law Commission English \
              translation, a PDF, not read); the Ministry of Home Affairs' public holiday \
              notices for 2080–2083 BS, for the month lengths, read 2026-10-04",
};

/// The codes of the countries whose table needs the `indic` feature, whose
/// calendar dates their year's start: [`ALL`] has them only when it is on.
/// A caller that asks for one in a build without it can say that the table
/// exists and is not built, which is not what a code no table has is.
pub const NEEDS_INDIC: &[&str] = &["NP"];

/// Every country table in this crate, in ISO 3166-1 alpha-2 order.
pub static ALL: &[&FiscalProfile] = &[
    &ANGOLA,
    &ARGENTINA,
    &AUSTRALIA,
    &BANGLADESH,
    &BAHRAIN,
    &BRAZIL,
    &BAHAMAS,
    &CANADA,
    &CHILE,
    &CHINA,
    &COLOMBIA,
    &COSTA_RICA,
    &CYPRUS,
    &CZECHIA,
    &GERMANY,
    &EGYPT,
    &SPAIN,
    &ETHIOPIA,
    &FIJI,
    &FRANCE,
    &UNITED_KINGDOM,
    &GHANA,
    &HONG_KONG,
    &HUNGARY,
    &INDONESIA,
    &ISRAEL,
    &INDIA,
    &IRAQ,
    &IRAN,
    &ICELAND,
    &ITALY,
    &JAMAICA,
    &JORDAN,
    &JAPAN,
    &SOUTH_KOREA,
    &KAZAKHSTAN,
    &LEBANON,
    &LATVIA,
    &MONGOLIA,
    &MOZAMBIQUE,
    &NIGERIA,
    #[cfg(feature = "indic")]
    &NEPAL,
    &NEW_ZEALAND,
    &OMAN,
    &PHILIPPINES,
    &PAKISTAN,
    &QATAR,
    &RUSSIA,
    &SWEDEN,
    &SINGAPORE,
    &THAILAND,
    &TUNISIA,
    &TURKEY,
    &TAIWAN,
    &TANZANIA,
    &UKRAINE,
    &UNITED_STATES,
    &UZBEKISTAN,
    &VIETNAM,
    &SAMOA,
    &SOUTH_AFRICA,
];

/// The profile for an ISO 3166-1 alpha-2 code, if this crate has one.
#[must_use]
pub fn by_code(code: &str) -> Option<&'static FiscalProfile> {
    ALL.iter()
        .copied()
        .find(|profile| hc_core::catalogue::matches(code, profile.code))
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::{buddhist, ethiopic, gregorian, persian_33};

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
        // The Crown financial year 2026/27 begins 1 July 2026; the 2025 tax
        // year begins 1 April 2024 (Inland Revenue: "1 April 2024 to 31 March
        // 2025 tax year"). One country, two conventions.
        let crown = NEW_ZEALAND.in_force(SystemKind::Government, 2026).unwrap();
        let tax = NEW_ZEALAND.in_force(SystemKind::PersonalTax, 2025).unwrap();
        assert_eq!(crown.label, LabelConvention::LabelledByStartYear);
        assert_eq!(tax.label, LabelConvention::LabelledByEndYear);
        assert_eq!(crown.span(2026).unwrap().first, greg(2026, 7, 1));
        assert_eq!(tax.span(2025).unwrap().first, greg(2024, 4, 1));
        assert_eq!(tax.span(2025).unwrap().last, greg(2025, 3, 31));
    }

    #[test]
    fn south_africa_labels_its_two_years_in_opposite_directions_too() {
        let government = SOUTH_AFRICA.government(2026).unwrap();
        let tax = SOUTH_AFRICA
            .in_force(SystemKind::PersonalTax, 2026)
            .unwrap();
        assert_eq!(government.span(2026).unwrap().first, greg(2026, 4, 1));
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
        assert_eq!(system.start.calendar, StartCalendar::SOLAR_HIJRI_33);
        let span = system.span(1403).unwrap();
        assert_eq!(span.first, persian_33::to_fixed(1403, 1, 1).unwrap());
        assert_eq!(
            span.last,
            Rd(persian_33::to_fixed(1404, 1, 1).unwrap().0 - 1)
        );
    }

    #[test]
    fn the_iranian_fiscal_year_matches_the_published_gregorian_starts() {
        // The first day of each Solar Hijri year 1399 to 1410, from the
        // correspondence table of Wikipedia's "Solar Hijri calendar"
        // (`wikipedia-solar-hijri-calendar`, secondary), read 2026-10-03.
        // 1404 starts on 21 March 2025; Birashk's 2 820-year cycle gives the
        // 20th.
        let published = [
            (1399, (2020, 3, 20)),
            (1400, (2021, 3, 21)),
            (1401, (2022, 3, 21)),
            (1402, (2023, 3, 21)),
            (1403, (2024, 3, 20)),
            (1404, (2025, 3, 21)),
            (1405, (2026, 3, 21)),
            (1406, (2027, 3, 21)),
            (1407, (2028, 3, 20)),
            (1408, (2029, 3, 20)),
            (1409, (2030, 3, 21)),
            (1410, (2031, 3, 21)),
        ];
        let system = IRAN.government(1403).unwrap();
        for (label, (year, month, day)) in published {
            assert_eq!(
                system.span(label).unwrap().first,
                greg(year, month, day),
                "Iranian fiscal year {label}"
            );
        }
        // The table's last days: 1402 ended on 19 March 2024, the least
        // obvious value in the set, and 1404 on 20 March 2026.
        assert_eq!(system.span(1402).unwrap().last, greg(2024, 3, 19));
        assert_eq!(system.span(1404).unwrap().last, greg(2026, 3, 20));
        assert_eq!(system.span(1406).unwrap().last, greg(2028, 3, 19));
    }

    #[test]
    fn the_iranian_fiscal_year_is_not_the_2820_year_cycle() {
        // The 2 820-year cycle puts AP 1404 on 20 March 2025, a day before
        // Iran did; the 33-year rule does not.
        use hc_calendars_solar::persian;
        assert_eq!(
            persian::to_fixed(1404, 1, 1).unwrap(),
            greg(2025, 3, 20),
            "the 2 820-year cycle's start of 1404, a day before the entry's"
        );
        let system = IRAN.government(1404).unwrap();
        assert_eq!(system.span(1404).unwrap().first, greg(2025, 3, 21));
        assert_ne!(
            system.span(1404).unwrap().first,
            persian::to_fixed(1404, 1, 1).unwrap()
        );
        assert!(system.start.calendar.is_approximate());
    }

    #[test]
    fn the_ethiopian_fiscal_year_runs_hamle_1_to_sene_30() {
        let system = ETHIOPIA.government(2016).unwrap();
        assert_eq!(system.start.calendar, StartCalendar::ETHIOPIC);
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
        for label in 2014..=2090 {
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
        // modelled, so neither label 1995 nor 1996 is a Swedish budget year
        // here, and no day of those eighteen months falls in one.
        assert!(SWEDEN.government(1994).is_some());
        assert_eq!(SWEDEN.government(1995), None);
        assert_eq!(SWEDEN.government(1996), None);
        assert!(SWEDEN.government(1997).is_some());
        let government = SystemKind::Government;
        assert_eq!(
            SWEDEN
                .at(government, greg(1995, 6, 30))
                .map(|(_, label)| label),
            Some(1994)
        );
        assert_eq!(SWEDEN.at(government, greg(1995, 7, 1)), None);
        assert_eq!(SWEDEN.at(government, greg(1996, 12, 31)), None);
        assert_eq!(
            SWEDEN
                .at(government, greg(1997, 1, 1))
                .map(|(_, label)| label),
            Some(1997)
        );
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
            let system = profile.government(2026).unwrap();
            assert_eq!(
                system.start,
                YearStart::gregorian(4, 1),
                "{}",
                profile.english_name
            );
            assert_eq!(system.span(2026).unwrap().first, greg(2026, 4, 1));
            assert_eq!(system.span(2026).unwrap().last, greg(2027, 3, 31));
        }
    }

    #[test]
    fn the_july_countries_all_start_on_the_first_of_july() {
        for profile in [&AUSTRALIA, &NEW_ZEALAND, &EGYPT, &PAKISTAN] {
            let system = profile.government(2026).unwrap();
            assert_eq!(
                system.start,
                YearStart::gregorian(7, 1),
                "{}",
                profile.english_name
            );
            assert_eq!(system.span(2026).unwrap().last, greg(2027, 6, 30));
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
                // A system is read from its establishment or later, and a
                // stretch of years no source reaches lies inside the years
                // the system was in force.
                assert!(
                    system
                        .valid_from
                        .is_none_or(|first| system.read_from >= first),
                    "{} {}",
                    profile.code,
                    system.name
                );
                for &(first, last) in system.unread {
                    assert!(
                        first <= last && first > system.read_from,
                        "{}",
                        profile.code
                    );
                    assert!(
                        system.valid_until.is_none_or(|until| last <= until),
                        "{}",
                        profile.code
                    );
                }
                assert!(!system.note.is_empty() || system.authority.is_national_rule());
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
        // Nepal's profile needs the Bikram Sambat calendar of `indic`.
        assert_eq!(ALL.len(), 60 + usize::from(cfg!(feature = "indic")));
        for code in NEEDS_INDIC {
            assert_eq!(
                ALL.iter().any(|profile| profile.code == *code),
                cfg!(feature = "indic"),
                "{code}"
            );
        }
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
        #[cfg(feature = "indic")]
        assert_eq!(by_code("NP").unwrap().english_name, "Nepal");
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
        assert_eq!(system.start.calendar, StartCalendar::THAI_BUDDHIST);
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

    #[cfg(feature = "indic")]
    #[test]
    fn nepal_starts_its_year_on_1_shrawan() {
        let system = NEPAL.government(2082).unwrap();
        // Shrawan 1 of 2081, 2082 and 2083 BS, from the gazetted months. The
        // table is read from 2083, so the earlier years are projected.
        for (label, year, month, day) in [
            (2081, 2024, 7, 16),
            (2082, 2025, 7, 17),
            (2083, 2026, 7, 17),
        ] {
            let span = system.projected_span(label).unwrap();
            assert_eq!(span.first, greg(year, month, day), "{label}");
        }
        assert_eq!(system.span(2083).unwrap().first, greg(2026, 7, 17));
        assert_eq!(system.span(2082), Err(FiscalError::NotRead));
        // 2082/83 ends on the last day of Asar 2083, the day before
        // 2083/84 begins.
        assert_eq!(system.projected_span(2082).unwrap().last, greg(2026, 7, 16));
        assert_eq!(system.projected_label_at(greg(2026, 1, 1)).unwrap(), 2082);
        assert!(system.start.calendar.is_approximate());
    }

    #[test]
    fn the_hong_kong_and_singapore_years_are_the_april_shape_too() {
        for profile in [&HONG_KONG, &SINGAPORE] {
            let system = profile.government(2026).unwrap();
            assert_eq!(system.label, LabelConvention::LabelledByStartYear);
            assert_eq!(system.span(2026).unwrap().first, greg(2026, 4, 1));
        }
        // Singapore's personal tax basis period is the calendar year, which
        // is a different year in the same country.
        let tax = SINGAPORE.in_force(SystemKind::PersonalTax, 2026).unwrap();
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
                // A system read in none of its years (the United Kingdom's
                // income tax of 1799 to 1815) has no span to convert.
                if system
                    .valid_until
                    .is_some_and(|last| last < system.read_from)
                {
                    continue;
                }
                // Pick a label the system actually covers: its first, or
                // 2024 clamped into its range for the open-ended ones.
                let label = system
                    .valid_from
                    .map_or(system.read_from, |first| first.max(system.read_from))
                    .min(system.valid_until.unwrap_or(i64::MAX));
                let span = system.span(label).unwrap_or_else(|error| {
                    panic!("{} {} at {label}: {error:?}", profile.code, system.name)
                });
                assert!(span.days() > 300, "{} {}", profile.code, system.name);
                assert_eq!(system.label_at(span.first).unwrap(), label);
            }
        }
    }

    // ── Years the sources do not reach ──────────────────────────────────

    #[test]
    fn the_us_federal_year_was_the_calendar_year_until_the_act_of_1842() {
        // The Treasury's letter of 15 December 1842 (H. Doc. 27-16, govinfo)
        // asks for estimates for "the half calendar year ending 30th June,
        // 1843" and for "the fiscal year ending 30th June, 1844": the first
        // July year is FY1844, and the half year before it is in no system.
        let october = &UNITED_STATES.systems[0];
        let july = &UNITED_STATES.systems[1];
        let calendar = &UNITED_STATES.systems[2];
        let first = july.span(1844).unwrap();
        assert_eq!(
            (first.first, first.last),
            (greg(1843, 7, 1), greg(1844, 6, 30))
        );
        assert_eq!(july.span(1843), Err(FiscalError::OutsideValidity));
        assert!(UNITED_STATES.government(1843).is_none());
        let half_year = [greg(1843, 1, 1), greg(1843, 3, 15), greg(1843, 6, 30)];
        for day in half_year {
            assert_eq!(UNITED_STATES.at(SystemKind::Government, day), None);
        }
        assert_eq!(
            UNITED_STATES
                .at(SystemKind::Government, greg(1843, 7, 1))
                .map(|(_, label)| label),
            Some(1844)
        );
        // A Treasury report on "the first half of the year 1842" reads the
        // calendar year 1842; before it the answer is a gap, never "in force".
        assert_eq!(calendar.span(1842).unwrap().first, greg(1842, 1, 1));
        assert_eq!(calendar.span(1800), Err(FiscalError::NotRead));
        // The calendar year is in force from the year the Treasury was
        // established, 1789: before it the state had no fiscal year at all.
        assert_eq!(calendar.span(1789), Err(FiscalError::NotRead));
        assert_eq!(calendar.span(1788), Err(FiscalError::OutsideValidity));
        assert_eq!(calendar.span(1342), Err(FiscalError::OutsideValidity));
        assert_eq!(july.span(1800), Err(FiscalError::OutsideValidity));
        assert_eq!(october.span(1800), Err(FiscalError::OutsideValidity));
        assert_eq!(
            UNITED_STATES.at(SystemKind::Government, greg(1800, 6, 1)),
            None
        );
    }

    #[test]
    fn japans_april_year_began_in_1886_and_the_july_year_before_it_in_1875() {
        // The National Archives of Japan: 明治8年 July start, 明治18年度 the
        // nine months of 1 July 1885 to 31 March 1886, 明治19年度 April.
        let july = &JAPAN.systems[0];
        let april = &JAPAN.systems[1];
        assert_eq!(july.span(1875).unwrap().first, greg(1875, 7, 1));
        assert_eq!(july.span(1884).unwrap().last, greg(1885, 6, 30));
        assert_eq!(april.span(1886).unwrap().first, greg(1886, 4, 1));
        assert_eq!(april.span(1886).unwrap().last, greg(1887, 3, 31));
        // 明治18年度 is in neither system.
        for day in [greg(1885, 7, 1), greg(1885, 12, 31), greg(1886, 3, 31)] {
            assert_eq!(JAPAN.at(SystemKind::Government, day), None);
        }
        assert_eq!(
            JAPAN
                .at(SystemKind::Government, greg(1886, 4, 1))
                .map(|(_, label)| label),
            Some(1886)
        );
        // Before 1886 the April year is absent, an answer; the 会計法 of
        // 1889 reads 1890 to 1920 and the 財政法 of 1947 from 1947.
        assert_eq!(april.span(1885), Err(FiscalError::OutsideValidity));
        assert!(april.span(1920).is_ok() && april.span(1947).is_ok());
        // 1921 to 1946 were not read: a gap, not a year without a fiscal year.
        for label in [1921, 1930, 1946] {
            assert_eq!(april.span(label), Err(FiscalError::NotRead), "{label}");
        }
        assert_eq!(JAPAN.at(SystemKind::Government, greg(1930, 6, 1)), None);
    }

    #[test]
    fn canada_and_the_united_kingdom_begin_their_april_years_where_a_source_does() {
        // Until 1906 the Canadian year ran from 1 July (House of Commons
        // Procedure and Practice); the first whole April year is 1907-08.
        let canada = CANADA.government(1907).unwrap();
        assert_eq!(canada.span(1907).unwrap().first, greg(1907, 4, 1));
        assert_eq!(canada.span(1907).unwrap().last, greg(1908, 3, 31));
        assert!(CANADA.government(1906).is_none());
        // The United Kingdom's year ending 31 March rests on the Public
        // Revenue and Consolidated Fund Charges Act 1854 (from 1 April 1854;
        // Hansard reports 1855-56), not on 1753, a year no source gives.
        let uk = UNITED_KINGDOM.government(1854).unwrap();
        assert_eq!(uk.span(1855).unwrap().first, greg(1855, 4, 1));
        assert_eq!(uk.span(1855).unwrap().last, greg(1856, 3, 31));
        assert_eq!(uk.span(1854), Err(FiscalError::NotRead));
        assert_eq!(uk.span(1853), Err(FiscalError::OutsideValidity));
        assert!(UNITED_KINGDOM.government(1753).is_none());
    }

    #[test]
    fn a_system_is_absent_before_the_state_or_the_tax_it_belongs_to_existed() {
        // Australia: the Commonwealth's year since 1901 (Wikipedia); the
        // first whole year read is 1902.
        let au = &AUSTRALIA.systems[0];
        assert_eq!(au.span(1900), Err(FiscalError::OutsideValidity));
        assert_eq!(au.span(1901), Err(FiscalError::NotRead));
        assert!(au.span(1902).is_ok());
        // Hong Kong: the island was occupied in 1841, so a year before it is
        // absent and the years since are a gap until the first year read.
        for system in HONG_KONG.systems {
            assert_eq!(system.span(1840), Err(FiscalError::OutsideValidity));
            assert_eq!(system.span(1841), Err(FiscalError::NotRead));
        }
        // The United Kingdom's personal tax: levied 1799 to 1802 and from
        // 1803 to 1816, and for good from 1842; a gap where it was levied,
        // absent where it lapsed.
        let tax = |label| {
            UNITED_KINGDOM
                .systems
                .iter()
                .filter(|system| system.kind == SystemKind::PersonalTax)
                .map(|system| system.span(label))
                .find(|answer| *answer != Err(FiscalError::OutsideValidity))
                .unwrap_or(Err(FiscalError::OutsideValidity))
        };
        assert_eq!(tax(1798), Err(FiscalError::OutsideValidity));
        assert_eq!(tax(1799), Err(FiscalError::NotRead));
        assert_eq!(tax(1801), Err(FiscalError::NotRead));
        assert_eq!(tax(1802), Err(FiscalError::OutsideValidity));
        assert_eq!(tax(1803), Err(FiscalError::NotRead));
        assert_eq!(tax(1815), Err(FiscalError::NotRead));
        for lapsed in [1816, 1830, 1841] {
            assert_eq!(tax(lapsed), Err(FiscalError::OutsideValidity), "{lapsed}");
        }
        assert_eq!(tax(1842), Err(FiscalError::NotRead));
        assert_eq!(tax(2006), Err(FiscalError::NotRead));
        assert!(tax(2007).is_ok());
    }

    #[test]
    fn a_system_the_notes_say_existed_is_a_gap_before_the_year_read_not_absent() {
        // Iran: the Public Accounting Act approved 1349/10/15, in force from
        // 1 Farvardin 1350, fixes the Solar Hijri year; 1975 CE is 1354.
        let (iran, label) = IRAN.at(SystemKind::Government, greg(1975, 6, 1)).unwrap();
        assert_eq!(label, 1354);
        assert_eq!(iran.span(1349), Err(FiscalError::NotRead));
        // Ethiopia and Nepal are read from the years of the pages: 1990 CE is
        // a gap in both, not a year the system did not exist.
        let ethiopia = &ETHIOPIA.systems[0];
        assert_eq!(
            ethiopia.label_at(greg(1990, 1, 1)),
            Err(FiscalError::NotRead)
        );
        assert_eq!(ethiopia.span(2013), Err(FiscalError::NotRead));
        assert_eq!(ethiopia.span(2014).unwrap().first, greg(2021, 7, 8));
        #[cfg(feature = "indic")]
        assert_eq!(
            NEPAL.systems[0].label_at(greg(1990, 1, 1)),
            Err(FiscalError::NotRead)
        );
        // Thailand: the October year is named for the year it ends in, so
        // ปีงบประมาณ 2505 began on 1 October 2504 (1961) and the calendar
        // year before it ends at 2503, leaving B.E. 2504 in neither system.
        let october = &THAILAND.systems[0];
        let calendar = &THAILAND.systems[1];
        assert_eq!(october.span(2505).unwrap().first, greg(1961, 10, 1));
        assert_eq!(calendar.span(2503).unwrap().last, greg(1960, 12, 31));
        assert!(THAILAND.government(2504).is_none());
        assert_eq!(october.span(2504), Err(FiscalError::OutsideValidity));
        assert_eq!(calendar.span(2504), Err(FiscalError::OutsideValidity));
        assert_eq!(calendar.span(2483), Err(FiscalError::OutsideValidity));
    }

    #[test]
    fn egypt_moved_from_the_calendar_year_to_july_in_1980() {
        // Law 53 of 1973, art. 2 as issued: 1 January to 31 December. The
        // July year began in 1980; 1 January to 30 June 1980 is in neither.
        let calendar = &EGYPT.systems[0];
        let july = &EGYPT.systems[1];
        assert_eq!(calendar.span(1974).unwrap().first, greg(1974, 1, 1));
        assert_eq!(calendar.span(1979).unwrap().last, greg(1979, 12, 31));
        assert_eq!(calendar.span(1973), Err(FiscalError::NotRead));
        assert_eq!(calendar.span(1980), Err(FiscalError::OutsideValidity));
        assert_eq!(july.span(1980).unwrap().first, greg(1980, 7, 1));
        assert_eq!(july.span(1979), Err(FiscalError::OutsideValidity));
        for day in [greg(1980, 1, 1), greg(1980, 6, 30)] {
            assert_eq!(EGYPT.at(SystemKind::Government, day), None);
        }
    }

    #[test]
    fn swedens_calendar_year_before_1923_is_read_from_the_titles_of_1921() {
        // Prop. 1920:1 is titled "under år 1921": the calendar-year budget.
        let before = &SWEDEN.systems[0];
        assert_eq!(before.span(1921).unwrap().first, greg(1921, 1, 1));
        assert_eq!(before.span(1920), Err(FiscalError::NotRead));
        assert_eq!(before.span(1923), Err(FiscalError::OutsideValidity));
        // "budgetåret 1923—1924" was 1 July 1923 to 30 June 1924, and the six
        // months of 1 January to 30 June 1923 are in no system.
        let july = &SWEDEN.systems[1];
        assert_eq!(july.span(1923).unwrap().first, greg(1923, 7, 1));
        assert_eq!(SWEDEN.at(SystemKind::Government, greg(1923, 3, 1)), None);
        // The calendar-year company year is read from the law of 1976.
        let company = SWEDEN.in_force(SystemKind::CorporateDefault, 1977).unwrap();
        assert_eq!(company.span(1976), Err(FiscalError::NotRead));
        assert!(company.span(1977).is_ok());
    }

    #[test]
    fn the_calendar_year_countries_are_read_from_their_instruments() {
        // BHO and HGrG of 19 August 1969, in force 1 January 1970.
        assert_eq!(
            GERMANY.government(1969).unwrap().span(1969),
            Err(FiscalError::NotRead)
        );
        assert!(GERMANY.government(1970).unwrap().span(1970).is_ok());
        // Hong Kong's Inland Revenue Ordinance, 1947: the year of assessment
        // is the twelve months commencing on 1 April 1947.
        let tax = HONG_KONG.in_force(SystemKind::PersonalTax, 1947).unwrap();
        assert_eq!(tax.span(1947).unwrap().first, greg(1947, 4, 1));
        assert_eq!(tax.span(1946), Err(FiscalError::NotRead));
        // India: the April year is established in 1867 and read from 1868.
        let india = INDIA.government(1868).unwrap();
        assert_eq!(india.span(1868).unwrap().first, greg(1868, 4, 1));
        assert_eq!(india.span(1867), Err(FiscalError::NotRead));
        assert_eq!(india.span(1866), Err(FiscalError::OutsideValidity));
        assert!(INDIA.government(1700).is_none());
    }

    #[test]
    fn the_countries_read_from_one_instrument_answer_from_its_first_whole_year() {
        let years = [
            ("KR", 2007, (1, 1)),
            ("AR", 1993, (1, 1)),
            ("CZ", 2001, (1, 1)),
            ("UZ", 2014, (1, 1)),
            ("BD", 1974, (7, 1)),
            ("JM", 2015, (4, 1)),
        ];
        for (code, first, (month, day)) in years {
            let profile = by_code(code).unwrap();
            let system = profile.government(first).unwrap();
            assert_eq!(
                system.span(first).unwrap().first,
                greg(first, month, day),
                "{code}"
            );
            assert_eq!(system.span(first - 1), Err(FiscalError::NotRead), "{code}");
        }
        // Bangladesh names the year by the day it begins in: "the year
        // beginning on the first day of July, 1981".
        let bangladesh = by_code("BD").unwrap().government(1981).unwrap();
        assert_eq!(bangladesh.span(1981).unwrap().first, greg(1981, 7, 1));
        assert_eq!(bangladesh.span(1981).unwrap().last, greg(1982, 6, 30));
        // Qatar's calendar year follows the March year-end of 2015.
        let qatar = by_code("QA").unwrap().government(2016).unwrap();
        assert_eq!(qatar.span(2016).unwrap().first, greg(2016, 1, 1));
        assert_eq!(qatar.span(2015), Err(FiscalError::OutsideValidity));
    }

    fn check_gap_before_the_first_year_read(system: &YearSystem) {
        let low = system.valid_from.unwrap_or(system.read_from - 120);
        for label in low.max(system.read_from - 120)..system.read_from {
            assert!(
                matches!(
                    system.span(label),
                    Err(FiscalError::NotRead | FiscalError::OutsideValidity)
                ),
                "{} answered {label} before {}",
                system.name,
                system.read_from
            );
            assert!(!system.reads(label), "{} {label}", system.name);
        }
        // Past its last year, the system is absent, which is an answer.
        if let Some(until) = system.valid_until {
            assert_eq!(
                system.span(until + 1),
                Err(FiscalError::OutsideValidity),
                "{}",
                system.name
            );
        }
        let read_year_is_open = system
            .valid_until
            .is_none_or(|until| until >= system.read_from)
            && !system
                .unread
                .iter()
                .any(|&(first, last)| first <= system.read_from && system.read_from <= last);
        if read_year_is_open {
            assert!(system.span(system.read_from).is_ok(), "{}", system.name);
        }
    }

    #[test]
    fn no_system_answers_a_year_before_the_first_year_its_sources_reach() {
        use crate::academic;

        let mut count = 0;
        for profile in ALL {
            for system in profile.systems {
                check_gap_before_the_first_year_read(system);
                count += 1;
            }
        }
        for profile in academic::ALL {
            check_gap_before_the_first_year_read(&profile.school);
            count += 1;
            if let Some(university) = &profile.university {
                check_gap_before_the_first_year_read(university);
                count += 1;
            }
        }
        assert!(count >= 80, "{count}");
    }
}

hc_core::catalogue_tests! {
    type: &'static FiscalProfile,
    id: |profile| profile.code,
    sorted_by: |profile| profile.code,
    provenance: |profile| profile.sources,
    tests: profile_table_tests,
    all: ALL,
    lookup: by_code,
}
