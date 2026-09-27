//! The Ethiopian and Swahili hours: the local clock read six hours on, on a
//! twelve-hour dial counted from about sunrise and about sunset.
//!
//! Both reckonings count twelve hours of the day from 06:00 local time and
//! twelve of the night from 18:00, so the dial reads the civil hour less
//! six: 07:00 is hour 1, noon is hour 6 of the day, 18:00 is 12, and
//! midnight hour 6 of the night. Minutes and seconds are the civil clock's.
//! The civil clock is the caller's: East Africa Time, UTC+3, in Ethiopia
//! and on the Swahili coast, but this module reads whatever wall clock it
//! is given.
//!
//! The two reckonings differ in where the halves meet and in what they
//! call the parts of the day, and so are two conventions under policy §5:
//!
//! - **`ethiopian-hours`**: "The daytime cycle begins at dawn 12:00 (6:00:00
//!   AM EAT) and ends at dusk 11:59:59 (5:59:59 PM EAT). The nighttime cycle
//!   begins at dusk 12:00 (6:00:00 PM EAT)" (Wikipedia, "Time in Ethiopia",
//!   read 2026-09-27, `wikipedia-time-in-ethiopia`); "The Ethiopian day
//!   begins at sunrise or 0600 hours … 8 am is 2 o'clock … 6 pm, which is
//!   12 o'clock and then the counting begins again, 7 pm is 1 o'clock" (UN
//!   Women's Association, *Welcome to Addis Ababa 1994–1995*, as the UNDP
//!   Emergencies Unit for Ethiopia published it, read in the Internet
//!   Archive's copy of 22 July 2011, `undp-eue-ethiopian-time`). The day
//!   half runs from 06:00 to 17:59:59.
//! - **`swahili-hours`**: "7:00 am is referred to as saa moja asubuhi to
//!   mean that it is the first hour of the day. 7:00 pm is called saa moja
//!   usiku to indicate that it is the first hour of the night" (University
//!   of Kansas Kiswahili programme, Lesson 17, "Time", read 2026-09-27,
//!   `ku-kiswahili-lesson-17`). The lesson names the hours 7 am to 6 pm as
//!   the day's twelve and 7 pm to 6 am as the night's, so here the day half
//!   runs from 07:00 to 18:59:59, each reading taking the half of its whole
//!   civil hour; that boundary is read from the lesson's hour table and not
//!   stated in it. The same table names the part of the day for each civil
//!   hour — *usiku* 7 pm to 3 am, *alfajiri* 4 to 6 am, *asubuhi* 7 to
//!   11 am, *mchana* noon to 3 pm, *jioni* 4 to 6 pm — which
//!   [`Reckoning::period`] returns.
//!
//! The Amharic names of the parts of the day are not carried: the one
//! teaching source read, the University of Wisconsin–Madison students'
//! *Resources for Self-Instructional Learners*, puts noon in "Tewat", the
//! morning, where the pages a web search summarised put it in the day,
//! "Ken"; those pages were not read, and no authority was found to settle
//! it. A reckoning from the day's actual
//! sunrise, which "dawn" in some descriptions suggests, would be a third
//! convention; no source read defines one.
//!
//! `docs/systems/hours-of-the-day.md` works an example through.

use hc_calendar::CivilTime;

use crate::error::ValueResult;

/// The half of the twenty-four hours a reading is counted in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Half {
    /// The twelve hours from about sunrise.
    Day,
    /// The twelve hours from about sunset.
    Night,
}

/// A reading on the twelve-hour dial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HourReading {
    /// The hour on the dial, 1 to 12.
    pub hour: u8,
    /// The minute, as on the civil clock.
    pub minute: u8,
    /// The second, as on the civil clock; 60 for a leap second.
    pub second: u8,
    /// The fraction of the second, in attoseconds.
    pub subsec_attos: u64,
    /// The half the hour is counted in.
    pub half: Half,
}

/// A part of the day, as a reckoning's source names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Period {
    /// The word, in the reckoning's language.
    pub name: &'static str,
    /// The source's English gloss.
    pub english: &'static str,
}

/// A six-hour reckoning of the civil clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reckoning {
    /// Identifier, for example `"ethiopian-hours"`.
    pub id: &'static str,
    /// English name.
    pub name: &'static str,
    /// The civil hour at which the day half begins: 6 or 7.
    pub day_begins: u8,
    /// The part of the day for each civil hour from 0 to 23, where the
    /// source names one.
    pub periods: Option<&'static [Period; 24]>,
    /// The documents the reckoning is taken from.
    pub source: &'static str,
}

const USIKU: Period = Period {
    name: "usiku",
    english: "night",
};
const ALFAJIRI: Period = Period {
    name: "alfajiri",
    english: "dawn",
};
const ASUBUHI: Period = Period {
    name: "asubuhi",
    english: "morning",
};
const MCHANA: Period = Period {
    name: "mchana",
    english: "afternoon",
};
const JIONI: Period = Period {
    name: "jioni",
    english: "evening",
};

/// The University of Kansas lesson's parts of the day, by civil hour: "12 am
/// saa sita usiku" to "3 am saa tisa usiku", "4 am saa kumi alfajiri" to "6
/// am saa kumi na mbili alfajiri", "7 am saa moja asubuhi" to "11 am", "12
/// pm saa sita mchana" to "3 pm saa tisa mchana", "4 pm saa kumi jioni" to
/// "6 pm", and "7 pm saa moja usiku" to "11 pm".
const SWAHILI_PERIODS: [Period; 24] = [
    USIKU, USIKU, USIKU, USIKU, ALFAJIRI, ALFAJIRI, ALFAJIRI, ASUBUHI, ASUBUHI, ASUBUHI, ASUBUHI,
    ASUBUHI, MCHANA, MCHANA, MCHANA, MCHANA, JIONI, JIONI, JIONI, USIKU, USIKU, USIKU, USIKU,
    USIKU,
];

hc_core::catalogue! {
    type: Reckoning,
    id: |reckoning| reckoning.id,
    provenance: |reckoning| reckoning.source,
    tests: reckoning_catalogue_tests,

    /// Every reckoning.
    pub const ALL;
    /// The reckoning with this identifier.
    pub fn by_id;

    entries: {
        /// The Ethiopian hours: the day half from 06:00 to 17:59:59.
        pub const ETHIOPIAN = Reckoning {
            id: "ethiopian-hours",
            name: "Ethiopian hours",
            day_begins: 6,
            periods: None,
            source: "Wikipedia, Time in Ethiopia: the daytime cycle begins at 12:00 \
                (6:00:00 AM EAT) and ends at 11:59:59 (5:59:59 PM EAT), read 2026-09-27 \
                [wikipedia-time-in-ethiopia]; UN Women's Association, Welcome to Addis \
                Ababa 1994-1995, as the UNDP Emergencies Unit for Ethiopia published it: \
                the day begins at 0600, 8 am is 2 o'clock, 7 pm is 1 o'clock, Internet \
                Archive copy of 2011-07-22 [undp-eue-ethiopian-time]",
        };
        /// The Swahili hours: the day half from 07:00 to 18:59:59, with the
        /// parts of the day of the University of Kansas lesson.
        pub const SWAHILI = Reckoning {
            id: "swahili-hours",
            name: "Swahili hours",
            day_begins: 7,
            periods: Some(&SWAHILI_PERIODS),
            source: "University of Kansas Kiswahili programme, Lesson 17, Time: 7:00 am \
                is saa moja asubuhi, the first hour of the day, and 7:00 pm saa moja \
                usiku, the first hour of the night, with the part of the day for each \
                hour, read 2026-09-27 [ku-kiswahili-lesson-17]",
        };
    }
}

impl Reckoning {
    /// The civil hours at which the day and the night halves begin.
    const fn half_start(self, half: Half) -> u8 {
        match half {
            Half::Day => self.day_begins,
            Half::Night => (self.day_begins + 12) % 24,
        }
    }

    /// The reading of a civil time of day.
    #[must_use]
    pub const fn reading(self, local: CivilTime) -> HourReading {
        let civil = local.hour();
        let hour = match (civil + 6) % 12 {
            0 => 12,
            hour => hour,
        };
        let from_day = (civil + 24 - self.day_begins) % 24;
        HourReading {
            hour,
            minute: local.minute(),
            second: local.second(),
            subsec_attos: local.subsec_attos(),
            half: if from_day < 12 {
                Half::Day
            } else {
                Half::Night
            },
        }
    }

    /// The civil time of day of a reading.
    ///
    /// # Errors
    ///
    /// [`crate::ValueError::Calendar`] with
    /// [`hc_calendar::CalendarError::DayOutOfRange`] for an hour outside 1
    /// to 12, a minute or second out of range, or a second 60 anywhere but
    /// civil 23:59.
    pub fn civil(self, reading: HourReading) -> ValueResult<CivilTime> {
        if !(1..=12).contains(&reading.hour) {
            return Err(hc_calendar::CalendarError::DayOutOfRange.into());
        }
        let start = self.half_start(reading.half);
        // The one hour of the half's twelve whose dial reading this is.
        let offset = (reading.hour + 6 + 24 - start) % 12;
        let civil = (start + offset) % 24;
        Ok(CivilTime::new(
            civil,
            reading.minute,
            reading.second,
            reading.subsec_attos,
        )?)
    }

    /// The part of the day the source names for a civil time, if it names
    /// one.
    #[must_use]
    pub fn period(self, local: CivilTime) -> Option<Period> {
        self.periods
            .map(|periods| periods[usize::from(local.hour())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(hour: u8, minute: u8) -> CivilTime {
        CivilTime::hms(hour, minute, 0).expect("valid")
    }

    fn dial(hour: u8, minute: u8, half: Half) -> HourReading {
        HourReading {
            hour,
            minute,
            second: 0,
            subsec_attos: 0,
            half,
        }
    }

    /// The UNDP page's examples: 8 am is 2 o'clock, 10 am is 4 o'clock,
    /// 6 pm is 12 o'clock, 7 pm is 1 o'clock and 10 pm 4 o'clock; and
    /// Wikipedia's halves, 06:00 the day's 12 and 18:00 the night's.
    #[test]
    fn the_ethiopian_examples() {
        for (hour, reading) in [
            (8, dial(2, 0, Half::Day)),
            (10, dial(4, 0, Half::Day)),
            (18, dial(12, 0, Half::Night)),
            (19, dial(1, 0, Half::Night)),
            (22, dial(4, 0, Half::Night)),
            (6, dial(12, 0, Half::Day)),
            (7, dial(1, 0, Half::Day)),
            (17, dial(11, 0, Half::Day)),
            (5, dial(11, 0, Half::Night)),
        ] {
            assert_eq!(ETHIOPIAN.reading(at(hour, 0)), reading, "{hour}:00");
            assert_eq!(ETHIOPIAN.civil(reading), Ok(at(hour, 0)));
        }
        // Noon is 6 o'clock of the day and midnight 6 o'clock of the night.
        assert_eq!(ETHIOPIAN.reading(CivilTime::NOON), dial(6, 0, Half::Day));
        assert_eq!(
            ETHIOPIAN.reading(CivilTime::MIDNIGHT),
            dial(6, 0, Half::Night)
        );
        assert_eq!(ETHIOPIAN.period(CivilTime::NOON), None);
    }

    /// The Kansas lesson: "7 am saa moja asubuhi", "12 pm saa sita
    /// mchana", "4 pm saa kumi jioni", "6 pm saa kumi na mbili jioni",
    /// "7 pm saa moja usiku", "12 am saa sita usiku", "6 am saa kumi na
    /// mbili alfajiri"; and "saa nne na nusu asubuhi [10:30 am]".
    #[test]
    fn the_swahili_examples() {
        for (hour, minute, reading, period) in [
            (7, 0, dial(1, 0, Half::Day), "asubuhi"),
            (12, 0, dial(6, 0, Half::Day), "mchana"),
            (16, 0, dial(10, 0, Half::Day), "jioni"),
            (18, 0, dial(12, 0, Half::Day), "jioni"),
            (19, 0, dial(1, 0, Half::Night), "usiku"),
            (0, 0, dial(6, 0, Half::Night), "usiku"),
            (6, 0, dial(12, 0, Half::Night), "alfajiri"),
            (10, 30, dial(4, 30, Half::Day), "asubuhi"),
        ] {
            let local = at(hour, minute);
            assert_eq!(SWAHILI.reading(local), reading, "{hour}:{minute:02}");
            assert_eq!(SWAHILI.civil(reading), Ok(local));
            assert_eq!(
                SWAHILI.period(local).map(|period| period.name),
                Some(period)
            );
        }
    }

    /// The two differ only in the hours 06:00–06:59 and 18:00–18:59.
    #[test]
    fn the_reckonings_differ_only_at_the_twelfth_hours() {
        for hour in 0..24 {
            let local = at(hour, 30);
            let (ethiopian, swahili) = (ETHIOPIAN.reading(local), SWAHILI.reading(local));
            assert_eq!(ethiopian.hour, swahili.hour);
            assert_eq!(
                ethiopian.half == swahili.half,
                hour != 6 && hour != 18,
                "{hour}"
            );
        }
    }

    /// Every second of the day, and the leap second, round-trips in both.
    #[test]
    fn every_minute_round_trips() {
        for reckoning in ALL {
            for hour in 0..24 {
                for minute in 0..60 {
                    let local = CivilTime::new(hour, minute, 59, 5).expect("valid");
                    let reading = reckoning.reading(local);
                    assert!((1..=12).contains(&reading.hour));
                    assert_eq!(reckoning.civil(reading), Ok(local));
                }
            }
            let leap = CivilTime::hms(23, 59, 60).expect("valid");
            let reading = reckoning.reading(leap);
            assert_eq!(
                (reading.hour, reading.second, reading.half),
                (5, 60, Half::Night)
            );
            assert_eq!(reckoning.civil(reading), Ok(leap));
            assert!(reckoning.civil(dial(13, 0, Half::Day)).is_err());
            assert!(reckoning.civil(dial(0, 0, Half::Day)).is_err());
            assert!(
                reckoning
                    .civil(HourReading {
                        second: 60,
                        ..dial(1, 59, Half::Day)
                    })
                    .is_err()
            );
        }
    }
}
