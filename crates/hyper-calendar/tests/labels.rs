//! The unit walk and the labels over the real calendars, which only the
//! facade has in one place.
//!
//! `hc-calendar` tests the walk on toy calendars and `hc-format` tests the
//! renderer on the Gregorian calendar and a stand-in; what neither can see
//! is the Japanese nengō, the Hebrew leap month, the Chinese leap month or
//! the Long Count as they are actually implemented, and the vocabulary as
//! it is actually keyed. These are the cases the timeline was asked for,
//! held to the answers it was promised.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "regional",
    feature = "i18n",
    feature = "format",
))]
#![expect(
    clippy::expect_used,
    reason = "a fixture that does not convert is a failed test, and the \
              message says which"
)]

use hyper_calendar::hc_calendar::units::{Unit, UnitSpan, units};
use hyper_calendar::hc_calendar::{CalendarError, CalendarRegistry, DynCalendar, Rd};
use hyper_calendar::hc_calendars_solar::gregorian;
use hyper_calendar::hc_format::label;
use hyper_calendar::hc_i18n::Locale;
use hyper_calendar::lines;

fn registry() -> CalendarRegistry {
    hyper_calendar::registry()
}

fn day(year: i64, month: u8, day: u8) -> Rd {
    gregorian::to_fixed(year, month, day).expect("a Gregorian date")
}

fn locale(tag: &str) -> Locale {
    Locale::parse(tag).expect("a tag")
}

/// The label of one unit of the calendar on a day, in a locale.
fn label_on(calendar: &dyn DynCalendar, on: Rd, unit: Unit, tag: &str) -> String {
    let fields = calendar.fixed_to_fields(on).expect("converts");
    let locale = label::locale_for(calendar, Some(&locale(tag)));
    label::label(calendar, &fields, unit, &locale)
}

fn date_on(calendar: &dyn DynCalendar, on: Rd, tag: &str) -> String {
    let fields = calendar.fixed_to_fields(on).expect("converts");
    let locale = label::locale_for(calendar, Some(&locale(tag)));
    label::date(calendar, &fields, &locale)
}

fn contiguous(spans: &[UnitSpan]) {
    for pair in spans.windows(2) {
        assert_eq!(pair[0].end, pair[1].start, "{pair:?}");
    }
    for span in spans {
        assert!(span.start < span.end, "{span:?}");
    }
}

#[test]
fn the_japanese_eras_are_labelled_from_the_locale_and_from_the_calendar() {
    let registry = registry();
    let japanese = registry.get_by_name("japanese").expect("japanese");
    // 令和元年 on the day the era began; 令和6年 five years on.
    assert_eq!(
        label_on(japanese, day(2019, 5, 1), Unit::Year, "ja"),
        "令和元年"
    );
    assert_eq!(
        label_on(japanese, day(2024, 3, 1), Unit::Year, "ja"),
        "令和6年"
    );
    assert_eq!(label_on(japanese, day(2024, 3, 1), Unit::Era, "ja"), "令和");
    assert_eq!(
        label_on(japanese, day(2024, 3, 1), Unit::Year, "en"),
        "6 Reiwa"
    );
    assert_eq!(
        date_on(japanese, day(2026, 9, 21), "ja-JP"),
        "令和8年9月21日"
    );
    assert_eq!(
        date_on(japanese, day(2026, 9, 21), "en"),
        "September 21, 8 Reiwa"
    );
    // 嘉永三年: an era the locale data does not list, written from the
    // calendar's own table — in kanji for Japanese, romanised for English.
    let kaei = day(1850, 6, 1);
    let fields = japanese.fixed_to_fields(kaei).expect("converts");
    assert_eq!(fields.era, Some("kaei"));
    assert_eq!(fields.year, 3);
    assert_eq!(label_on(japanese, kaei, Unit::Year, "ja"), "嘉永3年");
    assert_eq!(label_on(japanese, kaei, Unit::Era, "ja"), "嘉永");
    assert_eq!(label_on(japanese, kaei, Unit::Year, "en"), "3 Kaei");
    assert_eq!(label_on(japanese, kaei, Unit::Era, "ko"), "嘉永");
    // The walk over the eras from the Restoration to Reiwa.
    let spans = units(japanese, Unit::Era, day(1868, 1, 1), day(2019, 12, 31));
    contiguous(&spans);
    let labels: Vec<String> = spans
        .iter()
        .map(|span| {
            let dated = span.date().expect("an era");
            label::label(japanese, &dated.fields, Unit::Era, &locale("ja"))
        })
        .collect();
    assert!(
        labels.ends_with(&[
            "明治".to_owned(),
            "大正".to_owned(),
            "昭和".to_owned(),
            "平成".to_owned(),
            "令和".to_owned()
        ]),
        "{labels:?}"
    );
    let reiwa = spans.last().expect("reiwa");
    assert_eq!(reiwa.start, day(2019, 5, 1));
    let heisei = &spans[spans.len() - 2];
    assert_eq!(heisei.start, day(1989, 1, 8));
    assert_eq!(heisei.end, day(2019, 5, 1));
    // Years within an era: 令和元年 begins with the era, not on New Year's
    // Day, and the next year does.
    let spans = units(japanese, Unit::Year, day(2019, 5, 1), day(2020, 1, 2));
    contiguous(&spans);
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start, day(2019, 5, 1));
    assert_eq!(spans[1].start, day(2020, 1, 1));
    assert!(
        spans[1].date().expect("a year").leap,
        "令和2年 is the leap year 2020"
    );
    assert!(!spans[0].date().expect("a year").leap);
}

#[test]
fn the_hebrew_and_hijri_calendars_are_labelled_as_their_sources_write_them() {
    let registry = registry();
    let hebrew = registry.get_by_name("hebrew").expect("hebrew");
    // 1 Tishri 5784 was 2023-09-16; 5784 is a leap year with an Adar I.
    let rosh_hashanah = day(2023, 9, 16);
    assert_eq!(label_on(hebrew, rosh_hashanah, Unit::Year, "en"), "5784");
    assert_eq!(label_on(hebrew, rosh_hashanah, Unit::Era, "en"), "AM");
    assert_eq!(label_on(hebrew, rosh_hashanah, Unit::Month, "en"), "Tishri");
    assert_eq!(label_on(hebrew, rosh_hashanah, Unit::Year, "he"), "5784");
    assert_eq!(label_on(hebrew, rosh_hashanah, Unit::Month, "he"), "תשרי");
    // Japanese has no words for the Hebrew months, so a Japanese request
    // is answered in English, not in Hebrew; only `native` asks for Hebrew.
    assert_eq!(
        label::locale_for(hebrew, Some(&locale("ja"))).to_string(),
        "en"
    );
    assert_eq!(label_on(hebrew, rosh_hashanah, Unit::Month, "ja"), "Tishri");
    assert_eq!(label::locale_for(hebrew, None).to_string(), "he");
    let adar_i = day(2024, 2, 11);
    assert_eq!(label_on(hebrew, adar_i, Unit::Month, "en"), "Adar I");
    // Up to the eve of 1 Tishri 5785, 2024-10-03.
    let spans = units(hebrew, Unit::Month, rosh_hashanah, day(2024, 10, 2));
    contiguous(&spans);
    assert_eq!(spans.len(), 13, "a leap year has thirteen months");
    let names: Vec<String> = spans
        .iter()
        .map(|span| {
            let dated = span.date().expect("a month");
            label::label(hebrew, &dated.fields, Unit::Month, &locale("en"))
        })
        .collect();
    assert_eq!(names[4], "Shevat");
    assert_eq!(names[5], "Adar I");
    assert_eq!(names[6], "Adar II", "the Adar of a leap year");
    assert_eq!(names[7], "Nisan");
    assert!(spans[5].date().expect("a month").leap);
    assert!(spans[5].days() == 30, "Adar I has thirty days");
    // 5785 is a common year: its Adar is plain Adar. 1 Adar 5785 was
    // 2025-03-01.
    assert_eq!(label_on(hebrew, day(2025, 3, 1), Unit::Month, "en"), "Adar");
    assert_eq!(label_on(hebrew, day(2025, 3, 1), Unit::Month, "he"), "אדר");
    assert_eq!(label_on(hebrew, adar_i, Unit::Month, "he"), "אדר א׳");

    let hijri = registry
        .get_by_name("islamic-civil")
        .expect("islamic-civil");
    // 1 Ramadan 1445 was 2024-03-11 in the civil tabular calendar.
    let ramadan = day(2024, 3, 11);
    assert_eq!(label_on(hijri, ramadan, Unit::Year, "en"), "1445 AH");
    assert_eq!(label_on(hijri, ramadan, Unit::Month, "en"), "Ramadan");
    assert_eq!(label_on(hijri, ramadan, Unit::Year, "ar"), "١٤٤٥ هـ");
    assert_eq!(label_on(hijri, ramadan, Unit::Month, "ar"), "رمضان");
    assert_eq!(date_on(hijri, ramadan, "ar"), "١ رمضان ١٤٤٥ هـ");
    // Japanese names neither, so a Japanese request is answered in
    // English, not in Arabic.
    assert_eq!(label_on(hijri, ramadan, Unit::Month, "ja"), "Ramadan");
}

#[test]
fn the_chinese_calendar_is_labelled_by_its_cycle_and_its_day_names() {
    let registry = registry();
    let chinese = registry.get_by_name("chinese").expect("chinese");
    // 2023-03-22 was 癸卯年闰二月初一.
    let leap_new_moon = day(2023, 3, 22);
    assert_eq!(
        label_on(chinese, leap_new_moon, Unit::Year, "zh-Hans"),
        "癸卯年"
    );
    assert_eq!(
        label_on(chinese, leap_new_moon, Unit::Month, "zh-Hans"),
        "闰二月"
    );
    assert_eq!(
        label_on(chinese, leap_new_moon, Unit::Day, "zh-Hans"),
        "初一"
    );
    assert_eq!(
        date_on(chinese, leap_new_moon, "zh-Hans"),
        "癸卯年闰二月初一"
    );
    assert_eq!(
        date_on(chinese, leap_new_moon, "zh-Hant"),
        "癸卯年閏二月初一"
    );
    assert_eq!(
        label_on(chinese, leap_new_moon, Unit::Month, "ja"),
        "閏二月"
    );
    assert_eq!(label_on(chinese, leap_new_moon, Unit::Year, "ja"), "癸卯年");
    assert_eq!(label_on(chinese, leap_new_moon, Unit::Day, "ja"), "1日");
    assert_eq!(
        label_on(chinese, leap_new_moon, Unit::Month, "en"),
        "leap Second Month"
    );
    assert_eq!(
        label_on(chinese, day(2023, 3, 25), Unit::Day, "zh-Hans"),
        "初四"
    );
    // A Japanese calendar is not the Chinese one: 師走 stays with the
    // Tenpō calendar, and the Chinese twelfth month is 十二月 in Japanese.
    let tenpo = registry.get_by_name("japanese-tenpo").expect("tenpo");
    let twelfth = day(1850, 1, 20);
    assert_eq!(label_on(tenpo, twelfth, Unit::Month, "ja"), "師走");
    assert_eq!(label_on(chinese, twelfth, Unit::Month, "ja"), "十二月");
    assert_eq!(label_on(chinese, twelfth, Unit::Month, "zh-Hans"), "腊月");
    // The months of 2023, the leap month among them.
    let spans = units(chinese, Unit::Month, day(2023, 1, 22), day(2024, 2, 10));
    contiguous(&spans);
    assert_eq!(spans.len(), 13);
    let leap: Vec<&UnitSpan> = spans
        .iter()
        .filter(|span| span.date().is_some_and(|dated| dated.leap))
        .collect();
    assert_eq!(leap.len(), 1);
    assert_eq!(leap[0].start, leap_new_moon);
    assert_eq!(leap[0].end, day(2023, 4, 20));
    // And the years, by their cycle names.
    let spans = units(chinese, Unit::Year, day(2023, 6, 1), day(2025, 6, 1));
    contiguous(&spans);
    let names: Vec<String> = spans
        .iter()
        .map(|span| {
            let dated = span.date().expect("a year");
            label::label(chinese, &dated.fields, Unit::Year, &locale("zh-Hans"))
        })
        .collect();
    assert_eq!(names, ["癸卯年", "甲辰年", "乙巳年"]);
    assert_eq!(spans[1].start, day(2024, 2, 10));
    assert!(
        spans[0].date().expect("a year").leap,
        "癸卯 has a leap month"
    );
}

#[test]
fn the_gregorian_years_a_bounded_calendar_and_the_long_count_walk() {
    let registry = registry();
    let gregorian = registry.get_by_name("gregory").expect("gregory");
    let spans = units(gregorian, Unit::Year, day(2023, 6, 1), day(2025, 6, 1));
    contiguous(&spans);
    let years: Vec<(i64, bool)> = spans
        .iter()
        .map(|span| {
            let dated = span.date().expect("a year");
            (dated.fields.year, dated.leap)
        })
        .collect();
    assert_eq!(years, [(2023, false), (2024, true), (2025, false)]);
    assert_eq!(spans[0].start, day(2023, 1, 1));
    assert_eq!(spans[2].end, day(2026, 1, 1));
    let months = units(gregorian, Unit::Month, day(2024, 2, 10), day(2024, 3, 5));
    assert_eq!(months.len(), 2);
    assert_eq!(months[0].days(), 29);
    assert_eq!(
        label::label(
            gregorian,
            &months[0].date().expect("a month").fields,
            Unit::Month,
            &locale("en")
        ),
        "February"
    );

    // The Rumi calendar was kept from 1840 to 1925: both ends are refusals.
    let rumi = registry.get_by_name("rumi").expect("rumi");
    let meta = rumi.meta();
    let (earliest, latest) = (
        meta.earliest.expect("bounded"),
        meta.latest.expect("bounded"),
    );
    let spans = units(rumi, Unit::Year, Rd(earliest.0 - 100), Rd(latest.0 + 100));
    contiguous(&spans);
    assert_eq!(spans[0].error(), Some(CalendarError::BeforeEpoch));
    assert_eq!(
        (spans[0].start, spans[0].end),
        (Rd(earliest.0 - 100), earliest)
    );
    let last = spans.last().expect("a span");
    assert_eq!(last.error(), Some(CalendarError::AfterSupportedRange));
    assert_eq!(
        (last.start, last.end),
        (Rd(latest.0 + 1), Rd(latest.0 + 100))
    );
    assert!(
        spans[1..spans.len() - 1]
            .iter()
            .all(|span| span.date().is_some())
    );
    assert_eq!(spans[1].start, earliest, "the first year is clipped");
    assert_eq!(spans[spans.len() - 2].end, Rd(latest.0 + 1));

    // The Long Count has no eras, years or months, only days.
    let long_count = registry.get_by_name("maya-longcount").expect("long count");
    for (unit, field) in [
        (Unit::Era, "era"),
        (Unit::Year, "year"),
        (Unit::Month, "month"),
    ] {
        let spans = units(long_count, unit, day(2012, 12, 1), day(2013, 1, 1));
        assert_eq!(spans.len(), 1, "{unit:?}");
        assert_eq!(
            spans[0].error(),
            Some(CalendarError::UnsupportedField(field)),
            "{unit:?}"
        );
    }
    let spans = units(long_count, Unit::Day, day(2012, 12, 21), day(2012, 12, 23));
    assert_eq!(spans.len(), 2);
    let baktun_end = spans[0].date().expect("a day");
    assert_eq!(baktun_end.fields.extra.get("baktun"), Some(13));
    // Nobody names the Long Count, so its label is its fields in order.
    let text = label::date(long_count, &baktun_end.fields, &locale("en"));
    assert!(text.contains("baktun=13"), "{text}");
}

#[test]
fn the_lines_carry_the_labels_and_name_the_locale_used() {
    let registry = registry();
    let text = lines::describe_day(&registry, day(2026, 9, 21), "ja");
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(
        rows.iter()
            .all(|row| row.len() == lines::DESCRIBE_DAY_COLUMNS)
    );
    let japanese = rows
        .iter()
        .find(|row| row[0] == "japanese")
        .expect("japanese");
    assert_eq!(japanese[15..17], ["令和8年9月21日", "ja"]);
    // Japanese does not name the Hebrew calendar, so it is English, not
    // Hebrew: a named locale never borrows the calendar's own language.
    let hebrew = rows.iter().find(|row| row[0] == "hebrew").expect("hebrew");
    assert_eq!(hebrew[16], "en");
    let text = lines::describe_day(&registry, day(2026, 9, 21), lines::NATIVE);
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    let chinese = rows
        .iter()
        .find(|row| row[0] == "chinese")
        .expect("chinese");
    assert_eq!(chinese[16], "zh-Hans");
    let gregorian = rows
        .iter()
        .find(|row| row[0] == "gregory")
        .expect("gregory");
    assert_eq!(gregorian[16], "en");
    let hebrew = rows.iter().find(|row| row[0] == "hebrew").expect("hebrew");
    assert_eq!(hebrew[16], "he");

    let japanese = registry.get_by_name("japanese").expect("japanese");
    let text = lines::calendar_units(
        japanese,
        Unit::Era,
        day(1989, 1, 1),
        day(2019, 12, 31),
        "ja",
    );
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(
        rows.iter()
            .all(|row| row.len() == lines::CALENDAR_UNITS_COLUMNS)
    );
    let labels: Vec<&str> = rows.iter().map(|row| row[2]).collect();
    assert_eq!(labels, ["昭和", "平成", "令和"]);

    let text = lines::calendars(&registry, day(2026, 9, 21), "en");
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(rows.iter().all(|row| row.len() == lines::CALENDARS_COLUMNS));
    assert_eq!(rows.len(), registry.len());
    let hebrew = rows.iter().find(|row| row[0] == "hebrew").expect("hebrew");
    assert_eq!(hebrew[1], "Hebrew Calendar");
    assert_eq!(hebrew[9], "he");

    let text = lines::locales();
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(rows.iter().all(|row| row.len() == lines::LOCALES_COLUMNS));
    assert!(
        rows.iter()
            .any(|row| row[0] == "zh-Hans" && row[2] == "简体中文")
    );
}

/// The second cell of `lines::calendars` for one calendar in one locale.
fn calendar_name(tag: &str, id: &str) -> String {
    let registry = registry();
    lines::calendars(&registry, day(2026, 9, 21), tag)
        .lines()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .find(|row| row[0] == id)
        .map(|row| row[1].to_owned())
        .expect("a listed calendar")
}

#[test]
fn a_calendar_the_locale_has_no_name_for_is_named_nothing() {
    // Tibetan has no CLDR calendar names, so neither the Japanese nor the
    // Hebrew calendar is named in it; before, each came back in its own
    // language, which is what `native` asks for and `bo` does not.
    assert_eq!(calendar_name("bo", "japanese"), "");
    assert_eq!(calendar_name("bo", "hebrew"), "");
    // German names the Japanese calendar and not the Tibetan one.
    assert_eq!(calendar_name("de", "japanese"), "Japanischer Kalender");
    assert_eq!(calendar_name("de", "tibetan"), "");
    // A tag that does not parse is the root locale, which names nothing.
    assert_eq!(calendar_name("not a tag", "gregory"), "");
    // Only `native` asks for the calendar's own language.
    assert_eq!(calendar_name(lines::NATIVE, "japanese"), "和暦");
    assert_eq!(calendar_name(lines::NATIVE, "hebrew"), "לוח השנה העברי");
}

#[test]
fn the_dangi_calendar_is_named_as_cldr_names_it_in_both_chinese_scripts() {
    // CLDR 48 `zh.xml` and `zh_Hant.xml`, type `dangi`.
    assert_eq!(calendar_name("zh-Hans", "dangi"), "檀纪历");
    assert_eq!(calendar_name("zh-Hant", "dangi"), "檀紀曆");
    assert_eq!(calendar_name("ko", "dangi"), "단기력");
    assert_eq!(calendar_name("ja", "dangi"), "ダンギ暦");
}

#[test]
fn a_japanese_request_writes_the_umm_al_qura_calendar_in_english() {
    let registry = registry();
    let on = day(2026, 9, 26);
    let row = |tag: &str| -> Vec<String> {
        lines::describe_day(&registry, on, tag)
            .lines()
            .map(|line| line.split('\t').map(str::to_owned).collect::<Vec<_>>())
            .find(|row| row[0] == "islamic-umalqura")
            .expect("islamic-umalqura")
    };
    let japanese = row("ja");
    assert_eq!(japanese[15..17], ["Rabi II 15, 1448 AH", "en"]);
    assert!(
        !japanese[15]
            .chars()
            .any(|c| ('\u{0600}'..='\u{06FF}').contains(&c)),
        "no Arabic script under ja: {}",
        japanese[15]
    );
    // Only `native` reaches for Arabic.
    assert_eq!(row(lines::NATIVE)[16], "ar");
    let umalqura = registry
        .get_by_name("islamic-umalqura")
        .expect("islamic-umalqura");
    let units = lines::calendar_units(umalqura, Unit::Month, on, day(2026, 9, 27), "ja");
    assert!(units.trim_end().ends_with("\ten"), "{units}");
}
