//! Rûz-ı Hızır and Rûz-ı Kasım: the Turkish folk year of a summer half and
//! a winter half, and the days counted in the winter half.
//!
//! The folk year of Anatolia and the Balkans has two halves. The *Hızır
//! günleri*, the summer, run from 6 May, Hıdırellez, to 7 November; the
//! *Kasım günleri*, the winter, from 8 November to 5 May (Wikipedia (tr),
//! 「Hıdırellez」, `wikipedia-tr-hidirellez`, retrieved 2026-09-28, which
//! gives Hıdırellez as 23 April of the Rumi, Julian, calendar and 6 May of
//! the Gregorian). The winter half is counted by its days, and the named
//! days fall on its counts: *erbain*, the forty days, enters on Kasım 46;
//! *hamsin*, the fifty, on Kasım 86; and the three *cemre* fall into the
//! air on Kasım 105, 20 February, into the water on Kasım 112, 27 February,
//! and into the earth on Kasım 119, 6 March, or 5 March in a leap year
//! (Bilkent Üniversitesi, 「Cemre」, `bilkent-cemre`, retrieved 2026-09-28,
//! after Mikdat Kadıoğlu). The same page gives the halves as 180 and 186
//! days, which is a leap year's count.
//!
//! Counting 8 November as Kasım 1 reproduces every one of the source's
//! dates, including the move of the third *cemre* to 5 March in a leap year,
//! so that is the count here. The source also writes the first *cemre* as
//! "19-20 Şubat", the night before the day, and gives it as 20 February in
//! the next paragraph; the day is 20 February.
//!
//! The days are the Gregorian ones the sources give. The Rumi dates they
//! stand for, 23 April and 26 October, fall on 6 May and 8 November only
//! from 1900 to 2099; a count kept on the Rumi calendar is not carried,
//! since no source read gives the Rumi day of Kasım. The Alevi Hızır fast
//! is not carried: its conventions disagree.

use hc_calendar::Rd;

use crate::gregorian;

/// The half of the folk year a day is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Half {
    /// *Hızır günleri*, 6 May to 7 November.
    Hizir,
    /// *Kasım günleri*, 8 November to 5 May.
    Kasim,
}

impl Half {
    /// The Turkish name, as the source writes it.
    #[must_use]
    pub const fn turkish_name(self) -> &'static str {
        match self {
            Self::Hizir => "Hızır günleri",
            Self::Kasim => "Kasım günleri",
        }
    }
}

/// A day of the folk year: its half, and its count within the half from 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FolkDay {
    /// The half.
    pub half: Half,
    /// The day of the half, from 1.
    pub day: u16,
}

/// The named days of the folk year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedDay {
    /// Hıdırellez, Rûz-ı Hızır: Hızır 1, 6 May.
    Hidirellez,
    /// The first of the Kasım days: Kasım 1, 8 November.
    Kasim,
    /// *Erbain* enters: Kasım 46.
    Erbain,
    /// *Hamsin* enters: Kasım 86.
    Hamsin,
    /// The first *cemre*, into the air: Kasım 105.
    CemreAir,
    /// The second *cemre*, into the water: Kasım 112.
    CemreWater,
    /// The third *cemre*, into the earth: Kasım 119.
    CemreEarth,
}

impl NamedDay {
    /// Every named day, in the order of the folk year from Hıdırellez.
    pub const ALL: [Self; 7] = [
        Self::Hidirellez,
        Self::Kasim,
        Self::Erbain,
        Self::Hamsin,
        Self::CemreAir,
        Self::CemreWater,
        Self::CemreEarth,
    ];

    /// The day of the folk year this is.
    #[must_use]
    pub const fn folk_day(self) -> FolkDay {
        let (half, day) = match self {
            Self::Hidirellez => (Half::Hizir, 1),
            Self::Kasim => (Half::Kasim, 1),
            Self::Erbain => (Half::Kasim, 46),
            Self::Hamsin => (Half::Kasim, 86),
            Self::CemreAir => (Half::Kasim, 105),
            Self::CemreWater => (Half::Kasim, 112),
            Self::CemreEarth => (Half::Kasim, 119),
        };
        FolkDay { half, day }
    }

    /// The Turkish name, as the sources write it.
    #[must_use]
    pub const fn turkish_name(self) -> &'static str {
        match self {
            Self::Hidirellez => "Hıdırellez",
            Self::Kasim => "Kasım",
            Self::Erbain => "erbain",
            Self::Hamsin => "hamsin",
            Self::CemreAir => "birinci cemre",
            Self::CemreWater => "ikinci cemre",
            Self::CemreEarth => "üçüncü cemre",
        }
    }

    /// The day this falls on in Gregorian `year`: a day of the Kasım half
    /// that falls after the new year is counted from 8 November of the
    /// year before.
    #[must_use]
    pub const fn in_year(self, year: i64) -> Rd {
        let folk = self.folk_day();
        let start = match self {
            Self::Hidirellez => hizir_start(year),
            Self::Kasim | Self::Erbain => kasim_start(year),
            _ => kasim_start(year - 1),
        };
        Rd(start.0 + folk.day as i64 - 1)
    }
}

/// Hıdırellez of `year`, 6 May.
const fn hizir_start(year: i64) -> Rd {
    gregorian::from_year_month_day(year, 5, 6)
}

/// The first Kasım day of `year`, 8 November.
const fn kasim_start(year: i64) -> Rd {
    gregorian::from_year_month_day(year, 11, 8)
}

/// The day of the folk year a day is.
#[must_use]
pub const fn folk_day(day: Rd) -> FolkDay {
    let (year, _, _) = gregorian::year_month_day_from_rd(day);
    let (half, start) = if day.0 < hizir_start(year).0 {
        (Half::Kasim, kasim_start(year - 1))
    } else if day.0 < kasim_start(year).0 {
        (Half::Hizir, hizir_start(year))
    } else {
        (Half::Kasim, kasim_start(year))
    };
    // A half is at most 186 days, so the count fits.
    FolkDay {
        half,
        day: (day.0 - start.0 + 1) as u16,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        hc_calendar::gregorian::to_fixed(year, month, day).expect("a date")
    }

    #[test]
    fn the_cemre_fall_on_the_sources_days() {
        // Bilkent, 「Cemre」: Kasım 105 on 20 February, 112 on 27 February,
        // 119 on 6 March, "Şubatın 29 çektiği dört senede bir 5 Mart'ta".
        for (year, earth) in [(2026, 6), (2025, 6), (2024, 5), (2028, 5)] {
            assert_eq!(NamedDay::CemreAir.in_year(year), ymd(year, 2, 20));
            assert_eq!(NamedDay::CemreWater.in_year(year), ymd(year, 2, 27));
            assert_eq!(NamedDay::CemreEarth.in_year(year), ymd(year, 3, earth));
        }
        assert_eq!(
            folk_day(ymd(2026, 2, 20)),
            FolkDay {
                half: Half::Kasim,
                day: 105
            }
        );
    }

    #[test]
    fn the_halves_meet_on_hidirellez_and_on_the_eighth_of_november() {
        // Wikipedia (tr), 「Hıdırellez」: Hızır from 6 May to 7 November,
        // Kasım from 8 November to 5 May.
        let at = |y, m, d| folk_day(ymd(y, m, d));
        assert_eq!(
            at(2026, 5, 6),
            FolkDay {
                half: Half::Hizir,
                day: 1
            }
        );
        assert_eq!(at(2026, 5, 5).half, Half::Kasim);
        assert_eq!(
            at(2026, 11, 7),
            FolkDay {
                half: Half::Hizir,
                day: 186
            }
        );
        assert_eq!(
            at(2026, 11, 8),
            FolkDay {
                half: Half::Kasim,
                day: 1
            }
        );
        // The Kasım half is 180 days when it holds a 29 February, as the
        // source counts it, and 179 otherwise.
        assert_eq!(at(2024, 5, 5).day, 180);
        assert_eq!(at(2026, 5, 5).day, 179);
        assert_eq!(NamedDay::Hidirellez.in_year(2026), ymd(2026, 5, 6));
        assert_eq!(NamedDay::Kasim.in_year(2026), ymd(2026, 11, 8));
    }

    #[test]
    fn erbain_and_hamsin_enter_on_their_counts() {
        // Bilkent: erbain on Kasım 46, hamsin on Kasım 86.
        assert_eq!(NamedDay::Erbain.in_year(2025), ymd(2025, 12, 23));
        assert_eq!(NamedDay::Hamsin.in_year(2026), ymd(2026, 2, 1));
        for named in NamedDay::ALL {
            for year in [2023, 2024, 2025, 2026] {
                let day = named.in_year(year);
                assert_eq!(folk_day(day), named.folk_day(), "{named:?} {year}");
                assert_eq!(gregorian::year_from_rd(day), year, "{named:?}");
            }
        }
        assert_eq!(NamedDay::CemreEarth.turkish_name(), "üçüncü cemre");
        assert_eq!(Half::Hizir.turkish_name(), "Hızır günleri");
    }
}
