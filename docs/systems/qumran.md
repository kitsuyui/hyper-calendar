# The Qumran and Jubilees 364-day year, and the mishmarot

Backs the identifier `qumran` in `hc-calendars-solar`.

## What it is

About twenty of the Dead Sea Scrolls from Qumran deal with calendars, and
the one most of them assume is a schematic year of 364 days, the year of
the Book of Jubilees and, with differences, of the Astronomical Book of
Enoch [wikipedia-qumran-calendrical-texts]. It was the covenanters'
answer to the 354-day lunar year of the Jerusalem Temple, and it came
with the *mishmarot*: rosters of the weekly service of the twenty-four
priestly courses of 1 Chronicles 24, kept against the day the covenanters
would serve in a new Temple [talmon2000, pp. 110–111]. Whether it was ever
kept in practice for long is disputed, and so is whether it was corrected
against the sun: Glessmer reads a leap week into 4Q319, Beckwith thinks
none was wanted [wikipedia-qumran-calendrical-texts, after vanderkam1998].
No text states an intercalation.

## How it works

**The year.** Twelve numbered months in four quarters of 30, 30 and 31
days, 91 days or thirteen weeks each, 364 in all. "The first and fifteenth
days of the first month of each quarter fall invariably on the fourth day
of the week" — Wednesday, the day the luminaries were made — so "the
Sabbaths always fall on the same monthly dates, and each festival falls on
the same weekday" [talmon2000, p. 110]. The scrolls count: "The
twenty-third of it [the second month] is a Sabbath. The thirtieth [of it]
is a Sabbath. The seventh of the third [month] is a Sabbath … The
twenty-eighth of it is a Sabbath. After it, the first and second day [of
the week] a day is added and the quarter terminates [with] ninety-one
days" (4Q320–321, as Talmon quotes them). The Passover lamb is on Tuesday
14/I and Passover on Wednesday 15/I, the Omer on Sunday 26/I, Shavuot on
Sunday 15/III, Yom Kippur on Friday 10/VII and Sukkot on Wednesday 15/VII
[talmon2000, p. 110].

**No intercalation.** The year is a day and a quarter short of the sun's,
so it moves through the seasons by a month in 25 years and through the
whole year in about 290. A calendar that runs on the week alone has that
price, and this library pays it rather than choosing one of the modern
correction schemes.

**The courses.** The texts trace the rotation to the creation: "Creation
[was] on the fourth [day] of Ga[mul]" (4Q319 i.11), and 4Q320 opens its
cycle "on the fourth in the week of the sons of Gamul, in the first of the
month, in the first year", Gamul "at the head of all years"
[talmon2000, pp. 110–111]. Each course serves a week and the next follows;
52 weeks a year against 24 courses closes after six years, 312 weeks, and
4Q328–329 name the courses at the head of each year: Gamul, Jedaiah,
Mijamin, Shekaniah, Jeshbeab, Happittet [talmon2000, p. 111]. A week of
service is dated by its days, "on the third (day) in the week of the sons
of Maaziah [falls] the Pesah"; 4Q325 instead names each Sabbath after "the
watches that enter the Temple on Saturday afternoon and begin their duties
on Sunday morning" [talmon2000, p. 111].

**Worked example.** Take the first year. Its first day is a Wednesday in
Gamul's week. 14/I is thirteen days later, a Tuesday, the third day of the
week, and two weeks on from Gamul's, so in the week of Delaiah's successor,
Maaziah — the Pesah of 4Q320. 1/II is day 31, a Friday; four weeks and two
days after the first day, in the fifth week, Jedaiah's (Gamul, Delaiah,
Maaziah, Joiarib, Jedaiah); the next day, 2/II, is the Sabbath 4Q325 calls
Ḥarim's, the course entering that afternoon.

## What is carried

- **Identifier** `qumran`: year, month and day; the shape declares twelve
  numbered months, the seven-day week, and the `mishmar` cycle of 24 named
  courses in the order of 1 Chronicles 24, in Talmon's spellings. English
  names the months "First Month" to "Twelfth Month" (`hc-i18n`).
- **Fields** `to_fields` adds `day-of-week` as the scrolls number it, 1
  for the first day to 7 for the Sabbath; `mishmar`, the course of the week
  from 1 (Joiarib) to 24 (Maaziah); and `cycle-year`, 1 to 6, 1 being
  Gamul's. `qumran::entering_course` gives 4Q325's name for a Sabbath.
- **The epoch is a convention of this library**, and so is the year
  count. No text and no source read ties this year to a Julian day, and a
  year that drifts through the seasons has no place in them to find. Year
  1, day 1 is `EPOCH`, Wednesday 23 March AD 1 (Julian; 21 March
  Gregorian), the first Wednesday on or after the Julian 21 March of AD 1,
  chosen only to be near an equinox at the turn of the era; years count
  back through 0 and below it, over −99 999 to 99 999. The weekday, the
  month, the festivals and the courses are right on any epoch that is a
  Wednesday; the Julian or Gregorian date beside them is this convention,
  not a reconstruction, and anybody with a scholar's correlation should
  add it under its own identifier.
- **Usage** unrecorded: no source read dates when the calendar was kept.
- **Not carried:** the lunar data of 4Q317 and 4Q320–321, the 294-year
  *Otot* cycle of 4Q319, the festivals as observances (for `hc-holiday`),
  the Enoch form with the four added days outside the count, and any
  intercalation scheme.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| Every quarter's 1st and 15th are Wednesdays | `every_quarter_begins_on_the_fourth_day_of_the_week` | all |
| The Sabbaths of 4Q320–321 (23/II, 30/II, 7/III, 14/III, 21/III, 28/III, 21/VI), the 91-day quarter ending on the first and second days of the week, and the festivals' weekdays | `the_sabbaths_and_festivals_fall_where_the_scrolls_put_them` | all |
| The first year's festivals by day and course, 4Q320 4.ii as Talmon restores it: Pesah, the Omer, the Second Pesah, Weeks, Remembrance, Atonement, Booths | `the_first_years_festivals_fall_in_the_courses_of_4q320` | all seven |
| The heads of the six years, 4Q328–329, and thirteen weeks for every course in six years | `the_six_years_are_headed_by_the_courses_of_4q329` | all |
| 4Q325: 1/II on the sixth day in Jedaiah's week, 2/II the Sabbath of Ḥarim | `the_sabbath_is_named_for_the_course_that_enters` | all |
| The drift: a hundred years are 99.66 Julian years | `the_year_drifts_against_the_seasons` | all |
| Round trips over the whole range and every day of ten years round the epoch | `every_day_round_trips` | all |

The texts are fragmentary and several of these entries are restorations;
they are checked here as Talmon restores them. That the restorations are
consistent with one rotation from Gamul, in every entry tested, is some
evidence that the rotation is the one the scrolls assume.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [talmon2000] | The year, the Sabbaths and festivals, the courses and their rotation, the texts quoted | Yes, the scan, pp. 110–111, 2026-09-26 |
| [wikipedia-qumran-calendrical-texts] | The quarters, Jubilees and Enoch, the intercalation debate | Yes, 2026-09-26 |
| [vanderkam1998] | The intercalation proposals | Not read |

## Code

`crates/hc-calendars-solar/src/qumran.rs`. Anchors:
`the_sabbaths_and_festivals_fall_where_the_scrolls_put_them`,
`the_first_years_festivals_fall_in_the_courses_of_4q320`,
`the_six_years_are_headed_by_the_courses_of_4q329`. The rotation is
`course`; the convention `EPOCH`.
