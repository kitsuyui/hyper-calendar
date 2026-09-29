//! Nepal — the days of the Home Ministry's notices beyond section 2.1: the
//! days for one community, faith or group of employees, the day for the
//! schools, and the days kept with the offices open.
//!
//! The table itself, [`NEPAL`](super::NEPAL), is in `asia.rs`, with the
//! days the notices give every office in the country; this file holds the
//! rest of what the notices for 2080 to 2083 BS list, and the table takes
//! them in through [`NP_ALL_RULES`]. The document is
//! `docs/systems/nepal-holidays.md` in the repository, whose table lists
//! every section and item of the four notices and what is carried of each.
//!
//! # What the notices give, and to whom
//!
//! | Section | Heading | Items carried | Scope |
//! | --- | --- | --- | --- |
//! | 2.2 | सम्बन्धित धर्म, संस्कृति, भौगोलिक क्षेत्र र स्थान विशेषका लागि मात्र | Gai Jatra; Dura Mhaipru Nakuma (2083) | the group [`NEWAR`], [`DURA`] |
//! | 3 | महिला कर्मचारीका लागि मात्र | Haritalika Teej, Jitiya | the group [`WOMEN`] |
//! | 4 | शिक्षण संस्थाका लागि मात्र | Basanta Panchami | everyone, [`Kind::School`] |
//! | 6.2 | अपाङ्गता भएका कर्मचारीको लागि मात्र | the Day of Persons with Disabilities | the group [`PERSONS_WITH_DISABILITIES`] |
//! | 7.2 | सम्बन्धित धर्मावलम्बीहरूका लागि मात्र | Falgunanda Jayanti, the Prophet's birthday, Guru Nanak Jayanti | the groups [`KIRAT`], [`MUSLIMS`], [`SIKHS`] |
//! | 8 | राष्ट्रियरूपमा मनाइने तर कार्यालय खुल्ने प्रकृतिका दिवस | three days, the offices open | everyone, [`Kind::Observance`] |
//!
//! A day each notice dates the same way — Falgunanda Jayanti on Kārtika 25
//! and the section 8 days on their Bikram Sambat dates in all four, the Day
//! of Persons with Disabilities on 3 December — is a rule from 2023, the
//! first year of the first notice read, 2080 BS, and a gap in every year
//! before it, whose notices were not read. A day on a tithi — Teej,
//! Jitiya, Gai Jatra, Basanta Panchami — is the notices' own dates, a
//! [`Listing`] of the four years, and a gap in a year before or after
//! them: no rule was fitted to them. Dura Mhaipru Nakuma, first listed for
//! 2083 BS, is absent from 2023 to 2025, whose notices were read and do
//! not list it, and a gap before. Guru Nanak Jayanti is dated by the notices of
//! 2080 and 2081 BS, 27 November 2023 and 15 November 2024, which
//! [`GURU_NANAK_JAYANTI`] gives, and "the day of" it by the others; the
//! Prophet's birthday is "the day of" it in all four, predicted on the
//! tabular Hijri calendar, approximate.
//!
//! # What is not carried, and why
//!
//! * The days of section 5, जात्रा बिदा (काठमाडौं उपत्यकालाई मात्र) —
//!   Gai Jatra, Indra Jatra, Bhoto Jatra and Ghode Jatra for the Kathmandu
//!   Valley alone — and Siruwa Pawani, "झापा, मोरङ, सुनसरी, सिराहा र
//!   सप्तरी जिल्लामा": each is for districts, and ISO 3166-2:NP codes the
//!   seven provinces and no district. The Valley's three districts are in
//!   Bagmati, `NP-P3`, and are not the whole of it, and Siruwa Pawani's are
//!   in two provinces. So is Fagu Purnima's split in section 2.1, the
//!   fifty-six hill districts one day and the twenty-one Terai districts the
//!   next.
//! * Gaura Parva, in section 2.2 with no community or place printed beside
//!   it: whom the notice gives it to is not stated.
//! * Bhoto Jatra has no date in any notice, "(सो जात्रा हुने दिन)", and
//!   Siruwa Pawani none either.
//! * Section 9's days for the missions abroad, which each mission sets, and
//!   the six days a province may declare under the notes (द्रष्टव्य), which
//!   are the provinces' own instruments, not read.

use crate::group::{DURA, KIRAT, MUSLIMS, NEWAR, PERSONS_WITH_DISABILITIES, SIKHS, WOMEN};
use crate::hindu::GURU_NANAK_JAYANTI;
use crate::rule::{CalendarSystem, HolidayRule, Kind, Listing, Rule, joined};

use super::asia::NP_RULES;

/// The first Gregorian year of the first notice read, 2080 BS.
const NP_NOTICES_FIRST: i32 = 2023;

/// The Prophet's birthday, 12 Rabīʿ al-Awwal, on the tabular calendar.
const NP_MAWLID: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 3, 12);

/// The days the notices for 2080 to 2083 BS date on a tithi, as they date
/// them, each with its Bikram Sambat date and weekday as printed.
static NP_NOTICE_DAYS: Listing = Listing::Named(&[
    // Section 2.2: Gai Jatra, "देशभरका नेवार समुदायका लागि मात्र".
    (2023, 8, 31, "gai-jatra"), // 2080 भदौ १४ बिहीबार
    (2024, 8, 20, "gai-jatra"), // 2081 भदौ ४ मङ्गलबार
    (2025, 8, 10, "gai-jatra"), // 2082 साउन २५ आइतबार
    (2026, 8, 29, "gai-jatra"), // 2083 भदौ १३ शनिबार
    // Section 3: Haritalika Teej.
    (2023, 9, 18, "teej"), // 2080 असोज १ सोमबार
    (2024, 9, 6, "teej"),  // 2081 भदौ २१ शुक्रबार
    (2025, 8, 26, "teej"), // 2082 भदौ १० मङ्गलबार
    (2026, 9, 14, "teej"), // 2083 भदौ २९ सोमबार
    // Section 3: Jitiya, "जितिया पर्व मनाउने महिला कर्मचारीको लागि".
    (2023, 10, 7, "jitiya"), // 2080 असोज २० शनिबार
    (2024, 9, 25, "jitiya"), // 2081 असोज ९ बुधबार
    (2025, 9, 15, "jitiya"), // 2082 भदौ ३० सोमबार
    (2026, 10, 4, "jitiya"), // 2083 असोज १८ आइतबार
    // Section 4: Basanta Panchami, for the educational institutions. The
    // notice for 2080 dates it in 2024, and the first year is 2024.
    (2024, 2, 14, "basanta-panchami"), // 2080 फागुन २ बुधबार
    (2025, 2, 3, "basanta-panchami"),  // 2081 माघ २१ सोमबार
    (2026, 1, 23, "basanta-panchami"), // 2082 माघ ९ शुक्रबार
    (2027, 2, 11, "basanta-panchami"), // 2083 माघ २८ बिहीबार
    // Section 2.2 of the notice for 2083 alone: Dura Mhaipru Nakuma,
    // "देशभरका दुरा समुदायका लागि मात्र".
    (2026, 12, 30, "dura"), // 2083 पुस १५ बुधबार
]);

/// A day of [`NP_NOTICE_DAYS`], in the years its rows cover, and a gap in
/// any other: the notices before 2080 BS were not read.
const fn np_listed(
    name: &'static str,
    local_name: &'static str,
    key: &'static str,
    first: i32,
    last: i64,
) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::listed(NP_NOTICE_DAYS.named(key), first as i64, last),
    )
}

/// A day the notices date in the Bikram Sambat, the same in all four,
/// answered from the first notice read, the years before a gap.
const fn np_bs(name: &'static str, local_name: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::in_calendar(CalendarSystem::BIKRAM_SAMBAT, month, day),
    )
    .read_from(NP_NOTICES_FIRST)
}

/// The days of the notices' sections 2.2, 3, 4, 6.2, 7.2 and 8, in that
/// order. The local names are the notices'.
pub(super) static NP_SECTION_RULES: &[HolidayRule] = &[
    np_listed("Gai Jatra", "गाईजात्रा", "gai-jatra", 2023, 2026)
        .for_groups(&[NEWAR])
        .cited("गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 2.2: देशभरका नेवार समुदायका लागि मात्र"),
    // The notices for 2080 to 2082 BS, read, do not list it.
    np_listed("Dura Mhaipru Nakuma", "दुरा म्हैप्रु नकुमा", "dura", 2026, 2026)
        .years(Some(2026), None)
        .for_groups(&[DURA])
        .cited("गृह मन्त्रालय, २०८३ सालको सार्वजनिक विदा, 2.2 (ग): देशभरका दुरा समुदायका लागि मात्र"),
    // Not in the notices read before 2083 BS, and a gap before those.
    HolidayRule::fixed_public("Dura Mhaipru Nakuma", "दुरा म्हैप्रु नकुमा", Rule::NO_DAY)
        .years(None, Some(2025))
        .read_from(NP_NOTICES_FIRST)
        .for_groups(&[DURA]),
    np_listed("Haritalika Teej", "हरितालिका (तीज) व्रत", "teej", 2023, 2026)
        .for_groups(&[WOMEN])
        .cited("गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 3 (क): महिला कर्मचारीका लागि मात्र"),
    np_listed("Jitiya", "जितिया पर्व", "jitiya", 2023, 2026)
        .for_groups(&[WOMEN])
        .cited(
            "गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 3 (ख): जितिया पर्व मनाउने महिला कर्मचारीको लागि",
        ),
    np_listed(
        "Basanta Panchami",
        "वसन्त पञ्चमी",
        "basanta-panchami",
        2024,
        2027,
    )
    .of_kind(Kind::School)
    .cited("गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 4: शिक्षण संस्थाका लागि मात्र"),
    HolidayRule::fixed_public(
        "International Day of Persons with Disabilities",
        "अन्तर्राष्ट्रिय अपाङ्गता दिवस",
        Rule::gregorian(12, 3),
    )
    .read_from(NP_NOTICES_FIRST)
    .for_groups(&[PERSONS_WITH_DISABILITIES])
    .cited(
        "गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 6.2: अपाङ्गता भएका नेपालभित्रका सम्पूर्ण कर्मचारीको लागि",
    ),
    np_bs("Falgunanda Jayanti", "फाल्गुनन्द जयन्ती", 7, 25)
        .for_groups(&[KIRAT])
        .cited("गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 7.2 (क): किराँत धर्मावलम्बी"),
    HolidayRule::fixed_public("Prophet Muhammad's Birthday", "मोहम्मद जयन्ती", NP_MAWLID)
        .approximate()
        .read_from(NP_NOTICES_FIRST)
        .for_groups(&[MUSLIMS])
        .cited("गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 7.2 (ख): नेपाली मुस्लिम धर्मावलम्बी"),
    HolidayRule::fixed_public("Guru Nanak Jayanti", "गुरु नानक जयन्ती", GURU_NANAK_JAYANTI)
        .approximate()
        .read_from(NP_NOTICES_FIRST)
        .for_groups(&[SIKHS])
        .cited("गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 7.2 (ग): नेपाली सिख धर्मावलम्बी"),
    np_bs(
        "National Day for the Elimination of Caste Discrimination and Untouchability",
        "जातीय भेदभाव तथा छुवाछुत उन्मूलन राष्ट्रिय दिवस",
        2,
        21,
    )
    .of_kind(Kind::Observance)
    .cited("गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 8: कार्यालय खुल्ने प्रकृतिका दिवस"),
    np_bs("Civil Service Day", "निजामती सेवा दिवस", 5, 22)
        .of_kind(Kind::Observance)
        .cited("गृह मन्त्रालय, सार्वजनिक बिदा सम्बन्धी सूचना, 8: कार्यालय खुल्ने प्रकृतिका दिवस"),
    HolidayRule::observance(
        "Gen Z Martyrs' Day",
        "जेनजी सहिद दिवस",
        Rule::in_calendar(CalendarSystem::BIKRAM_SAMBAT, 5, 23),
    )
    .years(Some(2026), None)
    .cited("गृह मन्त्रालय, २०८३ सालको सार्वजनिक विदा, 8 (ग): कार्यालय खुल्ने प्रकृतिका दिवस"),
];

/// How many rules [`NEPAL`](super::NEPAL) has in all.
const NP_ALL_LEN: usize = NP_RULES.len() + NP_SECTION_RULES.len();

/// Every rule of [`NEPAL`](super::NEPAL): section 2.1's of `asia.rs`, then
/// the other sections' here.
pub(super) static NP_ALL_RULES: [HolidayRule; NP_ALL_LEN] = joined(&[NP_RULES, NP_SECTION_RULES]);
