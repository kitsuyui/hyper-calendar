//! The Hindu festival rules: a vocabulary the traditions table and the
//! national tables share.
//!
//! A Hindu festival is a tithi of a month of the amānta lunisolar calendar,
//! kept on the day the tithi holds the part of the day the rite belongs to
//! — which is not always the day that carries the tithi at sunrise. Rāma
//! Navamī is the day the ninth tithi holds midday, Dīpāvalī the day the
//! new-moon tithi holds the evening, Janmāṣṭamī the night the eighth holds
//! midnight. [`Rule::Tithi`] says so for each, and this module is the list
//! of what each festival's rule is, with the source that says it.
//!
//! # Source
//!
//! The rules are the *dharmaśāstra* conventions the *Rashtriya Panchang*
//! follows in its "Principal Festivals and Anniversaries" list, and every
//! rule here reproduces that list for Śaka 1945 and 1946 (2023–2025) —
//! `tests/traditions.rs` is the check. The Smārta reckoning of Janmāṣṭamī
//! is the one listed; the Vaiṣṇava one, a day later when the two differ,
//! is not carried.
//!
//! # Whose sunrise
//!
//! Every rule reads the day at the Central Station of the national
//! calendar, as the panchang does. A regional table that follows a local
//! sunrise can build the same rule with another
//! [`HinduLunarCalendar`].

use hc_calendars_indic::{HinduLunarCalendar, Prevalence};
use hc_seasons::Meridian;
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};

use crate::rule::{Rule, WhenTwice};

/// The calendar every rule here is dated in: the national almanac's.
pub const CALENDAR: HinduLunarCalendar = HinduLunarCalendar::RASHTRIYA;

/// A tithi rule in the national calendar.
const fn tithi(month: u8, tithi: u8, prevails: Prevalence, when_twice: WhenTwice) -> Rule {
    Rule::Tithi {
        month,
        tithi,
        prevails,
        when_twice,
        calendar: CALENDAR,
    }
}

/// Chaitra śukla 1, the lunar new year: Ugadi, Gudi Padwa, Cheti Chand.
pub const UGADI: Rule = tithi(1, 1, Prevalence::Sunrise, WhenTwice::Earlier);

/// Rāma Navamī: Chaitra śukla 9 at midday.
pub const RAMA_NAVAMI: Rule = tithi(1, 9, Prevalence::Midday, WhenTwice::Earlier);

/// Mahāvīra Jayantī: Chaitra śukla 13, the day that carries it at sunrise.
pub const MAHAVIR_JAYANTI: Rule = tithi(1, 13, Prevalence::Sunrise, WhenTwice::Earlier);

/// Akṣaya Tṛtīyā: Vaiśākha śukla 3 at midday.
pub const AKSHAYA_TRITIYA: Rule = tithi(2, 3, Prevalence::Midday, WhenTwice::Earlier);

/// Buddha Pūrṇimā: the full moon of Vaiśākha, at midday.
pub const BUDDHA_PURNIMA: Rule = tithi(2, 15, Prevalence::Midday, WhenTwice::Earlier);

/// Guru Pūrṇimā: the full moon of Āṣāḍha, at midday.
pub const GURU_PURNIMA: Rule = tithi(4, 15, Prevalence::Midday, WhenTwice::Earlier);

/// Rakṣā Bandhana: the full moon of Śrāvaṇa, in the afternoon.
pub const RAKSHA_BANDHAN: Rule = tithi(5, 15, Prevalence::Afternoon, WhenTwice::Earlier);

/// Kṛṣṇa Janmāṣṭamī, Smārta: Śrāvaṇa kṛṣṇa 8 at midnight.
pub const JANMASHTAMI: Rule = tithi(5, 23, Prevalence::Midnight, WhenTwice::Later);

/// Gaṇeśa Caturthī: Bhādrapada śukla 4 at midday.
pub const GANESH_CHATURTHI: Rule = tithi(6, 4, Prevalence::Midday, WhenTwice::Earlier);

/// The first day of Śāradīya Navarātri: Āśvina śukla 1 at sunrise.
pub const NAVARATRI: Rule = tithi(7, 1, Prevalence::Sunrise, WhenTwice::Earlier);

/// Mahāṣṭamī of Durgā Pūjā: Āśvina śukla 8 at sunrise.
pub const DURGA_ASHTAMI: Rule = tithi(7, 8, Prevalence::Sunrise, WhenTwice::Earlier);

/// Vijayā Daśamī, Dussehra: Āśvina śukla 10 in the afternoon.
pub const VIJAYA_DASHAMI: Rule = tithi(7, 10, Prevalence::Afternoon, WhenTwice::Earlier);

/// Dīpāvalī, Lakṣmī Pūjā: the new moon of Āśvina, in the evening.
pub const DIWALI: Rule = tithi(7, 30, Prevalence::Evening, WhenTwice::Later);

/// Guru Nānak Jayantī: the full moon of Kārtika, at midday.
pub const GURU_NANAK_JAYANTI: Rule = tithi(8, 15, Prevalence::Midday, WhenTwice::Earlier);

/// Mahā Śivarātri: Māgha kṛṣṇa 14 at midnight.
pub const MAHA_SHIVARATRI: Rule = tithi(11, 29, Prevalence::Midnight, WhenTwice::Earlier);

/// Holikā Dahana: the full moon of Phālguna, in the evening.
pub static HOLIKA_DAHAN: Rule = tithi(12, 15, Prevalence::Evening, WhenTwice::Earlier);

/// Holī, the day of colours: the day after Holikā Dahana.
pub const HOLI: Rule = Rule::Offset {
    base: &HOLIKA_DAHAN,
    days: 1,
};

/// Makara Saṅkrānti, the Sun's entry into Makara: Pongal, Māgh Bihu,
/// Uttarāyaṇa. The day of the saṅkrānti at the Indian meridian, with the
/// Lahiri ayanamsa the national calendar uses.
pub const MAKAR_SANKRANTI: Rule = Rule::Sankranti {
    sign: SiderealSign::MAKARA,
    ayanamsa: Ayanamsa::LAHIRI,
    meridian: Meridian::INDIA,
};

/// Meṣa Saṅkrānti, the solar new year: Vaisākhī, Puthandu, Pohela
/// Boishakh, Vishu.
pub const MESHA_SANKRANTI: Rule = Rule::Sankranti {
    sign: SiderealSign::MESHA,
    ayanamsa: Ayanamsa::LAHIRI,
    meridian: Meridian::INDIA,
};
