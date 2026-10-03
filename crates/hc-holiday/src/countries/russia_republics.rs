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
//! days. Eleven republics do so, or set a day of their own, by a law;
//! the five of the North Caucasus without such a law by a decree of their
//! head or a resolution of their government, each year for the Muslim
//! festivals; four more, Karelia, Khakassia, Mari El and Udmurtia, have a
//! law that gives no day off and are read as keeping none. What the
//! instruments read give is carried:
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
//!   Shagaa, Sagaalgan, Tsagan Sar, Ysyakh where it moves — are the dates
//!   the year's decree or resolution sets, a [`Listing`] of the years read,
//!   and a gap in a year whose act was not read, back to the law that made
//!   the day non-working or, where no instrument read gives that year, in
//!   every year before. The years read reach back to 2012 for Buryatia, to
//!   2014 for Altai and Kalmykia, and to between 2015 and 2017 for the rest,
//!   where an act's text, a list of its title and date, or the government's
//!   announcement of it was read. The Hijri festivals' dates are each
//!   republic's announcement, on its Spiritual Administration's advice, and
//!   no rule predicts them.
//! * **A day off moved from the weekend.** Where a republic's law moves a
//!   day off that falls on a Saturday or a Sunday to the next working day,
//!   the day it moves to depends on the federal days off and the
//!   Government's transfers around it, and on the republic's own act, which
//!   was not read: a year in which one of the republic's days falls on the
//!   weekend is a gap, "Day off moved from a holiday on the weekend", and
//!   a year in which none does has nothing to move. Where an act that was
//!   read names the day (Buryatia, Dagestan, Tuva, Adygea 2023, Kalmykia
//!   2020), it is an entry of its own, a `Day off transferred ...`, and a
//!   weekend day worked in exchange a [`Kind::Workday`]; a year is a gap
//!   only where more of the republic's days fall on the weekend than the
//!   acts read move.
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
    // Tatarstan: the President's and then the Rais's decrees «Об определении
    // дней проведения праздников Ураза-байрам и Курбан-байрам», which give the
    // day each festival begins (2015 No. УП-569, 2016 No. УП-525, 2017
    // No. УП-401, 2018 No. УП-355, 2019 No. УП-244, 2020 No. УП-265, 2021
    // No. УП-115, 2022 No. УП-256 and 2023 No. 189, read; 2026, 17 January
    // 2026 No. 17, read; 2025 and 2024, No. 125 and No. 94, as ConsultantPlus
    // lists them).
    (2015, 7, 17, "ta-uraza"),
    (2016, 7, 5, "ta-uraza"),
    (2017, 6, 25, "ta-uraza"),
    (2018, 6, 15, "ta-uraza"),
    (2019, 6, 4, "ta-uraza"),
    (2020, 5, 24, "ta-uraza"),
    (2021, 5, 13, "ta-uraza"),
    (2022, 5, 2, "ta-uraza"),
    (2023, 4, 21, "ta-uraza"),
    (2015, 9, 24, "ta-kurban"),
    (2016, 9, 12, "ta-kurban"),
    (2017, 9, 1, "ta-kurban"),
    (2018, 8, 21, "ta-kurban"),
    (2019, 8, 11, "ta-kurban"),
    (2020, 7, 31, "ta-kurban"),
    (2021, 7, 20, "ta-kurban"),
    (2022, 7, 9, "ta-kurban"),
    (2023, 6, 28, "ta-kurban"),
    (2024, 4, 10, "ta-uraza"),
    (2025, 3, 30, "ta-uraza"),
    (2026, 3, 20, "ta-uraza"),
    (2024, 6, 16, "ta-kurban"),
    (2025, 6, 6, "ta-kurban"),
    (2026, 5, 27, "ta-kurban"),
    // Bashkortostan: the Government's resolutions (26 September 2022 No. 567
    // for 2023, 2 September 2021 No. 431 for 2022, 19 August 2020 No. 507 for
    // 2021, 18 October 2016 No. 444 for 2017, 26 October 2015 No. 452 for 2016
    // and 15 October 2014 No. 466 for 2015, read; 19 June 2023 No. 353
    // for 2024 and 15 July 2025 No. 322 for 2026, read; 14 May 2024
    // No. 197 for 2025, as Garant's reference page and ConsultantPlus list
    // it). The resolutions for 2018, 2019 and 2020 were not read.
    (2015, 7, 17, "ba-uraza"),
    (2016, 7, 5, "ba-uraza"),
    (2017, 6, 25, "ba-uraza"),
    (2021, 5, 13, "ba-uraza"),
    (2022, 5, 2, "ba-uraza"),
    (2023, 4, 21, "ba-uraza"),
    (2015, 9, 24, "ba-kurban"),
    (2016, 9, 12, "ba-kurban"),
    (2017, 9, 1, "ba-kurban"),
    (2021, 7, 20, "ba-kurban"),
    (2022, 7, 9, "ba-kurban"),
    (2023, 6, 28, "ba-kurban"),
    (2024, 4, 10, "ba-uraza"),
    (2025, 3, 30, "ba-uraza"),
    (2026, 3, 20, "ba-uraza"),
    (2024, 6, 16, "ba-kurban"),
    (2025, 6, 6, "ba-kurban"),
    (2026, 5, 27, "ba-kurban"),
    // Buryatia: the Head's decrees «О празднике Белого месяца
    // "Сагаалган"» (the President's No. 11 of 01.02.2012; the Head's No. 216
    // of 19.12.2014, No. 260 of 11.12.2017, No. 219 of 28.11.2018, No. 242 of
    // 06.12.2019, No. 263 of 21.12.2020, No. 255 of 05.12.2022, No. 231 of
    // 25.12.2023, No. 1 of 09.01.2025 and No. 320 of 05.12.2025). The texts of
    // 2015, 2021 and 2023 were read; the others by Garant's note of what the
    // decree declares, as the table cites. 2022 (No. 341 of 15.12.2021)
    // was not read.
    (2012, 2, 22, "bu-sagaalgan"),
    (2015, 2, 19, "bu-sagaalgan"),
    (2018, 2, 16, "bu-sagaalgan"),
    (2019, 2, 5, "bu-sagaalgan"),
    (2020, 2, 24, "bu-sagaalgan"),
    (2021, 2, 12, "bu-sagaalgan"),
    (2023, 2, 21, "bu-sagaalgan"),
    (2024, 2, 10, "bu-sagaalgan"),
    (2025, 3, 1, "bu-sagaalgan"),
    (2026, 2, 18, "bu-sagaalgan"),
    // The days off the decrees move: Monday 24 February 2020 was already a
    // day off (the carry-over of 23 February), and Saturdays 10 February
    // 2024 and 1 March 2025 are weekend days.
    (2020, 2, 25, "bu-transferred"),
    (2024, 2, 12, "bu-transferred"),
    (2025, 3, 3, "bu-transferred"),
    // Adygea: the Head's decrees (2023, No. 113 of 03.10.2022, read, with
    // Saturday 15 April worked and Monday 24 April off, and Sunday 1 October
    // worked and Friday 6 October off; 2026, No. 117, read, with Saturday
    // 25 April worked and Monday 20 April off; 2024 and 2025 as
    // ConsultantPlus lists them).
    (2023, 4, 21, "ad-uraza"),
    (2024, 4, 10, "ad-uraza"),
    (2025, 3, 30, "ad-uraza"),
    (2026, 3, 20, "ad-uraza"),
    (2023, 4, 25, "ad-radonitsa"),
    (2024, 5, 14, "ad-radonitsa"),
    (2025, 4, 29, "ad-radonitsa"),
    (2026, 4, 21, "ad-radonitsa"),
    (2023, 6, 28, "ad-kurban"),
    (2024, 6, 16, "ad-kurban"),
    (2025, 6, 6, "ad-kurban"),
    (2026, 5, 27, "ad-kurban"),
    (2023, 4, 24, "ad-transferred"),
    (2023, 10, 6, "ad-transferred"),
    (2026, 4, 20, "ad-transferred"),
    (2023, 4, 15, "ad-worked"),
    (2023, 10, 1, "ad-worked"),
    (2026, 4, 25, "ad-worked"),
    // Chechnya: the Head's decrees (2026, No. 46 of 10 March and No. 86 of
    // 18 May, read; 2024 and 2025 as ConsultantPlus lists them).
    // Earlier years: the Head's decrees «О нерабочих (праздничных) днях» of
    // 29.06.2016 No. 90, 05.09.2016 No. 133 (as Garant's list of the days
    // gives them, in the Internet Archive's copy of 2017), 29.08.2017
    // No. 146, 30.05.2018 No. 93 and 08.04.2023 No. 53 (texts read), 18.05.2020
    // No. 96 (text on Rossiyskaya Gazeta's site, read) and 15.06.2023 No. 95
    // (Garant's list, in its copy of 2023).
    (2016, 7, 5, "ce-uraza"),
    (2016, 7, 6, "ce-uraza"),
    (2016, 7, 7, "ce-uraza"),
    (2016, 7, 8, "ce-uraza"),
    (2018, 6, 15, "ce-uraza"),
    (2018, 6, 16, "ce-uraza"),
    (2018, 6, 17, "ce-uraza"),
    (2018, 6, 18, "ce-uraza"),
    (2020, 5, 23, "ce-uraza"),
    (2020, 5, 24, "ce-uraza"),
    (2020, 5, 25, "ce-uraza"),
    (2020, 5, 26, "ce-uraza"),
    (2023, 4, 20, "ce-uraza"),
    (2023, 4, 21, "ce-uraza"),
    (2023, 4, 22, "ce-uraza"),
    (2024, 4, 9, "ce-uraza"),
    (2024, 4, 10, "ce-uraza"),
    (2024, 4, 11, "ce-uraza"),
    (2025, 3, 30, "ce-uraza"),
    (2025, 3, 31, "ce-uraza"),
    (2025, 4, 1, "ce-uraza"),
    (2026, 3, 19, "ce-uraza"),
    (2026, 3, 20, "ce-uraza"),
    (2026, 3, 21, "ce-uraza"),
    (2016, 9, 12, "ce-kurban"),
    (2016, 9, 13, "ce-kurban"),
    (2016, 9, 14, "ce-kurban"),
    (2017, 8, 31, "ce-kurban"),
    (2017, 9, 1, "ce-kurban"),
    (2017, 9, 2, "ce-kurban"),
    (2023, 6, 28, "ce-kurban"),
    (2023, 6, 29, "ce-kurban"),
    (2023, 6, 30, "ce-kurban"),
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
    // Earlier years: the Government's resolutions as its own announcements
    // quote them (No. 215 of 15.07.2015, No. 127 of 30.05.2019, No. 92 of
    // 20.05.2020, No. 180 of 14.07.2021 for Kurban Bayram, No. 216 of
    // 06.07.2022 for Kurban Bayram, No. 139 of 17.04.2023, No. 127 of
    // 12.09.2018 and the resolutions of 03.09.2024 and 10.09.2025 for the Day
    // of Unity), with the weekend day moved where the announcement says so.
    (2015, 7, 18, "da-uraza"),
    (2019, 6, 5, "da-uraza"),
    (2019, 6, 6, "da-uraza"),
    (2019, 6, 7, "da-uraza"),
    (2020, 5, 25, "da-uraza"),
    (2020, 5, 26, "da-uraza"),
    (2023, 4, 20, "da-uraza"),
    (2023, 4, 21, "da-uraza"),
    (2024, 4, 10, "da-uraza"),
    (2024, 4, 11, "da-uraza"),
    (2024, 4, 12, "da-uraza"),
    (2025, 3, 31, "da-uraza"),
    (2025, 4, 1, "da-uraza"),
    (2026, 3, 19, "da-uraza"),
    (2026, 3, 20, "da-uraza"),
    (2021, 7, 20, "da-kurban"),
    (2022, 7, 9, "da-kurban"),
    (2024, 6, 17, "da-kurban"),
    (2025, 6, 6, "da-kurban"),
    (2026, 5, 27, "da-kurban"),
    (2026, 5, 28, "da-kurban"),
    (2026, 5, 29, "da-kurban"),
    (2018, 9, 15, "da-unity"),
    (2024, 9, 15, "da-unity"),
    (2025, 9, 15, "da-unity"),
    (2026, 9, 15, "da-unity"),
    (2015, 7, 20, "da-transferred"),
    (2018, 9, 17, "da-transferred"),
    (2021, 7, 19, "da-transferred"),
    (2022, 7, 11, "da-transferred"),
    (2024, 9, 16, "da-transferred"),
    // Ingushetia: the Head's decrees (2026, No. 84 of 20 May for Kurban
    // Bayram, read, and No. 38 for Uraza Bayram; 2024 and 2025 as
    // ConsultantPlus lists them).
    // Earlier years: the Head's decrees as the Head's own announcements give
    // them, read in the Internet Archive's copies (2015 No. 194 of 19.09.2015
    // for Kurban Bayram; 2016 for Uraza Bayram as amended on 05.07.2016).
    (2016, 7, 6, "in-uraza"),
    (2016, 7, 7, "in-uraza"),
    (2016, 7, 8, "in-uraza"),
    (2017, 6, 26, "in-uraza"),
    (2017, 6, 27, "in-uraza"),
    (2023, 4, 20, "in-uraza"),
    (2023, 4, 21, "in-uraza"),
    (2023, 4, 22, "in-uraza"),
    (2024, 4, 10, "in-uraza"),
    (2024, 4, 11, "in-uraza"),
    (2024, 4, 12, "in-uraza"),
    (2025, 3, 31, "in-uraza"),
    (2025, 4, 1, "in-uraza"),
    (2026, 3, 19, "in-uraza"),
    (2026, 3, 20, "in-uraza"),
    (2026, 3, 21, "in-uraza"),
    (2015, 9, 24, "in-kurban"),
    (2016, 9, 12, "in-kurban"),
    (2017, 9, 1, "in-kurban"),
    (2020, 7, 31, "in-kurban"),
    (2021, 7, 19, "in-kurban"),
    (2021, 7, 20, "in-kurban"),
    (2021, 7, 21, "in-kurban"),
    (2022, 7, 11, "in-kurban"),
    (2023, 6, 28, "in-kurban"),
    (2023, 6, 29, "in-kurban"),
    (2023, 6, 30, "in-kurban"),
    (2024, 6, 17, "in-kurban"),
    (2025, 6, 6, "in-kurban"),
    (2025, 6, 7, "in-kurban"),
    (2026, 5, 27, "in-kurban"),
    (2026, 5, 28, "in-kurban"),
    (2026, 5, 29, "in-kurban"),
    // Kabardino-Balkaria: the Head's decrees «Об объявлении ... нерабочим
    // праздничным днем». Those of 2020 (Uraza, No. 56-УГ) and 2021 (Kurban,
    // No. 82-УГ) were read; the others are listed on the portal of official
    // publication by title and date, which gives the day and not the
    // festival: 2015 Nos. 100-УГ and 135-УГ, 2016 Nos. 79-УГ and 105-УГ,
    // 2017 Nos. 92-УГ and 117-УГ, 2018 Nos. 86-УГ and 125-УГ, 2019 Nos. 47-УГ
    // and 59-УГ, 2020 No. 92-УГ, 2021 No. 46-УГ, 2022 No. 68-УГ, 2023
    // Nos. 35-УГ, 39-УГ and 60-УГ; 2024 and 2025 and 2026 likewise (Kurban
    // Bayram 2024 and Uraza Bayram 2025, read; 2026 as ConsultantPlus lists
    // it). No decree for Uraza Bayram of 2022 was found.
    (2015, 7, 17, "kb-uraza"),
    (2016, 7, 5, "kb-uraza"),
    (2017, 6, 26, "kb-uraza"),
    (2018, 6, 15, "kb-uraza"),
    (2019, 6, 5, "kb-uraza"),
    (2020, 5, 25, "kb-uraza"),
    (2021, 5, 13, "kb-uraza"),
    (2023, 4, 21, "kb-uraza"),
    (2024, 4, 10, "kb-uraza"),
    (2025, 3, 31, "kb-uraza"),
    (2026, 3, 20, "kb-uraza"),
    (2015, 9, 24, "kb-kurban"),
    (2016, 9, 12, "kb-kurban"),
    (2017, 9, 1, "kb-kurban"),
    (2018, 8, 21, "kb-kurban"),
    (2019, 8, 12, "kb-kurban"),
    (2020, 7, 31, "kb-kurban"),
    (2021, 7, 20, "kb-kurban"),
    (2022, 7, 11, "kb-kurban"),
    (2023, 6, 28, "kb-kurban"),
    (2024, 6, 17, "kb-kurban"),
    (2025, 6, 6, "kb-kurban"),
    (2026, 5, 27, "kb-kurban"),
    (2023, 4, 25, "kb-radonitsa"),
    (2024, 5, 14, "kb-radonitsa"),
    (2025, 4, 29, "kb-radonitsa"),
    (2026, 4, 21, "kb-radonitsa"),
    // Karachay-Cherkessia: the Head's decrees for 2026, No. 31 and No. 57,
    // read.
    (2026, 3, 20, "kc-uraza"),
    (2026, 5, 27, "kc-kurban"),
    // Tuva: the Supreme Khural's resolutions for Shagaa and the
    // Government's for Naadym, and No. 742 moving Constitution Day 2026 to
    // 8 May, as ConsultantPlus lists them.
    // Earlier years: the Supreme Khural's resolutions «О дне празднования
    // Шагаа» (2016 No. 673 ПВХ-II, 2017 No. 1192 ПВХ-II as Garant's note gives
    // it, 2019 No. 2034, 2020 No. 194, 2021 No. 744, 2022 No. 1249 and 2023
    // No. 1742 ПВХ-III; the one for 2018, No. 1610, was not read), and the
    // Government's for Naadym (2017 No. 335, 2018 No. 171, 2021 No. 487), with
    // the days they move; and No. 927 ПВХ-III of 23.04.2021, which moves
    // Constitution Day from Thursday 6 May to Friday 7 May.
    (2016, 2, 9, "ty-shagaa"),
    (2017, 2, 27, "ty-shagaa"),
    (2019, 2, 5, "ty-shagaa"),
    (2020, 2, 24, "ty-shagaa"),
    (2021, 2, 12, "ty-shagaa"),
    (2022, 2, 2, "ty-shagaa"),
    (2023, 2, 21, "ty-shagaa"),
    (2024, 2, 10, "ty-shagaa"),
    (2025, 3, 1, "ty-shagaa"),
    (2026, 2, 18, "ty-shagaa"),
    (2017, 8, 13, "ty-naadym"),
    (2018, 7, 14, "ty-naadym"),
    (2018, 7, 15, "ty-naadym"),
    (2021, 9, 24, "ty-naadym"),
    (2024, 7, 19, "ty-naadym"),
    (2025, 7, 18, "ty-naadym"),
    (2026, 7, 24, "ty-naadym"),
    (2021, 5, 7, "ty-constitution"),
    (2026, 5, 8, "ty-constitution"),
    (2017, 8, 14, "ty-transferred"),
    (2018, 7, 16, "ty-transferred"),
    (2018, 7, 17, "ty-transferred"),
    (2019, 2, 4, "ty-shifted"),
    (2020, 2, 25, "ty-shifted"),
    (2019, 2, 2, "ty-worked"),
    // Kalmykia: the Head's decrees «Об объявлении ... года Днем национального
    // праздника», whose titles on the portal of official publication give the
    // day (Garant's notes for Tsagan Sar 2015 No. 4, 2016 No. 8 and 2017
    // No. 5): Tsagan Sar 2018 No. 3, 2019 No. 6, 2020 No. 19 (text read; No. 37
    // of 17.02.2020 moves the day off from Monday 24 to Tuesday 25 February),
    // 2021 No. 8, 2022 No. 10, 2023 No. 25, 2024 No. 15, 2025
    // No. 45 and 2026 No. 8; the Buddha's birthday 2015 No. 82, 2016 No. 31,
    // 2018 No. 41, 2019 No. 100, 2020 No. 142, 2021 No. 85, 2022 No. 104, 2023
    // No. 87, 2024 No. 81, 2025 No. 113 and 2026 No. 95; Zul 2014 No. 191, 2015
    // No. 174, 2017 No. 113, 2019 No. 241, 2020 No. 311, 2021 No. 183, 2022
    // No. 251, 2023 No. 212, 2024 No. 286 and 2025 No. 279.
    (2015, 2, 19, "kl-tsagan-sar"),
    (2016, 2, 9, "kl-tsagan-sar"),
    (2017, 2, 27, "kl-tsagan-sar"),
    (2018, 2, 16, "kl-tsagan-sar"),
    (2019, 2, 5, "kl-tsagan-sar"),
    (2020, 2, 24, "kl-tsagan-sar"),
    (2021, 2, 12, "kl-tsagan-sar"),
    (2022, 3, 3, "kl-tsagan-sar"),
    (2023, 2, 21, "kl-tsagan-sar"),
    (2024, 2, 10, "kl-tsagan-sar"),
    (2025, 2, 28, "kl-tsagan-sar"),
    (2026, 2, 18, "kl-tsagan-sar"),
    (2015, 6, 2, "kl-buddha"),
    (2016, 5, 21, "kl-buddha"),
    (2018, 5, 29, "kl-buddha"),
    (2019, 6, 17, "kl-buddha"),
    (2020, 6, 5, "kl-buddha"),
    (2021, 5, 26, "kl-buddha"),
    (2022, 6, 14, "kl-buddha"),
    (2023, 6, 4, "kl-buddha"),
    (2024, 5, 23, "kl-buddha"),
    (2025, 6, 11, "kl-buddha"),
    (2026, 5, 31, "kl-buddha"),
    (2020, 2, 25, "kl-transferred"),
    (2014, 12, 16, "kl-zul"),
    (2015, 12, 5, "kl-zul"),
    (2017, 12, 12, "kl-zul"),
    (2019, 12, 21, "kl-zul"),
    (2020, 12, 10, "kl-zul"),
    (2021, 12, 29, "kl-zul"),
    (2022, 12, 18, "kl-zul"),
    (2023, 12, 7, "kl-zul"),
    (2024, 12, 25, "kl-zul"),
    (2025, 12, 14, "kl-zul"),
    // The Altai Republic: the Head's decrees for Chaga Bayram (20.01.2014
    // No. 21-у as Garant's note gives it, and 22.01.2018 No. 20-у, read; the
    // others, as ConsultantPlus lists them).
    (2014, 2, 2, "al-chaga"),
    (2018, 2, 17, "al-chaga"),
    (2024, 2, 17, "al-chaga"),
    (2025, 2, 8, "al-chaga"),
    (2026, 2, 21, "al-chaga"),
    // North Ossetia–Alania: the first Monday of Uastyrdzhi, as
    // ConsultantPlus lists it; for 2025 also the Head's decree No. 453 of
    // 12.11.2025, whose title on the portal of official publication declares
    // 17 November (the list gives 17 and 24 November).
    (2024, 11, 18, "se-uastyrdzhi"),
    (2025, 11, 17, "se-uastyrdzhi"),
    (2025, 11, 24, "se-uastyrdzhi"),
    (2026, 11, 23, "se-uastyrdzhi"),
]);

/// The instruments, cited by the rules.
mod cite {
    pub(super) const KR: &str = "Закон Республики Карелия № 346-ЗРК of 1999: 8 June is a holiday \
                                 with no day off; the text read is of that year, its date of effect not read";
    pub(super) const KK: &str = "Закон Республики Хакасия of 1992, as restated in 2005: 3 July is a \
                                 holiday with no day off; the text read is the restatement";
    pub(super) const ME: &str = "Закон Республики Марий Эл № 21-З of 05.07.2022, articles 1 to 4: no \
                                 day off; in force on publication, so read from 2023";
    pub(super) const UD: &str = "Закон Удмуртской Республики № 81-РЗ of 2020, article 3: holidays \
                                 with no day off; the text read is of that year, its date of effect not read";
    pub(super) const TA: &str = "Закон Республики Татарстан от 19.02.1992 № 1448-XII «О праздничных днях и \
                                 памятных датах Республики Татарстан», статья 1, as restated by 39-ЗРТ \
                                 (2003) and 74-ЗРТ (2010) and amended by 67-ЗРТ (2016) and 24-ЗРТ (2023)";
    pub(super) const BA: &str = "Закон Республики Башкортостан от 27.02.1992 № ВС-10/21 «О праздничных и \
                                 памятных днях в Республике Башкортостан», статья 1, as worded by 364-з of \
                                 01.03.2011";
    pub(super) const AD: &str = "Закон Республики Адыгея от 14.02.1995 № 168-1 «О праздничных днях и \
                                 памятных датах», статьи 2 и 4, article 2 as worded by № 384 of 06.11.2020";
    pub(super) const BU: &str = "Закон Республики Бурятия от 10.10.2017 № 2562-V «О праздничных днях и \
                                 памятных датах в Республике Бурятия» (in force 01.01.2018; before it, the \
                                 law of 23.12.2008 № 675-IV) and the Head's decree each year, «О празднике \
                                 Белого месяца \"Сагаалган\"», under article 4";
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
    pub(super) const KB: &str = "Указы Главы Кабардино-Балкарской Республики, each year, «Об объявлении ... \
                                 нерабочим праздничным днем» (Uraza Bayram 2024 and 2025 also «в связи с \
                                 обращением Духовного управления мусульман»)";
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

/// A fixed day of one republic. Its years are the call's: `.years` from
/// the act that set it, and `.read_from` the first year of the act read
/// where the day is older, the years between a gap.
const fn fixed(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::gregorian(month, day))
        .in_regions(region)
        .cited(source)
}

/// A day of one republic as its acts from `first` to `last` set it, and
/// a gap in any other year from the act that established it, which the
/// call gives with `.years`; with none, every earlier year is a gap.
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
    .in_regions(region)
    .cited(source)
}

/// A republic's day in the years `first` to `last` whose act was not read:
/// a gap in each, which the listing's own range cannot say for a year
/// between two it covers.
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

/// The gap a republic's law leaves in a year one of its days off falls on
/// the weekend: the law moves it to the next working day, which the
/// federal days and the republic's own act decide. `fixed` are its fixed
/// days, and `prefix` the keys of its rows in [`RU_REPUBLIC_DAYS`].
fn moved(year: i64, fixed: &[(u8, u8)], prefix: &str) -> Option<Days> {
    moved_given(year, fixed, prefix, "")
}

/// The same, where the acts read name the day a weekend day moves to, in
/// rows keyed `given`: a year is a gap when more of the republic's days fall
/// on the weekend than the acts read move.
fn moved_given(year: i64, fixed: &[(u8, u8)], prefix: &str, given: &str) -> Option<Days> {
    let on_weekend = |day: Rd| matches!(Weekday::from_rd(day), Weekday::Saturday | Weekday::Sunday);
    let Listing::Named(rows) = RU_REPUBLIC_DAYS else {
        return Some(Days::new());
    };
    let mut weekend = fixed
        .iter()
        .filter(|&&(month, day)| gregorian::to_fixed(year, month, day).is_ok_and(on_weekend))
        .count();
    let mut moved_by_acts = 0;
    for &(row_year, month, day, key) in rows {
        if row_year != year {
            continue;
        }
        if key == given {
            moved_by_acts += 1;
        } else if key.starts_with(prefix)
            && !key.ends_with("-worked")
            && gregorian::to_fixed(year, month, day).is_ok_and(on_weekend)
        {
            weekend += 1;
        }
    }
    if weekend > moved_by_acts {
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
        return moved_given(year, &[(8, 15)], "ty-", "ty-transferred");
    }
    moved_given(year, &[(5, 6), (8, 15)], "ty-", "ty-transferred")
}
/// Buryatia's weekend gap. Article 4 moves a weekend day that coincides
/// with Sagaalgan to the next working day, and the decrees that were read
/// say which day: a year in which Sagaalgan falls on the weekend and no
/// decree read gives the day it moves to is a gap.
fn bu_moved(year: i64) -> Option<Days> {
    moved_given(year, &[], "bu-", "bu-transferred")
}
fn da_moved(year: i64) -> Option<Days> {
    moved_given(year, &[], "da-", "da-transferred")
}
fn kl_moved(year: i64) -> Option<Days> {
    moved_given(year, &[(7, 5)], "kl-", "kl-transferred")
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

/// Buryatia's Sagaalgan as the Head's decrees for `first` to `last` give
/// it, with no day in a year between that no decree was read for.
const fn sagaalgan(first: i32, last: i32) -> HolidayRule {
    HolidayRule::fixed_public(
        "Sagaalgan",
        "Праздник Белого месяца «Сагаалган»",
        Rule::listed(
            RU_REPUBLIC_DAYS.named("bu-sagaalgan"),
            first as i64,
            last as i64,
        ),
    )
    .years(Some(first), Some(last))
    .in_regions(BU)
    .cited(cite::BU)
}

/// Sagaalgan in the years `first` to `last` whose decree was not read.
const fn sagaalgan_unread(first: i32, last: i32) -> HolidayRule {
    HolidayRule::fixed_public(
        "Sagaalgan",
        "Праздник Белого месяца «Сагаалган»",
        Rule::UNREAD,
    )
    .years(Some(first), Some(last))
    .in_regions(BU)
    .cited(cite::BU)
}

const TA: &[&str] = &["RU-TA"];
const BA: &[&str] = &["RU-BA"];
const AD: &[&str] = &["RU-AD"];
const BU: &[&str] = &["RU-BU"];
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
        AD,
        cite::AD,
    )
    .years(Some(1995), None)
    .read_from(2014),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "ad-uraza",
        2023,
        2026,
        AD,
        cite::AD,
    )
    .years(Some(1995), None),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "ad-kurban",
        2023,
        2026,
        AD,
        cite::AD,
    )
    .years(Some(2021), None),
    listed(
        "Radonitsa",
        "День поминовения усопших (Радоница)",
        "ad-radonitsa",
        2023,
        2026,
        AD,
        cite::AD,
    )
    .years(Some(2021), None),
    HolidayRule::fixed_public(
        "Day off transferred by the Head",
        "Перенесённый выходной день",
        Rule::listed(RU_REPUBLIC_DAYS.named("ad-transferred"), 2023, 2026),
    )
    .years(Some(2023), Some(2026))
    .in_regions(AD)
    .cited(cite::AD),
    // The decrees for 2024 and 2025 say in their titles that they move a day
    // off ("и переносе выходного дня"); their texts were not read.
    unread(
        "Day off transferred by the Head",
        "Перенесённый выходной день",
        2024,
        2025,
        AD,
        cite::AD,
    ),
    HolidayRule::workday(
        "Working day, a day off transferred by the Head",
        "Рабочий день",
        Rule::listed(RU_REPUBLIC_DAYS.named("ad-worked"), 2023, 2026),
    )
    .years(Some(2023), Some(2026))
    .in_regions(AD)
    .cited(cite::AD),
    moved_day(ad_moved, 2014, None, AD, cite::AD),
    // ── The Altai Republic ──────────────────────────────────────────────
    listed(
        "Chaga Bayram",
        "Чага-Байрам",
        "al-chaga",
        2014,
        2026,
        AL,
        cite::AL,
    )
    .years(Some(2013), None),
    unread("Chaga Bayram", "Чага-Байрам", 2015, 2017, AL, cite::AL),
    unread("Chaga Bayram", "Чага-Байрам", 2019, 2023, AL, cite::AL),
    fixed(
        "Day of the Formation of the Altai Republic",
        "День образования Республики Алтай",
        7,
        3,
        AL,
        cite::AL,
    )
    .years(Some(2026), None)
    .of_kind(Kind::Government),
    moved_day(al_moved, 2013, None, AL, cite::AL),
    // ── Bashkortostan ───────────────────────────────────────────────────
    fixed("Republic Day", "День Республики", 10, 11, BA, cite::BA)
        .years(Some(1992), None)
        .read_from(2011),
    listed(
        "Uraza Bayram",
        "Ураза-байрам",
        "ba-uraza",
        2015,
        2026,
        BA,
        cite::BA,
    )
    .years(Some(1992), None),
    listed(
        "Kurban Bayram",
        "Курбан-байрам",
        "ba-kurban",
        2015,
        2026,
        BA,
        cite::BA,
    )
    .years(Some(1992), None),
    unread("Uraza Bayram", "Ураза-байрам", 2018, 2020, BA, cite::BA),
    unread("Kurban Bayram", "Курбан-байрам", 2018, 2020, BA, cite::BA),
    moved_day(ba_moved, 2011, None, BA, cite::BA),
    // ── Buryatia ────────────────────────────────────────────────────────
    // The law of 23.12.2008 and the decrees under it are read for 2012 and
    // 2015 only; 2013, 2014, 2016, 2017 and 2022 are gaps, and so is every
    // year from 2009 to 2011. The law of 2017, in force from 2018, has
    // article 4 move a weekend day to the next working day.
    sagaalgan_unread(2009, 2011),
    sagaalgan(2012, 2012),
    sagaalgan_unread(2013, 2014),
    sagaalgan(2015, 2015),
    sagaalgan_unread(2016, 2017),
    sagaalgan(2018, 2021),
    sagaalgan_unread(2022, 2022),
    sagaalgan(2023, 2026).years(Some(2023), None),
    HolidayRule::fixed_public(
        "Day off transferred by the Head",
        "Перенесённый выходной день",
        Rule::listed(RU_REPUBLIC_DAYS.named("bu-transferred"), 2018, 2026),
    )
    .years(Some(2018), Some(2026))
    .in_regions(BU)
    .cited(cite::BU),
    moved_day(bu_moved, 2018, None, BU, cite::BU),
    // ── Chechnya ────────────────────────────────────────────────────────
    fixed(
        "Constitution Day of the Chechen Republic",
        "День Конституции Чеченской Республики",
        3,
        23,
        CE,
        cite::CE_23_MARCH,
    )
    .years(Some(2004), None),
    fixed("Day of Peace", "День мира", 4, 16, CE, cite::CE_16_APRIL).years(Some(2010), None),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "ce-uraza",
        2016,
        2026,
        CE,
        cite::CE,
    ),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "ce-kurban",
        2016,
        2026,
        CE,
        cite::CE,
    ),
    unread("Uraza Bayram", "Ураза-Байрам", 2017, 2017, CE, cite::CE),
    unread("Uraza Bayram", "Ураза-Байрам", 2019, 2019, CE, cite::CE),
    unread("Uraza Bayram", "Ураза-Байрам", 2021, 2022, CE, cite::CE),
    unread("Kurban Bayram", "Курбан-Байрам", 2018, 2022, CE, cite::CE),
    // ── Chuvashia ───────────────────────────────────────────────────────
    fixed("Republic Day", "День Республики", 6, 24, CU, cite::CU).years(Some(2000), None),
    // ── Dagestan ────────────────────────────────────────────────────────
    fixed(
        "Constitution Day of the Republic of Dagestan",
        "День Конституции Республики Дагестан",
        7,
        26,
        DA,
        cite::DA_CONSTITUTION,
    )
    .years(Some(1995), None),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "da-uraza",
        2015,
        2026,
        DA,
        cite::DA,
    )
    .years(Some(1991), None),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "da-kurban",
        2021,
        2026,
        DA,
        cite::DA_KURBAN,
    )
    .years(Some(2000), None),
    listed(
        "Day of Unity of the Peoples of Dagestan",
        "День единства народов Дагестана",
        "da-unity",
        2018,
        2026,
        DA,
        cite::DA,
    )
    .years(Some(2011), None),
    unread(
        "Day of Unity of the Peoples of Dagestan",
        "День единства народов Дагестана",
        2019,
        2023,
        DA,
        cite::DA,
    ),
    unread("Uraza Bayram", "Ураза-Байрам", 2016, 2018, DA, cite::DA),
    unread("Uraza Bayram", "Ураза-Байрам", 2021, 2022, DA, cite::DA),
    unread(
        "Kurban Bayram",
        "Курбан-Байрам",
        2023,
        2023,
        DA,
        cite::DA_KURBAN,
    ),
    HolidayRule::fixed_public(
        "Day off transferred by the Government of Dagestan",
        "Перенесённый выходной день",
        Rule::listed(RU_REPUBLIC_DAYS.named("da-transferred"), 2015, 2026),
    )
    .years(Some(2015), Some(2026))
    .in_regions(DA)
    .cited(cite::DA),
    moved_day(da_moved, 2015, None, DA, cite::DA),
    // ── Ingushetia ──────────────────────────────────────────────────────
    listed(
        "Eid al-Fitr",
        "Мархаш",
        "in-uraza",
        2016,
        2026,
        IN,
        cite::IN,
    ),
    listed(
        "Eid al-Adha",
        "Гӏурба",
        "in-kurban",
        2015,
        2026,
        IN,
        cite::IN,
    ),
    unread("Eid al-Fitr", "Мархаш", 2018, 2022, IN, cite::IN),
    unread("Eid al-Adha", "Гӏурба", 2018, 2019, IN, cite::IN),
    // ── Kabardino-Balkaria ──────────────────────────────────────────────
    fixed(
        "Day of the Revival of the Balkar People",
        "День возрождения балкарского народа",
        3,
        28,
        KB,
        cite::KB_28_MARCH,
    )
    .years(Some(1994), None)
    .read_from(1995),
    fixed(
        "Day of Statehood of the Kabardino-Balkarian Republic",
        "День государственности Кабардино-Балкарской Республики (День республики)",
        9,
        1,
        KB,
        cite::KB_1_SEPTEMBER,
    )
    .years(Some(1997), None)
    .read_from(1998),
    fixed(
        "Day of the Adyghe (Circassians)",
        "День адыгов (черкесов)",
        9,
        20,
        KB,
        cite::KB_20_SEPTEMBER,
    )
    .years(Some(2014), None),
    listed(
        "Kurban Bayram",
        "Курбан-Байрам",
        "kb-kurban",
        2015,
        2026,
        KB,
        cite::KB,
    ),
    listed(
        "Uraza Bayram",
        "Ураза-Байрам",
        "kb-uraza",
        2015,
        2026,
        KB,
        cite::KB,
    ),
    unread("Uraza Bayram", "Ураза-Байрам", 2022, 2022, KB, cite::KB),
    listed(
        "Radonitsa",
        "Радоница",
        "kb-radonitsa",
        2023,
        2026,
        KB,
        cite::KB,
    ),
    // ── Kalmykia ────────────────────────────────────────────────────────
    fixed(
        "Day of the Republic of Kalmykia",
        "День Республики Калмыкия",
        7,
        5,
        KL,
        cite::KL,
    )
    .years(Some(2020), None),
    listed(
        "Tsagan Sar",
        "Цаган Сар",
        "kl-tsagan-sar",
        2015,
        2026,
        KL,
        cite::KL,
    )
    .years(Some(2005), None),
    listed(
        "Buddha Shakyamuni's Birthday",
        "День рождения Будды Шакьямуни",
        "kl-buddha",
        2015,
        2026,
        KL,
        cite::KL,
    )
    .years(Some(2005), None),
    unread(
        "Buddha Shakyamuni's Birthday",
        "День рождения Будды Шакьямуни",
        2017,
        2017,
        KL,
        cite::KL,
    ),
    listed("Zul", "Зул", "kl-zul", 2014, 2025, KL, cite::KL).years(Some(2005), None),
    unread("Zul", "Зул", 2016, 2016, KL, cite::KL),
    unread("Zul", "Зул", 2018, 2018, KL, cite::KL),
    HolidayRule::fixed_public(
        "Day off transferred by the Head",
        "Перенесённый выходной день",
        Rule::listed(RU_REPUBLIC_DAYS.named("kl-transferred"), 2020, 2020),
    )
    .years(Some(2020), Some(2020))
    .in_regions(KL)
    .cited(cite::KL),
    moved_day(kl_moved, 2020, None, KL, cite::KL),
    // ── Karachay-Cherkessia ─────────────────────────────────────────────
    fixed(
        "Day of the Revival of the Karachay People",
        "День возрождения карачаевского народа",
        5,
        3,
        KC,
        cite::KC_3_MAY,
    )
    .years(Some(2001), None)
    .read_from(2002),
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
    // ── Komi ────────────────────────────────────────────────────────────
    fixed(
        "Day of the Republic of Komi",
        "День Республики Коми",
        8,
        22,
        KO,
        cite::KO,
    )
    .years(Some(2014), None)
    .of_kind(Kind::Government),
    // ── Sakha (Yakutia) ─────────────────────────────────────────────────
    fixed(
        "Day of the Republic of Sakha (Yakutia)",
        "День Республики Саха (Якутия)",
        4,
        27,
        SA,
        cite::SA,
    )
    .years(Some(2019), None)
    .of_kind(Kind::Government),
    fixed(
        "Ysyakh",
        "День национального праздника «Ысыах»",
        6,
        21,
        SA,
        cite::SA,
    )
    .years(Some(2018), None)
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
    )
    .years(Some(2018), None),
    // ── Tatarstan ───────────────────────────────────────────────────────
    fixed(
        "Day of the Republic of Tatarstan",
        "День Республики Татарстан",
        8,
        30,
        TA,
        cite::TA,
    )
    .years(Some(1992), None)
    .read_from(2004),
    fixed(
        "Constitution Day of the Republic of Tatarstan",
        "День Конституции Республики Татарстан",
        11,
        6,
        TA,
        cite::TA,
    )
    .years(Some(1992), None)
    .read_from(2003),
    listed(
        "Uraza Bayram",
        "Ураза-байрам",
        "ta-uraza",
        2015,
        2026,
        TA,
        cite::TA,
    )
    .years(Some(2011), None),
    listed(
        "Kurban Bayram",
        "Курбан-байрам",
        "ta-kurban",
        2015,
        2026,
        TA,
        cite::TA,
    )
    .years(Some(1992), None),
    // Until 67-ЗРТ of 2016 a day off on the weekend moved to the next
    // working day; from 2017 no further day is given.
    moved_day(ta_moved, 2004, Some(2016), TA, cite::TA),
    // ── Tuva ────────────────────────────────────────────────────────────
    fixed("Republic Day", "День Республики", 8, 15, TY, cite::TY)
        .years(Some(1999), None)
        .read_from(2013),
    fixed("Constitution Day", "День Конституции", 5, 6, TY, cite::TY)
        .years(Some(1999), None)
        .read_from(2013)
        .years(Some(2013), Some(2020)),
    fixed("Constitution Day", "День Конституции", 5, 6, TY, cite::TY).years(Some(2022), Some(2025)),
    fixed("Constitution Day", "День Конституции", 5, 6, TY, cite::TY).years(Some(2027), None),
    // Moved by resolutions: to Friday 7 May in 2021, to 8 May in 2026.
    listed(
        "Constitution Day",
        "День Конституции",
        "ty-constitution",
        2021,
        2021,
        TY,
        cite::TY,
    )
    .years(Some(2021), Some(2021)),
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
    listed("Shagaa", "Шагаа", "ty-shagaa", 2016, 2026, TY, cite::TY).years(Some(1999), None),
    unread("Shagaa", "Шагаа", 2018, 2018, TY, cite::TY),
    listed("Naadym", "Наадым", "ty-naadym", 2017, 2026, TY, cite::TY).years(Some(1999), None),
    unread("Naadym", "Наадым", 2019, 2020, TY, cite::TY),
    unread("Naadym", "Наадым", 2022, 2023, TY, cite::TY),
    HolidayRule::fixed_public(
        "Day off transferred by resolution",
        "Перенесённый выходной день",
        Rule::listed(RU_REPUBLIC_DAYS.named("ty-transferred"), 2017, 2018),
    )
    .years(Some(2017), Some(2018))
    .in_regions(TY)
    .cited(cite::TY),
    HolidayRule::fixed_public(
        "Day off transferred by resolution",
        "Перенесённый выходной день",
        Rule::listed(RU_REPUBLIC_DAYS.named("ty-shifted"), 2019, 2020),
    )
    .years(Some(2019), Some(2020))
    .in_regions(TY)
    .cited(cite::TY),
    HolidayRule::workday(
        "Working day, a day off transferred by resolution",
        "Рабочий день",
        Rule::listed(RU_REPUBLIC_DAYS.named("ty-worked"), 2019, 2019),
    )
    .years(Some(2019), Some(2019))
    .in_regions(TY)
    .cited(cite::TY),
    moved_day(ty_moved, 2013, None, TY, cite::TY),
    // The four republics whose law was read and gives no day off: a day of
    // their own is none in the years from the law read, and a gap before
    // it, because the acts before were not read (ADR 0013).
    no_day_law("Karelia", &["RU-KR"], 1999, cite::KR),
    no_day_law("Khakassia", &["RU-KK"], 2005, cite::KK),
    no_day_law("Mari El", &["RU-ME"], 2023, cite::ME),
    no_day_law("Udmurtia", &["RU-UD"], 2020, cite::UD),
];

/// A republic whose law read gives no non-working day of its own: no day
/// from `first`, the first year the law read answers for, and a gap in every
/// year before it, the earlier acts not having been read. The rule is named
/// for the republic so that the gap says whose.
const fn no_day_law(
    republic: &'static str,
    regions: &'static [&'static str],
    first: i32,
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(republic, "", Rule::NO_DAY)
        .in_regions(regions)
        .read_from(first)
        .cited(source)
}

/// How many rules [`RUSSIA`](super::RUSSIA) has in all.
const RU_ALL_LEN: usize = RU_RULES.len() + RU_REPUBLIC_RULES.len();

/// Every rule of [`RUSSIA`](super::RUSSIA): the federal ones of `europe.rs`,
/// then the republics' here.
pub(super) static RU_ALL_RULES: [HolidayRule; RU_ALL_LEN] = joined(&[RU_RULES, RU_REPUBLIC_RULES]);
