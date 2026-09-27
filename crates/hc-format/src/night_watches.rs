//! 更点 — the Chinese night in five watches of five points each.
//!
//! The night from 19:00 to 05:00 is five 更 of two hours, one to each
//! double hour from 戌 to 寅: 一更 (初更), 19:00–21:00, called 黃昏 under
//! the Han; 二更, 人定; 三更, 夜半, 23:00–01:00; 四更, 雞鳴; and 五更, 平旦,
//! 03:00–05:00 (Wikipedia (zh), 「更」, `wikipedia-zh-geng`, retrieved
//! 2026-09-28). Each 更 is five 點 of 24 minutes, and "三更两点" is 23:48
//! (Wikipedia (zh), 「點 (時間)」, `wikipedia-zh-dian`, retrieved
//! 2026-09-28): the points are counted as struck, so that a watch begins
//! at its 0th point and its 2nd is struck 48 minutes in. The watchman
//! struck four points in a watch and called the next watch on the fifth,
//! which is why the count runs from 0 to 4.
//!
//! This is the fixed reckoning, read from the civil clock. The watches were
//! also kept as fifths of the night from dusk to dawn, which changes with
//! the season; the sources read give no instant for that dusk and dawn, so
//! it is not carried. `docs/systems/hours-of-the-day.md` in the repository
//! places this beside the other reckonings of the hours.

use hc_calendar::CivilTime;

/// A reading of the night watches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NightWatch {
    /// The watch, 1 (一更) to 5 (五更).
    pub watch: u8,
    /// The points struck since the watch began, 0 to 4.
    pub points: u8,
}

/// The five watches' numbers, as the source writes them.
pub const WATCH_NAMES: [&str; 5] = ["一更", "二更", "三更", "四更", "五更"];

/// The five watches' Han names.
pub const HAN_NAMES: [&str; 5] = ["黃昏", "人定", "夜半", "雞鳴", "平旦"];

/// The five watches' double hours, 戌 to 寅.
pub const BRANCHES: [&str; 5] = ["戌", "亥", "子", "丑", "寅"];

/// The civil hour the first watch begins, 19:00.
pub const FIRST_WATCH_HOUR: u8 = 19;

/// The minutes in a watch.
pub const WATCH_MINUTES: u32 = 120;

/// The minutes in a point.
pub const POINT_MINUTES: u32 = 24;

impl NightWatch {
    /// The watch's number, 一更 to 五更.
    #[must_use]
    pub const fn name(self) -> &'static str {
        WATCH_NAMES[(self.watch - 1) as usize]
    }

    /// The watch's Han name, 黃昏 to 平旦.
    #[must_use]
    pub const fn han_name(self) -> &'static str {
        HAN_NAMES[(self.watch - 1) as usize]
    }

    /// The watch's double hour, 戌 to 寅.
    #[must_use]
    pub const fn branch(self) -> &'static str {
        BRANCHES[(self.watch - 1) as usize]
    }
}

/// The watch and the points of a civil time of night by the fixed
/// reckoning, or `None` from 05:00 to 18:59, which is not night.
#[must_use]
pub const fn fixed_night_watch(local: CivilTime) -> Option<NightWatch> {
    let minutes = local.hour() as u32 * 60 + local.minute() as u32;
    let since = (minutes + 24 * 60 - FIRST_WATCH_HOUR as u32 * 60) % (24 * 60);
    if since >= 5 * WATCH_MINUTES {
        return None;
    }
    Some(NightWatch {
        watch: (since / WATCH_MINUTES + 1) as u8,
        points: (since % WATCH_MINUTES / POINT_MINUTES) as u8,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(hour: u8, minute: u8) -> Option<NightWatch> {
        fixed_night_watch(CivilTime::new(hour, minute, 0, 0).expect("a time"))
    }

    #[test]
    fn the_third_watch_is_the_double_hour_of_zi() {
        // 「更」: 三更 is 子時, 23:00 to 01:00, called 夜半.
        let third = at(23, 0).unwrap();
        assert_eq!((third.watch, third.points), (3, 0));
        assert_eq!(
            (third.name(), third.han_name(), third.branch()),
            ("三更", "夜半", "子")
        );
        assert_eq!(at(0, 59).unwrap().watch, 3);
        assert_eq!(at(1, 0).unwrap().watch, 4);
    }

    #[test]
    fn the_second_point_of_the_third_watch_is_struck_at_23_48() {
        // 「點 (時間)」: "三更两点就是指子時两點，即夜间11点48分".
        assert_eq!(
            at(23, 48),
            Some(NightWatch {
                watch: 3,
                points: 2
            })
        );
        assert_eq!(
            at(23, 47),
            Some(NightWatch {
                watch: 3,
                points: 1
            })
        );
        assert_eq!(
            at(0, 36),
            Some(NightWatch {
                watch: 3,
                points: 4
            })
        );
    }

    #[test]
    fn the_night_runs_from_seven_in_the_evening_to_five_in_the_morning() {
        assert_eq!(at(18, 59), None);
        assert_eq!(
            at(19, 0),
            Some(NightWatch {
                watch: 1,
                points: 0
            })
        );
        assert_eq!(
            at(4, 59),
            Some(NightWatch {
                watch: 5,
                points: 4
            })
        );
        assert_eq!(at(5, 0), None);
        assert_eq!(at(12, 0), None);
        let names: Vec<&str> = (1..=5)
            .map(|watch| NightWatch { watch, points: 0 }.han_name())
            .collect();
        assert_eq!(names, HAN_NAMES);
        for hour in 0..24 {
            for minute in 0..60 {
                if let Some(watch) = at(hour, minute) {
                    assert!((1..=5).contains(&watch.watch));
                    assert!(watch.points < 5);
                }
            }
        }
    }
}
