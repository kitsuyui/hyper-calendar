//! The southern sixty-year cycle of year names, Prabhava to Kṣaya: the
//! *saṃvatsara* the Tamil solar year carries.
//!
//! The cycle, its two reckonings and the checks are written up in
//! `docs/systems/hindu-calendars.md` in the repository, under "The sixty
//! year names"; this page states the code's own facts.
//!
//! Sixty names, "often known as the 'Brihaspati samvatsara chakra'", are
//! given to the years in succession from Prabhava (Sewell and Dikshit, *The
//! Indian Calendar*, 1896, Art. 53). The north reckons them by Jupiter's
//! mean motion and expunges one about every 85 years; south of the Narmada
//! the expunction was given up from Śaka 828 or 831, and the names "are
//! made to correspond with the luni-solar year as well as the solar" — the
//! same name for the Tamil solar year and for the Telugu and Kannada
//! lunisolar year that begins in it. The rule is Sewell and Dikshit's
//! (Art. 62): "add 11 to the current Saka year, and divide by 60; the
//! remainder is the corresponding luni-solar cycle year", counted from
//! Prabhava as 1; and "at present the northern samvatsara has advanced by
//! 12 on the southern". Only the southern cycle is here, and only the Tamil
//! solar calendar carries it ([`crate::hindu_solar::TAMIL`]); the Tamil
//! year of 2024–25 is Krodhin, the 38th, which Tamil almanacs print as
//! Krodhi, குரோதி.
//!
//! [`NAMES`] are the Sanskrit names as Sewell and Dikshit list them, without
//! their diacritics; the Tamil names are `hc-i18n`'s, from the University of
//! Madras's *Tamil Lexicon*.

/// The kind of cycle, as a calendar declares it and a locale names it.
pub const CYCLE: &str = "samvatsara";

/// The number of names in the cycle.
pub const LENGTH: u8 = 60;

/// The sixty names, Prabhava first, as Sewell and Dikshit list them (Table
/// I, col. 6, and Table XII), without diacritics.
pub const NAMES: [&str; 60] = [
    "Prabhava",
    "Vibhava",
    "Sukla",
    "Pramoda",
    "Prajapati",
    "Angiras",
    "Srimukha",
    "Bhava",
    "Yuvan",
    "Dhatri",
    "Isvara",
    "Bahudhanya",
    "Pramathin",
    "Vikrama",
    "Vrisha",
    "Chitrabhanu",
    "Subhanu",
    "Tarana",
    "Parthiva",
    "Vyaya",
    "Sarvajit",
    "Sarvadharin",
    "Virodhin",
    "Vikrita",
    "Khara",
    "Nandana",
    "Vijaya",
    "Jaya",
    "Manmatha",
    "Durmukha",
    "Hemalamba",
    "Vilamba",
    "Vikarin",
    "Sarvari",
    "Plava",
    "Subhakrit",
    "Sobhana",
    "Krodhin",
    "Visvavasu",
    "Parabhava",
    "Plavanga",
    "Kilaka",
    "Saumya",
    "Sadharana",
    "Virodhakrit",
    "Paridhavin",
    "Pramadin",
    "Ananda",
    "Rakshasa",
    "Anala",
    "Pingala",
    "Kalayukta",
    "Siddharthin",
    "Raudra",
    "Durmati",
    "Dundubhi",
    "Rudhirodgarin",
    "Raktaksha",
    "Krodhana",
    "Kshaya",
];

/// What Sewell and Dikshit add to the current Śaka year before dividing by
/// sixty (Art. 62).
pub const SAKA_ADDEND: i64 = 11;

/// The position, 1 for Prabhava through 60 for Kṣaya, of the southern
/// cycle's year that begins in an *expired* Śaka year — the Śaka year the
/// *Rashtriya Panchang* prints, one less than the current year Sewell and
/// Dikshit count.
#[must_use]
pub const fn southern_of_saka(saka: i64) -> u8 {
    match (saka + 1 + SAKA_ADDEND).rem_euclid(LENGTH as i64) {
        0 => LENGTH,
        position => position as u8,
    }
}

/// The name of a position, 1 through 60.
#[must_use]
pub const fn name(position: u8) -> Option<&'static str> {
    if position == 0 || position > LENGTH {
        None
    } else {
        Some(NAMES[(position - 1) as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sewell_and_dikshits_rule_names_their_own_examples() {
        // Art. 62's rule on the book's worked examples: the Tamil year
        // Rudhirodgarin, "K.Y. 4904 expired, Saka 1725 expired" (1803–04);
        // "Angiras samvatsara in luni-solar or southern reckoning", K.Y. 4853
        // expired, Śaka 1674 (1752); and the Telugu year of "Saka 1744
        // expired, Chitrabhanu samvatsara in the luni-solar 60-year or
        // southern cycle reckoning, Vijaya in the northern" (1822–23).
        assert_eq!(name(southern_of_saka(1_725)), Some("Rudhirodgarin"));
        assert_eq!(name(southern_of_saka(1_674)), Some("Angiras"));
        assert_eq!(name(southern_of_saka(1_744)), Some("Chitrabhanu"));
    }

    #[test]
    fn the_cycle_runs_prabhava_to_kshaya_and_closes() {
        // Prabhava is the southern year that began in 1987, Śaka 1909.
        assert_eq!(southern_of_saka(1_909), 1);
        assert_eq!(southern_of_saka(1_909 + 59), 60);
        assert_eq!(southern_of_saka(1_909 + 60), 1);
        assert_eq!(southern_of_saka(1_909 - 1), 60);
        assert_eq!(name(0), None);
        assert_eq!(name(61), None);
        assert_eq!(name(60), Some("Kshaya"));
    }
}
