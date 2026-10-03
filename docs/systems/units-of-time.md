# Units of time that an authority defined

Backs the crate `hc-units`, behind the facade feature `units` (re-exported
as `hyper_calendar::hc_units`, with `Quantity`, `Ratio`, `Tempo` and `Unit`
in the prelude): `Ratio`, `Unit` and its 53 entries in `unit::ALL` with
`unit::by_id`, `Quantity`, `Family`, `media::Rate` with `FRAME_RATES` and
`SAMPLE_RATES`, and `tempo::Tempo`, `NoteValue`, `TimeSignature` and
`Ppqn`. No calendar identifier is registered, and no WebAssembly or C
export exists yet ([policy.md](../policy.md) §5, §3).

## What it is

A unit of time is a name for a length. For some the length is *defined*: an
authority wrote it down as a multiple of the second, and it has no error
bar. For others it is *measured*: the tropical year, the sidereal day and
the synodic month are numbers with an uncertainty that is revised. This
document is about the first kind. The second kind lives with the model that
measured it, in `hc-astro`, `hc-planetary` and `hc-deep-time`.

**The second and its prefixes.** The BIPM fixes the second by taking the
unperturbed ground-state hyperfine transition frequency of caesium-133 to
be 9 192 631 770 Hz [bipm-second]. It lists 24 prefixes, from quecto
(10⁻³⁰) to quetta (10³⁰). Deci to micro and deca to giga date from 1960,
femto and atto from 1964, zepto, yocto, zetta and yotta from 1991, and
ronto, quecto, ronna and quetta from CGPM Resolution 3 of 2022
[bipm-si-prefixes].

**The civil units.** A minute is 60 s [wikipedia-minute] and a day is
86 400 s [wikipedia-day]. UTC lets a minute have 61 s and a civil day
86 401 or 86 399 s, so these are nominal lengths [wikipedia-minute;
wikipedia-day]. A Julian year is exactly 365.25 days, 31 557 600 s
[wikipedia-julian-year-astronomy]. The Gregorian calendar has 97 leap years
in 400, so its mean year is 365 97⁄400 days and 400 years are 146 097 days
[wikipedia-gregorian-calendar].

**Subdivisions of the day from other traditions.**

- The Hebrew calendar divides the hour into 1080 parts, *halakim*
  [maimonides-kiddush-hachodesh, chapter 6, halacha 2], so a *helek* is
  3⅓ s [wikipedia-helek]. Maimonides gives the *rega*, "moment", as a
  seventy-sixth of the part [sefaria-maimonides-kh-10].
- The *Sūrya Siddhānta* counts *prāṇa*, which the translation glosses as
  "a portion of time which contains four seconds". Six prāṇas make a
  *pala*, sixty palas a *ghaṭikā*, and sixty ghaṭikās a sidereal day and
  night [sastri1861, verses 11 and 12]. A *muhūrta* is a thirtieth of a
  day, 48 minutes [wikipedia-muhurta; wikipedia-hindu-units-of-time].
- The medieval European *moment* was a fortieth of a solar hour, which
  varied with the season, so that "on average" it was 90 s
  [wikipedia-moment-time].
- The Chinese 刻 *kè* was usually a hundredth of a day until 1628 and a
  ninety-sixth since, with short periods of 96, 108 and 120 to the day
  before [wikipedia-traditional-chinese-timekeeping]. The 時辰 *shíchén*
  is a twelfth of a day, two hours [wikipedia-shichen].

**Decimal and hexadecimal days.** Article XI of the decree of 4 frimaire
an II divides the day, midnight to midnight, into 10 hours, each into ten
parts and so on; "la 100ᵉ partie de l'heure est appelée minute décimale; la
100ᵉ partie de la minute est appelée seconde décimale"
[decret-4-frimaire-an-ii]. Swatch divides the day into 1000 *.beats* of
"1 Minute 26.4 Seconds" [swatch-internet-time]. John W. Nystrom proposed a
day of 16 hexadecimal hours, in 1862 or 1863 [wikipedia-hexadecimal-time].

**Media, computing and science.** Facebook defined the *flick* as
1/705 600 000 s, "exactly", so that frames and audio samples at the usual
rates are whole numbers of it [facebook-flicks]. Windows `FILETIME` counts
100-nanosecond intervals from 1601 [ms-filetime], and .NET's `DateTime.Ticks`
counts the same interval from year 1 [ms-datetime-ticks]. A *jiffy* is the
period of a 60 Hz or 50 Hz mains cycle, or the time light takes to cross a
centimetre, or something else [wikipedia-jiffy-time]. A *microfortnight*
is 1.2096 s [wikipedia-fff-system]. A *shake* is 10 ns
[wikipedia-shake-unit] and a *svedberg* is exactly 10⁻¹³ s
[wikipedia-svedberg].

**Frame rates.** NTSC colour, approved by the FCC in December 1953,
reduced the frame rate from 30 to 30/1.001 fps, about 29.97
[wikipedia-ntsc]. The line rate became 1/286 of the 4.5 MHz audio
subcarrier, and the colour subcarrier 35/44 of it [wikipedia-colorburst].
Film shown on NTSC is slowed by the same 1000/1001 ratio, to 23.976 fps;
the 2:3 cadence that spreads four film frames over five video frames is a
separate step [wikipedia-telecine].

**Musical time.** A MIDI file states a tempo as the microseconds in a
quarter note, in a three-byte *Set Tempo* event; with none given the tempo
is 120 beats per minute [midimusic-smf-spec]. `07 A1 20` is 500 000 µs,
120 per minute [midi-org-tempo-forum]. The header's division gives ticks
per quarter note, and tempo over division is one tick
[midimusic-smf-spec; midi-org-delta-time-forum]. A dot adds half a note's
value and the *x*th dot adds 1/2ˣ [wikipedia-dotted-note]; a triplet note
is 2⁄3 of a normal one [wikipedia-tuplet]; the lower number of a time
signature is a power of two, and in compound meter the beat is a dotted
note, as the dotted quarter in 6/8 [wikipedia-time-signature].

## How it works

**The representation.** A `Ratio` is an `i128` numerator over a positive
`i128` denominator in lowest terms, so equal quantities are equal values
and `Eq` and `Hash` agree with it. Addition, multiplication and division
are `checked_*` and return `UnitError::Overflow` rather than wrap;
multiplication cross-reduces first, so `1 flick × 705 600 000` stays in
range. `Ord` compares by cross-multiplication and falls back to `f64` only
when that overflows. `Display` prints `3` or `10/3`, never a decimal.

A `Unit` is a plain struct: `id`, `name`, an optional `symbol`, `seconds`
as a `Ratio`, a `Family` and an `authority` string. It is not an enum, so a
caller can write down a unit the table does not have and convert with it
([policy.md](../policy.md) §5, "Where the list itself is open").
`Unit::per(other)` is the quotient of the two lengths, so the number of `b`
in one `a`, exactly. A `Quantity` is a `Ratio` count of a `Unit`;
`Quantity::to` converts it and `divides_into` asks whether the result is a
whole number, answering `false` where the arithmetic overflows. `by_id`
looks an identifier up by `hc_core::catalogue::matches`: the white space
around it is ignored and ASCII letters match in either case, so
`" FLICK "` finds `flick`.

**Why a `Ratio` and not a `Duration`.** A `hc_core::Duration` is whole
seconds and a count of attoseconds, which is a decimal grid. A `Ratio`
fits it exactly when its reduced denominator divides 10¹⁸ = 2¹⁸·5¹⁸, and
`to_duration` returns `UnitError::Inexact` otherwise instead of rounding.
`to_duration_rounded` rounds to the nearest attosecond where the caller
asks. Of the 53 units, 44 have an exact `Duration`. The nine that do not
are the four SI units below the attosecond, the two jiffy lengths 1/60 s
and the light-centimetre, the flick, the rega and the helek.

**The lengths.** Each is derived from the source's own ratio, so none is
typed in as a decimal.

| Unit | Seconds | Derivation |
| --- | --- | --- |
| `rega` | 5/114 | a 76th of a helek |
| `helek` | 10/3 | an 1080th of an hour |
| `prana` | 4 | 1/21 600 of a day, a sixth of a vighati |
| `vighati` | 24 | 1/3600 of a day, a sixtieth of a ghati |
| `ghati` | 1440 | a sixtieth of a day |
| `muhurta` | 2880 | a thirtieth of a day, two ghati |
| `moment` | 90 | a fortieth of an hour |
| `ke-hundred` | 864 | a hundredth of a day |
| `ke-ninety-six` | 900 | a ninety-sixth of a day |
| `shichen` | 7200 | a twelfth of a day |
| `decimal-second` | 108/125 | 0.864 s, a hundredth of a decimal minute |
| `decimal-minute` | 432/5 | 86.4 s, a hundredth of a decimal hour |
| `beat` | 432/5 | a thousandth of a day, the decimal minute's length |
| `decimal-hour` | 8640 | a tenth of a day |
| `hex-second` | 675/512 | 1/65 536 of a day |
| `hex-minute` | 675/32 | 1/4096 of a day |
| `hex-maxime` | 675/2 | 1/256 of a day |
| `hex-hour` | 5400 | 1/16 of a day |
| `flick` | 1/705 600 000 | 705 600 000 = 2⁹·3²·5⁵·7² |
| `tick-filetime` | 1/10 000 000 | 100 ns |
| `shake` | 1/100 000 000 | 10 ns |
| `svedberg` | 1/10¹³ | 100 fs |
| `jiffy-light-centimetre` | 1/29 979 245 800 | 1 cm over the exact c |
| `jiffy-mains-60` | 1/60 | one cycle of 60 Hz |
| `jiffy-mains-50` | 1/50 | one cycle of 50 Hz |
| `microfortnight` | 756/625 | 1.2096 s, a millionth of a fortnight |

The 14 SI units are the second and 13 prefixed seconds, from `quectosecond`
to `gigasecond`. The 13 civil units are the minute, hour, day, week
(7 days), fortnight (14 days), the common year (365 days, 31 536 000 s),
the mean Gregorian year (31 556 952 s) with its quarter (7 889 238 s) and
month (2 629 746 s), the leap year (366 days, 31 622 400 s), and the
Julian year (31 557 600 s), century (3 155 760 000 s) and millennium
(31 557 600 000 s).

**Competing conventions get names.** Policy §5 applies. The 刻 of 100 to
the day and of 96 to the day are `ke-hundred` and `ke-ninety-six`, two
units of different length, not one with a flag. A 時辰 is 8⅓ of the first
and exactly 8 of the second. The jiffy is three units, `jiffy-mains-60`,
`jiffy-mains-50` and `jiffy-light-centimetre`.
`mean-gregorian-month` is named for the average it is, because a calendar
month is not a length of time: adding one to a date is `hc-calendar`'s
question.

**Frame and sample rates.** A `Rate` is events per second as a `Ratio`; its
`period` is the reciprocal, and `one_in(unit)` is one event counted in a
unit. `FRAME_RATES` holds 23.976, 24, 25, 29.97, 30, 48, 50, 59.94, 60, 90,
100 and 120 fps, nine whole rates and the three NTSC ones. `SAMPLE_RATES`
holds 8, 16, 22.05, 24, 32, 44.1, 48, 88.2, 96 and 192 kHz.
`Rate::ntsc_pulldown(id)` multiplies a rate by 1000/1001. The NTSC rates
are 24 000/1001, 30 000/1001 and 60 000/1001. The 29.97 is what the line
rate of 4.5 MHz over 286 gives for a frame of 525 lines:
4 500 000 / (286 × 525) reduces to 30 000/1001 [wikipedia-colorburst], and
the other two are 24 and 60 times the same factor [wikipedia-telecine;
wikipedia-frame-rate]. The flick is built so that every one of
these periods is a whole number of flicks: a frame at *r* fps is
705 600 000/*r* flicks, and the 1000/1001 rate is 1001 times
705 600/*r*, a whole number for each of 24, 25, 30, 48, 50, 60, 90, 100 and
120 [facebook-flicks].

**Tempo.** A `Tempo` is a `Ratio` of beats per minute that is positive.
`beat()` is 60 ÷ BPM seconds. `to_midi_micros_per_beat()` is
60 000 000 ÷ BPM, truncated to an integer in 1 to 16 777 215 (24 bits)
together with a flag that says whether the truncation lost anything; the
tempo below about 3.6 BPM does not fit and returns `Overflow`.
`from_midi_micros_per_beat` is the exact inverse. A `NoteValue` is
`halvings` of a whole note, `dots` and an optional tuplet `(space, count)`;
its fraction of a whole note is 1/2^`halvings` × (2^(*n*+1) − 1)/2^*n* for
*n* dots × `space`/`count`. `duration_at(tempo, beat_unit)` is the note's
fraction over the beat unit's, times the beat. A `TimeSignature` takes a
count and a power of two below the line; its `bar(tempo)` is the count of
beats, counting the tempo's beat as the signature's lower note. `Ppqn`
(96, 480 and 960 as named constants, any `u32` otherwise) divides a beat
into ticks.

**Worked example.** One flick is 1/705 600 000 s, and 705 600 000 =
2⁹·3²·5⁵·7².

1. *A count of flicks to a `Ratio`.* A 25 fps frame is 1/25 s, which is
   705 600 000 ÷ 25 = 28 224 000 flicks. Going back,
   `Quantity::whole(28_224_000, FLICK).seconds()` is
   28 224 000/705 600 000, and 705 600 000 ÷ 28 224 000 = 25, so the
   `Ratio` is 1/25 s.
2. *The `Ratio` to a `Duration`.* The denominator 25 = 5² divides
   10¹⁸ = 2¹⁸·5¹⁸, so `to_duration` is exact: 10¹⁸ ÷ 25 =
   40 000 000 000 000 000 attoseconds, 0.04 s.
3. *An NTSC frame.* `NTSC_30.ntsc_pulldown("29.97")` is 30 × 1000/1001 =
   30 000/1001 Hz. (30 000 and 1001 = 7·11·13 share no factor.) Its
   period is 1001/30 000 s, which in flicks is 1001/30 000 ×
   705 600 000 = 1001 × 23 520 = 23 543 520, a whole number as the flick
   promises.
4. *The refusal.* The denominator 30 000 = 2⁴·3·5⁴ has a factor 3, so it
   divides no power of 10, and `to_duration` returns
   `Err(UnitError::Inexact)`. The `Duration` would need
   10¹⁸ × 1001/30 000 = 33 366 666 666 666 666.67 attoseconds;
   `to_duration_rounded` returns 33 366 666 666 666 667.
5. *The refusal belongs to the denominator, not to the size.* Thirty
   thousand of these frames are 30 000 × 1001/30 000 = 1001 s, with
   denominator 1, and convert exactly to a `Duration` of 1001 s. A single
   flick is refused for the factors 3² and 7², and its rounded value is
   1 417 233 560 attoseconds (10¹⁸ ÷ 705 600 000 =
   1 417 233 560.09…). A 48 kHz sample is 14 700 flicks, 1/48 000 s, and is
   refused for the factor 3. A `Ratio` of 1/2¹⁸ s converts exactly, to
   3 814 697 265 625 attoseconds, and 1/2⁶⁰ s does not.

The tempo arithmetic is the same. 138 BPM is a beat of 60/138 = 10/23 s. A
dotted quarter triplet is 3/8 × 2/3 = 1/4 of a whole note, which is one
beat, so its length is 10/23 s with no decimal written down. 140 BPM is a
beat of 3/7 s, and MIDI would store 60 000 000/140 = 428 571.43 µs;
`to_midi_micros_per_beat` returns `(428 571, false)`, where 120 BPM returns
`(500 000, true)`. A bar of 6/8 at 120 BPM with the dotted quarter as the
beat is 6 eighths, each 1/8 ÷ 3/8 = 1/3 beat = 1/6 s: 1 s. `bar()` counts
the tempo's beat as the lower note, so it returns 3 s for the same
signature and tempo; the caller who means a dotted beat uses
`duration_at`.

## What is carried

- **53 units**, by `unit::ALL` shortest first, found by `unit::by_id`:
  14 SI, 13 civil, 10 horological, 4 decimal, 4 hexadecimal, 5 media
  (`flick`, `tick-filetime`, `microfortnight` and the two mains jiffies)
  and 3 scientific (`svedberg`, `jiffy-light-centimetre`, `shake`). The
  tables above give each length. The `authority` string of each names who
  defines it, and `unit_catalogue`'s tests check that none is empty.
- **22 rates**, 12 frame rates and 10 sample rates, as `Rate`.
- **Tempo, note values, time signatures and MIDI ticks**, as above.
- **Use.** `hc-humanize`'s `unit` module takes its mean year, quarter and
  month, week, day, hour, minute and second from here. Two other crates
  hold a figure of their own for a unit that `hc-units` also carries, and
  agree with it: `hc-core::internet_time` (a beat is 86 400 ms) and
  `hc-calendars-solar`'s `DECIMAL_SECOND_ATTOS` (864 000 000 000 000 000
  attoseconds, which is the exact `Duration` of `decimal-second`).
  `hc-core` is below `hc-units`, so it cannot call it.
- **Not exported.** The WebAssembly and C READMEs say that no line format
  has been designed for `hc-units`.

Not carried:

- **Eleven SI prefixes**: deci, centi, deca, hecto, tera, peta, exa, zetta,
  yotta, ronna and quetta (the list was read [bipm-si-prefixes]; not yet
  added).
- **A 刻 of 108 or 120 to the day** (a source names them for short
  periods; the periods and their authorities are not read
  [wikipedia-traditional-chinese-timekeeping; wikipedia-shichen]).
- **Other jiffies**: the Linux jiffy of 1 to 10 ms, which depends on the
  kernel's configuration; 1/100 s in animation; 1/65 536 s in Stratus VOS;
  the light-foot [wikipedia-jiffy-time]. Not yet added; no authority for
  the others read.
- **Other lengths of the muhūrta.** A 40-minute muhūrta in Jain usage is
  mentioned on Wikipedia's page [wikipedia-muhurta]; not checked against a Jain
  source.
- **The sub-prāṇa units** (*truṭi*, *tatpara*, *nimeṣa*, *kāṣṭhā*, *kalā*),
  which the page read puts at about 29.6 µs for the truṭi and 48 s for the
  kalā [wikipedia-hindu-units-of-time]. Not yet added; no verse for them was
  read.
- **Unequal hours.** A temporal hour depends on sunrise and sunset, so it
  is not a unit here. Policy §5 names the zmanim functions as the place for
  those.
- **A frame rate of 119.88** (120 × 1000/1001), which Facebook's README
  lists [facebook-flicks]; `ntsc_pulldown` computes it but no constant
  names it. Nor do the 1000/1001 rates of 25, 48, 50, 90 and 100.
- **The 2:3 pulldown cadence, SMPTE timecode and drop-frame counting**
  [wikipedia-telecine; wikipedia-smpte-timecode]: they are labels and
  patterns over frames, and nothing in this crate counts frames. Not yet
  done.
- **The SMPTE form of a MIDI division** (frames a second times ticks a
  frame) and the time signature's metronome fields `cc` and `bb`
  [midimusic-smf-spec]. Not yet done.
- **Epochs.** `tick-filetime` is the length 100 ns; that FILETIME counts
  from 1601 and `DateTime.Ticks` from year 1 is not part of the unit
  [ms-filetime; ms-datetime-ticks].
- **Any measured period**, by the line the README draws: the tropical
  year, the sidereal day, the synodic month, the galactic year. They live
  in `hc-astro`, `hc-planetary` and `hc-deep-time` with their uncertainties.

## Accuracy

The lengths are exact by definition; nothing here carries an error term.
Exactness holds for the length as an authority states it and not for the
interval a clock measures. A `day` is 86 400 s; a UTC day of a leap second
is 86 401 s [wikipedia-day]. Each check below was made on 2026-10-03.

**What the crate's tests pin.** `cargo test -p hc-units` passes 26 tests:
19 in `lib.rs` and 7 generated by the catalogue macro (every identifier is
findable, in any case and padded; none is empty or duplicated; the table is
sorted by length; every entry names a source). The 19 pin the helek and its
1080 to the hour, the two 刻 as 100 and 96 to the day, the beat equal to the
decimal minute and 1000 to the day, the Julian year as 1461/4 days, the
flick's refusal and its rounding, a flick dividing every rate in `media`,
the pulldown giving 30 000/1001, a 29.97 frame not being whole
milliseconds, the 120 and 138 BPM figures, the MIDI default and 140 BPM,
dots and tuplets, `4/4` at 90 BPM, the refusal of a lower number of 5, the
tick of 1/960 s, the `Ord` fallback and the `Duration` round trip. The
lengths of the other units, the 13 SI prefixes among them, are not pinned
by a test in the crate. I derived each from the sources above by quotient
(`day.per(ghati)` 60, `ghati.per(vighati)` 60, `vighati.per(prana)` 6,
`day.per(muhurta)` 30, `hour.per(moment)` 40, `day.per(hex_second)` 65 536,
`millisecond.per(tick_filetime)` 10 000, `fortnight.per(microfortnight)`
1 000 000 and so on) and ran each in a scratch crate; all agree, as does the
generated table in `docs/supported.md`. `hc-humanize`'s tests pin the mean
year at 31 556 952 s and the month at 2 629 746 s.

**Against Facebook's README.** Every one of the 12 frame rates and 10 sample
rates is a whole number of flicks (24 fps 29 400 000, 25 fps 28 224 000,
23.976 fps 29 429 400, 29.97 fps 23 543 520, 59.94 fps 11 771 760, 60 fps
11 760 000, 48 kHz 14 700, 192 kHz 3 675), including the four the README
quotes. Its README lists the 1000/1001 variants of 24, 30, 60 and 120
[facebook-flicks]; the crate's comment says "any of those frame rates",
which is wider. It holds: `ntsc_pulldown` of each of 24, 25, 30, 48, 50,
60, 90, 100 and 120 is a whole number of flicks (29 429 400, 28 252 224,
23 543 520, 14 714 700, 14 126 112, 11 771 760, 7 847 840, 7 063 056 and
5 885 880).

**Disagreements between the code's own words and the sources.**

- **`ke-ninety-six`.** The crate dates the change to the 時憲曆 reform of
  1645 (README, `unit` module documentation, `authority`); the page read
  says 1⁄96 "since 1628" [wikipedia-traditional-chinese-timekeeping].
  The Shixian calendar page gives 1645 as its first year and does not
  mention the 刻 [wikipedia-shixian-calendar]. Which year is right was not
  settled from a primary source. The `authority` string also says the 96 was
  "standard in China, Japan and Korea thereafter"; the page read names no
  Japanese or Korean usage, and the Japanese clock page says the traditional
  day and night were each divided into six periods of seasonal length
  [wikipedia-japanese-clock].
  That clause has no source read.
- **The *Sūrya Siddhānta*'s day.** The text's sixty ghaṭikās make a
  *sidereal* day and night [sastri1861]. The crate takes the ghati as a
  sixtieth of the nominal 86 400 s day. A sidereal day is about 86 164 s,
  so a prāṇa by the text would be about 3.99 s and not 4 s. The 4 s is the
  translator's gloss. This is a convention the crate adopts and not the
  text's length, and the sidereal day is a measured quantity that belongs
  to `hc-astro`.
- **`moment`.** A moment is a fortieth of a *solar* hour, which varies with
  the season, and 90 s is its average [wikipedia-moment-time]. The crate
  carries the average as if it were a definition.
- **`microfortnight`.** The crate's documentation says VMS measures
  "password retries" in it. The page read says VMS's TIMEPROMPTWAIT, the
  wait at boot for an operator to set the date and time, is in
  microfortnights, and does not mention passwords [wikipedia-fff-system].
- **`mean-gregorian-*`.** The `authority` is "Gregorian calendar; CLDR
  relative-time means". The year of 365.2425 days, 31 556 952 s, follows
  from the Gregorian rule [wikipedia-gregorian-calendar]. CLDR 48's
  `units.xml` defines the month as 1/12 and the quarter as 1/4 of the year
  and gives the year no length in seconds; its only year in seconds is the
  Julian year, 31 557 600 [cldr48-units-supplemental]. The figures are
  right and the second half of the attribution is not.
- **`jiffy-light-centimetre`.** The length is exact, 1 cm over the defined
  c = 299 792 458 m/s [bipm-si-defining-constants], 1/29 979 245 800 s or
  33.356 409… ps; the page read gives "approximately 33.3564 picoseconds"
  [wikipedia-jiffy-time]. The `authority` says "BIPM, 1983"; the BIPM page
  read does not say when c was fixed, and Lewis's own paper was not read.
- **Frame rates in `media`.** The comment on `FILM_24` says "the
  sound-film rate since 1929"; the page read says sound film came in 1926
  and 24 fps was chosen as a compromise, under a section dated 1926 to 1930
  [wikipedia-frame-rate]. The `media` module says the colour standard
  reduced the field rate so that the subcarrier "avoid beating against the
  audio carrier". The sources read agree in outline: the line rate was set
  to 1/286 of the 4.5 MHz sound subcarrier with the colour subcarrier at
  35/44 of it [wikipedia-colorburst], and the page on frame rates gives the
  aim as less "dot crawl" [wikipedia-frame-rate]. The crate's name
  `ntsc_pulldown` is the 1000/1001 slowdown alone; Wikipedia keeps it
  apart from the cadence [wikipedia-telecine].
- **`Ppqn::CLASSIC`.** The comment says 96 PPQN is "the original MIDI File
  Format 0 default". The specification text read has 96 only in an example
  and names no default division; 480 and 960 are not in it
  [midimusic-smf-spec]. The three constants are conventions the crate
  names, not defaults the specification states.
- **`helek`.** Its `authority` begins "Mishnah". Maimonides gives the hour
  of 1080 parts [maimonides-kiddush-hachodesh] and the moment as a
  seventy-sixth [sefaria-maimonides-kh-10]; the Mishnah was not read.
- **`muhurta`.** Its `authority` names the Vedāṅga Jyotiṣa and the
  *Sūrya Siddhānta*. The Wikipedia page on the muhūrta names neither
  [wikipedia-muhurta]; the other page read gives the 48-minute muhūrta
  under the *Sūrya Siddhānta* with no verse [wikipedia-hindu-units-of-time].
- **README.** It says SI is "quectosecond through gigasecond"; 13 of 24
  prefixes are in the table (see "Not carried").

**Disagreements between the code's documentation and the code.** These were
found by running the code and are not fixed here.

- `Ratio::to_duration_rounded` is documented as rounding "half away from
  zero". It rounds the fractional part half up, toward positive infinity:
  +0.5 as gives 1 as, −0.5 as gives 0 and −1.5 as gives −1 as, where half
  away from zero gives −1 and −2. Away from ties it is the nearest value:
  −1/3 s gives −333 333 333 333 333 333 as.
- `Ratio::from_duration` says the limit of about 1.7×10²⁰ s is "five
  thousand times the age of the universe". The limit is `i128::MAX` ÷ 10¹⁸ =
  170 141 183 460 469 231 731 s, and the age is about 4.35×10¹⁷ s, so the
  factor is about 390.
- `Ord for Ratio` says the cross product of a quectosecond and a Julian
  millennium is 3×10⁴¹. It is 10³⁰ × 31 557 600 000 = 3.16×10⁴⁰, as the
  test's comment says. It overflows `i128` all the same (1.7×10³⁸).
- `to_duration` documents `Overflow` only for a value too large for a
  `Duration`. A `Ratio` of denominator 10³⁰ and a large remainder, such as
  500 000 000 000 000 000 000 000 000 001/10³⁰ (about 0.5 s), returns
  `Overflow` from both `to_duration` and `to_duration_rounded`, because
  remainder × 10¹⁸ leaves `i128`, although the value is small. No unit in
  the table reaches this.
- `NoteValue::fraction_of_whole` says `DivideByZero` for "a tuplet with a
  zero on either side". A tuplet `(0, 3)` returns a fraction of 0 and no
  error; `(2, 0)` returns `DivideByZero`. Halvings to 126 and dots to 125
  are accepted, where it says about 120 halvings.
- `TimeSignature::new` and `Tempo::from_bpm_ratio` return
  `UnitError::DivideByZero` for a lower number that is not a power of two and
  for a negative tempo. The error is not a division by zero.
- `Quantity::to`'s documentation names a `FRAME_AT_24` that does not exist.
- `Tempo::to_midi_micros_per_beat` truncates: 138 BPM is 434 782.6 µs and
  returns 434 782, where rounding gives 434 783. The specification read says
  nothing about how a writer rounds [midimusic-smf-spec].

**How the figures here were made.** By hand from the sources, then by
running `cargo test -p hc-units` and a scratch crate that printed every
number in the worked example and the tables.

## Sources

Read directly as HTML on 2026-10-03, except where noted:

- [bipm-second]: the definition of the second. The BIPM's SI Brochure and
  its table of non-SI units are PDFs and were not read; the minute, hour and
  day of the table are cited to the secondary pages below.
- [bipm-si-prefixes]: the 24 prefixes and the years of adoption.
- [bipm-si-defining-constants]: c = 299 792 458 m/s.
- [facebook-flicks]: the definition and the rates the flick divides.
- [maimonides-kiddush-hachodesh]: chapter 6, halacha 2, the hour of 1080
  parts, read through Sefaria's text interface.
- [sefaria-maimonides-kh-10]: chapter 10, halacha 1, the rega.
- [sastri1861]: page 12 of the Wikisource transcription, verses 11 and 12,
  the prāṇa, pala, ghaṭikā and the sidereal day and night.
- [decret-4-frimaire-an-ii]: article XI and the date.
- [swatch-internet-time]: the 1000 .beats and 1 minute 26.4 seconds.
- [ms-filetime], [ms-datetime-ticks]: the 100-nanosecond interval and the
  two epochs.
- [midimusic-smf-spec]: Set Tempo, the division and the time signature
  event. This is an HTML transcription by David Back, not the Association's
  page. The Association's specification, RP-001, is a PDF and was not read.
- [midi-org-tempo-forum], [midi-org-delta-time-forum]: threads on the MIDI
  Association's site, used for the 500 000 µs figure and the tick formula.
- [cldr48-units-supplemental]: the CLDR year, month and quarter, read from
  the raw file of tag `release-48`.
- Secondary, Wikipedia: [wikipedia-helek], [wikipedia-moment-time],
  [wikipedia-muhurta], [wikipedia-hindu-units-of-time],
  [wikipedia-traditional-chinese-timekeeping], [wikipedia-shichen],
  [wikipedia-shixian-calendar], [wikipedia-japanese-clock],
  [wikipedia-decimal-time], [wikipedia-hexadecimal-time],
  [wikipedia-jiffy-time], [wikipedia-shake-unit], [wikipedia-svedberg],
  [wikipedia-fff-system], [wikipedia-minute], [wikipedia-day],
  [wikipedia-julian-year-astronomy], [wikipedia-gregorian-calendar],
  [wikipedia-ntsc], [wikipedia-colorburst], [wikipedia-frame-rate],
  [wikipedia-telecine], [wikipedia-smpte-timecode],
  [wikipedia-sampling-signal-processing], [wikipedia-dotted-note],
  [wikipedia-tuplet], [wikipedia-time-signature].

Cited by the code and **not read**: the SI Brochure, 9th edition (the
minute, hour and day); ISO 8601-1:2019 (the week); the IAU 1976 System of
Astronomical Constants (the Julian year, century and millennium); the
Mishnah and Maimonides's other chapters beyond 6 and 10; the *Sūrya
Siddhānta* beyond verses 11 and 12 of one translation, and the Vedāṅga
Jyotiṣa; Bartholomeus Anglicus; John W. Nystrom's *Project of a New System
of Arithmetic* (1862); the DEC VMS documentation; the IUPAC Gold Book on
the svedberg; Gilbert Lewis on the jiffy; the Manhattan Project usage of the
shake; the FCC's 1953 colour standard and SMPTE's documents; the *Oxford
English Dictionary* entry for the fortnight. The 8 kHz to 192 kHz roles in
the `media` comments were checked against one table
[wikipedia-sampling-signal-processing], which gave no entry for 24 kHz
or 32 kHz.

## Code

`crates/hc-units/src/ratio.rs` (`Ratio`, `to_duration`,
`to_duration_rounded`), `unit.rs` (`Unit`, `Quantity`, `Family`, the table
and `by_id`), `media.rs` (`Rate`, `FRAME_RATES`, `SAMPLE_RATES`), `tempo.rs`
(`Tempo`, `NoteValue`, `TimeSignature`, `Ppqn`) and `error.rs`
(`UnitError`). The tests that anchor it are in `lib.rs`:
`a_flick_divides_every_frame_and_sample_rate_exactly`,
`the_ntsc_pulldown_stays_exact`, `a_flick_has_no_exact_duration_and_says_so`,
`a_helek_is_three_and_a_third_seconds`,
`the_two_ke_are_separate_units_of_different_length`,
`a_swatch_beat_is_a_french_decimal_minute`,
`a_julian_year_is_three_hundred_sixty_five_and_a_quarter_days`,
`one_hundred_twenty_bpm_is_half_a_second_a_beat`,
`the_midi_default_tempo_round_trips`,
`a_tempo_midi_cannot_store_exactly_admits_it`,
`dots_and_tuplets_are_exact`,
`a_dotted_quarter_triplet_at_one_hundred_thirty_eight_bpm` and
`a_ratio_comparison_that_overflows_cross_multiplication_still_orders`, with
the seven generated `unit::unit_catalogue` tests. The facade is checked by
`the_units_feature_reaches_its_crate` in
`crates/hyper-calendar/tests/facade.rs`, and the generated table of
`docs/supported.md` and the README's count of 53 by
`the_readme_coverage_rows_match_the_index` in
`crates/hyper-calendar/tests/supported.rs`. `hc-humanize`'s
`crates/hc-humanize/src/unit.rs` is the one caller.
