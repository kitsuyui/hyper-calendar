//! Academic years.
//!
//! An academic year is a year that does not begin on 1 January, so it shares
//! this crate's machinery — but it is a different kind of thing from a fiscal
//! year and the difference matters more than the similarity.
//!
//! # Most countries do not legislate when school starts
//!
//! Three of the eight countries here do. Japan's
//! 学校教育法施行規則第五十九条 says the school year begins on 1 April and
//! ends on 31 March of the following year, and the rule reaches
//! kindergartens, junior high schools, high schools and special-needs
//! schools by 準用. France's Code de l'éducation art. L. 521-1 puts the
//! calendar in the Minister's hands, who publishes it as an *arrêté*. New
//! Zealand's Education and Training Act 2020 s. 66(1) requires the Minister
//! to fix term dates by *Gazette* notice, leaving boards only a week's
//! latitude on the Term 1 start.
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
    valid_from: Some(1947),
    valid_until: None,
    note: "Fixed nationally by ministerial ordinance, which is unusual: in most countries the \
           school year is set locally. Universities may set their own 学年, and some run \
           September-start programmes.",
};

/// Build a school-year entry. Private so that every public value in this
/// module carries the full documentation the table promises.
const fn school(
    name: &'static str,
    local_name: &'static str,
    authority: Authority,
    month: u8,
    day: u8,
    note: &'static str,
) -> YearSystem {
    YearSystem {
        name,
        local_name,
        kind: SystemKind::Academic,
        authority,
        start: YearStart::gregorian(month, day),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
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
        "学校教育法施行規則第163条 gives the start and end of a university's 学年 to its 学長, \
         so April is the near-universal choice rather than the rule. September-entry \
         programmes exist and use the same hook.",
    )),
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "学校教育法施行規則（昭和22年文部省令第11号）第59条, 第39条, 第79条, 第104条, \
              第163条; 学校教育法施行令第29条 — e-Gov 法令検索",
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
        "England, Wales and Northern Ireland start in early September; Scotland in mid-August. \
         Term dates are set by the local authority or academy trust, not nationally.",
    ),
    university: Some(school(
        "United Kingdom university year",
        "",
        Authority::PerInstitution,
        9,
        1,
        "Michaelmas term begins late September or early October and varies by institution; \
         Scottish universities generally start in September.",
    )),
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Education Act 2002 s. 32 (term dates set by the local authority or governing \
              body); Education Act 1996 s. 579(1) (\"the first school term to begin after \
              July\"); mygov.scot, school term and holiday dates",
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
        "The rentrée is fixed each year by ministerial arrêté and falls in the first days of \
         September; the three zones stagger holidays within the year, not its start.",
    ),
    university: Some(school(
        "French university year",
        "année universitaire",
        Authority::PerInstitution,
        9,
        1,
        "Set by each établissement; September or early October.",
    )),
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Code de l'éducation art. L. 521-1; arrêté fixing the calendrier scolaire, \
              Journal officiel",
};

/// Germany 🇩🇪 — August by *Land*, October for the universities.
///
/// The school year is a *Land* matter and cannot constitutionally be a
/// federal one. The legal year in North Rhine-Westphalia, for instance, runs
/// 1 August to 31 July (§ 7 SchulG NRW) with the ministry issuing the
/// *Ferienordnung*; teaching resumes somewhere between early August and
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
        "The legal year runs 1 August to 31 July in the Länder whose school law the author \
         checked; teaching resumes between early August and mid-September by Land. This entry \
         is the legal boundary, not the first day of lessons.",
    ),
    university: Some(school(
        "German university year",
        "Studienjahr",
        Authority::PerRegion,
        10,
        1,
        "Wintersemester 1 October to 31 March, Sommersemester 1 April to 30 September, set by \
         Land higher-education law. The Vorlesungszeit is shorter than the semester.",
    )),
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "§ 7 SchulG NRW; Kultusministerkonferenz Ferienregelung; Landeshochschulgesetze \
              (semester dates not verified against a specific article)",
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
        "Each state and territory sets its own term dates; New South Wales uses two different \
         Term 1 start dates within the state. Late January to early February; 28 January is a \
         representative date, not a rule.",
    ),
    university: Some(school(
        "Australian university year",
        "",
        Authority::PerInstitution,
        2,
        1,
        "Semester 1 begins in late February or early March, per institution.",
    )),
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Victorian Department of Education term dates; Western Australia Government \
              Gazette future term dates; NSW Department of Education",
};

/// New Zealand 🇳🇿 — late January, by Gazette notice, with a statutory window.
///
/// The instructive contrast with Australia: the calendars look almost
/// identical and the legal footing is the opposite. Education and Training
/// Act 2020 s. 66(1), with the Education (When State Schools Must Be Open
/// and Closed) Regulations 2024, requires the Minister to fix term dates by
/// notice in the *Gazette*. Terms 2 to 4 are ministerial; boards choose only
/// the Term 1 start, and only inside a statutory window of about a week.
pub static NEW_ZEALAND: AcademicProfile = AcademicProfile {
    code: "NZ",
    english_name: "New Zealand",
    school: school(
        "New Zealand school year",
        "",
        Authority::Regulation,
        1,
        28,
        "Fixed by Gazette notice. Boards choose the Term 1 start within a statutory window of \
         roughly a week — 28 January to 4 February for 2027 — so this date is the window's \
         opening, not a fixed day.",
    ),
    university: Some(school(
        "New Zealand university year",
        "",
        Authority::PerInstitution,
        2,
        1,
        "Semester 1 begins in late February, per institution.",
    )),
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Education and Training Act 2020 s. 66(1); Education (When State Schools Must Be \
              Open and Closed) Regulations 2024 regs 6-7; New Zealand Gazette",
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
        "Two dominant patterns: an April-to-March session (much of the north and west, and \
         CBSE-affiliated schools by circular) and a June reopening (Kerala, Karnataka, Tamil \
         Nadu and much of the south, monsoon-driven, late May to early July). Saying \"India \
         starts in June\" is as over-general as saying April. No national statute fixes the \
         date; the RTE Act 2009 fixes working days, not the start.",
    ),
    university: None,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Right to Education Act 2009 s. 19 and Schedule; CBSE affiliation circulars; \
              state education department calendars",
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
        "Set by the local district board. Late July to mid-September, with a strong regional \
         gradient: the South returns in early August, the Northeast after Labor Day. This date \
         is the middle of that range and is not a claim about any district.",
    ),
    university: Some(school(
        "United States university year",
        "",
        Authority::PerInstitution,
        8,
        15,
        "Fall semester begins mid-August to early September, per institution.",
    )),
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Education Commission of the States, Instructional Time Policies; NCES state \
              education reforms tables; Pew Research Center, back-to-school dates",
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
    ALL.iter().copied().find(|profile| profile.code == code)
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> hc_calendar::Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn japans_school_year_runs_april_to_march_like_its_fiscal_year() {
        let span = JAPAN.school.span(2024).unwrap();
        assert_eq!(span.first, greg(2024, 4, 1));
        assert_eq!(span.last, greg(2025, 3, 31));
    }

    #[test]
    fn only_japan_france_and_new_zealand_claim_a_national_school_year() {
        let national: [&str; 3] = ["JP", "FR", "NZ"];
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
            let span = profile.school.span(2025).unwrap();
            assert_eq!(gregorian::year_from_fixed(span.first).unwrap(), 2025);
            assert_eq!(gregorian::year_from_fixed(span.last).unwrap(), 2026);
            // ... and ends before the next year's start, in January.
            let (_, month, _) = gregorian::from_fixed(span.last).unwrap();
            assert_eq!(month, 1);
        }
    }

    #[test]
    fn a_northern_hemisphere_school_year_straddles_two_gregorian_years() {
        for profile in [&UNITED_KINGDOM, &FRANCE, &GERMANY, &UNITED_STATES] {
            let span = profile.school.span(2025).unwrap();
            assert_eq!(gregorian::year_from_fixed(span.first).unwrap(), 2025);
            assert_eq!(gregorian::year_from_fixed(span.last).unwrap(), 2026);
        }
    }

    #[test]
    fn the_german_university_year_does_not_follow_the_german_school_year() {
        let school = GERMANY.school.span(2025).unwrap();
        let university = GERMANY.university.unwrap().span(2025).unwrap();
        assert_ne!(school.first, university.first);
        assert_eq!(school.first, greg(2025, 8, 1));
        assert_eq!(university.first, greg(2025, 10, 1));
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
                let days = profile.school.span(label).unwrap().days();
                assert!((365..=366).contains(&days), "{}", profile.code);
            }
        }
    }
}
