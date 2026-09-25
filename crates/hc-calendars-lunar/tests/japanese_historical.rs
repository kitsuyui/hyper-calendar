//! Japan's pre-Tenpō calendars against the published tables.
//!
//! The whole value of a historical calendar is whether it reproduces the
//! dates in the documents, so this file is the deliverable and the module's
//! own unit tests are the scaffolding.
//!
//! `data/japanese_month_lengths.txt` holds the first day and the length of
//! **every month of every year from 862 to 1843** — 982 years, 12 146
//! months, 300 592 days — taken from the 西暦との対照表 published in the
//! Japanese Wikipedia article for each 元号, which are transcriptions of
//! 内田正男『日本暦日原典』. The file's own header records how it was
//! gathered and the three cross-checks it passed. It is one source family,
//! and this file says so rather than implying independence it does not have.
//!
//! Every agreement rate asserted here is also printed, so `cargo test --
//! --nocapture` gives the numbers the README quotes.

use hc_calendar::{Calendar, CalendarError, Month, Rd};
use hc_calendars_lunar::LunisolarCalendar;
use hc_calendars_lunar::japanese_historical::{horyaku, jokyo, kansei, senmyo};
use hc_calendars_lunar::lunisolar::{LunisolarParameters, SolarTermMode};

/// The published table.
const TABLE: &str = include_str!("data/japanese_month_lengths.txt");

/// One lunisolar year as the table gives it.
struct TableYear {
    /// The fixed day of the first day of the first month.
    start: i64,
    /// The ordinal of the intercalary month, or zero for none.
    leap: u8,
    /// The length of each month in order, with the leap month in place.
    lengths: Vec<u8>,
}

impl TableYear {
    /// The months of this year as `(first day, month, length)`.
    fn months(&self) -> Vec<(i64, Month, u8)> {
        let mut out = Vec::with_capacity(self.lengths.len());
        let mut cursor = self.start;
        let mut ordinal = 1u8;
        let mut repeated = false;
        for &length in &self.lengths {
            let is_leap = self.leap != 0 && ordinal == self.leap && repeated;
            if self.leap != 0 && ordinal == self.leap {
                repeated = true;
            }
            let month = if is_leap {
                Month::leap(ordinal)
            } else {
                Month::regular(ordinal)
            };
            out.push((cursor, month, length));
            cursor += i64::from(length);
            if is_leap || self.leap == 0 || ordinal != self.leap {
                ordinal += 1;
            }
        }
        out
    }
}

/// Parse the table, ignoring its comment header.
fn table() -> Vec<TableYear> {
    TABLE
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            // Parsed leniently, because a malformed file should fail in
            // `the_published_table_is_internally_consistent` with a legible
            // message rather than in a panic somewhere inside an iterator.
            let start = parts.next().and_then(|f| f.parse().ok()).unwrap_or(0);
            let leap = parts.next().and_then(|f| f.parse().ok()).unwrap_or(0);
            let lengths = parts
                .next()
                .unwrap_or_default()
                .chars()
                .map(|c| if c == 'L' { 30 } else { 29 })
                .collect();
            TableYear {
                start,
                leap,
                lengths,
            }
        })
        .collect()
}

/// The four systems with the span each is responsible for.
fn systems() -> [(&'static str, &'static LunisolarParameters, i64, i64); 4] {
    [
        (
            "Senmyō",
            &senmyo::PARAMETERS,
            senmyo::EARLIEST.0,
            senmyo::LATEST.0,
        ),
        (
            "Jōkyō",
            &jokyo::PARAMETERS,
            jokyo::EARLIEST.0,
            jokyo::LATEST.0,
        ),
        (
            "Hōryaku",
            &horyaku::PARAMETERS,
            horyaku::EARLIEST.0,
            horyaku::LATEST.0,
        ),
        (
            "Kansei",
            &kansei::PARAMETERS,
            kansei::EARLIEST.0,
            kansei::LATEST.0,
        ),
    ]
}

/// What fraction of the table a parameter set reproduces, as percentages of
/// new years, of intercalary months, of month starts and of individual days.
struct Agreement {
    new_years: f64,
    leap_months: f64,
    month_starts: f64,
    days: f64,
}

fn measure(parameters: &'static LunisolarParameters, first: i64, last: i64) -> Agreement {
    measure_sampled(parameters, first, last, 1)
}

/// The same, taking every `stride`-th year of the table.
///
/// A stride exists only to keep the comparison tests affordable in a debug
/// build; the headline measurement uses every year.
fn measure_sampled(
    parameters: &'static LunisolarParameters,
    first: i64,
    last: i64,
    stride: usize,
) -> Agreement {
    let engine = LunisolarCalendar::new(parameters);
    let (mut new_year_hits, mut new_year_total) = (0usize, 0usize);
    let (mut leap_hits, mut leap_total) = (0usize, 0usize);
    let (mut start_hits, mut start_total) = (0usize, 0usize);
    let (mut day_hits, mut day_total) = (0usize, 0usize);
    for year in table().into_iter().step_by(stride) {
        if year.start < first || year.start > last {
            continue;
        }
        new_year_total += 1;
        leap_total += 1;
        let first_day = engine.from_fixed(Rd(year.start));
        if first_day.map(|date| (date.month, date.day)) == Ok((Month::regular(1), 1)) {
            new_year_hits += 1;
        }
        if let Ok(date) = first_day
            && parameters
                .leap_month(date.year)
                .unwrap_or(None)
                .unwrap_or(0)
                == year.leap
        {
            leap_hits += 1;
        }
        for (start, month, length) in year.months() {
            let span = i64::from(length).min(last - start + 1);
            start_total += 1;
            day_total += usize::try_from(span).unwrap_or(0);
            let head = engine
                .from_fixed(Rd(start))
                .map(|date| (date.month, date.day));
            if head == Ok((month, 1)) {
                start_hits += 1;
            }
            // Day numbers rise by one inside a month, so if the table's first
            // and last day of a month both come out right, so does everything
            // between them. Only the months that fail need walking, which
            // turns a 300 000-day scan into a 12 000-month one and leaves the
            // answer identical.
            let tail = u8::try_from(span).unwrap_or(0);
            let ends_match = engine
                .from_fixed(Rd(start + span - 1))
                .map(|date| (date.month, date.day))
                == Ok((month, tail));
            if head == Ok((month, 1)) && ends_match {
                day_hits += usize::try_from(span).unwrap_or(0);
                continue;
            }
            for offset in 0..span {
                let wanted = Ok((month, u8::try_from(offset + 1).unwrap_or(0)));
                if engine
                    .from_fixed(Rd(start + offset))
                    .map(|date| (date.month, date.day))
                    == wanted
                {
                    day_hits += 1;
                }
            }
        }
    }
    let percent = |hits: usize, total: usize| 100.0 * hits as f64 / total as f64;
    Agreement {
        new_years: percent(new_year_hits, new_year_total),
        leap_months: percent(leap_hits, leap_total),
        month_starts: percent(start_hits, start_total),
        days: percent(day_hits, day_total),
    }
}

#[test]
fn the_published_table_is_internally_consistent() {
    // Before measuring anything against the table, check the table. Every
    // month must be 29 or 30 days, the years must abut with no gap and no
    // overlap, and a year must have thirteen months exactly when it names an
    // intercalary one.
    let years = table();
    assert_eq!(years.len(), 982, "982 lunisolar years, 862 through 1843");
    assert_eq!(years[0].start, senmyo::EARLIEST.0);
    let mut months = 0;
    for (index, year) in years.iter().enumerate() {
        let wanted = if year.leap == 0 { 12 } else { 13 };
        assert_eq!(year.lengths.len(), wanted, "year starting {}", year.start);
        assert!(year.leap <= 12);
        for &length in &year.lengths {
            assert!((29..=30).contains(&length));
            months += 1;
        }
        let span: i64 = year.lengths.iter().map(|&l| i64::from(l)).sum();
        if let Some(next) = years.get(index + 1) {
            assert_eq!(year.start + span, next.start, "year {index} does not abut");
        }
    }
    assert_eq!(months, 12_146);
    let span = years[981].start + 354 - years[0].start;
    assert!(span > 300_000, "{span} days covered");
}

#[test]
fn the_four_named_anchors_from_the_documents_come_out_right() {
    // 本能寺の変, 天正10年6月2日 = 1582-06-21 Julian = 1582-07-01 proleptic
    // Gregorian.
    assert_eq!(
        senmyo::SenmyoCalendar.from_fixed(gregorian(1582, 7, 1)),
        Ok(date(1_582, Month::regular(6), 2))
    );
    // 関ヶ原の戦い, 慶長5年9月15日 = 1600-10-21 Gregorian.
    assert_eq!(
        senmyo::SenmyoCalendar.from_fixed(gregorian(1600, 10, 21)),
        Ok(date(1_600, Month::regular(9), 15))
    );
    // 赤穂事件討ち入り, 元禄15年12月14日 = 1703-01-30 Gregorian. The
    // lunisolar year had begun in 1702, which is how this crate numbers it.
    assert_eq!(
        jokyo::JokyoCalendar.from_fixed(gregorian(1703, 1, 30)),
        Ok(date(1_702, Month::regular(12), 14))
    );
    // 貞享2年1月1日 = 1685-02-04, the first day of the Jōkyō reform.
    assert_eq!(
        jokyo::JokyoCalendar.from_fixed(gregorian(1685, 2, 4)),
        Ok(date(1_685, Month::regular(1), 1))
    );
    assert_eq!(jokyo::new_year(1_685), Ok(gregorian(1685, 2, 4)));
    // And the day before it is the last day Senmyō-reki ever named.
    assert_eq!(
        senmyo::SenmyoCalendar.from_fixed(gregorian(1685, 2, 3)),
        Ok(date(1_684, Month::regular(12), 30))
    );
}

#[test]
fn dated_events_scattered_over_nine_centuries_come_out_right() {
    // Dates taken from Japanese Wikipedia articles on individual events and
    // people — a different transcription path from the era tables, and so a
    // partial independent check on them. Western dates are Julian before
    // 1582-10-15 and Gregorian after, which is that site's convention; the
    // fixed days below are proleptic Gregorian either way.
    let anchors: &[(&'static LunisolarParameters, Rd, i64, Month, u8, &str)] = &[
        (
            &senmyo::PARAMETERS,
            senmyo::EARLIEST,
            862,
            Month::regular(1),
            1,
            "貞観4年1月1日, the first day of Senmyō-reki",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1000, 1, 10),
            999,
            Month::regular(12),
            1,
            "長保元年12月1日",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1000, 4, 2),
            1_000,
            Month::regular(2),
            25,
            "長保2年2月25日",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1200, 1, 19),
            1_200,
            Month::regular(1),
            2,
            "正治2年1月2日",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1200, 3, 27),
            1_200,
            Month::leap(2),
            11,
            "正治2年閏2月11日 — an intercalary month in the documents",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1200, 10, 20),
            1_200,
            Month::regular(9),
            11,
            "正治2年9月11日",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1250, 2, 13),
            1_250,
            Month::regular(1),
            11,
            "建長2年1月11日",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1300, 1, 26),
            1_300,
            Month::regular(1),
            4,
            "正安2年1月4日",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1400, 2, 2),
            1_400,
            Month::regular(1),
            7,
            "応永7年1月7日",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1450, 10, 6),
            1_450,
            Month::regular(9),
            1,
            "宝徳2年9月1日",
        ),
        (
            &senmyo::PARAMETERS,
            julian(1500, 2, 19),
            1_500,
            Month::regular(1),
            20,
            "明応9年1月20日",
        ),
        (
            &senmyo::PARAMETERS,
            gregorian(1600, 11, 6),
            1_600,
            Month::regular(10),
            1,
            "慶長5年10月1日",
        ),
        (
            &senmyo::PARAMETERS,
            gregorian(1650, 12, 14),
            1_650,
            Month::leap(10),
            21,
            "慶安3年閏10月21日",
        ),
        (
            &jokyo::PARAMETERS,
            gregorian(1700, 4, 20),
            1_700,
            Month::regular(3),
            2,
            "元禄13年3月2日",
        ),
        (
            &horyaku::PARAMETERS,
            horyaku::EARLIEST,
            1_755,
            Month::regular(1),
            1,
            "宝暦5年1月1日, the first day of Hōryaku-reki",
        ),
        (
            &kansei::PARAMETERS,
            kansei::EARLIEST,
            1_798,
            Month::regular(1),
            1,
            "寛政10年1月1日, the first day of Kansei-reki",
        ),
        (
            &kansei::PARAMETERS,
            gregorian(1800, 6, 11),
            1_800,
            Month::leap(4),
            19,
            "寛政12年閏4月19日",
        ),
        (
            &kansei::PARAMETERS,
            kansei::LATEST,
            1_843,
            Month::regular(12),
            29,
            "天保14年12月29日, the last day before the Tenpō reform",
        ),
    ];
    let mut wrong = Vec::new();
    for &(parameters, rd, year, month, day, label) in anchors {
        let got = LunisolarCalendar::new(parameters).from_fixed(rd);
        if got != Ok(date(year, month, day)) {
            wrong.push(format!("{label}: {got:?}"));
        }
    }
    assert!(wrong.is_empty(), "{} wrong: {wrong:?}", wrong.len());
}

#[test]
fn every_system_reproduces_the_published_table_at_the_stated_rate() {
    // The headline measurement. The asserted floors sit a little under the
    // rates measured when this was written, which are printed alongside so
    // that a change in either direction is visible rather than silent.
    let floors = [
        // (new years, leap months, month starts, days)
        ("Senmyō", 93.0, 92.0, 95.0, 95.0),
        ("Jōkyō", 96.0, 98.0, 98.0, 98.0),
        ("Hōryaku", 96.0, 89.0, 97.0, 97.0),
        ("Kansei", 96.0, 96.0, 98.0, 98.0),
    ];
    for ((name, parameters, first, last), (_, ny, lp, ms, dy)) in systems().into_iter().zip(floors)
    {
        let agreement = measure(parameters, first, last);
        println!(
            "{name:8}: new years {:.2}%  leap months {:.2}%  month starts {:.2}%  days {:.2}%",
            agreement.new_years, agreement.leap_months, agreement.month_starts, agreement.days
        );
        assert!(agreement.new_years >= ny, "{name} new years");
        assert!(agreement.leap_months >= lp, "{name} leap months");
        assert!(agreement.month_starts >= ms, "{name} month starts");
        assert!(agreement.days >= dy, "{name} days");
    }
}

#[test]
fn the_systems_own_tropical_year_is_what_places_the_intercalary_month() {
    // The claim this whole module exists to make, measured three ways over
    // the 823 years of Senmyō-reki.
    //
    //   * its own 歳実 under 恒気 — what it actually used;
    //   * the modern apparent Sun under 定気 — the rule China adopted in
    //     1645 and Japan in 1844, applied eight centuries too early;
    //   * 恒気 on the modern tropical year — the right rule with a year
    //     length nobody in 862 had.
    //
    // The first is worth nearly thirty points of intercalary-month agreement
    // over either of the others. A reconstruction driven by modern solar
    // theory would not be a worse Senmyō-reki; it would be a different
    // calendar.
    const STRIDE: usize = 5;
    let own = measure_sampled(
        &senmyo::PARAMETERS,
        senmyo::EARLIEST.0,
        senmyo::LATEST.0,
        STRIDE,
    );

    static MODERN_TERMS: LunisolarParameters = LunisolarParameters {
        id: hc_calendar::CalendarId("japanese-senmyo-dingqi"),
        english_name: "Senmyō-reki with modern apparent solar terms",
        meridians: &hc_calendars_lunar::japanese_historical::MERIDIANS,
        epoch: hc_calendars_lunar::lunisolar::CHINESE_EPOCH,
        year_offset: hc_calendars_lunar::japanese_historical::YEAR_OFFSET,
        solar_term_mode: SolarTermMode::Apparent,
        mean_motion: Some(senmyo::MODEL),
        earliest: Some(senmyo::EARLIEST),
        latest: Some(senmyo::LATEST),
    };
    let modern_terms = measure_sampled(&MODERN_TERMS, senmyo::EARLIEST.0, senmyo::LATEST.0, STRIDE);

    static MODERN_YEAR: LunisolarParameters = LunisolarParameters {
        id: hc_calendar::CalendarId("japanese-senmyo-modern-year"),
        english_name: "Senmyō-reki on the modern tropical year",
        meridians: &hc_calendars_lunar::japanese_historical::MERIDIANS,
        epoch: hc_calendars_lunar::lunisolar::CHINESE_EPOCH,
        year_offset: hc_calendars_lunar::japanese_historical::YEAR_OFFSET,
        solar_term_mode: SolarTermMode::Mean,
        mean_motion: Some(hc_calendars_lunar::MeanMotionModel {
            tropical_year: 365.2422,
            // Back to the unfitted solstice, since the fitted phase belongs
            // to the system's own year and not to this one.
            solstice_epoch: 314_464.358_529,
            ..senmyo::MODEL
        }),
        earliest: Some(senmyo::EARLIEST),
        latest: Some(senmyo::LATEST),
    };
    let modern_year = measure_sampled(&MODERN_YEAR, senmyo::EARLIEST.0, senmyo::LATEST.0, STRIDE);

    println!(
        "Senmyō-reki intercalary months: own 歳実 {:.2}%, modern 定気 {:.2}%, modern year {:.2}%",
        own.leap_months, modern_terms.leap_months, modern_year.leap_months
    );
    assert!(own.leap_months > modern_terms.leap_months + 20.0);
    assert!(own.leap_months > modern_year.leap_months + 20.0);
    assert!(own.days > modern_terms.days + 4.0);
    assert!(own.days > modern_year.days + 4.0);
}

#[test]
fn the_systems_own_conjunction_tables_do_worse_than_the_true_conjunction() {
    // The other half of the honesty. Reconstructing 定朔 from Senmyō-reki's
    // own 日躔 and 月離 peaks — a single sine each, where the system used a
    // table of daily increments — reproduces the published month starts
    // several points worse than simply taking the true conjunction. Both are
    // shipped; this measures the gap rather than burying it.
    for (name, default, tabulated, first, last) in [
        (
            "Senmyō",
            &senmyo::PARAMETERS,
            &senmyo::PARAMETERS_TABULATED,
            senmyo::EARLIEST.0,
            senmyo::LATEST.0,
        ),
        (
            "Jōkyō",
            &jokyo::PARAMETERS,
            &jokyo::PARAMETERS_TABULATED,
            jokyo::EARLIEST.0,
            jokyo::LATEST.0,
        ),
        (
            "Kansei",
            &kansei::PARAMETERS,
            &kansei::PARAMETERS_TABULATED,
            kansei::EARLIEST.0,
            kansei::LATEST.0,
        ),
    ] {
        let apparent = measure_sampled(default, first, last, 5);
        let own_tables = measure_sampled(tabulated, first, last, 5);
        println!(
            "{name:8}: month starts — true conjunction {:.2}%, own tables {:.2}%",
            apparent.month_starts, own_tables.month_starts
        );
        assert!(
            apparent.month_starts > own_tables.month_starts,
            "{name}: the tabulated model should be the weaker one"
        );
        // But not uselessly weaker: it is still a working calendar.
        assert!(own_tables.month_starts > 80.0, "{name}");
    }
}

#[test]
fn every_day_of_every_system_round_trips() {
    // Independent of the table: whatever date the calendar gives a day, that
    // date must give the day back. Every one of the 341 000 days the four
    // systems cover.
    let mut days = 0;
    for (name, parameters, first, last) in systems() {
        let engine = LunisolarCalendar::new(parameters);
        // Every day of the three Edo systems and every third day of
        // Senmyō-reki's 823 years; the module's own tests take every seventh,
        // so between them no run of six days goes unvisited.
        let stride = if first == senmyo::EARLIEST.0 { 3 } else { 1 };
        let mut rd = first;
        while rd <= last {
            let date = engine.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(engine.to_fixed(date), Ok(Rd(rd)), "{name} at RD {rd}");
            days += 1;
            rd += stride;
        }
    }
    assert!(days > 150_000, "{days} days");
}

#[test]
fn the_four_systems_and_tenpo_tile_the_millennium_without_a_gap() {
    // Every day from 862-02-03 Julian to 1872-12-31 belongs to exactly one
    // Japanese lunisolar calendar, and each refuses every day outside its own
    // period of use.
    let spans = [
        (senmyo::EARLIEST.0, senmyo::LATEST.0),
        (jokyo::EARLIEST.0, jokyo::LATEST.0),
        (horyaku::EARLIEST.0, horyaku::LATEST.0),
        (kansei::EARLIEST.0, kansei::LATEST.0),
        (
            hc_calendars_lunar::japanese_tenpo::EARLIEST.0,
            hc_calendars_lunar::japanese_tenpo::LATEST.0,
        ),
    ];
    for window in spans.windows(2) {
        assert_eq!(window[0].1 + 1, window[1].0, "{window:?}");
    }
    for (name, parameters, first, last) in systems() {
        let engine = LunisolarCalendar::new(parameters);
        assert_eq!(
            engine.from_fixed(Rd(first - 1)),
            Err(CalendarError::BeforeEpoch),
            "{name}"
        );
        assert_eq!(
            engine.from_fixed(Rd(last + 1)),
            Err(CalendarError::AfterSupportedRange),
            "{name}"
        );
    }
}

#[test]
fn the_dynamic_registry_offers_the_right_calendar_for_a_historical_day() {
    use hc_calendar::CalendarRegistry;

    let mut registry = CalendarRegistry::new();
    hc_calendars_lunar::register_all(&mut registry);
    // A day in 1582 is named by Senmyō-reki and by nothing else Japanese.
    let described = registry.describe_day(gregorian(1582, 7, 1));
    let japanese: Vec<_> = described
        .iter()
        .filter(|(_, fields)| fields.is_ok())
        .map(|(id, _)| id.0)
        .filter(|id| id.starts_with("japanese-"))
        .collect();
    assert_eq!(japanese, ["japanese-senmyo"]);
    // And a day in 1800 by Kansei-reki alone.
    let described = registry.describe_day(gregorian(1800, 6, 11));
    let japanese: Vec<_> = described
        .iter()
        .filter(|(_, fields)| fields.is_ok())
        .map(|(id, _)| id.0)
        .filter(|id| id.starts_with("japanese-"))
        .collect();
    assert_eq!(japanese, ["japanese-kansei"]);
}

// --- small helpers -------------------------------------------------------

/// A date in this crate's shared lunisolar representation.
fn date(year: i64, month: Month, day: u8) -> hc_calendars_lunar::LunisolarDate {
    hc_calendars_lunar::LunisolarDate::new(year, month, day)
}

/// Days elapsed before the first of each month in an ordinary year.
const MONTH_OFFSETS: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// The fixed day of a proleptic Gregorian date.
fn gregorian(year: i64, month: usize, day: i64) -> Rd {
    let prior = year - 1;
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    Rd(365 * prior + prior.div_euclid(4) - prior.div_euclid(100)
        + prior.div_euclid(400)
        + MONTH_OFFSETS[month - 1]
        + i64::from(month > 2 && leap)
        + day)
}

/// The fixed day of a Julian date, which is what Japanese Wikipedia gives
/// for everything before 1582-10-15.
fn julian(year: i64, month: usize, day: i64) -> Rd {
    let prior = year - 1;
    Rd(365 * prior
        + prior.div_euclid(4)
        + MONTH_OFFSETS[month - 1]
        + i64::from(month > 2 && year % 4 == 0)
        + day
        - 2)
}
