//! Russia — the non-working days the republics set for themselves.
//!
//! The regime is written up in `docs/systems/russia-transfers.md` in the
//! repository, in its section on the republics, with a table of all
//! twenty-one. The table itself, [`RUSSIA`](super::RUSSIA), is in
//! `europe.rs`; this file holds the republics' rules of it, each scoped to
//! its republic's ISO 3166-2 code, and the table takes them in through
//! [`RU_ALL_RULES`].
//!
//! # What a republic sets
//!
//! Article 112 of the Labour Code lists the federal holidays, and Article
//! 4(7) of the Federal Law «О свободе совести и о религиозных объединениях»
//! lets a subject of the Federation make its religious holidays non-working
//! days. Twelve republics do so, or set a day of their own, by a law;
//! the five of the North Caucasus without such a law by a decree of their
//! head or a resolution of their government, each year for the Muslim
//! festivals. What the instruments read give is carried:
//!
//! * **The fixed days** — Tatarstan's 30 August and 6 November,
//!   Bashkortostan's 11 October, Adygea's 5 October, Chechnya's 23 March
//!   and 16 April, and the rest — are rules from the first year the text
//!   read was in force on the date. Where that text is a later wording of
//!   a law that set the day, the years from the law's own to the wording's
//!   are gaps; where the instrument read is the one that set the day, the
//!   years before it are absent, and its own year a gap where it does not
//!   say when it came into force.
//! * **The movable days** — Uraza Bayram and Kurban Bayram, Radonitsa,
//!   Shagaa, Tsagan Sar, Ysyakh where it moves — are the dates the year's
//!   decree or resolution sets, a [`Listing`] of the years read, and a gap
//!   in a year whose act was not read, back to the law that made the day
//!   non-working or, where no instrument read gives that year, in every
//!   year before. The Hijri festivals' dates are each
//!   republic's announcement, on its Spiritual Administration's advice, and
//!   no rule predicts them.
//! * **A day off moved from the weekend.** Where a republic's law moves a
//!   day off that falls on a Saturday or a Sunday to the next working day,
//!   the day it moves to depends on the federal days off and the
//!   Government's transfers around it, and on the republic's own act, which
//!   was not read: a year in which one of the republic's days falls on the
//!   weekend is a gap, "Day off moved from a holiday on the weekend", and
//!   a year in which none does has nothing to move.
//! * **Who.** A day that is non-working for everyone in the republic is
//!   [`Kind::Public`]; Sakha's, Komi's and the Altai Republic's 3 July,
//!   non-working only for the republic's bodies and the organisations its
//!   budget funds, are [`Kind::Government`].

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::rule::{Days, HolidayRule, Kind, Listing, Rule, joined};

use super::europe::RU_RULES;

/// The dates the republics' annual acts set, keyed by republic and day.
/// Where the act itself was not read, the date is its entry in
/// ConsultantPlus's reference list of the regions' non-working days,
/// `consultant.ru/document/cons_doc_LAW_311098/`, as marked.
static RU_REPUBLIC_DAYS: Listing = Listing::Named(&[
    // Tatarstan: the Rais's decrees (2026, 17 January 2026 No. 17, read;
    // 2025 and 2024, No. 125 and No. 94, as ConsultantPlus lists them).
    (2024, 4, 10, "ta-uraza"),
    (2025, 3, 30, "ta-uraza"),
    (2026, 3, 20, "ta-uraza"),
    (2024, 6, 16, "ta-kurban"),
    (2025, 6, 6, "ta-kurban"),
    (2026, 5, 27, "ta-kurban"),
    // Bashkortostan: the Government's resolutions (19 June 2023 No. 353
    // for 2024 and 15 July 2025 No. 322 for 2026, read; 14 May 2024
    // No. 197 for 2025, as Garant's reference page and ConsultantPlus list
    // it).
    (2024, 4, 10, "ba-uraza"),
    (2025, 3, 30, "ba-uraza"),
    (2026, 3, 20, "ba-uraza"),
    (2024, 6, 16, "ba-kurban"),
    (2025, 6, 6, "ba-kurban"),
    (2026, 5, 27, "ba-kurban"),
    // Adygea: the Head's decrees (2026, No. 117, read, with Saturday
    // 25 April worked and Monday 20 April off; 2024 and 2025 as
    // ConsultantPlus lists them).
    (2024, 4, 10, "ad-uraza"),
    (2025, 3, 30, "ad-uraza"),
    (2026, 3, 20, "ad-uraza"),
    (2024, 5, 14, "ad-radonitsa"),
    (2025, 4, 29, "ad-radonitsa"),
    (2026, 4, 21, "ad-radonitsa"),
    (2024, 6, 16, "ad-kurban"),
    (2025, 6, 6, "ad-kurban"),
    (2026, 5, 27, "ad-kurban"),
    (2026, 4, 20, "ad-transferred"),
    (2026, 4, 25, "ad-worked"),
    // Chechnya: the Head's decrees (2026, No. 46 of 10 March and No. 86 of
    // 18 May, read; 2024 and 2025 as ConsultantPlus lists them).
    (2024, 4, 9, "ce-uraza"),
    (2024, 4, 10, "ce-uraza"),
    (2024, 4, 11, "ce-uraza"),
    (2025, 3, 30, "ce-uraza"),
    (2025, 3, 31, "ce-uraza"),
    (2025, 4, 1, "ce-uraza"),
    (2026, 3, 19, "ce-uraza"),
    (2026, 3, 20, "ce-uraza"),
    (2026, 3, 21, "ce-uraza"),
    (2024, 6, 17, "ce-kurban"),
    (2024, 6, 18, "ce-kurban"),
    (2025, 6, 6, "ce-kurban"),
    (2025, 6, 7, "ce-kurban"),
    (2025, 6, 8, "ce-kurban"),
    (2026, 5, 27, "ce-kurban"),
    (2026, 5, 28, "ce-kurban"),
    (2026, 5, 29, "ce-kurban"),
    // Dagestan: the Government's resolutions (2026, No. 47 for Uraza
    // Bayram and No. 164 for the Day of Unity, read; the rest as
    // ConsultantPlus lists them).
    (2024, 4, 10, "da-uraza"),
    (2024, 4, 11, "da-uraza"),
    (2024, 4, 12, "da-uraza"),
    (2025, 3, 31, "da-uraza"),
    (2025, 4, 1, "da-uraza"),
    (2026, 3, 19, "da-uraza"),
    (2026, 3, 20, "da-uraza"),
    (2024, 6, 17, "da-kurban"),
    (2025, 6, 6, "da-kurban"),
    (2026, 5, 27, "da-kurban"),
    (2026, 5, 28, "da-kurban"),
    (2026, 5, 29, "da-kurban"),
    (2026, 9, 15, "da-unity"),
    // Ingushetia: the Head's decrees (2026, No. 84 of 20 May for Kurban
    // Bayram, read, and No. 38 for Uraza Bayram; 2024 and 2025 as
    // ConsultantPlus lists them).
    (2024, 4, 10, "in-uraza"),
    (2024, 4, 11, "in-uraza"),
    (2024, 4, 12, "in-uraza"),
    (2025, 3, 31, "in-uraza"),
    (2025, 4, 1, "in-uraza"),
    (2026, 3, 19, "in-uraza"),
    (2026, 3, 20, "in-uraza"),
    (2026, 3, 21, "in-uraza"),
    (2024, 6, 17, "in-kurban"),
    (2025, 6, 6, "in-kurban"),
    (2025, 6, 7, "in-kurban"),
    (2026, 5, 27, "in-kurban"),
    (2026, 5, 28, "in-kurban"),
    (2026, 5, 29, "in-kurban"),
    // Kabardino-Balkaria: the Head's decrees (Kurban Bayram 2024 and
    // Uraza Bayram 2025, read; 2026 as ConsultantPlus lists it).
    (2024, 6, 17, "kb-kurban"),
    (2026, 5, 27, "kb-kurban"),
    (2025, 3, 31, "kb-uraza"),
    (2026, 3, 20, "kb-uraza"),
    (2026, 4, 21, "kb-radonitsa"),
    // Karachay-Cherkessia: the Head's decrees for 2026, No. 31 and No. 57,
    // read.
    (2026, 3, 20, "kc-uraza"),
    (2026, 5, 27, "kc-kurban"),
    // Tuva: the Supreme Khural's resolutions for Shagaa and the
    // Government's for Naadym, and No. 742 moving Constitution Day 2026 to
    // 8 May, as ConsultantPlus lists them.
    (2024, 2, 10, "ty-shagaa"),
    (2025, 3, 1, "ty-shagaa"),
    (2026, 2, 18, "ty-shagaa"),
    (2024, 7, 19, "ty-naadym"),
    (2025, 7, 18, "ty-naadym"),
    (2026, 7, 24, "ty-naadym"),
    (2026, 5, 8, "ty-constitution"),
    // Kalmykia: the Head's decrees, as ConsultantPlus lists them.
    (2024, 2, 10, "kl-tsagan-sar"),
    (2025, 2, 28, "kl-tsagan-sar"),
    (2026, 2, 18, "kl-tsagan-sar"),
    (2024, 5, 23, "kl-buddha"),
    (2025, 6, 11, "kl-buddha"),
    (2026, 5, 31, "kl-buddha"),
    (2024, 12, 25, "kl-zul"),
    (2025, 12, 14, "kl-zul"),
    // The Altai Republic: the Government's resolutions for Chaga Bayram,
    // as ConsultantPlus lists them.
    (2024, 2, 17, "al-chaga"),
    (2025, 2, 8, "al-chaga"),
    (2026, 2, 21, "al-chaga"),
    // North Ossetia–Alania: the first Monday of Uastyrdzhi, as
    // ConsultantPlus lists it (and the Head's decree No. 453 for 2026).
    (2024, 11, 18, "se-uastyrdzhi"),
    (2026, 11, 23, "se-uastyrdzhi"),
]);

/// The instruments, cited by the rules.
mod cite {
    pub(super) const TA: &str = "Закон Республики Татарстан от 19.02.1992 № 1448-XII «О праздничных днях и \
                                 памятных датах Республики Татарстан», статья 1, as restated by 39-ЗРТ \
                                 (2003) and 74-ЗРТ (2010) and amended by 67-ЗРТ (2016) and 24-ЗРТ (2023)";
    pub(super) const BA: &str = "Закон Республики Башкортостан от 27.02.1992 № ВС-10/21 «О праздничных и \
                                 памятных днях в Республике Башкортостан», статья 1, as worded by 364-з of \
                                 01.03.2011";
    pub(super) const AD: &str = "Закон Республики Адыгея от 14.02.1995 № 168-1 «О праздничных днях и \
                                 памятных датах», статьи 2 и 4, article 2 as worded by № 384 of 06.11.2020";
    pub(super) const SA: &str = "Закон Республики Саха (Якутия) от 26.04.2018 1993-З № 1545-V «О \
                                 дополнительных нерабочих праздничных днях», статья 2: for the organisations \
                                 the republic's budget funds";
    pub(super) const CE_23_MARCH: &str =
        "Указ Главы Администрации Чеченской Республики от 24.03.2003 № 34";
    pub(super) const CE_16_APRIL: &str = "Указ Президента Чеченской Республики от 04.05.2009 № 155";
    pub(super) const CE: &str = "Указы Главы Чеченской Республики, each year, «на основании обращения \
                                 Духовного управления мусульман»";
    pub(super) const DA_CONSTITUTION: &str =
        "Указ Государственного Совета Республики Дагестан от 18.07.1995 № 138";
    pub(super) const DA_KURBAN: &str = "Указ Государственного Совета Республики Дагестан от 15.03.2000 № 67, \
                                        and the Government's resolution each year";
    pub(super) const DA: &str = "Постановления Правительства Республики Дагестан, each year";
    pub(super) const IN: &str =
        "Указы Главы Республики Ингушетия, each year, under Article 4(7) of 125-ФЗ";
    pub(super) const KB_28_MARCH: &str =
        "Указ Президента Кабардино-Балкарской Республики 1994 № 19";
    pub(super) const KB_1_SEPTEMBER: &str =
        "Постановление Парламента Кабардино-Балкарской Республики 1997 № 172-П-П";
    pub(super) const KB_20_SEPTEMBER: &str =
        "Указ Главы Кабардино-Балкарской Республики 2014 № 166-УГ";
    pub(super) const KB: &str = "Указы Главы Кабардино-Балкарской Республики, each year, «в связи с обращением \
                                 Духовного управления мусульман»";
    pub(super) const KC_3_MAY: &str =
        "Указ Президента Карачаево-Черкесской Республики от 27.04.2001 № 37";
    pub(super) const KC: &str = "Указы Главы Карачаево-Черкесской Республики for 2026, № 31 и № 57";
    pub(super) const TY: &str = "Закон Республики Тыва от 12.02.1999 № 143 «О праздничных днях Республики \
                                 Тыва», статья 1, as worded in 2012 (№ 1555 ВХ-I)";
    pub(super) const KL: &str = "Закон Республики Калмыкия от 13.10.2004 № 156-III-З, статья 1, as restated \
                                 by 55-VI-З (in force 26.07.2019)";
    pub(super) const AL: &str = "Закон Республики Алтай от 24.04.2003 № 11-11, as amended by 1-РЗ (2013), \
                                 27-РЗ (2021) and 4-РЗ (2026)";
    pub(super) const SE: &str = "Закон Республики Северная Осетия-Алания от 02.10.2018 № 61-РЗ";
    pub(super) const CU: &str = "Закон Чувашской Республики от 04.05.2000 № 4";
    pub(super) const KO: &str = "Закон Республики Коми от 05.05.2014 № 30-РЗ: for the republic's bodies and \
                                 institutions";
}

/// A fixed day of one republic, from `first`.
const fn fixed(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::gregorian(month, day))
        .years(Some(first), None)
        .in_regions(region)
        .cited(source)
}

/// A day of one republic as its acts from `first` to `last` set it, a gap
/// in a later year and absent before `first`.
const fn listed(
    name: &'static str,
    local_name: &'static str,
    key: &'static str,
    first: i32,
    last: i64,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::listed(RU_REPUBLIC_DAYS.named(key), first as i64, last),
    )
    .years(Some(first), None)
    .in_regions(region)
    .cited(source)
}

/// The years from `first` to `last` of a republic's day whose acts were
/// not read: a gap in each.
const fn unread(
    name: &'static str,
    local_name: &'static str,
    first: i32,
    last: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::UNREAD)
        .years(Some(first), Some(last))
        .in_regions(region)
        .cited(source)
}

/// Every year before `first` of a republic's day whose acts before it
/// were not read, and whose first year no instrument read gives: a gap in
/// each.
const fn unread_before(
    name: &'static str,
    local_name: &'static str,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::UNREAD)
        .years(None, Some(first - 1))
        .in_regions(region)
        .cited(source)
}

/// The gap a republic's law leaves in a year one of its days off falls on
/// the weekend: the law moves it to the next working day, which the
/// federal days and the republic's own act decide. `fixed` are its fixed
/// days, and `prefix` the keys of its rows in [`RU_REPUBLIC_DAYS`].
fn moved(year: i64, fixed: &[(u8, u8)], prefix: &str) -> Option<Days> {
    let on_weekend = |day: Rd| matches!(Weekday::from_rd(day), Weekday::Saturday | Weekday::Sunday);
    let fixed_on_weekend = fixed
        .iter()
        .any(|&(month, day)| gregorian::to_fixed(year, month, day).is_ok_and(on_weekend));
    let Listing::Named(rows) = RU_REPUBLIC_DAYS else {
        return Some(Days::new());
    };
    let listed_on_weekend = rows.iter().any(|&(row_year, month, day, key)| {
        row_year == year
            && key.starts_with(prefix)
            && !key.ends_with("-worked")
            && gregorian::to_fixed(year, month, day).is_ok_and(on_weekend)
    });
    if fixed_on_weekend || listed_on_weekend {
        None
    } else {
        Some(Days::new())
    }
}

fn ta_moved(year: i64) -> Option<Days> {
    moved(year, &[(8, 30), (11, 6)], "ta-")
}
fn ba_moved(year: i64) -> Option<Days> {
    moved(year, &[(10, 11)], "ba-")
}
fn ad_moved(year: i64) -> Option<Days> {
    moved(year, &[(10, 5)], "ad-")
}
fn sa_moved(year: i64) -> Option<Days> {
    moved(year, &[(4, 27), (6, 21)], "sa-")
}
fn ty_moved(year: i64) -> Option<Days> {
    if year == 2026 {
        return moved(year, &[(8, 15)], "ty-");
    }
    moved(year, &[(5, 6), (8, 15)], "ty-")
}
fn kl_moved(year: i64) -> Option<Days> {
    moved(year, &[(7, 5)], "kl-")
}
fn al_moved(year: i64) -> Option<Days> {
    if year >= 2026 {
        moved(year, &[(7, 3)], "al-")
    } else {
        moved(year, &[], "al-")
    }
}

/// A republic's gap for a day moved off the weekend, in the years its law
/// moves one.
const fn moved_day(
    function: fn(i64) -> Option<Days>,
    first: i32,
    last: Option<i32>,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(
        "Day off moved from a holiday on the weekend",
        "Перенесённый выходной день",
        Rule::Unsettled(function),
    )
    .years(Some(first), last)
    .in_regions(region)
    .cited(source)
}

const TA: &[&str] = &["RU-TA"];
const BA: &[&str] = &["RU-BA"];
const AD: &[&str] = &["RU-AD"];
const SA: &[&str] = &["RU-SA"];
const CE: &[&str] = &["RU-CE"];
const DA: &[&str] = &["RU-DA"];
const IN: &[&str] = &["RU-IN"];
const KB: &[&str] = &["RU-KB"];
const KC: &[&str] = &["RU-KC"];
const TY: &[&str] = &["RU-TY"];
const KL: &[&str] = &["RU-KL"];
const AL: &[&str] = &["RU-AL"];
const SE: &[&str] = &["RU-SE"];
const CU: &[&str] = &["RU-CU"];
const KO: &[&str] = &["RU-KO"];

/// The republics' days, republic by republic in ISO 3166-2 order of the
/// ones that set a day.
pub(super) static RU_REPUBLIC_RULES: &[HolidayRule] = &[
    // ── Adygea ──────────────────────────────────────────────────────────
    fixed(
        "Day of the Formation of the Republic of Adygea",
        "День образования Республики Адыгея",
        10,
        5,
        2014,
        AD,
        cite::AD,
    ),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "ad-uraza",
        2024,
        2026,
        AD,
        cite::AD,
    ),
    unread(
        "Day of the Formation of the Republic of Adygea",
        "День образования Республики Адыгея",
        1995,
        2013,
        AD,
        cite::AD,
    ),
    unread("Uraza Bayram", "Ураза-Байрам", 1995, 2023, AD, cite::AD),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "ad-kurban",
        2024,
        2026,
        AD,
        cite::AD,
    ),
    unread("Kurban Bayram", "Курбан-Байрам", 2021, 2023, AD, cite::AD),
    listed(
        "Radonitsa",
        "День поминовения усопших (Радоница)",
        "ad-radonitsa",
        2024,
        2026,
        AD,
        cite::AD,
    ),
    unread(
        "Radonitsa",
        "День поминовения усопших (Радоница)",
        2021,
        2023,
        AD,
        cite::AD,
    ),
    HolidayRule::fixed_public(
        "Day off transferred by the Head",
        "Перенесённый выходной день",
        Rule::listed(RU_REPUBLIC_DAYS.named("ad-transferred"), 2026, 2026),
    )
    .years(Some(2026), Some(2026))
    .in_regions(AD)
    .cited(cite::AD),
    HolidayRule::workday(
        "Working day, a day off transferred by the Head",
        "Рабочий день",
        Rule::listed(RU_REPUBLIC_DAYS.named("ad-worked"), 2026, 2026),
    )
    .years(Some(2026), Some(2026))
    .in_regions(AD)
    .cited(cite::AD),
    moved_day(ad_moved, 2014, None, AD, cite::AD),
    // ── The Altai Republic ──────────────────────────────────────────────
    listed(
        "Chaga Bayram",
        "Чага-Байрам",
        "al-chaga",
        2024,
        2026,
        AL,
        cite::AL,
    ),
    unread("Chaga Bayram", "Чага-Байрам", 2013, 2023, AL, cite::AL),
    fixed(
        "Day of the Formation of the Altai Republic",
        "День образования Республики Алтай",
        7,
        3,
        2026,
        AL,
        cite::AL,
    )
    .of_kind(Kind::Government),
    moved_day(al_moved, 2013, None, AL, cite::AL),
    // ── Bashkortostan ───────────────────────────────────────────────────
    fixed(
        "Republic Day",
        "День Республики",
        10,
        11,
        2011,
        BA,
        cite::BA,
    ),
    listed(
        "Uraza Bayram",
        "Ураза-байрам",
        "ba-uraza",
        2024,
        2026,
        BA,
        cite::BA,
    ),
    unread("Republic Day", "День Республики", 1992, 2010, BA, cite::BA),
    unread("Uraza Bayram", "Ураза-байрам", 1992, 2023, BA, cite::BA),
    listed(
        "Kurban Bayram",
        "Курбан-байрам",
        "ba-kurban",
        2024,
        2026,
        BA,
        cite::BA,
    ),
    unread("Kurban Bayram", "Курбан-байрам", 1992, 2023, BA, cite::BA),
    moved_day(ba_moved, 2011, None, BA, cite::BA),
    // ── Chechnya ────────────────────────────────────────────────────────
    fixed(
        "Constitution Day of the Chechen Republic",
        "День Конституции Чеченской Республики",
        3,
        23,
        2004,
        CE,
        cite::CE_23_MARCH,
    ),
    fixed(
        "Day of Peace",
        "День мира",
        4,
        16,
        2010,
        CE,
        cite::CE_16_APRIL,
    ),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "ce-uraza",
        2024,
        2026,
        CE,
        cite::CE,
    ),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "ce-kurban",
        2024,
        2026,
        CE,
        cite::CE,
    ),
    unread_before("Uraza Bayram", "Ураза-Байрам", 2024, CE, cite::CE),
    unread_before("Kurban Bayram", "Курбан-Байрам", 2024, CE, cite::CE),
    // ── Chuvashia ───────────────────────────────────────────────────────
    fixed("Republic Day", "День Республики", 6, 24, 2000, CU, cite::CU),
    // ── Dagestan ────────────────────────────────────────────────────────
    fixed(
        "Constitution Day of the Republic of Dagestan",
        "День Конституции Республики Дагестан",
        7,
        26,
        1995,
        DA,
        cite::DA_CONSTITUTION,
    ),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "da-uraza",
        2024,
        2026,
        DA,
        cite::DA,
    ),
    unread("Uraza Bayram", "Ураза-Байрам", 1991, 2023, DA, cite::DA),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "da-kurban",
        2024,
        2026,
        DA,
        cite::DA_KURBAN,
    ),
    unread(
        "Kurban Bayram",
        "Курбан-Байрам",
        2000,
        2023,
        DA,
        cite::DA_KURBAN,
    ),
    listed(
        "Day of Unity of the Peoples of Dagestan",
        "День единства народов Дагестана",
        "da-unity",
        2026,
        2026,
        DA,
        cite::DA,
    ),
    unread(
        "Day of Unity of the Peoples of Dagestan",
        "День единства народов Дагестана",
        2011,
        2025,
        DA,
        cite::DA,
    ),
    // ── Ingushetia ──────────────────────────────────────────────────────
    listed(
        "Eid al-Fitr",
        "Мархаш",
        "in-uraza",
        2024,
        2026,
        IN,
        cite::IN,
    ),
    listed(
        "Eid al-Adha",
        "Гӏурба",
        "in-kurban",
        2024,
        2026,
        IN,
        cite::IN,
    ),
    unread_before("Eid al-Fitr", "Мархаш", 2024, IN, cite::IN),
    unread_before("Eid al-Adha", "Гӏурба", 2024, IN, cite::IN),
    // ── Kabardino-Balkaria ──────────────────────────────────────────────
    fixed(
        "Day of the Revival of the Balkar People",
        "День возрождения балкарского народа",
        3,
        28,
        1995,
        KB,
        cite::KB_28_MARCH,
    ),
    unread(
        "Day of the Revival of the Balkar People",
        "День возрождения балкарского народа",
        1994,
        1994,
        KB,
        cite::KB_28_MARCH,
    ),
    fixed(
        "Day of Statehood of the Kabardino-Balkarian Republic",
        "День государственности Кабардино-Балкарской Республики (День республики)",
        9,
        1,
        1998,
        KB,
        cite::KB_1_SEPTEMBER,
    ),
    unread(
        "Day of Statehood of the Kabardino-Balkarian Republic",
        "День государственности Кабардино-Балкарской Республики (День республики)",
        1997,
        1997,
        KB,
        cite::KB_1_SEPTEMBER,
    ),
    fixed(
        "Day of the Adyghe (Circassians)",
        "День адыгов (черкесов)",
        9,
        20,
        2014,
        KB,
        cite::KB_20_SEPTEMBER,
    ),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "kb-kurban",
        2024,
        2026,
        KB,
        cite::KB,
    ),
    unread("Kurban Bayram", "Курбан-Байрам", 2025, 2025, KB, cite::KB),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "kb-uraza",
        2025,
        2026,
        KB,
        cite::KB,
    ),
    unread("Uraza Bayram", "Ураза-Байрам", 2024, 2024, KB, cite::KB),
    listed(
        "Radonitsa",
        "Радоница",
        "kb-radonitsa",
        2026,
        2026,
        KB,
        cite::KB,
    ),
    unread_before("Kurban Bayram", "Курбан-Байрам", 2024, KB, cite::KB),
    unread_before("Uraza Bayram", "Ураза-Байрам", 2024, KB, cite::KB),
    unread_before("Radonitsa", "Радоница", 2024, KB, cite::KB),
    unread("Radonitsa", "Радоница", 2024, 2025, KB, cite::KB),
    // ── Kalmykia ────────────────────────────────────────────────────────
    fixed(
        "Day of the Republic of Kalmykia",
        "День Республики Калмыкия",
        7,
        5,
        2020,
        KL,
        cite::KL,
    ),
    listed(
        "Tsagan Sar",
        "Цаган Сар",
        "kl-tsagan-sar",
        2024,
        2026,
        KL,
        cite::KL,
    ),
    unread("Tsagan Sar", "Цаган Сар", 2005, 2023, KL, cite::KL),
    listed(
        "Buddha Shakyamuni's Birthday",
        "День рождения Будды Шакьямуни",
        "kl-buddha",
        2024,
        2026,
        KL,
        cite::KL,
    ),
    unread(
        "Buddha Shakyamuni's Birthday",
        "День рождения Будды Шакьямуни",
        2005,
        2023,
        KL,
        cite::KL,
    ),
    listed("Zul", "Зул", "kl-zul", 2024, 2025, KL, cite::KL),
    unread("Zul", "Зул", 2005, 2023, KL, cite::KL),
    moved_day(kl_moved, 2020, None, KL, cite::KL),
    // ── Karachay-Cherkessia ─────────────────────────────────────────────
    fixed(
        "Day of the Revival of the Karachay People",
        "День возрождения карачаевского народа",
        5,
        3,
        2002,
        KC,
        cite::KC_3_MAY,
    ),
    unread(
        "Day of the Revival of the Karachay People",
        "День возрождения карачаевского народа",
        2001,
        2001,
        KC,
        cite::KC_3_MAY,
    ),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "kc-uraza",
        2026,
        2026,
        KC,
        cite::KC,
    ),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "kc-kurban",
        2026,
        2026,
        KC,
        cite::KC,
    ),
    unread_before("Uraza Bayram", "Ураза-Байрам", 2026, KC, cite::KC),
    unread_before("Kurban Bayram", "Курбан-Байрам", 2026, KC, cite::KC),
    // ── Komi ────────────────────────────────────────────────────────────
    fixed(
        "Day of the Republic of Komi",
        "День Республики Коми",
        8,
        22,
        2014,
        KO,
        cite::KO,
    )
    .of_kind(Kind::Government),
    // ── Sakha (Yakutia) ─────────────────────────────────────────────────
    fixed(
        "Day of the Republic of Sakha (Yakutia)",
        "День Республики Саха (Якутия)",
        4,
        27,
        2019,
        SA,
        cite::SA,
    )
    .of_kind(Kind::Government),
    fixed(
        "Ysyakh",
        "День национального праздника «Ысыах»",
        6,
        21,
        2018,
        SA,
        cite::SA,
    )
    .of_kind(Kind::Government),
    moved_day(sa_moved, 2018, None, SA, cite::SA),
    // ── North Ossetia–Alania ────────────────────────────────────────────
    listed(
        "Uastyrdzhi",
        "Уастырджи (Джеоргуыба)",
        "se-uastyrdzhi",
        2024,
        2026,
        SE,
        cite::SE,
    ),
    unread(
        "Uastyrdzhi",
        "Уастырджи (Джеоргуыба)",
        2018,
        2023,
        SE,
        cite::SE,
    ),
    unread(
        "Uastyrdzhi",
        "Уастырджи (Джеоргуыба)",
        2025,
        2025,
        SE,
        cite::SE,
    ),
    // ── Tatarstan ───────────────────────────────────────────────────────
    fixed(
        "Day of the Republic of Tatarstan",
        "День Республики Татарстан",
        8,
        30,
        2004,
        TA,
        cite::TA,
    ),
    fixed(
        "Constitution Day of the Republic of Tatarstan",
        "День Конституции Республики Татарстан",
        11,
        6,
        2003,
        TA,
        cite::TA,
    ),
    listed(
        "Uraza Bayram",
        "Ураза-байрам",
        "ta-uraza",
        2024,
        2026,
        TA,
        cite::TA,
    ),
    unread("Uraza Bayram", "Ураза-байрам", 2011, 2023, TA, cite::TA),
    listed(
        "Kurban Bayram",
        "Курбан-байрам",
        "ta-kurban",
        2024,
        2026,
        TA,
        cite::TA,
    ),
    unread(
        "Day of the Republic of Tatarstan",
        "День Республики Татарстан",
        1992,
        2003,
        TA,
        cite::TA,
    ),
    unread(
        "Constitution Day of the Republic of Tatarstan",
        "День Конституции Республики Татарстан",
        1992,
        2002,
        TA,
        cite::TA,
    ),
    unread("Kurban Bayram", "Курбан-байрам", 1992, 2023, TA, cite::TA),
    // Until 67-ЗРТ of 2016 a day off on the weekend moved to the next
    // working day; from 2017 no further day is given.
    moved_day(ta_moved, 2004, Some(2016), TA, cite::TA),
    // ── Tuva ────────────────────────────────────────────────────────────
    fixed("Republic Day", "День Республики", 8, 15, 2013, TY, cite::TY),
    fixed(
        "Constitution Day",
        "День Конституции",
        5,
        6,
        2013,
        TY,
        cite::TY,
    )
    .years(Some(2013), Some(2025)),
    fixed(
        "Constitution Day",
        "День Конституции",
        5,
        6,
        2027,
        TY,
        cite::TY,
    ),
    listed(
        "Constitution Day",
        "День Конституции",
        "ty-constitution",
        2026,
        2026,
        TY,
        cite::TY,
    )
    .years(Some(2026), Some(2026)),
    unread("Republic Day", "День Республики", 1999, 2012, TY, cite::TY),
    unread(
        "Constitution Day",
        "День Конституции",
        1999,
        2012,
        TY,
        cite::TY,
    ),
    listed("Shagaa", "Шагаа", "ty-shagaa", 2024, 2026, TY, cite::TY),
    unread("Shagaa", "Шагаа", 1999, 2023, TY, cite::TY),
    listed("Naadym", "Наадым", "ty-naadym", 2024, 2026, TY, cite::TY),
    unread("Naadym", "Наадым", 1999, 2023, TY, cite::TY),
    moved_day(ty_moved, 2013, None, TY, cite::TY),
];

/// How many rules [`RUSSIA`](super::RUSSIA) has in all.
const RU_ALL_LEN: usize = RU_RULES.len() + RU_REPUBLIC_RULES.len();

/// Every rule of [`RUSSIA`](super::RUSSIA): the federal ones of `europe.rs`,
/// then the republics' here.
pub(super) static RU_ALL_RULES: [HolidayRule; RU_ALL_LEN] = joined(&[RU_RULES, RU_REPUBLIC_RULES]);
