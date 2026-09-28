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
//! # Two conventions in print
//!
//! Northern almanacs and announcements name the year by one of two of
//! these rules, and the names differ by one in some years; each is carried
//! under its rule's name, per policy §5, through [`MeanSignRule::of_saka`]:
//!
//! - [`SURYA_SIDDHANTA_BIJA`], the name current at the apparent Meṣa
//!   saṅkrānti with the *bīja*, as Table I gives it from 1501 to 1900. Drik
//!   Panchang heads Vikrama 2081, 2082 and 2083 Pingala, Kalayukta and
//!   Siddharthi, and ends each name a fortnight after the saṅkrānti, some
//!   two hours before the rule does (`drikpanchang-day-2024-2026`); the
//!   Hrishikesh Panchang of Varanasi titles 2081 Pingala
//!   (`hrishikesh-panchang-2081`). Drik Panchang also heads 1942 Jaya and
//!   1943 Durmukha, 2027 Raudra and 2028 Dundubhi, the two expunctions of
//!   Manmatha and Durmati, and heads 25 March 2031, the day after Chaitra
//!   śukla 1, "2088 Krodhana" while it has Raktaksha run to 31 March: the
//!   name current at the saṅkrānti, not at the pratipadā
//!   (`drikpanchang-samvatsara-days`). The registered pūrṇimānta calendar
//!   names its years by this rule ([`crate::hindu_purnimanta`]), as
//!   [`northern_of_saka`] does.
//! - [`SURYA_SIDDHANTA`], the name current at the apparent Meṣa saṅkrānti
//!   by the *Sūrya Siddhānta* without the *bīja*, which Table I uses to
//!   A.D. 1500. The Hindi press's New Year announcements of Vikrama 2068 to
//!   2083 (2011–26) were read, one or more a year. From 2068 to 2074 the
//!   two rules give the same names and the press prints them. From 2075 to
//!   2077 (2018–20) every announcement read prints the *bīja* rule's
//!   names, Virodhakrit, Paridhavi and Pramadi, and not this rule's
//!   Paridhavin, Pramadin and Ananda. From 2078 to 2083 (2021–26) the
//!   announcements read print this rule's names in 2022 and 2025, Nala and
//!   Siddharthi, and are split in the other four years, some printing this
//!   rule's Rakshasa, Pingala, Kalayukta and Raudra and some the *bīja*
//!   rule's names. So this rule is one of the press's two conventions from
//!   2021, not the press's convention. The announcements and their names
//!   are listed in `docs/systems/hindu-calendars.md`.
//!
//!   The conventions differ in when they take the name as well as in which
//!   Jupiter they count by, this rule's some forty days ahead of the
//!   *bīja*'s. This rule takes the name current at the Meṣa saṅkrānti.
//!   The press's own accounts take the one current at Chaitra śukla
//!   pratipadā, and for this rule the two part in 2018, 2019, 2020 and 2023.
//!   Neither reading gives every name the press printed: at the pratipadā
//!   the rule gives Virodhakrit, Paridhavi and Pramadi for 2018–20 but Nala
//!   for 2023, where the press printed Pingala. No almanac house's own print
//!   of this naming was read.
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

/// The extra field a calendar that names its years in the northern cycle
/// writes the name's position under, 1 for Prabhava through 60 for Kṣaya:
/// a key apart from the southern cycle's `samvatsara`
/// ([`crate::samvatsara::CYCLE`]), because the two reckonings give the
/// same year different names. Its values are the same sixty names, a cycle
/// of kind `samvatsara`.
pub const FIELD: &str = "barhaspatya-samvatsara";

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
    /// The rule's identifier, which names its authority.
    pub id: &'static str,
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
    id: "surya-siddhanta",
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
    id: "arya-siddhanta",
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
    id: "surya-siddhanta-bija",
    multiplier: 117,
    subtrahend: 60,
    divisor: 10_000,
    added_palas: 15,
};

/// Every rule carried.
pub const RULES: &[MeanSignRule] = &[SURYA_SIDDHANTA, ARYA_SIDDHANTA, SURYA_SIDDHANTA_BIJA];

/// The rule with this identifier, by [`hc_core::catalogue::matches`].
#[must_use]
pub fn by_id(id: &str) -> Option<MeanSignRule> {
    RULES
        .iter()
        .copied()
        .find(|rule| hc_core::catalogue::matches(id, rule.id))
}

hc_core::catalogue_tests! {
    type: MeanSignRule,
    id: |rule| rule.id,
    tests: mean_sign_rule_catalogue_tests,
    all: RULES,
    lookup: by_id,
}

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

    /// The position of the name this rule couples with the year that
    /// begins in an *expired* Śaka year — the Śaka year the *Rashtriya
    /// Panchang* prints: the name current at the apparent Meṣa saṅkrānti of
    /// its solar year, which is the name current at the lunisolar year's
    /// Chaitra śukla 1 as well unless a name begins between the two.
    #[must_use]
    pub const fn of_saka(self, saka: i64) -> u8 {
        self.current_at_sankranti(saka + SAKA_TO_KALI)
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
    SURYA_SIDDHANTA_BIJA.of_saka(saka)
}

/// The twelve saṃvatsaras of the twelve-year cycle of Jupiter, "named
/// after the lunar months" (Sewell and Dikshit, Art. 63, `sewell1896`), as
/// their Table XII spells them, Chaitra first: the positions
/// [`twelve_year_of`] returns, 1 to 12.
pub const TWELVE_YEAR_NAMES: [&str; 12] = [
    "Chaitra",
    "Vaisakha",
    "Jyeshtha",
    "Ashadha",
    "Sravana",
    "Bhadrapada",
    "Asvina",
    "Karttika",
    "Margasirsha",
    "Pausha",
    "Magha",
    "Phalguna",
];

/// The position, 1 for Chaitra through 12 for Phālguna, of the saṃvatsara
/// of the twelve-year cycle "of the mean-sign system" that Sewell and
/// Dikshit's Table XII couples with a name of the sixty-year cycle, 1 for
/// Prabhava through 60 for Kṣaya: Prabhava with Śrāvaṇa, and on
/// regularly, so that each twelve-year name comes five times in the sixty.
/// The two kinds of year "are similar in length", "and begin at the same
/// moment" (Art. 63), so the twelve-year name in progress is this of the
/// sixty-year name in progress by a [`MeanSignRule`]; Table XII's N.B. i
/// holds it only for the name "of the mean-sign (Northern) 60-year cycle",
/// not the southern one. A `position` outside 1 to 60 is clamped.
#[must_use]
pub const fn twelve_year_of(position: u8) -> u8 {
    let position = clamp_position(position);
    (position - 1 + 4) % 12 + 1
}

/// The sign Jupiter stands in by his mean longitude while a name of the
/// sixty-year cycle is current, 1 for Prabhava through 60 for Kṣaya:
/// Table XII's third column, Kumbha for Prabhava and one sign on for each
/// name. His apparent sign "is either the same, as or the next preceding,
/// or the next succeeding" (Table XII, N.B. ii). A `position` outside 1 to
/// 60 is clamped.
#[must_use]
pub const fn mean_sign_of(position: u8) -> SiderealSign {
    let position = clamp_position(position);
    match SiderealSign::from_index((position - 1 + 10) % 12) {
        Some(sign) => sign,
        None => SiderealSign::MESHA,
    }
}

/// The position, 1 to 12, of the twelve-year cycle's saṃvatsara in
/// progress at a moment by `rule`: [`twelve_year_of`] the sixty-year name
/// [`in_progress_at`] gives.
#[must_use]
pub fn twelve_year_in_progress_at(rule: MeanSignRule, moment: Moment) -> u8 {
    twelve_year_of(in_progress_at(rule, moment))
}

/// `position` into 1 to 60.
const fn clamp_position(position: u8) -> u8 {
    if position < 1 {
        1
    } else if position > LENGTH {
        LENGTH
    } else {
        position
    }
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
        // Drik Panchang for New Delhi heads Vikrama 2081, 2082 and 2083
        // "2081 Pingala", "2082 Kalayukta" and "2083 Siddharthi"
        // (`drikpanchang-day-2024-2026`).
        assert_eq!(name(northern_of_saka(1_947)), Some("Kalayukta"));
        assert_eq!(name(northern_of_saka(1_948)), Some("Siddharthin"));
        assert_eq!(SURYA_SIDDHANTA_BIJA.of_saka(1_948), northern_of_saka(1_948));
    }

    /// A moment in Indian Standard Time.
    fn ist(year: i64, month: u8, day: u8, hours: f64) -> Moment {
        let rd = hc_calendars_solar::gregorian::to_fixed(year, month, day).unwrap();
        Moment(rd.0 as f64 + (hours - 5.5) / 24.0)
    }

    #[test]
    fn drik_panchangs_names_end_where_the_bija_rule_ends_them() {
        // Drik Panchang gives each year's name with its end: "Pingala upto
        // 02:14 PM, Apr 29, 2024", "Kalayukta upto 03:07 PM, Apr 25, 2025",
        // "Siddharthi upto 03:53 PM, Apr 21, 2026", New Delhi time. The
        // rule with the bīja, from the Siddhānta's saṅkrānti, ends each
        // 128 to 132 minutes later.
        let rule = SURYA_SIDDHANTA_BIJA;
        for ((year, month, day, hours), ending) in [
            ((2024, 4, 29, 14.0 + 14.0 / 60.0), 51),
            ((2025, 4, 25, 15.0 + 7.0 / 60.0), 52),
            ((2026, 4, 21, 15.0 + 53.0 / 60.0), 53),
        ] {
            // 115 and 144 minutes after the printed end.
            let printed = ist(year, month, day, hours);
            assert_eq!(in_progress_at(rule, Moment(printed.0 + 0.08)), ending);
            assert_eq!(in_progress_at(rule, Moment(printed.0 + 0.1)), ending + 1);
        }
    }

    #[test]
    fn the_press_names_of_2011_to_2017_are_both_rules() {
        // Webdunia's New Year announcements of Vikrama 2068 and 2070 to
        // 2074, 2011 and 2013 to 2017, and Oneindia's of 2069, 2012
        // (`webdunia-samvat-2068-2074`, `oneindia-samvat-2069`): Krodhi,
        // Vishvavasu, Parabhava, Plavanga, Kilaka, Saumya and Sadharana,
        // the names both rules give, Śaka 1933 to 1939.
        for (saka, printed) in [
            (1_933, "Krodhin"),
            (1_934, "Visvavasu"),
            (1_935, "Parabhava"),
            (1_936, "Plavanga"),
            (1_937, "Kilaka"),
            (1_938, "Saumya"),
            (1_939, "Sadharana"),
        ] {
            assert_eq!(
                name(SURYA_SIDDHANTA_BIJA.of_saka(saka)),
                Some(printed),
                "{saka}"
            );
            assert_eq!(name(SURYA_SIDDHANTA.of_saka(saka)), Some(printed), "{saka}");
        }
    }

    #[test]
    fn the_press_names_of_2018_to_2020_are_the_bija_rules() {
        // Punjab Kesari, 19 March 2018: Vikrama 2075 "विरोधकृत"; Amar
        // Ujala, 1 April 2019: 2076 "परिधावी"; Future Point, 16 March
        // 2020: 2077 "प्रमादी" (`punjabkesari-samvat-2075`,
        // `amarujala-samvat-2076`, `futurepoint-samvat-2077`). The rule
        // without the bīja gives the next names at the saṅkrānti, and the
        // press's names at Chaitra śukla 1, 18 March 2018, 6 April 2019 and
        // 25 March 2020.
        for (saka, printed, without) in [
            (1_940, "Virodhakrit", "Paridhavin"),
            (1_941, "Paridhavin", "Pramadin"),
            (1_942, "Pramadin", "Ananda"),
        ] {
            assert_eq!(name(SURYA_SIDDHANTA_BIJA.of_saka(saka)), Some(printed));
            assert_eq!(name(SURYA_SIDDHANTA.of_saka(saka)), Some(without));
        }
        assert_eq!(in_progress_at(SURYA_SIDDHANTA, ist(2018, 3, 18, 6.0)), 45);
        assert_eq!(in_progress_at(SURYA_SIDDHANTA, ist(2019, 4, 6, 6.0)), 46);
        assert_eq!(in_progress_at(SURYA_SIDDHANTA, ist(2020, 3, 25, 6.0)), 47);
        // In 2023 the press's Pingala, ETV Bharat, 24 February 2023, is the
        // name at the saṅkrānti; at Chaitra śukla 1, 22 March 2023, the rule
        // has Nala in progress, which Zee News named, as the bīja rule does
        // (`etvbharat-samvat-2080`, `zeenews-samvat-2080`).
        assert_eq!(name(SURYA_SIDDHANTA.of_saka(1_945)), Some("Pingala"));
        assert_eq!(in_progress_at(SURYA_SIDDHANTA, ist(2023, 3, 22, 6.0)), 50);
        assert_eq!(name(SURYA_SIDDHANTA_BIJA.of_saka(1_945)), Some("Anala"));
    }

    #[test]
    fn some_press_names_of_2021_to_2026_are_the_rule_without_the_bija() {
        // Patrika, 12 April 2021: 2078 "राक्षस", Ananda expunged; Webdunia,
        // 1 April 2022: 2079 "नल"; ETV Bharat, 24 February 2023: 2080
        // "पिंगल" (`patrika-samvat-2078`, `webdunia-samvat-2079`,
        // `etvbharat-samvat-2080`). Webdunia, 4 January 2021, named 2078
        // Ananda, as the bīja rule does (`webdunia-samvat-2078`).
        let rule = SURYA_SIDDHANTA;
        assert_eq!(name(rule.of_saka(1_943)), Some("Rakshasa"));
        assert_eq!(name(rule.of_saka(1_944)), Some("Anala"));
        assert_eq!(name(SURYA_SIDDHANTA_BIJA.of_saka(1_943)), Some("Ananda"));
        // The Hindi press named Vikrama 2081, 2082 and 2083 Kalayukta
        // ("पंचांग भेद से इसका नाम कालयुक्त है", by the difference of the
        // almanacs, Webdunia, 8 December 2023), Siddharthi ("संवत का नाम
        // सिद्धार्थी", Dainik Tribune, 29 March 2025) and Raudra ("'रौद्र'
        // संवत्सर", Aaj Tak, 7 March 2026): one name after the bīja rule's,
        // and the Sūrya Siddhānta's without it (`webdunia-samvat-2081`,
        // `dainiktribune-samvat-2082`, `aajtak-samvat-2083`). Bansal News
        // and Aaj Tak named 2081 Pingala, and Asianet 2083 Siddharthi, as
        // the bīja rule does (`bansalnews-samvat-2081`,
        // `aajtak-samvat-2081`, `asianet-samvat-2083`).
        assert_eq!(name(rule.of_saka(1_946)), Some("Kalayukta"));
        assert_eq!(name(rule.of_saka(1_947)), Some("Siddharthin"));
        assert_eq!(name(rule.of_saka(1_948)), Some("Raudra"));
        for saka in 1_946..=1_948 {
            assert_eq!(rule.of_saka(saka), next(northern_of_saka(saka)));
        }
        // Each is in progress at the year's Chaitra śukla 1, 9 April 2024,
        // 30 March 2025 and 19 March 2026.
        assert_eq!(in_progress_at(rule, ist(2024, 4, 9, 6.0)), 52);
        assert_eq!(in_progress_at(rule, ist(2025, 3, 30, 6.0)), 53);
        assert_eq!(in_progress_at(rule, ist(2026, 3, 19, 6.0)), 54);
        // The Shiv Shakti Jyotish Kendra's account of 2082: Siddharthi
        // began "लगभग 15 मार्च, 2025" and lasts "लगभग 10 मार्च, 2026 ई. तक";
        // at the Meṣa saṅkrānti of 2082 it had run "०/२९/३२/५५", 0 months
        // 29 days 32 ghaṭikās 55 palas (`shivshakti-samvat-2082`). By the
        // rule it begins on 15 March 2025 and ends on 11 March 2026, the
        // day after the one the account names, and at the saṅkrānti it has
        // run the rule's 361 days less 331.36, 29 days 38 ghaṭikās, five
        // ghaṭikās more than printed.
        assert_eq!(in_progress_at(rule, ist(2025, 3, 15, 0.0)), 52);
        assert_eq!(in_progress_at(rule, ist(2025, 3, 16, 0.0)), 53);
        assert_eq!(in_progress_at(rule, ist(2026, 3, 11, 0.0)), 53);
        assert_eq!(in_progress_at(rule, ist(2026, 3, 12, 0.0)), 54);
        let kali = 1_947 + SAKA_TO_KALI;
        let elapsed = RULE_YEAR_DAYS as f64 - rule.days_to_end(kali);
        let printed = 29.0 + (32.0 + 55.0 / 60.0) / 60.0;
        assert!((elapsed - printed).abs() < 0.1, "{elapsed}");
    }

    /// A moment at Ujjain: a Julian date and the ghaṭikās and palas after
    /// mean sunrise, six in the morning of Ujjain's mean time.
    fn at_ujjain(year: i64, month: u8, day: u8, ghatikas: f64, palas: f64) -> Moment {
        let rd = hc_calendars_solar::julian::to_fixed(year, month, day).unwrap();
        let local = 0.25 + (ghatikas + palas / 60.0) / 60.0;
        Moment(rd.0 as f64 + local - crate::places::UJJAIN.longitude_degrees / 360.0)
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

    /// Sewell and Dikshit's Table XII, "the names and numbers of the
    /// samvatsaras, or years of the sixty-year cycle of Jupiter, with those
    /// of the twelve-year cycle corresponding thereto" (Art. 115): for each
    /// of the sixty, Prabhava first, the twelve-year saṃvatsara's number
    /// and Jupiter's mean sign, as read off the Internet Archive's OCR
    /// text.
    #[test]
    fn table_xii_couples_the_two_cycles() {
        let twelve: [u8; 60] = [
            5, 6, 7, 8, 9, 10, 11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 1, 2, 3, 4,
        ];
        let signs = [
            "kumbha",
            "mina",
            "mesha",
            "vrishabha",
            "mithuna",
            "karka",
            "simha",
            "kanya",
            "tula",
            "vrishchika",
            "dhanus",
            "makara",
        ];
        for position in 1..=LENGTH {
            let index = usize::from(position - 1);
            assert_eq!(twelve_year_of(position), twelve[index], "{position}");
            assert_eq!(mean_sign_of(position).id(), signs[index % 12], "{position}");
        }
        // Prabhava is Śrāvaṇa with Jupiter in mean Kumbha; Kṣaya Āṣāḍha
        // in Makara.
        assert_eq!(
            TWELVE_YEAR_NAMES[usize::from(twelve_year_of(1) - 1)],
            "Sravana"
        );
        assert_eq!(
            TWELVE_YEAR_NAMES[usize::from(twelve_year_of(60) - 1)],
            "Ashadha"
        );
        assert_eq!(twelve_year_of(0), twelve_year_of(1));
        assert_eq!(twelve_year_of(61), twelve_year_of(60));
    }

    #[test]
    fn the_twelve_year_name_turns_with_the_sixty_year_one() {
        // By the rule with the bīja, Pingala (51), the name Vikrama 2081
        // is headed with, runs out a fortnight after the Meṣa saṅkrānti of
        // 2024, and Kālayukta (52) is in progress on 1 June: Table XII
        // couples the two with Āśvina and Kārttika.
        let rule = SURYA_SIDDHANTA_BIJA;
        let at = |month, day| {
            let day = hc_calendars_solar::gregorian::to_fixed(2024, month, day).unwrap();
            Moment(day.0 as f64)
        };
        assert_eq!(in_progress_at(rule, at(4, 14)), 51);
        assert_eq!(twelve_year_in_progress_at(rule, at(4, 14)), 7);
        assert_eq!(in_progress_at(rule, at(6, 1)), 52);
        assert_eq!(twelve_year_in_progress_at(rule, at(6, 1)), 8);
        assert_eq!(TWELVE_YEAR_NAMES[6..8], ["Asvina", "Karttika"]);
    }
}
