//! Academic years.
//!
//! An academic year is a year that does not begin on 1 January, so it shares
//! this crate's machinery — but it is a different kind of thing from a fiscal
//! year and the difference matters more than the similarity.
//!
//! # Most countries do not legislate when school starts
//!
//! Two of the eight countries here do. Japan's
//! 学校教育法施行規則第五十九条 says the school year begins on 1 April and
//! ends on 31 March of the following year, and the rule reaches
//! kindergartens, junior high schools, high schools and special-needs
//! schools by 準用. France's Code de l'éducation art. L. 521-1 puts the
//! calendar in the Minister's hands, who publishes it as an *arrêté*. New
//! Zealand's Education and Training Act 2020 s. 66(1) is said to require the
//! Minister to fix term dates by *Gazette* notice, but the Act could not be
//! read and the one page read says term dates are set by individual
//! schools, so New Zealand's entry does not claim a national rule.
//!
//! The rest are not like that. In the United States the school year is set
//! by the district — the Education Commission of the States counts
//! twenty-seven states that leave the start date entirely to local boards.
//! In Germany it is a *Land* matter and constitutionally cannot be a federal
//! one; the Kultusministerkonferenz coordinates the summer-holiday windows
//! precisely so that the sixteen calendars do **not** coincide. Australia is
//! per state, and New South Wales runs two different Term 1 start dates
//! inside one state. India has two genuinely common patterns and neither is
//! national. Universities frequently differ from schools in the same
//! country — Germany's *Wintersemester* starts in October, and Japanese
//! universities are explicitly exempted from the rule that binds Japanese
//! schools.
//!
//! So every entry here carries an [`Authority`], and
//! [`Authority::is_national_rule`] is the method to check before printing an
//! answer as if it were a fact about a country. An entry marked
//! [`Authority::PerRegion`] or [`Authority::PerInstitution`] records the
//! modal choice and nothing more. That is not hedging: asserting a single
//! national start date for the United States would be inventing one, and
//! `docs/policy.md` §4 is explicit that a wrong answer delivered confidently
//! is worse than a refusal.
//!
//! A related trap: several countries have a national statute about the
//! *length* of the school year — 190 days in England and Wales, 180 in most
//! US states, 200 and 220 under India's Right to Education Act — and it is
//! easy to mistake one of those for a rule about the start date. They are
//! not, and this module models the year boundary only.
//!
//! # Academic years are not fiscal years
//!
//! They coincide in Japan, by design, and in India, where the April-to-March
//! session shares a boundary with the financial year. Nowhere else in this
//! table. The United Kingdom's financial year starts in April and its school
//! year in September; France's budget is the calendar year and its *rentrée*
//! is in September; Australia's financial year starts in July and its school
//! year in late January. Anyone tempted to reuse one for the other should
//! read the table first.

use crate::year_system::{
    Authority, LabelConvention, SourceDate, SystemKind, YearStart, YearSystem,
};

/// A country's academic year, as this crate can honestly state it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcademicProfile {
    /// The ISO 3166-1 alpha-2 code.
    pub code: &'static str,
    /// The English name of the country.
    pub english_name: &'static str,
    /// The school year, or the modal one.
    pub school: YearSystem,
    /// The university year, when it differs from the school year.
    pub university: Option<YearSystem>,
    /// When the sources behind this entry were last checked.
    pub sources_checked: SourceDate,
    /// Where the entry came from.
    pub sources: &'static str,
}

impl AcademicProfile {
    /// Whether the school year is fixed nationally rather than locally.
    #[must_use]
    pub const fn is_nationally_fixed(&self) -> bool {
        self.school.authority.is_national_rule()
    }
}

/// Japan's 学年度, 1 April to 31 March.
///
/// 学校教育法施行規則（昭和二十二年文部省令第十一号）第五十九条:
/// 「小学校の学年は、四月一日に始まり、翌年三月三十一日に終わる。」
/// 第三十九条 applies it to kindergartens, and the corresponding articles
/// carry it to junior high schools, high schools and 特別支援学校.
///
/// Exposed as a `const` rather than hidden inside the table because
/// [`crate::countries::JAPAN`] lists it alongside the 会計年度: in Japan the
/// two genuinely are the same year, which is the exception rather than the
/// rule.
pub const JAPAN_SCHOOL_YEAR: YearSystem = YearSystem {
    name: "Japanese school year",
    local_name: "学年度",
    kind: SystemKind::Academic,
    authority: Authority::Regulation,
    start: YearStart::gregorian(4, 1),
    label: LabelConvention::LabelledByStartYear,
    valid_from: None,
    valid_until: None,
    read_from: 1947,
    unread: &[],
    note: "Fixed nationally by ministerial ordinance, which is unusual: in most countries the \
           school year is set locally. 学校教育法施行規則 (昭和22年文部省令第11号) was \
           promulgated on 23 May 1947 and applies from 1 April 1947 by its supplementary \
           provision, so 1947 is a whole year; art. 59 is read in the consolidated text of \
           2026, and that the wording was the same in 1947 was not checked. The April start \
           is older for elementary schools (小学校令施行規則 of 1900, art. 25, in force on \
           1 September 1900, and a reference answer says uniform from 1892) and was not read \
           for the other school types, so years before 1947 are a gap. Universities may set \
           their own 学年, and some run September-start programmes.",
};

/// Build a school-year entry. Private so that every public value in this
/// module carries the full documentation the table promises.
const fn school(
    name: &'static str,
    local_name: &'static str,
    authority: Authority,
    month: u8,
    day: u8,
    first: (Option<i64>, i64),
    note: &'static str,
) -> YearSystem {
    YearSystem {
        name,
        local_name,
        kind: SystemKind::Academic,
        authority,
        start: YearStart::gregorian(month, day),
        label: LabelConvention::LabelledByStartYear,
        valid_from: first.0,
        valid_until: None,
        read_from: first.1,
        unread: &[],
        note,
    }
}

/// Japan 🇯🇵 — April, and nationally fixed for schools but not universities.
///
/// 第五十九条 fixes the school year, and 第七十九条 and 第百四条 carry it to
/// junior high schools and high schools by 準用. Universities are explicitly
/// exempt: 第百六十三条 gives the start and end of the 学年 to the 学長, which
/// is the legal hook the 秋入学 debate turns on. In practice essentially
/// every Japanese university chooses 1 April anyway, so the university entry
/// here is April with [`Authority::PerInstitution`] — the modal choice, not a
/// rule.
///
/// Terms and holidays *inside* the year are a separate matter, set locally by
/// the 教育委員会 under 学校教育法施行令第二十九条. This crate models the year
/// boundary only.
pub static JAPAN: AcademicProfile = AcademicProfile {
    code: "JP",
    english_name: "Japan",
    school: JAPAN_SCHOOL_YEAR,
    university: Some(school(
        "Japanese university year",
        "学年",
        Authority::PerInstitution,
        4,
        1,
        (None, 2026),
        "学校教育法施行規則第163条 gives the start and end of a university's 学年 to its 学長 \
         (read in e-Gov's consolidated text of 2026), and says no date; April is the \
         practice English and Japanese Wikipedia state, so April is the near-universal choice \
         and not the rule. September-entry programmes exist and use the same hook. The \
         Ministry's history of education says the 帝国大学 and 高等学校 moved from September \
         to April from 大正10年度 (1921); that is one kind of university, so no year before \
         2026 is carried.",
    )),
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "学校教育法施行規則（昭和22年文部省令第11号）第59条, 第39条, 第79条, 第163条 and 附則 \
              (e-Gov 法令検索, API); 学校教育法施行令第29条; 文部科学省「学制百年史」第三節 高等教育; \
              小学校令施行規則（明治33年文部省令第14号）第25条 (MEXT); Wikipedia, 「学年」 and \
              \"Academic term\" (secondary), read 2026-10-04",
};

/// United Kingdom 🇬🇧 — September, and Scotland earlier.
///
/// In England, Wales and Northern Ireland the school year begins in early
/// September; in Scotland it begins in mid-August, and Scottish local
/// authorities set their own term dates. The entry is dated 1 September as
/// the modal start and marked [`Authority::PerRegion`], because the four
/// nations run separate school calendars and within England the dates are
/// set by the local authority or the academy trust.
pub static UNITED_KINGDOM: AcademicProfile = AcademicProfile {
    code: "GB",
    english_name: "United Kingdom",
    school: school(
        "United Kingdom school year",
        "",
        Authority::PerRegion,
        9,
        1,
        (None, 2026),
        "England, Wales and Northern Ireland start in early September; Scotland in mid-August. \
         Term dates are set by the local authority or academy trust, not nationally. No page \
         read gives the date: the Education Act 1996 s. 579(1), read as revised, defines \
         the school year as beginning with the first term to begin after July (inserted on \
         14 June 1997), and GOV.UK and mygov.scot say only that dates vary; 1 September is \
         the table's representative date, from Wikipedia's accounts, and is read from 2026.",
    ),
    university: Some(school(
        "United Kingdom university year",
        "",
        Authority::PerInstitution,
        9,
        1,
        (None, 2026),
        "Michaelmas term begins late September or early October and varies by institution; \
         Scottish universities generally start in September. Wikipedia's \"Academic term\" \
         says September or October; no university or UCAS page was read, and 1 September is \
         the table's representative date.",
    )),
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Education Act 2002 s. 32 (term dates set by the local authority or governing \
              body); Education Act 1996 s. 579(1) (\"the first school term to begin after \
              July\"), legislation.gov.uk; GOV.UK and mygov.scot, school term and holiday \
              dates; Wikipedia, \"Academic term\" (secondary), read 2026-10-04",
};

/// France 🇫🇷 — September, and fixed by a national instrument.
///
/// France is the other country in this table with a genuine national rule.
/// Code de l'éducation art. L. 521-1 puts the school calendar in the hands of
/// the Minister of Education, who publishes it as an *arrêté* in the
/// *Journal officiel*, three years at a time and in three zones. The zones
/// stagger the holidays, not the *rentrée*, which falls in the first days of
/// September nationwide.
pub static FRANCE: AcademicProfile = AcademicProfile {
    code: "FR",
    english_name: "France",
    school: school(
        "French school year",
        "année scolaire",
        Authority::Regulation,
        9,
        1,
        (None, 2026),
        "The rentrée is fixed each year by ministerial arrêté and falls in the first days of \
         September; the three zones stagger holidays within the year, not its start. Code \
         de l'éducation art. L. 521-1 (in force since 22 June 2000, read on Légifrance) has \
         the Minister fix a national calendar for three years and says nothing of the \
         rentrée or of September; the September start rests on the French and English \
         Wikipedia pages on the school calendar, so the entry is read from 2026.",
    ),
    university: Some(school(
        "French university year",
        "année universitaire",
        Authority::PerInstitution,
        9,
        1,
        (None, 2026),
        "Set by each établissement; September or early October. No Code de l'éducation \
         article on the année universitaire was found; Wikipedia's \"Academic term\" says \
         September.",
    )),
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Code de l'éducation art. L. 521-1 (Légifrance); Wikipedia, \"Calendrier scolaire \
              en France\", \"Education in France\" and \"Academic term\" (secondary), read \
              2026-10-04; the arrêté fixing the calendrier scolaire (Journal officiel) was not \
              read",
};

/// Germany 🇩🇪 — August by *Land*, October for the universities.
///
/// The school year is a *Land* matter and cannot constitutionally be a
/// federal one. German Wikipedia says the Länder's school laws have it run
/// 1 August to 31 July, as the Länder agreed in the Hamburg Agreement of
/// 28 October 1964 (revised 14 October 1971), with two short school years
/// from April 1966 to July 1967, so 1967/68 is the first year of that
/// boundary; § 7 SchulG NRW, which is said to fix it in North Rhine-Westphalia,
/// could not be read. Teaching resumes somewhere between early August and
/// mid-September depending on the *Land*. The Kultusministerkonferenz
/// coordinates only the summer-holiday windows — they must fall between
/// 20 June and 15 September, in five rotating groups — precisely so that the
/// sixteen calendars do **not** coincide.
///
/// Universities are on semesters that do not follow the school year at all:
/// *Wintersemester* 1 October to 31 March, *Sommersemester* 1 April to
/// 30 September, with the *Vorlesungszeit* inside each much shorter.
pub static GERMANY: AcademicProfile = AcademicProfile {
    code: "DE",
    english_name: "Germany",
    school: school(
        "German school year",
        "Schuljahr",
        Authority::PerRegion,
        8,
        1,
        (Some(1967), 1967),
        "The legal year runs 1 August to 31 July in the Länder, from the Hamburg Agreement of \
         1964 (German Wikipedia's \"Hamburger Abkommen\" and \"Schuljahr\", secondary; the \
         agreement's text is a PDF and was not read); teaching resumes between early \
         August and mid-September by Land. This entry is the legal boundary, not the first \
         day of lessons. The years before were spring-start (Easter) in the Western zones \
         after 1945 and not carried: the page does not give their labels.",
    ),
    university: Some(school(
        "German university year",
        "Studienjahr",
        Authority::PerRegion,
        10,
        1,
        (None, 2026),
        "Wintersemester 1 October to 31 March, Sommersemester 1 April to 30 September, set by \
         Land higher-education law. The Vorlesungszeit is shorter than the semester. German \
         Wikipedia's \"Semester\" gives the dates; no Landeshochschulgesetz was read.",
    )),
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "German Wikipedia, \"Hamburger Abkommen\", \"Schuljahr\" and \"Semester\" (secondary); \
              Kultusministerkonferenz, Ferienregelung (kmk.org); § 7 SchulG NRW and the \
              Landeshochschulgesetze (not read), read 2026-10-04",
};

/// Australia 🇦🇺 — late January, and set by each state.
///
/// Victoria's term dates are approved by the Minister of Education in
/// five-year blocks; Western Australia publishes them in the *Government
/// Gazette*; New South Wales runs different start dates in its Eastern and
/// Western Divisions *within one state*. There is no national instrument,
/// and 28 January here is the modal start of Term 1, nothing more.
pub static AUSTRALIA: AcademicProfile = AcademicProfile {
    code: "AU",
    english_name: "Australia",
    school: school(
        "Australian school year",
        "",
        Authority::PerRegion,
        1,
        28,
        (None, 2027),
        "Each state and territory sets its own term dates; New South Wales uses two different \
         Term 1 start dates within the state. Late January to early February; 28 January is a \
         representative date, not a rule: for 2027 the pages read give Term 1 on 28 January \
         in Victoria, 1 February in Western Australia and 3 or 10 February in New South \
         Wales. They are the only years read, so the entry begins in 2027.",
    ),
    university: Some(school(
        "Australian university year",
        "",
        Authority::PerInstitution,
        2,
        1,
        (None, 2026),
        "Semester 1 begins in late February or early March, per institution. Wikipedia's \
         \"Academic term\" says semesters or trimesters; no university page was read, and \
         1 February is the table's representative date.",
    )),
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Victorian Department of Education, school term dates; Western Australia \
              Department of Education, future term dates; NSW Department of Education, 2027 \
              term dates; Wikipedia, \"Academic term\" (secondary), read 2026-10-04",
};

/// New Zealand 🇳🇿 — late January, and what fixes it is not known.
///
/// The Education and Training Act 2020 s. 66(1), with the Education (When
/// State Schools Must Be Open and Closed) Regulations 2024, is said to have
/// the Minister fix term dates by notice in the *Gazette*, with boards
/// choosing only the Term 1 start inside a window of about a week. Neither
/// could be read (legislation.govt.nz refused both),
/// and the one page read on how term dates are set, Wikipedia's "Education
/// in New Zealand", says they are set by individual schools within
/// government guidelines and not fixed nationally. So the entry does not say
/// that a statute or a regulation fixes the year: its authority is
/// [`Authority::Unread`].
pub static NEW_ZEALAND: AcademicProfile = AcademicProfile {
    code: "NZ",
    english_name: "New Zealand",
    school: school(
        "New Zealand school year",
        "",
        Authority::Unread,
        1,
        28,
        (None, 2026),
        "Wikipedia's \"Education in New Zealand\" says the year runs from late January to mid- \
         December in four terms and that term dates are set by individual schools within \
         government guidelines, not fixed nationally. The Gazette notice and the statutory \
         window said to apply (28 January to 4 February for 2027) were not \
         read, and 28 January is the table's representative date, not a fixed day.",
    ),
    university: Some(school(
        "New Zealand university year",
        "",
        Authority::PerInstitution,
        2,
        1,
        (None, 2026),
        "Semester 1 begins in late February, per institution: Wikipedia's \"Education in New \
         Zealand\" says universities run from late February to mid-November, so 1 February \
         is the table's representative date and not what the page says.",
    )),
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Wikipedia, \"Education in New Zealand\" (secondary); Education and Training Act \
              2020 s. 66(1) and the Education (When State Schools Must Be Open and Closed) \
              Regulations 2024 (legislation.govt.nz refused them), read 2026-10-04",
};

/// India 🇮🇳 — April **or** June, and saying one of them is over-general.
///
/// Education is on the Concurrent List and no national statute fixes the
/// start of the school year. Two patterns are both genuinely common and both
/// cover large populations: an **April to March** session, used across much
/// of northern, western and central India and specified for CBSE-affiliated
/// schools by circular, and a **June** reopening, used in Kerala, Karnataka,
/// Tamil Nadu and much of the south, where the date is driven by the
/// monsoon and ranges from late May to early July.
///
/// The two are not even mutually exclusive: a CBSE school in Kerala labels
/// its session April to March while following the Kerala reopening and
/// vacation calendar. The only national statute in the area is the Right to
/// Education Act 2009 s. 19 and its Schedule, which fixes 200 and 220
/// working days — a constraint on *length*, not on the start.
///
/// So the entry records April as the session label with an explicit note,
/// because a single date here is a simplification however it is chosen.
pub static INDIA: AcademicProfile = AcademicProfile {
    code: "IN",
    english_name: "India",
    school: school(
        "Indian academic session",
        "",
        Authority::PerRegion,
        4,
        1,
        (None, 2026),
        "Two dominant patterns: an April-to-March session (much of the north and west, and \
         CBSE-affiliated schools by circular) and a June reopening (Kerala, Karnataka, Tamil \
         Nadu and much of the south, monsoon-driven, late May to early July). Saying \"India \
         starts in June\" is as over-general as saying April. No national statute fixes the \
         date; the RTE Act 2009 fixes working days, not the start. Wikipedia's \"Education \
         in India\" gives April to March as the academic year and June as the usual start for \
         many institutions; the CBSE's circular and the RTE Act's schedule could not be read \
         (the CBSE's site refused them), so nothing is dated before 2026.",
    ),
    university: None,
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Wikipedia, \"Education in India\", \"Central Board of Secondary Education\" and \
              \"Right to Education Act\" (secondary); Right to Education Act 2009 s. 19 and \
              Schedule and the CBSE affiliation circulars (not read), read 2026-10-04",
};

/// United States 🇺🇸 — August or September, and set by the district.
///
/// There is no federal statute and no Department of Education instrument
/// fixing when school starts. The Education Commission of the States counts
/// fifteen states placing any parameter on start or finish dates and
/// twenty-seven leaving the start entirely to local districts; thirty-one
/// states and the District of Columbia require at least 180 instructional
/// days, which is again a constraint on length.
///
/// The regional gradient is large enough that a single national date is
/// meaningless: districts in Alabama, Kentucky, Mississippi and Tennessee
/// mostly return in the week of 7 August and some Georgia districts in late
/// July, while New England almost never returns before the week of
/// 28 August and much of New Jersey, New York and Pennsylvania waits until
/// after Labor Day. North Carolina's "no earlier than the Monday nearest
/// 26 August" rule exists to protect coastal tourism, which is a useful
/// reminder that these constraints are not always educational.
pub static UNITED_STATES: AcademicProfile = AcademicProfile {
    code: "US",
    english_name: "United States",
    school: school(
        "United States school year",
        "",
        Authority::PerInstitution,
        8,
        15,
        (None, 2023),
        "Set by the local district board. Late July to mid-September, with a strong regional \
         gradient: the South returns in early August, the Northeast after Labor Day. This date \
         is the middle of that range and is not a claim about any district; no page read \
         gives a date. The Education Commission of the States' page of 6 February 2023 is the \
         earliest read, and says that twenty-seven states leave the start to local districts.",
    ),
    university: Some(school(
        "United States university year",
        "",
        Authority::PerInstitution,
        8,
        15,
        (None, 2026),
        "Fall semester begins mid-August to early September, per institution: Wikipedia's \
         \"Academic term\" says September to December, and no institution's calendar was \
         read, so 15 August is the table's representative date.",
    )),
    sources_checked: SourceDate::new(2026, 10, 4),
    sources: "Education Commission of the States, Instructional Time Policies (2023-02-06); \
              NCES state education reforms tables; Wikipedia, \"Academic term\" (secondary), \
              read 2026-10-04",
};

/// Every academic-year entry in this crate.
pub static ALL: &[&AcademicProfile] = &[
    &AUSTRALIA,
    &GERMANY,
    &FRANCE,
    &UNITED_KINGDOM,
    &INDIA,
    &JAPAN,
    &NEW_ZEALAND,
    &UNITED_STATES,
];

/// The academic profile for an ISO 3166-1 alpha-2 code, if this crate has
/// one.
#[must_use]
pub fn by_code(code: &str) -> Option<&'static AcademicProfile> {
    ALL.iter()
        .copied()
        .find(|profile| hc_core::catalogue::matches(code, profile.code))
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use super::*;
    use crate::error::FiscalError;

    fn greg(year: i64, month: u8, day: u8) -> hc_calendar::Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn japans_school_year_runs_april_to_march_like_its_fiscal_year() {
        // 学校教育法施行規則 art. 59, applied from 1 April 1947.
        let span = JAPAN.school.span(2024).unwrap();
        assert_eq!(span.first, greg(2024, 4, 1));
        assert_eq!(span.last, greg(2025, 3, 31));
        assert_eq!(JAPAN.school.span(1947).unwrap().first, greg(1947, 4, 1));
    }

    #[test]
    fn a_school_year_before_the_pages_read_is_a_gap_not_an_answer() {
        // Japan's regulation applies from 1947; before it the April school
        // year existed for elementary schools but no page read gives the
        // other school types'.
        assert_eq!(JAPAN.school.span(1946), Err(FiscalError::NotRead));
        assert_eq!(JAPAN.school.span(1900), Err(FiscalError::NotRead));
        // Germany's 1 August year begins with 1967/68: before it there was
        // no such year, which is an answer.
        assert_eq!(GERMANY.school.span(1966), Err(FiscalError::OutsideValidity));
        assert_eq!(GERMANY.school.span(1967).unwrap().first, greg(1967, 8, 1));
        // The pages read for the others are of 2023 and 2026.
        assert_eq!(UNITED_STATES.school.span(2022), Err(FiscalError::NotRead));
        assert_eq!(UNITED_KINGDOM.school.span(2025), Err(FiscalError::NotRead));
        assert!(UNITED_KINGDOM.school.span(2026).is_ok());
        // Victoria, Western Australia and New South Wales are read for 2027.
        assert_eq!(AUSTRALIA.school.span(2026), Err(FiscalError::NotRead));
        assert_eq!(
            AUSTRALIA.school.span(2027).unwrap().first,
            greg(2027, 1, 28)
        );
    }

    #[test]
    fn only_japan_and_france_claim_a_national_school_year() {
        let national: [&str; 2] = ["JP", "FR"];
        for profile in ALL {
            assert_eq!(
                profile.is_nationally_fixed(),
                national.contains(&profile.code),
                "{} claimed the wrong authority",
                profile.english_name
            );
        }
    }

    #[test]
    fn the_united_states_school_year_is_not_a_national_rule() {
        assert!(!UNITED_STATES.is_nationally_fixed());
        assert_eq!(UNITED_STATES.school.authority, Authority::PerInstitution);
        assert!(!UNITED_STATES.school.note.is_empty());
    }

    #[test]
    fn a_southern_hemisphere_school_year_starts_and_ends_in_the_same_year() {
        // Australia and New Zealand start in late January, so the academic
        // year lies almost entirely inside one Gregorian year.
        for profile in [&AUSTRALIA, &NEW_ZEALAND] {
            let span = profile.school.span(2027).unwrap();
            assert_eq!(gregorian::year_from_fixed(span.first).unwrap(), 2027);
            assert_eq!(gregorian::year_from_fixed(span.last).unwrap(), 2028);
            // ... and ends before the next year's start, in January.
            let (_, month, _) = gregorian::from_fixed(span.last).unwrap();
            assert_eq!(month, 1);
        }
    }

    #[test]
    fn a_northern_hemisphere_school_year_straddles_two_gregorian_years() {
        for profile in [&UNITED_KINGDOM, &FRANCE, &GERMANY, &UNITED_STATES] {
            let span = profile.school.span(2027).unwrap();
            assert_eq!(gregorian::year_from_fixed(span.first).unwrap(), 2027);
            assert_eq!(gregorian::year_from_fixed(span.last).unwrap(), 2028);
        }
    }

    #[test]
    fn the_german_university_year_does_not_follow_the_german_school_year() {
        let school = GERMANY.school.span(2027).unwrap();
        let university = GERMANY.university.unwrap().span(2027).unwrap();
        assert_ne!(school.first, university.first);
        assert_eq!(school.first, greg(2027, 8, 1));
        assert_eq!(university.first, greg(2027, 10, 1));
    }

    #[test]
    fn the_academic_year_matches_the_fiscal_year_only_in_japan_and_india() {
        // Japan by design — the 会計年度 and the 学年度 are the same April
        // boundary in two instruments. India by coincidence of the same
        // date: the April-to-March session shares its boundary with the
        // financial year, which is part of why that session exists. Nowhere
        // else in this table do the two agree, and reusing one for the other
        // is therefore wrong five times out of six.
        use crate::countries;

        for code in ["JP", "GB", "FR", "DE", "AU", "NZ", "US", "IN"] {
            let academic = by_code(code).unwrap();
            let Some(fiscal) = countries::by_code(code).and_then(|c| c.government(2025)) else {
                continue;
            };
            let same = academic.school.start == fiscal.start;
            assert_eq!(same, code == "JP" || code == "IN", "{code}");
        }
    }

    #[test]
    fn every_academic_entry_has_a_source_and_a_note_where_it_is_not_a_rule() {
        for profile in ALL {
            assert_eq!(profile.code.len(), 2);
            assert!(!profile.sources.is_empty(), "{}", profile.code);
            assert_eq!(profile.school.kind, SystemKind::Academic);
            if !profile.is_nationally_fixed() {
                assert!(
                    !profile.school.note.is_empty(),
                    "{} must say why it is not a rule",
                    profile.code
                );
            }
            if let Some(university) = profile.university {
                assert_eq!(university.kind, SystemKind::Academic);
                assert!(!university.note.is_empty(), "{}", profile.code);
            }
        }
    }

    #[test]
    fn an_academic_profile_can_be_found_by_its_code() {
        assert_eq!(by_code("FR").unwrap().english_name, "France");
        assert!(by_code("ZZ").is_none());
    }

    #[test]
    fn every_academic_year_is_a_whole_year_long() {
        for profile in ALL {
            for label in 2000..2050 {
                let days = profile.school.projected_span(label).unwrap().days();
                assert!((365..=366).contains(&days), "{}", profile.code);
            }
        }
    }
}

hc_core::catalogue_tests! {
    type: &'static AcademicProfile,
    id: |profile| profile.code,
    sorted_by: |profile| profile.code,
    provenance: |profile| profile.sources,
    tests: profile_table_tests,
    all: ALL,
    lookup: by_code,
}
