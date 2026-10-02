//! Bangladesh — the optional holidays (ঐচ্ছিক ছুটি) of the Ministry of
//! Public Administration's annual notifications, each given to the people
//! of the faith or community whose section lists it.
//!
//! The table itself, [`BANGLADESH`](super::BANGLADESH), is in `asia.rs`,
//! with the general and executive-order holidays every office keeps; this
//! file holds the optional holidays and the table takes them in through
//! [`BD_ALL_RULES`]. The document is `docs/systems/bangladesh-holidays.md`
//! in the repository.
//!
//! # What the notifications say
//!
//! Besides the general holidays and the executive-order holidays, each
//! notification lists optional holidays in five sections: the Muslim,
//! Hindu, Christian and Buddhist sections, and one for the employees of
//! the small ethnic groups (ক্ষুদ্র নৃগোষ্ঠী) in the Chittagong Hill Tracts
//! and outside them. Each section lists its own days, and the totals are
//! five, nine, eight, seven and two. An employee may enjoy at most three
//! days of the section of their own religion in a year ("নিজ ধর্ম অনুযায়ী
//! অনধিক তিন দিনের ঐচ্ছিক ছুটি"), approved at the start of the year by
//! the competent authority, and may join them to the general holidays, the
//! executive-order holidays and the weekly holidays. A day is thus the
//! employee's to take on application, not a day off for the group:
//! [`Kind::Religious`], which [`Kind::is_day_off`] does not count, so
//! business-day arithmetic treats it as a working day. The cap of three is
//! this note's, and no computation: which three an employee chose is
//! theirs and their authority's.
//!
//! # Where the notifications were read
//!
//! The notifications are published on mopa.gov.bd as PDF files, which were
//! not opened. The days are those of the daily newspapers' reproductions of
//! the lists — bdnews24.com, Prothom Alo, Dainik Bangla, BVNews24 and
//! others for 2025, Ajker Patrika and Ekhon TV, with the weekdays, for 2026
//! — read as HTML, and so secondary. Where the reproductions of a list
//! disagree with each other, or with the day the list names, the day is not
//! carried and the year is a gap:
//!
//! * Shab-e-Meraj 2025 is printed as 28 February by four outlets and as
//!   28 January by two; 15 February is that year's Shab-e-Barat, 15 Sha'ban,
//!   so 27 Rajab cannot be 28 February.
//! * Akheri Chahar Shomba 2025 is printed as 20 September by every outlet,
//!   a Saturday: the last Wednesday of Safar 1447 fell in August.
//!
//! # Scope
//!
//! The days of a section are for its group alone, as [`HolidayRule::for_groups`]
//! says ([ADR 0011](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0011-a-day-for-one-group-is-a-scoped-rule.md)).
//! The 2026 notification lists Chaitra Sankranti, 13 April, among the
//! Buddhist days "for all districts but the three hill ones", where it is a
//! general holiday: the Buddhist rule is [`HolidayRule::except_in`] those
//! districts, and the 2025 one, whose notification makes no such
//! exception, is not.

use crate::group::{BUDDHISTS, CHRISTIANS, Group, HINDUS, MUSLIMS, SMALL_ETHNIC_GROUPS};
use crate::rule::{HolidayRule, Kind, Listing, Rule, joined};

use super::asia::BD_RULES;

/// The first year of the notifications read.
const FIRST: i64 = 2025;
/// The last.
const LAST: i64 = 2026;

/// The Muslim section's group.
const MUSLIM: &[Group] = &[MUSLIMS];
/// The Hindu section's.
const HINDU: &[Group] = &[HINDUS];
/// The Christian section's.
const CHRISTIAN: &[Group] = &[CHRISTIANS];
/// The Buddhist section's.
const BUDDHIST: &[Group] = &[BUDDHISTS];
/// The small ethnic groups' section's.
const SMALL_ETHNIC: &[Group] = &[SMALL_ETHNIC_GROUPS];

/// The three hill districts, Bandarban, Khagrachhari and Rangamati, whose
/// Chaitra Sankranti is a general holiday from 2026.
const HILL_DISTRICTS: &[&str] = &["BD-01", "BD-29", "BD-56"];

/// The optional holidays as the notifications list them: `(year, month,
/// day, name)`, a holiday of two days with a row for each.
static BD_OPTIONAL: Listing = Listing::Named(&[
    // The Muslim section. Eid-ul-Fitr's third day after is 3 April 2025,
    // 31 March plus three, and 24 March 2026; Eid-ul-Azha's fourth, 11 June
    // 2025 and 1 June 2026. Shab-e-Meraj and Akheri Chahar Shomba are not
    // carried for 2025: see the module's note.
    (2026, 1, 17, "Shab-e-Meraj"),
    (2025, 4, 3, "Eid-ul-Fitr (third day after)"),
    (2026, 3, 24, "Eid-ul-Fitr (third day after)"),
    (2025, 6, 11, "Eid-ul-Azha (fourth day after)"),
    (2026, 6, 1, "Eid-ul-Azha (fourth day after)"),
    (2026, 8, 12, "Akheri Chahar Shomba"),
    (2025, 10, 4, "Fateha-e-Yazdahum"),
    (2026, 9, 24, "Fateha-e-Yazdahum"),
    // The Hindu section.
    (2025, 2, 3, "Saraswati Puja"),
    (2026, 1, 23, "Saraswati Puja"),
    (2025, 2, 26, "Shivaratri Brata"),
    (2026, 2, 15, "Shivaratri Brata"),
    (2025, 3, 14, "Dolyatra"),
    (2026, 3, 3, "Dolyatra"),
    (2025, 3, 27, "Appearance of Harichand Thakur"),
    (2026, 3, 17, "Appearance of Harichand Thakur"),
    (2025, 9, 21, "Mahalaya"),
    (2026, 10, 10, "Mahalaya"),
    (2025, 9, 29, "Durga Puja (Saptami and Ashtami)"),
    (2025, 9, 30, "Durga Puja (Saptami and Ashtami)"),
    (2026, 10, 18, "Durga Puja (Saptami and Ashtami)"),
    (2026, 10, 19, "Durga Puja (Saptami and Ashtami)"),
    (2025, 10, 6, "Lakshmi Puja"),
    (2026, 10, 25, "Lakshmi Puja"),
    (2025, 10, 31, "Shyama Puja"),
    (2026, 11, 8, "Shyama Puja"),
    // The Christian section.
    (2025, 1, 1, "English New Year's Day"),
    (2026, 1, 1, "English New Year's Day"),
    (2025, 3, 5, "Ash Wednesday"),
    (2026, 2, 18, "Ash Wednesday"),
    (2025, 4, 17, "Holy Thursday"),
    (2026, 4, 2, "Holy Thursday"),
    (2025, 4, 18, "Good Friday"),
    (2026, 4, 3, "Good Friday"),
    (2025, 4, 19, "Holy Saturday"),
    (2026, 4, 4, "Holy Saturday"),
    (2025, 4, 20, "Easter Sunday"),
    (2026, 4, 5, "Easter Sunday"),
    (2025, 12, 24, "Christmas (the day before and the day after)"),
    (2025, 12, 26, "Christmas (the day before and the day after)"),
    (2026, 12, 24, "Christmas (the day before and the day after)"),
    (2026, 12, 26, "Christmas (the day before and the day after)"),
    // The Buddhist section. The days around Buddha Purnima are those of the
    // general holiday: 11 May 2025 and 1 May 2026.
    (2025, 2, 11, "Magha Purnima"),
    (2026, 2, 1, "Magha Purnima"),
    (2025, 4, 13, "Chaitra Sankranti"),
    (2026, 4, 13, "Chaitra Sankranti"),
    (
        2025,
        5,
        10,
        "Buddha Purnima (the day before and the day after)",
    ),
    (
        2025,
        5,
        12,
        "Buddha Purnima (the day before and the day after)",
    ),
    (
        2026,
        4,
        30,
        "Buddha Purnima (the day before and the day after)",
    ),
    (
        2026,
        5,
        2,
        "Buddha Purnima (the day before and the day after)",
    ),
    (2025, 7, 9, "Asalhi Purnima"),
    (2026, 7, 29, "Asalhi Purnima"),
    (2025, 9, 6, "Madhu Purnima"),
    (2026, 9, 26, "Madhu Purnima"),
    (2025, 10, 5, "Probarana Purnima"),
    (2026, 10, 25, "Probarana Purnima"),
    // The small ethnic groups' section: Boisabi and the like, 12 and 15
    // April in both years.
    (2025, 4, 12, "Boisabi and the like"),
    (2025, 4, 15, "Boisabi and the like"),
    (2026, 4, 12, "Boisabi and the like"),
    (2026, 4, 15, "Boisabi and the like"),
]);

/// A faith's optional holiday for the years the notifications are read:
/// leave on application, so [`Kind::Religious`], and a gap in any other
/// year.
const fn bd_optional(
    name: &'static str,
    local: &'static str,
    groups: &'static [Group],
    section: &'static str,
) -> HolidayRule {
    bd_optional_in(name, local, groups, section, FIRST, LAST)
}

/// The same for the years from `first` to `last`.
const fn bd_optional_in(
    name: &'static str,
    local: &'static str,
    groups: &'static [Group],
    section: &'static str,
    first: i64,
    last: i64,
) -> HolidayRule {
    HolidayRule::observance(
        name,
        local,
        Rule::listed(BD_OPTIONAL.named(name), first, last),
    )
    .of_kind(Kind::Religious)
    .for_groups(groups)
    .cited(section)
}

/// The citations of the five sections.
const MUSLIM_SECTION: &str = "Ministry of Public Administration, the holiday notifications for 2025 and 2026, \
     optional holidays, Muslim section (ঐচ্ছিক ছুটি, মুসলিম পর্ব): at most three days \
     of one's own religion";
const HINDU_SECTION: &str = "Ministry of Public Administration, the holiday notifications for 2025 and 2026, \
     optional holidays, Hindu section (ঐচ্ছিক ছুটি, হিন্দু পর্ব): at most three days \
     of one's own religion";
const CHRISTIAN_SECTION: &str = "Ministry of Public Administration, the holiday notifications for 2025 and 2026, \
     optional holidays, Christian section (ঐচ্ছিক ছুটি, খ্রিষ্টান পর্ব): at most three days \
     of one's own religion";
const BUDDHIST_SECTION: &str = "Ministry of Public Administration, the holiday notifications for 2025 and 2026, \
     optional holidays, Buddhist section (ঐচ্ছিক ছুটি, বৌদ্ধ পর্ব): at most three days \
     of one's own religion";
const SMALL_ETHNIC_SECTION: &str = "Ministry of Public Administration, the holiday notifications for 2025 and 2026, \
     optional holidays for the employees of the small ethnic groups in the Chittagong Hill \
     Tracts and outside them (পার্বত্য চট্টগ্রাম এলাকা ও এর বাইরে ক্ষুদ্র নৃগোষ্ঠীর অন্তর্ভুক্ত কর্মচারী)";

/// The optional holidays of both notifications, section by section.
pub(super) static BD_OPTIONAL_RULES: &[HolidayRule] = &[
    // The Muslim section.
    bd_optional_in(
        "Shab-e-Meraj",
        "শবে মেরাজ",
        MUSLIM,
        MUSLIM_SECTION,
        2026,
        LAST,
    ),
    bd_optional(
        "Eid-ul-Fitr (third day after)",
        "ঈদ-উল-ফিতর (ঈদের পরের তৃতীয় দিন)",
        MUSLIM,
        MUSLIM_SECTION,
    ),
    bd_optional(
        "Eid-ul-Azha (fourth day after)",
        "ঈদ-উল-আজহা (ঈদের পরের চতুর্থ দিন)",
        MUSLIM,
        MUSLIM_SECTION,
    ),
    bd_optional_in(
        "Akheri Chahar Shomba",
        "আখেরি চাহার সোম্বা",
        MUSLIM,
        MUSLIM_SECTION,
        2026,
        LAST,
    ),
    bd_optional("Fateha-e-Yazdahum", "ফাতেহা-ই-ইয়াজদাহম", MUSLIM, MUSLIM_SECTION),
    // The Hindu section.
    bd_optional("Saraswati Puja", "সরস্বতী পূজা", HINDU, HINDU_SECTION),
    bd_optional("Shivaratri Brata", "শিবরাত্রি ব্রত", HINDU, HINDU_SECTION),
    bd_optional("Dolyatra", "দোলযাত্রা", HINDU, HINDU_SECTION),
    bd_optional(
        "Appearance of Harichand Thakur",
        "হরিচাঁদ ঠাকুরের আবির্ভাব",
        HINDU,
        HINDU_SECTION,
    ),
    bd_optional("Mahalaya", "মহালয়া", HINDU, HINDU_SECTION),
    bd_optional(
        "Durga Puja (Saptami and Ashtami)",
        "দুর্গাপূজা (সপ্তমী ও অষ্টমী)",
        HINDU,
        HINDU_SECTION,
    ),
    bd_optional("Lakshmi Puja", "লক্ষ্মীপূজা", HINDU, HINDU_SECTION),
    bd_optional("Shyama Puja", "শ্যামাপূজা", HINDU, HINDU_SECTION),
    // The Christian section.
    bd_optional(
        "English New Year's Day",
        "ইংরেজি নববর্ষ",
        CHRISTIAN,
        CHRISTIAN_SECTION,
    ),
    bd_optional("Ash Wednesday", "ভস্ম বুধবার", CHRISTIAN, CHRISTIAN_SECTION),
    bd_optional(
        "Holy Thursday",
        "পুণ্য বৃহস্পতিবার",
        CHRISTIAN,
        CHRISTIAN_SECTION,
    ),
    bd_optional("Good Friday", "পুণ্য শুক্রবার", CHRISTIAN, CHRISTIAN_SECTION),
    bd_optional("Holy Saturday", "পুণ্য শনিবার", CHRISTIAN, CHRISTIAN_SECTION),
    bd_optional("Easter Sunday", "ইস্টার সানডে", CHRISTIAN, CHRISTIAN_SECTION),
    bd_optional(
        "Christmas (the day before and the day after)",
        "যিশু খ্রিষ্টের জন্মোৎসব (বড়দিনের আগের ও পরের দিন)",
        CHRISTIAN,
        CHRISTIAN_SECTION,
    ),
    // The Buddhist section.
    bd_optional("Magha Purnima", "মাঘী পূর্ণিমা", BUDDHIST, BUDDHIST_SECTION),
    // The 2025 notification lists Chaitra Sankranti with no exception; the
    // 2026 one lists it for every district but the three hill ones.
    bd_optional_in(
        "Chaitra Sankranti",
        "চৈত্র সংক্রান্তি",
        BUDDHIST,
        BUDDHIST_SECTION,
        FIRST,
        FIRST,
    )
    .years(None, Some(2025)),
    bd_optional_in(
        "Chaitra Sankranti",
        "চৈত্র সংক্রান্তি",
        BUDDHIST,
        BUDDHIST_SECTION,
        LAST,
        LAST,
    )
    .years(Some(2026), None)
    .except_in(HILL_DISTRICTS),
    bd_optional(
        "Buddha Purnima (the day before and the day after)",
        "বুদ্ধ পূর্ণিমা (পূর্বের ও পরের দিন)",
        BUDDHIST,
        BUDDHIST_SECTION,
    ),
    bd_optional("Asalhi Purnima", "আষাঢ়ী পূর্ণিমা", BUDDHIST, BUDDHIST_SECTION),
    bd_optional("Madhu Purnima", "মধু পূর্ণিমা", BUDDHIST, BUDDHIST_SECTION),
    bd_optional(
        "Probarana Purnima",
        "প্রবারণা পূর্ণিমা (আশ্বিনী পূর্ণিমা)",
        BUDDHIST,
        BUDDHIST_SECTION,
    ),
    // The small ethnic groups' section. A social festival, not a religious
    // day.
    bd_optional(
        "Boisabi and the like",
        "বৈসাবি ও পার্বত্য চট্টগ্রামের অন্যান্য ক্ষুদ্র নৃগোষ্ঠীর অনুরূপ সামাজিক উৎসব",
        SMALL_ETHNIC,
        SMALL_ETHNIC_SECTION,
    )
    .of_kind(Kind::Observance),
];

/// How many rules [`BANGLADESH`](super::BANGLADESH) has in all.
const BD_ALL_LEN: usize = BD_RULES.len() + BD_OPTIONAL_RULES.len();

/// Every rule of [`BANGLADESH`](super::BANGLADESH): the general and
/// executive-order holidays of `asia.rs`, then the optional holidays here.
pub(super) static BD_ALL_RULES: [HolidayRule; BD_ALL_LEN] = joined(&[BD_RULES, BD_OPTIONAL_RULES]);
