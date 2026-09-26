//! The northern sixty-year cycle, the *Bārhaspatya saṃvatsara*: the sixty
//! names of [`crate::samvatsara`] reckoned by Jupiter's mean motion
//! through the signs.
//!
//! The cycle, its rules, a year worked by hand and the checks are written
//! up in `docs/systems/hindu-calendars.md` in the repository, under "The
//! northern cycle"; this page states the code's own facts.
//!
//! A *saṃvatsara* of the north is the time Jupiter's mean place takes to
//! cross one sign, "about 361.026721 days" by the *Sūrya Siddhānta*, some
//! four days short of the solar year; so the names gain on the years, and
//! when two begin in one solar year "the first is said to be expunged"
//! (Sewell and Dikshit, *The Indian Calendar*, 1896, Art. 54,
//! `sewell1896`). In practice the name current at the beginning of a year
//! is "coupled with all the days of that year" (Art. 55), and the year is
//! the solar one: Sewell and Dikshit's Table I gives the name "current at
//! the beginning of the solar year, i.e., at the true (or apparent) Mesha
//! sankranti" (Art. 120).
//!
//! # The rules, as data
//!
//! Art. 59 gives one procedure with different numbers for each authority,
//! and each is a [`MeanSignRule`] here, per policy §5:
//! [`SURYA_SIDDHANTA`] (Art. 59 a), [`ARYA_SIDDHANTA`] (b) and
//! [`SURYA_SIDDHANTA_BIJA`], the *Sūrya Siddhānta* with the *bīja*
//! correction, "to be used for years after about 1500 A.D." (c). For the
//! expired Kali year *K*: multiply by the multiplier, subtract the
//! subtrahend, divide by the divisor; the whole quotient plus *K* plus 27,
//! modulo 60 and counted from Prabhava as 1, is the name current at the
//! apparent Meṣa saṅkrānti, and the remainder, subtracted from the divisor,
//! times 361 and divided by the divisor, plus a few palas, is the number of
//! days from that saṅkrānti to the end of the name. Leaving out the
//! subtrahend and the palas gives the same for the mean saṅkrānti
//! ([`MeanSignRule::at_mean_sankranti`]; Art. 59, note to rule c), which is
//! the year Art. 60's list of expunged names counts in.
//!
//! The registered pūrṇimānta calendar names its years by
//! [`SURYA_SIDDHANTA_BIJA`], as Table I does from 1501 to 1900
//! ([`crate::hindu_purnimanta`]).
//!
//! # The moment
//!
//! [`in_progress_at`] follows the rule within the year, as Art. 59 says it
//! can: from the Siddhānta's apparent Meṣa saṅkrānti, which
//! [`crate::surya_siddhanta`] computes, the name current at it runs to the
//! day the rule gives, and the next one after it. The rule measures a
//! Jovian year as 361 days; in a year that expunges a name, the expunged
//! one ends where the next year's rule, counted back 361 days from the end
//! it gives, puts it. Sewell and Dikshit give the results "correct within
//! two ghatikās" (48 minutes) where the saṅkrānti is known.
//!
//! Sewell and Dikshit give the rules' numbers and not their derivation
//! from the Siddhānta's revolutions of Jupiter, which Burgess's translation
//! of 1860 states (`burgess1860`, not read); the numbers are theirs.

use hc_calendar::fixed::Moment;
use hc_seasons::zodiac::SiderealSign;

use crate::samvatsara::LENGTH;

/// What the rules add to the whole quotient and the Kali year before
/// dividing by sixty (Art. 59): the name current at the Kali Yuga epoch is
/// the 27th, Vijaya.
pub const KALI_ADDEND: i64 = 27;

/// The expired Kali year exceeds the expired Śaka year by this much:
/// "Saka 1436 expired = Kali 4615 expired" (Art. 59 c).
pub const SAKA_TO_KALI: i64 = 3_179;

/// The days the rules count a Jovian year as, when they turn the remainder
/// into days (Art. 59).
pub const RULE_YEAR_DAYS: i64 = 361;

/// Palas in a day: sixty ghaṭikās of sixty palas.
pub const PALAS_PER_DAY: f64 = 3_600.0;

/// One authority's form of Sewell and Dikshit's rule (Art. 59).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeanSignRule {
    /// A short name for the authority.
    pub name: &'static str,
    /// What the expired Kali year is multiplied by.
    pub multiplier: i64,
    /// What is subtracted from the product: the Jovian motion between the
    /// apparent and the mean Meṣa saṅkrānti, in the divisor's units.
    pub subtrahend: i64,
    /// What the difference is divided by.
    pub divisor: i64,
    /// The palas added to the days the remainder gives.
    pub added_palas: i64,
}

/// The *Sūrya Siddhānta* without the *bīja*: "Multiply the expired Kali year
/// by 211. Subtract 108 from the product. Divide the result by 18000", and
/// add 15 palas to the days (Art. 59 a). Table I uses it to A.D. 1500.
pub const SURYA_SIDDHANTA: MeanSignRule = MeanSignRule {
    name: "surya-siddhanta",
    multiplier: 211,
    subtrahend: 108,
    divisor: 18_000,
    added_palas: 15,
};

/// The first *Ārya Siddhānta*: "Multiply the expired Kali year by 22.
/// Subtract 11 from the product. Divide the result by 1875", and add
/// 1 ghaṭikā 45 palas to the days (Art. 59 b). The *Jyotiṣatattva* rule is
/// the same for the mean saṅkrānti (Art. 59 d).
pub const ARYA_SIDDHANTA: MeanSignRule = MeanSignRule {
    name: "arya-siddhanta",
    multiplier: 22,
    subtrahend: 11,
    divisor: 1_875,
    added_palas: 105,
};

/// The *Sūrya Siddhānta* with the *bīja*, "to be used for years after about
/// 1500 A.D.": "Multiply the expired Kali year by 117. Subtract 60 from the
/// product. Divide the result by 10000", and add 15 palas to the days
/// (Art. 59 c). Table I uses it from A.D. 1501, and the registered
/// pūrṇimānta calendar names its years by it.
pub const SURYA_SIDDHANTA_BIJA: MeanSignRule = MeanSignRule {
    name: "surya-siddhanta-bija",
    multiplier: 117,
    subtrahend: 60,
    divisor: 10_000,
    added_palas: 15,
};

/// Every rule carried.
pub const RULES: &[MeanSignRule] = &[SURYA_SIDDHANTA, ARYA_SIDDHANTA, SURYA_SIDDHANTA_BIJA];

/// A count modulo sixty, into 1 to 60.
const fn position(count: i64) -> u8 {
    match count.rem_euclid(LENGTH as i64) {
        0 => LENGTH,
        rest => rest as u8,
    }
}

/// The position after `position`, 60 wrapping to 1.
const fn next(position: u8) -> u8 {
    if position >= LENGTH { 1 } else { position + 1 }
}

impl MeanSignRule {
    /// The same rule for the mean Meṣa saṅkrānti: "If we omit the
    /// subtraction of 108, 11, and 60, and do not add 15 p., 1 gh. 45 p.,
    /// and 15 p. respectively, the result will be correct with respect to
    /// the mean Mesha-sankranti" (Art. 59, note to rule c).
    #[must_use]
    pub const fn at_mean_sankranti(self) -> Self {
        Self {
            subtrahend: 0,
            added_palas: 0,
            ..self
        }
    }

    /// The whole quotient and the remainder of the rule's division.
    const fn divide(self, kali_expired: i64) -> (i64, i64) {
        let product = self.multiplier * kali_expired - self.subtrahend;
        (
            product.div_euclid(self.divisor),
            product.rem_euclid(self.divisor),
        )
    }

    /// The position, 1 for Prabhava through 60 for Kṣaya, of the name
    /// current at the Meṣa saṅkrānti of the solar year that follows the
    /// expired Kali year `kali_expired`.
    #[must_use]
    pub const fn current_at_sankranti(self, kali_expired: i64) -> u8 {
        let (quotient, _) = self.divide(kali_expired);
        position(quotient + kali_expired + KALI_ADDEND)
    }

    /// The days, with their fraction, from the Meṣa saṅkrānti of that year
    /// to the end of the name current at it (Art. 59).
    #[must_use]
    pub fn days_to_end(self, kali_expired: i64) -> f64 {
        let (_, remainder) = self.divide(kali_expired);
        ((self.divisor - remainder) * RULE_YEAR_DAYS) as f64 / self.divisor as f64
            + self.added_palas as f64 / PALAS_PER_DAY
    }

    /// The name expunged in that solar year — the one that begins and ends
    /// in it, so that the year after opens two names on — or `None`.
    #[must_use]
    pub const fn expunged_in(self, kali_expired: i64) -> Option<u8> {
        let now = self.current_at_sankranti(kali_expired);
        let after = self.current_at_sankranti(kali_expired + 1);
        if after == next(now) {
            None
        } else {
            Some(next(now))
        }
    }
}

/// The position of the name current at the Meṣa saṅkrānti of the solar
/// year that begins in an *expired* Śaka year — the Śaka year the
/// *Rashtriya Panchang* prints — by the *Sūrya Siddhānta* with the *bīja*:
/// the name Sewell and Dikshit's Table I, col. 7, couples with the year.
#[must_use]
pub const fn northern_of_saka(saka: i64) -> u8 {
    SURYA_SIDDHANTA_BIJA.current_at_sankranti(saka + SAKA_TO_KALI)
}

/// The apparent Meṣa saṅkrānti of the *Sūrya Siddhānta* at or before a
/// moment, and the expired Kali year of the solar year it opens.
fn sankranti_at_or_before(moment: Moment) -> (Moment, i64) {
    let year = crate::surya_siddhanta::SIDEREAL_YEAR;
    let mut sankranti =
        crate::surya_siddhanta::ingress_after(SiderealSign::MESHA, Moment(moment.0 - year));
    let later = crate::surya_siddhanta::ingress_after(
        SiderealSign::MESHA,
        Moment(sankranti.0 + year / 2.0),
    );
    if later.0 <= moment.0 {
        sankranti = later;
    }
    let epoch = crate::hindu_old::HINDU_EPOCH.0 as f64;
    let kali_expired = hc_core::math::round((sankranti.0 - epoch) / year) as i64;
    (sankranti, kali_expired)
}

/// The position, 1 to 60, of the name in progress at a moment by `rule`:
/// the name current at the last apparent Meṣa saṅkrānti until the day the
/// rule gives, and the names after it from then on (Art. 59).
#[must_use]
pub fn in_progress_at(rule: MeanSignRule, moment: Moment) -> u8 {
    let (sankranti, kali_expired) = sankranti_at_or_before(moment);
    let current = rule.current_at_sankranti(kali_expired);
    let end = sankranti.0 + rule.days_to_end(kali_expired);
    if moment.0 < end {
        return current;
    }
    let following = next(current);
    if rule.expunged_in(kali_expired).is_none() {
        return following;
    }
    // The expunged name ends where the one after it begins: the end the
    // next year's rule gives, less a Jovian year of the rule's 361 days.
    let (next_sankranti, _) = sankranti_at_or_before(Moment(sankranti.0 + 366.0));
    let expunged_ends =
        next_sankranti.0 + rule.days_to_end(kali_expired + 1) - RULE_YEAR_DAYS as f64;
    if moment.0 < expunged_ends {
        following
    } else {
        next(following)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::samvatsara::name;

    /// Days, ghaṭikās and palas as a number of palas.
    fn palas(days: f64) -> f64 {
        days * PALAS_PER_DAY
    }

    #[test]
    fn the_three_rules_work_sewell_and_dikshits_examples() {
        // Art. 59, example 1: Saka 233 expired, Kali 3412 expired, by the
        // Sūrya Siddhānta: "No. 58 Raktakshin ... was the samvatsara current
        // at the beginning", ending "3 d. 32 gh. 2.2 pa." after the
        // saṅkrānti.
        assert_eq!(SURYA_SIDDHANTA.current_at_sankranti(233 + SAKA_TO_KALI), 58);
        assert_eq!(name(58), Some("Raktaksha"));
        let expected = 3.0 * 3_600.0 + 32.0 * 60.0 + 2.2;
        assert!((palas(SURYA_SIDDHANTA.days_to_end(3_412)) - expected).abs() < 0.01);
        // "since Krodhana commences within four days after Mesha it will
        // be expunged".
        assert_eq!(SURYA_SIDDHANTA.expunged_in(3_412), Some(59));

        // Example 2: Saka 230 expired, Kali 3409, by the Ārya Siddhānta:
        // "No. 55 Durmati ... was current", ending 2 d. 31 gh. 55.56 pa.
        // after, and "since Dundubhi commences within four days of the
        // Mesha sankranti, it will be expunged".
        assert_eq!(ARYA_SIDDHANTA.current_at_sankranti(230 + SAKA_TO_KALI), 55);
        let expected = 2.0 * 3_600.0 + 31.0 * 60.0 + 55.56;
        assert!((palas(ARYA_SIDDHANTA.days_to_end(3_409)) - expected).abs() < 0.01);
        assert_eq!(ARYA_SIDDHANTA.expunged_in(3_409), Some(56));

        // Example 3: Saka 1436 expired, Kali 4615, with the bīja: "Vrisha
        // was current at the Mesha-sankranti", ending 3 d. 47 gh. 40.8 p.
        // after; "At that moment Chitrabhanu begins, and since it began
        // within four days of the Mesha-sankranti, it is expunged".
        assert_eq!(SURYA_SIDDHANTA_BIJA.current_at_sankranti(4_615), 15);
        assert_eq!(name(15), Some("Vrisha"));
        let expected = 3.0 * 3_600.0 + 47.0 * 60.0 + 40.8;
        assert!((palas(SURYA_SIDDHANTA_BIJA.days_to_end(4_615)) - expected).abs() < 0.01);
        assert_eq!(SURYA_SIDDHANTA_BIJA.expunged_in(4_615), Some(16));
    }

    /// Art. 60's list of expunged names, "taking the year to begin at the
    /// mean Mesha sankranti": the current Śaka year and the name, for the
    /// Ārya rules and for the Sūrya Siddhānta "without bīja up to 1500
    /// A.D., and with bīja afterwards". The asterisked years of the
    /// Siddhānta column "differ from those given in Table I., col. 7",
    /// which counts from the apparent saṅkrānti.
    const ARYA_LIST: &[(i64, u8)] = &[
        (232, 57),
        (317, 23),
        (402, 49),
        (487, 15),
        (572, 41),
        (658, 8),
        (743, 34),
        (828, 60),
        (913, 26),
        (999, 53),
        (1_084, 19),
        (1_169, 45),
        (1_254, 11),
        (1_340, 38),
        (1_425, 4),
        (1_510, 30),
        (1_595, 56),
        (1_680, 22),
        (1_766, 49),
    ];

    /// The Sūrya Siddhānta column: current Śaka year, name, asterisk.
    const SURYA_LIST: &[(i64, u8, bool)] = &[
        (234, 59, false),
        (319, 25, true),
        (404, 51, true),
        (490, 18, false),
        (575, 44, true),
        (660, 10, true),
        (746, 37, false),
        (831, 3, false),
        (916, 29, true),
        (1_002, 56, false),
        (1_087, 22, false),
        (1_172, 48, true),
        (1_258, 15, false),
        (1_343, 41, false),
        (1_437, 16, false),
        (1_522, 42, true),
        (1_608, 9, false),
        (1_693, 35, true),
        (1_779, 2, false),
    ];

    /// Every year of a range in which `rule` expunges a name, with the name.
    fn expunctions(rule: MeanSignRule, sakas: core::ops::Range<i64>) -> Vec<(i64, u8)> {
        sakas
            .filter_map(|saka| {
                rule.expunged_in(saka - 1 + SAKA_TO_KALI)
                    .map(|name| (saka, name))
            })
            .collect()
    }

    #[test]
    fn the_siddhantas_expunged_names_are_sewell_and_dikshits_list() {
        // Every expunction of the Sūrya Siddhānta, at the mean saṅkrānti,
        // from Śaka 200 to 1800 current, without the bīja to 1500 and with
        // it after, is the list's, and there are no others.
        let mut surya = expunctions(SURYA_SIDDHANTA.at_mean_sankranti(), 200..1_423);
        surya.extend(expunctions(
            SURYA_SIDDHANTA_BIJA.at_mean_sankranti(),
            1_423..1_800,
        ));
        let listed: Vec<_> = SURYA_LIST.iter().map(|&(s, n, _)| (s, n)).collect();
        assert_eq!(surya, listed);
    }

    #[test]
    fn table_i_counts_from_the_apparent_sankranti_and_differs_where_marked() {
        // The asterisked years of the list are "in each case one earlier"
        // than Table I, which counts from the apparent saṅkrānti: there the
        // expunction falls a year later, on the name after. The rest agree.
        let mut table = expunctions(SURYA_SIDDHANTA, 200..1_423);
        table.extend(expunctions(SURYA_SIDDHANTA_BIJA, 1_423..1_800));
        let shifted: Vec<_> = SURYA_LIST
            .iter()
            .map(|&(saka, expunged, marked)| {
                if marked {
                    (saka + 1, expunged + 1)
                } else {
                    (saka, expunged)
                }
            })
            .collect();
        assert_eq!(table, shifted);
    }

    #[test]
    fn the_arya_column_of_the_list_is_a_year_after_the_arya_rule() {
        // The Ārya rule puts every expunction of the list's Ārya column a
        // year earlier, on the name before: Durmati's successor Dundubhi,
        // the 56th, in Śaka 231 current, where the list has Rudhirodgarin,
        // the 57th, in 232. Example 2 of Art. 59 is on the rule's side: in
        // Śaka 230 expired, 231 current, "Dundubhi commences within four
        // days of the Mesha sankranti" and "will be expunged". Art. 60's
        // note gives 231 and the 56th, 998 and the 52nd, and 1339 and the
        // 37th as others' reading of the Bṛhatsaṃhitā rule; this is the
        // Ārya rule's reading of all nineteen.
        let arya = expunctions(ARYA_SIDDHANTA.at_mean_sankranti(), 200..1_800);
        let earlier: Vec<_> = ARYA_LIST
            .iter()
            .map(|&(saka, expunged)| (saka - 1, if expunged == 1 { 60 } else { expunged - 1 }))
            .collect();
        assert_eq!(arya, earlier);
        assert_eq!(ARYA_SIDDHANTA.expunged_in(230 + SAKA_TO_KALI), Some(56));
    }

    #[test]
    fn the_northern_and_southern_names_stand_eleven_then_twelve_then_thirteen_apart() {
        // The book's worked example of 1822: Saka 1744 expired is "Vijaya"
        // in the Bṛhaspati cycle and "Chitrabhanu" in the southern.
        assert_eq!(name(northern_of_saka(1_744)), Some("Vijaya"));
        assert_eq!(
            name(crate::samvatsara::southern_of_saka(1_744)),
            Some("Chitrabhanu")
        );
        // Art. 54: Vibhava expunged in Śaka 1779 current, 1778 expired;
        // after it "the northern samvatsara has advanced by 12 on the
        // southern" (Art. 62).
        let gap = |saka: i64| {
            (i64::from(northern_of_saka(saka))
                - i64::from(crate::samvatsara::southern_of_saka(saka)))
            .rem_euclid(60)
        };
        assert_eq!(gap(1_744), 11);
        assert_eq!(gap(1_778), 11);
        assert_eq!(gap(1_779), 12);
        assert_eq!(gap(1_817), 12);
        // The next expunction by the rule is Manmatha, in Śaka 1864
        // expired (1942–43), and the one after it Durmati, in 1949
        // (2027–28), with none between.
        assert_eq!(
            SURYA_SIDDHANTA_BIJA.expunged_in(1_864 + SAKA_TO_KALI),
            Some(29)
        );
        assert_eq!(gap(1_864), 12);
        assert_eq!(gap(1_865), 13);
        assert_eq!(
            SURYA_SIDDHANTA_BIJA.expunged_in(1_949 + SAKA_TO_KALI),
            Some(55)
        );
        assert_eq!(gap(1_949), 13);
        assert_eq!(gap(1_950), 14);
        let between: Vec<_> = (1_865..1_949)
            .filter_map(|saka| SURYA_SIDDHANTA_BIJA.expunged_in(saka + SAKA_TO_KALI))
            .collect();
        assert!(between.is_empty());
    }

    #[test]
    fn printed_northern_years_carry_the_rules_names() {
        // The Hrishikesh Panchang of Varanasi for 2024–25: "श्री संवत् २०८१
        // शकः १९४६ पिङ्गल नामाब्दः", Vikrama 2081, Śaka 1946, the year
        // named Pingala, its title as Exotic India lists it
        // (`hrishikesh-panchang-2081`; the almanac itself not read).
        assert_eq!(name(northern_of_saka(1_946)), Some("Pingala"));
    }

    /// A moment at Ujjain: a Julian date and the ghaṭikās and palas after
    /// mean sunrise, six in the morning of Ujjain's mean time.
    fn at_ujjain(year: i64, month: u8, day: u8, ghatikas: f64, palas: f64) -> Moment {
        let rd = hc_calendars_solar::julian::to_fixed(year, month, day).unwrap();
        let local = 0.25 + (ghatikas + palas / 60.0) / 60.0;
        Moment(rd.0 as f64 + local - crate::surya_siddhanta::UJJAIN_LONGITUDE_DEGREES / 360.0)
    }

    #[test]
    fn the_moment_follows_the_rule_through_an_expunged_year() {
        // Example 3: the Meṣa saṅkrānti of Śaka 1436 expired was "March
        // 27th, 44 gh. 25 p., Monday", A.D. 1514, and "Vrisha ended at
        // 32 gh. 5.8 p. after mean sunrise at Ujjain on Friday, 31st March.
        // At that moment Chitrabhanu begins".
        let sankranti = crate::surya_siddhanta::ingress_after(
            SiderealSign::MESHA,
            at_ujjain(1_514, 3, 20, 0.0, 0.0),
        );
        let printed = at_ujjain(1_514, 3, 27, 44.0, 25.0);
        // The Siddhānta's saṅkrānti is a minute and a half from the one
        // Table I prints.
        assert!((sankranti.0 - printed.0).abs() * 1_440.0 < 3.0);
        let rule = SURYA_SIDDHANTA_BIJA;
        let end = at_ujjain(1_514, 3, 31, 32.0, 5.8);
        // Within three minutes either side.
        assert_eq!(in_progress_at(rule, Moment(end.0 - 0.002)), 15);
        assert_eq!(in_progress_at(rule, Moment(end.0 + 0.002)), 16);
        // Chitrabhanu is expunged: Subhanu begins before the next
        // saṅkrānti, and is the name current at it.
        assert_eq!(rule.current_at_sankranti(4_616), 17);
        let next =
            crate::surya_siddhanta::ingress_after(SiderealSign::MESHA, Moment(sankranti.0 + 300.0));
        assert_eq!(in_progress_at(rule, Moment(next.0 - 0.1)), 17);
        assert_eq!(in_progress_at(rule, Moment(end.0 + 300.0)), 16);
        assert_eq!(in_progress_at(rule, next), 17);
    }

    #[test]
    fn vibhava_begins_three_days_after_the_sankranti_of_1779() {
        // Art. 54: "Prabhava (No. 1) was current at the beginning of the
        // solar year Saka 1779. Vibhava (No. 2) commenced 3.3 days after
        // the beginning of that year ... and Sukla (No. 3) began 361.03
        // days after Vibhava, that is 364.3 days after the beginning".
        let kali = 1_778 + SAKA_TO_KALI;
        let rule = SURYA_SIDDHANTA_BIJA;
        assert_eq!(rule.current_at_sankranti(kali), 1);
        assert!((rule.days_to_end(kali) - 3.3).abs() < 0.05);
        let sankranti = crate::surya_siddhanta::ingress_after(
            SiderealSign::MESHA,
            at_ujjain(1_856, 3, 20, 0.0, 0.0),
        );
        let day = |days: f64| in_progress_at(rule, Moment(sankranti.0 + days));
        assert_eq!(day(3.2), 1);
        assert_eq!(day(3.4), 2);
        assert_eq!(day(364.2), 2);
        assert_eq!(day(364.4), 3);
    }

    #[test]
    fn pingala_gives_way_to_kalayukta_a_fortnight_into_saka_1946() {
        // The worked example of docs/systems/hindu-calendars.md: the
        // Siddhānta's Meṣa saṅkrānti of 2024 falls on 13 April at 17:55
        // Universal Time; Pingala, current at it, ends 15 d. 42 gh. 27.6 p.
        // later, on 29 April at 10:54, and Kalayukta begins.
        let kali = 1_946 + SAKA_TO_KALI;
        let rule = SURYA_SIDDHANTA_BIJA;
        let expected = 15.0 * 3_600.0 + 42.0 * 60.0 + 27.6;
        assert!((palas(rule.days_to_end(kali)) - expected).abs() < 0.01);
        let utc = |day: u8, hours: f64| {
            let rd = hc_calendars_solar::gregorian::to_fixed(2024, 4, day).unwrap();
            Moment(rd.0 as f64 + hours / 24.0)
        };
        let sankranti = crate::surya_siddhanta::ingress_after(SiderealSign::MESHA, utc(1, 0.0));
        assert!((sankranti.0 - utc(13, 17.92).0).abs() * 1_440.0 < 1.0);
        // Pingala began in the year before, whose name was Anala: the
        // saṅkrānti changes the year's name and not the one in progress.
        assert_eq!(northern_of_saka(1_945), 50);
        assert_eq!(in_progress_at(rule, utc(13, 17.8)), 51);
        assert_eq!(in_progress_at(rule, utc(13, 18.0)), 51);
        assert_eq!(in_progress_at(rule, utc(29, 10.8)), 51);
        assert_eq!(in_progress_at(rule, utc(29, 11.0)), 52);
        assert_eq!(name(52), Some("Kalayukta"));
    }

    #[test]
    fn the_cycle_wraps() {
        assert_eq!(position(0), 60);
        assert_eq!(position(61), 1);
        assert_eq!(next(60), 1);
        assert_eq!(RULES.len(), 3);
    }
}
