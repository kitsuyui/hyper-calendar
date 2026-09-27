//! The Hindu festival rules: a vocabulary the traditions table and the
//! national tables share.
//!
//! A Hindu festival is a tithi of a month of the amānta lunisolar calendar,
//! kept on the day the tithi holds the part of the day the rite belongs to
//! — which is not always the day that carries the tithi at sunrise. Rāma
//! Navamī is the day the ninth tithi holds midday, Dīpāvalī the day the
//! new-moon tithi holds the evening, Janmāṣṭamī the night the eighth holds
//! midnight. [`Rule::Tithi`] says so for each, and this module is the list
//! of what each festival's rule is, with the source that says it. One
//! festival is a nakṣatra and not a tithi: Thaipusam, Puṣya in the month
//! of Thai, is [`Rule::Nakshatra`].
//!
//! # Source
//!
//! The rules are the *dharmaśāstra* conventions the *Rashtriya Panchang*
//! follows in its "Principal Festivals and Anniversaries" list, and every
//! rule here reproduces that list for Śaka 1945 and 1946 (2023–2025) —
//! `tests/traditions.rs` is the check. The Smārta reckoning of Janmāṣṭamī
//! is the one listed; the Vaiṣṇava one, a day later when the two differ,
//! is not carried as a rule. The central government's holiday lists keep
//! it — on 16 August 2025 and 25 August 2027, a day after [`JANMASHTAMI`],
//! which the 2027 list names "Janmashtami (Vaishnav)" and the 2025 list
//! "Janmashtami" — and the India table carries those lists'
//! days for the years read, 2025 to 2027 (`dopt-holidays-2025-2027`). No
//! source read states the Vaiṣṇava rule, and two parting years of one
//! list are no anchor to fit one to, so it is not registered as a
//! convention of its own (`docs/policy.md` §5). The same lists keep Holī
//! a day after [`HOLI`] in 2026 and 2027 and Guru Nānak's birthday a day
//! after [`GURU_NANAK_JAYANTI`] in 2027, by a rule not read either.
//!
//! # Two Deepavalis
//!
//! Dīpāvalī is two days, and a government's holiday is one of them. Lakṣmī
//! Pūjā is the new moon of Āśvina in the evening ([`DIWALI`]), the Diwali
//! of northern India; Naraka Caturdaśī is the fourteenth tithi at dawn
//! ([`NARAKA_CHATURDASHI`]), the Deepavali of the Tamil calendar, a day
//! earlier in some years and the same day in others. From 2017 to 2027
//! they parted in 2017, 2018 and 2027, and each table that carries the day
//! follows the lists of those years that its government published:
//!
//! | Table | Rule | The lists read |
//! | --- | --- | --- |
//! | Singapore | Naraka Caturdaśī | The Ministry of Manpower: 18 October 2017, 6 November 2018 and 28 October 2027 (`mom-public-holidays-2017-2018`, `mom-public-holidays-consolidated`) |
//! | Malaysia | Naraka Caturdaśī | The *Jadual Hari Kelepasan Am Persekutuan 2027*: "Hari Deepavali … 28 Oktober, Khamis" (`bkpp-hari-kelepasan-am`) |
//! | Sri Lanka | tabulated | The Holidays Act order for 2027: 28 October, which the rule gives |
//! | Guyana | Naraka Caturdaśī | The Ministry of Public Security's "National Holidays" lists: 18 October 2017, marked tentative, and 6 November 2018 (`mops-national-holidays-2017-2018`) |
//! | Trinidad and Tobago | Naraka Caturdaśī | The President's appointment of 18 October 2017 under the Public Holidays and Festivals Act, as *Newsday* quotes it (`newsday-divali-2017`, secondary), and Legal Notice No. 135 of 2018 for 6 November 2018, known only by its title in the Judiciary's list of legal notices, not read (`tt-legal-notice-135-2018`) |
//! | India | Lakṣmī Pūjā | The Department of Personnel and Training's lists: 19 October 2017, 7 November 2018 and 29 October 2027, each letting a state that keeps Naraka Caturdaśī alone close central offices on that day instead; read as StaffNews and GConnect reproduce them (`dopt-holidays-2017-2018-2027`, secondary) |
//! | Mauritius | Lakṣmī Pūjā | The Prime Minister's Office's General Notices No. 814 of 2016 and No. 737 of 2017: 19 October 2017 and 7 November 2018 (`pmo-mu-public-holidays-2017-2018`) |
//! | Suriname | Lakṣmī Pūjā | The Ministry of Home Affairs' days of 19 October 2017 and 7 November 2018, as *Waterkant* reports them (`waterkant-divali-2017-2018`, secondary) |
//! | Myanmar, Kenya | Lakṣmī Pūjā | None: no list of 2017, 2018 or 2027 was found. Myanmar's notices of 2020 to 2025 are tabulated and agree with either rule but 2024's, which neither gives; Kenya's Act dates Diwali "depending upon the appearance of the moon" |
//!
//! Where no list of a parting year was found the table keeps Lakṣmī Pūjā,
//! the day of India's national list.
//!
//! # Whose sunrise
//!
//! Every rule reads the day at the Central Station of the national
//! calendar, as the panchang does. A regional table that follows a local
//! sunrise can build the same rule with another
//! [`HinduLunarCalendar`].

use hc_calendars_indic::nakshatra::PUSHYA;
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
        calendar: &CALENDAR,
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

/// Dīpāvalī, Lakṣmī Pūjā: the new moon of Āśvina, in the evening — the
/// Diwali of northern India and of the national list.
///
/// A holiday called Deepavali is not always this day: see
/// [`NARAKA_CHATURDASHI`], and the module documentation for which
/// governments keep which.
pub const DIWALI: Rule = tithi(7, 30, Prevalence::Evening, WhenTwice::Later);

/// Naraka Caturdaśī: Āśvina kṛṣṇa 14 at dawn, the first of two such days
/// — the Deepavali of the Tamil calendar, and the day several governments
/// outside India keep (see the module documentation).
///
/// The Calendar Reform Committee's list of festivals (`crc1955`) keeps
/// Naraka Caturdaśī on the day its tithi covers "a period of 4 ghatikas
/// before sunrise", and "if occurs on two successive days … on the first
/// day"; Kālī Pūjā and Dīpāvalī proper are the new moon that follows.
/// It is the day before [`DIWALI`] when the fourteenth tithi holds the
/// dawn and the new moon holds the next evening, and the same day when
/// the new moon begins between the two: 2017, 2018 and 2027 are years of
/// the first kind, 2019 to 2026 all of the second.
pub const NARAKA_CHATURDASHI: Rule = tithi(7, 29, Prevalence::Dawn, WhenTwice::Earlier);

/// Guru Nānak Jayantī: the full moon of Kārtika, at midday.
pub const GURU_NANAK_JAYANTI: Rule = tithi(8, 15, Prevalence::Midday, WhenTwice::Earlier);

/// The Parkash of Guru Gobind Singh as the SGPC keeps it: Pauṣa śukla 7,
/// Poh sudi 7, the day that carries it at sunrise.
///
/// The SGPC gives the Guru's birth as "22nd December 1666, (Poh Sudi
/// Saptmi)" (`sgpc-gurpurbs`). Which part of the day the tithi must hold
/// is not stated: sunrise is taken here, and gives the days the SGPC kept,
/// 9 January and 29 December 2022 and 17 January 2024
/// (`tribune-parkash-purb-2024`).
pub const GURU_GOBIND_SINGH_PARKASH: Rule = tithi(10, 7, Prevalence::Sunrise, WhenTwice::Earlier);

/// Mahā Śivarātri: Māgha kṛṣṇa 14 at midnight.
pub const MAHA_SHIVARATRI: Rule = tithi(11, 29, Prevalence::Midnight, WhenTwice::Earlier);

/// Thaipusam — Thai Poosam: Puṣya, Tamil Pusam, in the month of Thai, the
/// Sun in sidereal Makara, on the civil day that holds the greater part
/// of the Moon's stay in Puṣya, judged at the Indian meridian.
///
/// The almanacs say "Poosam in Thai" and no more, and the days two
/// governments gazette are the check: Malaysia's and Mauritius's
/// Thaipusam for 2020 to 2026, fourteen dates, two of them a day apart in
/// 2023, when Puṣya ran from the morning of 4 February to the afternoon
/// of the 5th and each country's own midnight cut it. All fourteen fall
/// out of this reading at each country's meridian, and all but Malaysia's
/// 2023 at India's. The day whose sunrise carries the nakṣatra is not the
/// rule: in 2024 Puṣya began an hour after sunrise on 25 January and
/// held the next sunrise, and both governments kept the 25th. The
/// full-moon tithi only picks between two stays of Puṣya in one Thai.
/// A table carries the rule approximate, because it is fitted to the
/// gazettes and not quoted from an almanac.
pub const THAIPUSAM: Rule = Rule::Nakshatra {
    nakshatra: PUSHYA,
    sign: SiderealSign::MAKARA,
    with_tithi: Some(15),
    ayanamsa: &Ayanamsa::LAHIRI,
    meridian: Meridian::INDIA,
};

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
    ayanamsa: &Ayanamsa::LAHIRI,
    meridian: Meridian::INDIA,
};

/// Meṣa Saṅkrānti, the solar new year: Vaisākhī, Puthandu, Pohela
/// Boishakh, Vishu.
pub const MESHA_SANKRANTI: Rule = Rule::Sankranti {
    sign: SiderealSign::MESHA,
    ayanamsa: &Ayanamsa::LAHIRI,
    meridian: Meridian::INDIA,
};

// ─────────────────────────────────────────────────────────────────────────
// The Jain days
// ─────────────────────────────────────────────────────────────────────────

/// Saṃvatsarī, the last day of the Śvetāmbara Paryuṣaṇa: Bhādrapada
/// śukla 4, the day that carries it at sunrise.
///
/// Jain almanacs are not the *Rashtriya Panchang*, and the source says
/// that "due to computational and other differences, there can be some
/// minor differences among various sects"; the Jain rules are therefore
/// flagged approximate where they are used.
pub static SAMVATSARI: Rule = tithi(6, 4, Prevalence::Sunrise, WhenTwice::Earlier);

/// The first of the eight days of the Śvetāmbara Paryuṣaṇa, counted back
/// from Saṃvatsarī: Śrāvaṇa kṛṣṇa 12 in the amānta reckoning, when no
/// tithi is skipped or repeated in between.
pub const PARYUSHANA_FIRST_DAY: Rule = Rule::Offset {
    base: &SAMVATSARI,
    days: -7,
};

/// Ananta Caturdaśī, the last day of the Digambara Daśa Lakṣaṇa:
/// Bhādrapada śukla 14 at sunrise.
pub static ANANT_CHATURDASHI: Rule = tithi(6, 14, Prevalence::Sunrise, WhenTwice::Earlier);

/// The first of the ten days of the Digambara Daśa Lakṣaṇa, counted back
/// from Ananta Caturdaśī: Bhādrapada śukla 5, the day after the Śvetāmbara
/// festival ends, when no tithi is skipped or repeated in between.
pub const DAS_LAKSHANA_FIRST_DAY: Rule = Rule::Offset {
    base: &ANANT_CHATURDASHI,
    days: -9,
};
