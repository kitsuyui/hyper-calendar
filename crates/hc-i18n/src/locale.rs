//! BCP 47 language tags, restricted to the parts a calendar needs.
//!
//! A [`Locale`] here is `language[-Script][-REGION][-variant]` plus the four
//! `-u-` extension keys that change how a date is printed: `ca` (calendar),
//! `nu` (numbering system), `fw` (first day of week) and `hc` (hour cycle).
//! Those keys are defined in Unicode TR 35 §3.6; the tag grammar itself is
//! RFC 5646.
//!
//! Anything else in a tag — other singletons (`-t-`, `-x-`), other `-u-`
//! keys, `-u-` attributes — is **rejected rather than dropped**. A formatter
//! that silently ignored `-u-co-phonebk` would print a date that does not
//! match the tag it was asked for, and a parse/render round-trip would stop
//! being lossless.

use core::fmt;
use core::str::FromStr;

use hc_calendar::Weekday;

use crate::error::{I18nError, I18nResult};
use crate::util::{Case, StackString, Subtag};

/// How many `-`-separated subtags a tag may have before this crate gives up.
const MAX_SUBTAGS: usize = 24;

/// Enough room for `language-Script-REGION-variant-u-ca-…-fw-…-hc-…-nu-…`.
pub(crate) const MAX_RENDERED_TAG: usize = 96;

/// The hour cycle requested by `-u-hc-`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HourCycle {
    /// `h11`: 0–11, with a day period. Used in Japanese 12-hour clocks.
    H11,
    /// `h12`: 1–12, with a day period. The usual English 12-hour clock.
    H12,
    /// `h23`: 0–23. The ISO 8601 clock.
    H23,
    /// `h24`: 1–24. Rare; midnight ends the day rather than starting it.
    H24,
}

impl HourCycle {
    /// The `-u-hc-` value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::H11 => "h11",
            Self::H12 => "h12",
            Self::H23 => "h23",
            Self::H24 => "h24",
        }
    }

    /// Parse a `-u-hc-` value.
    #[must_use]
    pub fn from_extension_value(value: &str) -> Option<Self> {
        match value {
            "h11" => Some(Self::H11),
            "h12" => Some(Self::H12),
            "h23" => Some(Self::H23),
            "h24" => Some(Self::H24),
            _ => None,
        }
    }

    /// Whether the cycle needs a day period (am/pm) to be unambiguous.
    #[must_use]
    pub const fn needs_day_period(self) -> bool {
        matches!(self, Self::H11 | Self::H12)
    }
}

impl fmt::Display for HourCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Parse a `-u-fw-` value into a weekday.
#[must_use]
pub fn weekday_from_extension_value(value: &str) -> Option<Weekday> {
    match value {
        "mon" => Some(Weekday::Monday),
        "tue" => Some(Weekday::Tuesday),
        "wed" => Some(Weekday::Wednesday),
        "thu" => Some(Weekday::Thursday),
        "fri" => Some(Weekday::Friday),
        "sat" => Some(Weekday::Saturday),
        "sun" => Some(Weekday::Sunday),
        _ => None,
    }
}

/// Render a weekday as a `-u-fw-` value.
#[must_use]
pub const fn weekday_extension_value(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "mon",
        Weekday::Tuesday => "tue",
        Weekday::Wednesday => "wed",
        Weekday::Thursday => "thu",
        Weekday::Friday => "fri",
        Weekday::Saturday => "sat",
        Weekday::Sunday => "sun",
    }
}

/// Regions whose week does not start on Monday.
///
/// CLDR carries this as `weekData/firstDay` in `supplementalData.xml`. Only
/// the exceptions are listed; everything absent from the table starts its
/// week on Monday, which is both the ISO 8601 rule and the CLDR default.
const REGION_FIRST_DAY: &[(&str, Weekday)] = &[
    ("AE", Weekday::Saturday),
    ("AF", Weekday::Saturday),
    ("BH", Weekday::Saturday),
    ("DJ", Weekday::Saturday),
    ("DZ", Weekday::Saturday),
    ("EG", Weekday::Saturday),
    ("IQ", Weekday::Saturday),
    ("IR", Weekday::Saturday),
    ("JO", Weekday::Saturday),
    ("KW", Weekday::Saturday),
    ("LY", Weekday::Saturday),
    ("OM", Weekday::Saturday),
    ("QA", Weekday::Saturday),
    ("SD", Weekday::Saturday),
    ("SY", Weekday::Saturday),
    ("MV", Weekday::Friday),
    ("BR", Weekday::Sunday),
    ("CA", Weekday::Sunday),
    ("CO", Weekday::Sunday),
    ("HK", Weekday::Sunday),
    ("IL", Weekday::Sunday),
    ("IN", Weekday::Sunday),
    ("JP", Weekday::Sunday),
    ("KR", Weekday::Sunday),
    ("MX", Weekday::Sunday),
    ("PE", Weekday::Sunday),
    ("PH", Weekday::Sunday),
    ("SA", Weekday::Sunday),
    ("TW", Weekday::Sunday),
    ("US", Weekday::Sunday),
    ("ZA", Weekday::Sunday),
];

/// The first day of the week customary in a region, if it is not Monday.
#[must_use]
pub fn region_first_day_of_week(region: &str) -> Option<Weekday> {
    REGION_FIRST_DAY
        .iter()
        .find(|(code, _)| *code == region)
        .map(|(_, day)| *day)
}

/// A language tag: identity plus the four calendar-relevant `-u-` keys.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Locale {
    language: Subtag<8>,
    script: Option<Subtag<4>>,
    region: Option<Subtag<3>>,
    variant: Option<Subtag<8>>,
    calendar: Option<Subtag<20>>,
    numbering: Option<Subtag<8>>,
    first_day: Option<Weekday>,
    hour_cycle: Option<HourCycle>,
}

impl Locale {
    /// The root locale, `und`.
    ///
    /// CLDR calls it `root`; BCP 47 spells the same thing `und`. Parsing
    /// accepts both and rendering always produces `und`.
    pub const ROOT: Self = Self {
        language: Subtag::literal("und"),
        script: None,
        region: None,
        variant: None,
        calendar: None,
        numbering: None,
        first_day: None,
        hour_cycle: None,
    };

    /// Parse a language tag.
    ///
    /// # Errors
    ///
    /// Returns [`I18nError::InvalidTag`] if the tag does not match the
    /// RFC 5646 grammar this crate accepts, [`I18nError::UnsupportedExtension`]
    /// for a singleton other than `u`, [`I18nError::UnknownExtensionKey`] for
    /// a `-u-` key outside `ca`/`nu`/`fw`/`hc`, and
    /// [`I18nError::InvalidExtensionValue`] for a key whose value it does not
    /// recognise.
    pub fn parse(tag: &str) -> I18nResult<Self> {
        if tag.is_empty() {
            return Err(I18nError::EmptyTag);
        }
        if !tag.is_ascii() {
            return Err(I18nError::InvalidTag);
        }

        let mut subtags = [""; MAX_SUBTAGS];
        let mut count = 0usize;
        for part in tag.split(['-', '_']) {
            if count == MAX_SUBTAGS {
                return Err(I18nError::TooManySubtags);
            }
            subtags[count] = part;
            count += 1;
        }

        let mut locale = Self::ROOT;
        let first = subtags[0];
        if !first.eq_ignore_ascii_case("root") {
            if first.len() < 2 || first.len() > 8 || !first.bytes().all(|b| b.is_ascii_alphabetic())
            {
                return Err(I18nError::InvalidTag);
            }
            locale.language =
                Subtag::normalised(first, Case::Lower).ok_or(I18nError::SubtagTooLong)?;
        }

        let mut index = 1usize;
        while index < count {
            let part = subtags[index];
            if part.len() == 1 {
                break;
            }
            if part.len() == 4
                && part.bytes().all(|b| b.is_ascii_alphabetic())
                && locale.script.is_none()
                && locale.region.is_none()
                && locale.variant.is_none()
            {
                locale.script =
                    Some(Subtag::normalised(part, Case::Title).ok_or(I18nError::SubtagTooLong)?);
            } else if is_region(part) && locale.region.is_none() && locale.variant.is_none() {
                locale.region =
                    Some(Subtag::normalised(part, Case::Upper).ok_or(I18nError::SubtagTooLong)?);
            } else if is_variant(part) && locale.variant.is_none() {
                locale.variant =
                    Some(Subtag::normalised(part, Case::Lower).ok_or(I18nError::SubtagTooLong)?);
            } else {
                return Err(I18nError::InvalidTag);
            }
            index += 1;
        }

        while index < count {
            let singleton = subtags[index];
            if singleton.len() != 1 {
                return Err(I18nError::InvalidTag);
            }
            if !singleton.eq_ignore_ascii_case("u") {
                return Err(I18nError::UnsupportedExtension);
            }
            index += 1;
            if index == count {
                return Err(I18nError::InvalidTag);
            }
            while index < count && subtags[index].len() != 1 {
                let key = subtags[index];
                if key.len() != 2 {
                    // A 3-to-8 character subtag where a key belongs is a
                    // `-u-` attribute. This crate has no use for one and
                    // will not drop it silently.
                    return Err(I18nError::InvalidTag);
                }
                index += 1;
                let value_start = index;
                while index < count && subtags[index].len() > 2 {
                    index += 1;
                }
                locale.apply_extension(key, &subtags[value_start..index])?;
            }
        }

        Ok(locale)
    }

    fn apply_extension(&mut self, key: &str, value: &[&str]) -> I18nResult<()> {
        if value.is_empty() {
            return Err(I18nError::InvalidExtensionValue);
        }
        let mut joined = StackString::<20>::new();
        {
            use core::fmt::Write as _;
            for (position, part) in value.iter().enumerate() {
                if position > 0 {
                    joined
                        .write_char('-')
                        .map_err(|_| I18nError::SubtagTooLong)?;
                }
                joined
                    .write_str(part)
                    .map_err(|_| I18nError::SubtagTooLong)?;
            }
        }
        let text = joined.as_str();

        if key.eq_ignore_ascii_case("ca") {
            self.calendar =
                Some(Subtag::normalised(text, Case::Lower).ok_or(I18nError::SubtagTooLong)?);
        } else if key.eq_ignore_ascii_case("nu") {
            self.numbering =
                Some(Subtag::normalised(text, Case::Lower).ok_or(I18nError::SubtagTooLong)?);
        } else if key.eq_ignore_ascii_case("fw") {
            self.first_day = Some(
                weekday_from_extension_value(&lower_ascii::<8>(text))
                    .ok_or(I18nError::InvalidExtensionValue)?,
            );
        } else if key.eq_ignore_ascii_case("hc") {
            self.hour_cycle = Some(
                HourCycle::from_extension_value(&lower_ascii::<8>(text))
                    .ok_or(I18nError::InvalidExtensionValue)?,
            );
        } else {
            return Err(I18nError::UnknownExtensionKey);
        }
        Ok(())
    }

    /// The language subtag, lowercase. `und` for the root locale.
    #[must_use]
    pub fn language(&self) -> &str {
        self.language.as_str()
    }

    /// The script subtag in title case, if any.
    #[must_use]
    pub fn script(&self) -> Option<&str> {
        self.script.as_ref().map(Subtag::as_str)
    }

    /// The region subtag in upper case, if any.
    #[must_use]
    pub fn region(&self) -> Option<&str> {
        self.region.as_ref().map(Subtag::as_str)
    }

    /// The variant subtag, lowercase, if any.
    #[must_use]
    pub fn variant(&self) -> Option<&str> {
        self.variant.as_ref().map(Subtag::as_str)
    }

    /// The `-u-ca-` calendar identifier, if any.
    #[must_use]
    pub fn calendar(&self) -> Option<&str> {
        self.calendar.as_ref().map(Subtag::as_str)
    }

    /// The `-u-nu-` numbering system identifier, if any.
    #[must_use]
    pub fn numbering_system(&self) -> Option<&str> {
        self.numbering.as_ref().map(Subtag::as_str)
    }

    /// The `-u-fw-` first day of week, if any.
    #[must_use]
    pub const fn first_day_of_week(&self) -> Option<Weekday> {
        self.first_day
    }

    /// The `-u-hc-` hour cycle, if any.
    #[must_use]
    pub const fn hour_cycle(&self) -> Option<HourCycle> {
        self.hour_cycle
    }

    /// Whether this is the root locale with no subtags or extensions.
    #[must_use]
    pub fn is_root(&self) -> bool {
        *self == Self::ROOT
    }

    /// Whether any `-u-` extension key is set.
    #[must_use]
    pub const fn has_extensions(&self) -> bool {
        self.calendar.is_some()
            || self.numbering.is_some()
            || self.first_day.is_some()
            || self.hour_cycle.is_some()
    }

    /// The same locale with every `-u-` key removed.
    #[must_use]
    pub const fn without_extensions(mut self) -> Self {
        self.calendar = None;
        self.numbering = None;
        self.first_day = None;
        self.hour_cycle = None;
        self
    }

    /// The same locale with a `-u-ca-` calendar.
    ///
    /// # Errors
    ///
    /// Returns [`I18nError::SubtagTooLong`] if the identifier is empty,
    /// longer than 20 bytes or contains anything but ASCII alphanumerics and
    /// hyphens.
    pub fn with_calendar(mut self, calendar: &str) -> I18nResult<Self> {
        self.calendar =
            Some(Subtag::normalised(calendar, Case::Lower).ok_or(I18nError::SubtagTooLong)?);
        Ok(self)
    }

    /// The same locale with a `-u-nu-` numbering system.
    ///
    /// # Errors
    ///
    /// Returns [`I18nError::SubtagTooLong`] if the identifier is empty,
    /// longer than 8 bytes or not ASCII alphanumeric.
    pub fn with_numbering_system(mut self, numbering: &str) -> I18nResult<Self> {
        self.numbering =
            Some(Subtag::normalised(numbering, Case::Lower).ok_or(I18nError::SubtagTooLong)?);
        Ok(self)
    }

    /// The same locale with a `-u-fw-` first day of week.
    #[must_use]
    pub const fn with_first_day_of_week(mut self, weekday: Weekday) -> Self {
        self.first_day = Some(weekday);
        self
    }

    /// The same locale with a `-u-hc-` hour cycle.
    #[must_use]
    pub const fn with_hour_cycle(mut self, cycle: HourCycle) -> Self {
        self.hour_cycle = Some(cycle);
        self
    }

    /// The next locale up the CLDR inheritance chain, or `None` from root.
    ///
    /// Extensions go first — they are a request, not an identity — then the
    /// identity subtags are truncated from the right, which is the
    /// "truncation inheritance" rule of Unicode TR 35 §4.1.3.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        if self.has_extensions() {
            return Some(self.without_extensions());
        }
        let mut next = *self;
        if next.variant.is_some() {
            next.variant = None;
            return Some(next);
        }
        if next.region.is_some() {
            next.region = None;
            return Some(next);
        }
        if next.script.is_some() {
            next.script = None;
            return Some(next);
        }
        if next.language.as_str() == "und" {
            return None;
        }
        Some(Self::ROOT)
    }

    /// The fallback chain, starting with this locale and ending at root.
    #[must_use]
    pub const fn fallback(&self) -> Fallback {
        Fallback {
            current: Some(*self),
        }
    }

    /// Whether this locale renders exactly as `tag`.
    ///
    /// Comparison is against the canonical rendering, so `JA_jp` matches the
    /// data entry spelled `ja-JP`.
    #[must_use]
    pub fn matches_tag(&self, tag: &str) -> bool {
        use core::fmt::Write as _;
        let mut rendered = StackString::<MAX_RENDERED_TAG>::new();
        if write!(&mut rendered, "{self}").is_err() {
            return false;
        }
        rendered.as_str() == tag
    }

    /// The tag string, written into a caller-supplied sink.
    ///
    /// # Errors
    ///
    /// Returns [`I18nError::WriteFailed`] if the sink refuses a write.
    pub fn write_tag<W: fmt::Write>(&self, out: &mut W) -> I18nResult<()> {
        write!(out, "{self}")?;
        Ok(())
    }

    /// The tag string.
    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn to_tag(&self) -> alloc::string::String {
        use alloc::string::ToString as _;
        self.to_string()
    }
}

/// Lowercase a short ASCII string into an inline buffer.
fn lower_ascii<const N: usize>(text: &str) -> LowerAscii<N> {
    let mut bytes = [0u8; N];
    let len = text.len().min(N);
    for (slot, byte) in bytes.iter_mut().zip(text.as_bytes().iter().take(len)) {
        *slot = byte.to_ascii_lowercase();
    }
    LowerAscii { bytes, len }
}

/// The result of [`lower_ascii`]; derefs to the lowercased text.
struct LowerAscii<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> core::ops::Deref for LowerAscii<N> {
    type Target = str;

    fn deref(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
}

fn is_region(part: &str) -> bool {
    (part.len() == 2 && part.bytes().all(|b| b.is_ascii_alphabetic()))
        || (part.len() == 3 && part.bytes().all(|b| b.is_ascii_digit()))
}

fn is_variant(part: &str) -> bool {
    let bytes = part.as_bytes();
    match part.len() {
        5..=8 => bytes.iter().all(u8::is_ascii_alphanumeric),
        4 => bytes[0].is_ascii_digit() && bytes.iter().all(u8::is_ascii_alphanumeric),
        _ => false,
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.language.as_str())?;
        if let Some(script) = &self.script {
            write!(f, "-{script}")?;
        }
        if let Some(region) = &self.region {
            write!(f, "-{region}")?;
        }
        if let Some(variant) = &self.variant {
            write!(f, "-{variant}")?;
        }
        if !self.has_extensions() {
            return Ok(());
        }
        // TR 35 canonical order for `-u-` keys is alphabetical.
        f.write_str("-u")?;
        if let Some(calendar) = &self.calendar {
            write!(f, "-ca-{calendar}")?;
        }
        if let Some(first_day) = self.first_day {
            write!(f, "-fw-{}", weekday_extension_value(first_day))?;
        }
        if let Some(cycle) = self.hour_cycle {
            write!(f, "-hc-{cycle}")?;
        }
        if let Some(numbering) = &self.numbering {
            write!(f, "-nu-{numbering}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Locale({self})")
    }
}

impl FromStr for Locale {
    type Err = I18nError;

    fn from_str(tag: &str) -> I18nResult<Self> {
        Self::parse(tag)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::ROOT
    }
}

/// The fallback chain produced by [`Locale::fallback`].
#[derive(Debug, Clone, Copy)]
pub struct Fallback {
    current: Option<Locale>,
}

impl Iterator for Fallback {
    type Item = Locale;

    fn next(&mut self) -> Option<Locale> {
        let current = self.current?;
        self.current = current.parent();
        Some(current)
    }
}

impl core::iter::FusedIterator for Fallback {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString as _;
    use alloc::vec::Vec;

    fn chain(tag: &str) -> Vec<alloc::string::String> {
        Locale::parse(tag)
            .unwrap()
            .fallback()
            .map(|locale| locale.to_string())
            .collect()
    }

    #[test]
    fn a_plain_language_tag_round_trips() {
        for tag in ["ja", "en-US", "zh-Hans-CN", "und", "sr-Latn", "es-419"] {
            let locale = Locale::parse(tag).unwrap();
            assert_eq!(locale.to_string(), tag);
        }
    }

    #[test]
    fn subtag_case_is_normalised_on_the_way_in() {
        let locale = Locale::parse("JA_jp").unwrap();
        assert_eq!(locale.to_string(), "ja-JP");
        assert_eq!(
            Locale::parse("ZH-hans-cn").unwrap().to_string(),
            "zh-Hans-CN"
        );
        assert_eq!(Locale::parse("root").unwrap(), Locale::ROOT);
        assert_eq!(Locale::ROOT.to_string(), "und");
    }

    #[test]
    fn the_four_modelled_extension_keys_round_trip() {
        let tag = "ja-JP-u-ca-japanese-fw-sun-hc-h11-nu-jpan";
        let locale = Locale::parse(tag).unwrap();
        assert_eq!(locale.calendar(), Some("japanese"));
        assert_eq!(locale.numbering_system(), Some("jpan"));
        assert_eq!(locale.first_day_of_week(), Some(Weekday::Sunday));
        assert_eq!(locale.hour_cycle(), Some(HourCycle::H11));
        assert_eq!(locale.to_string(), tag);
    }

    #[test]
    fn extension_keys_are_rendered_in_canonical_order() {
        let locale = Locale::parse("en-US-u-nu-latn-ca-gregory").unwrap();
        assert_eq!(locale.to_string(), "en-US-u-ca-gregory-nu-latn");
    }

    #[test]
    fn a_calendar_value_may_span_two_subtags() {
        let locale = Locale::parse("ar-SA-u-ca-islamic-umalqura").unwrap();
        assert_eq!(locale.calendar(), Some("islamic-umalqura"));
        assert_eq!(locale.to_string(), "ar-SA-u-ca-islamic-umalqura");
    }

    #[test]
    fn a_variant_survives_the_round_trip() {
        let locale = Locale::parse("ca-ES-valencia").unwrap();
        assert_eq!(locale.variant(), Some("valencia"));
        assert_eq!(locale.to_string(), "ca-ES-valencia");
    }

    #[test]
    fn unmodelled_extensions_are_refused_rather_than_dropped() {
        assert_eq!(
            Locale::parse("en-u-co-phonebk"),
            Err(I18nError::UnknownExtensionKey)
        );
        assert_eq!(
            Locale::parse("en-t-ja"),
            Err(I18nError::UnsupportedExtension)
        );
        assert_eq!(
            Locale::parse("en-x-private"),
            Err(I18nError::UnsupportedExtension)
        );
        assert_eq!(
            Locale::parse("en-u-attr-ca-gregory"),
            Err(I18nError::InvalidTag)
        );
    }

    #[test]
    fn malformed_tags_are_rejected() {
        assert_eq!(Locale::parse(""), Err(I18nError::EmptyTag));
        assert_eq!(Locale::parse("e"), Err(I18nError::InvalidTag));
        assert_eq!(Locale::parse("en--US"), Err(I18nError::InvalidTag));
        // Nine letters is longer than any language subtag may be.
        assert_eq!(Locale::parse("esperanto"), Err(I18nError::InvalidTag));
        assert_eq!(Locale::parse("en-Latn-Cyrl"), Err(I18nError::InvalidTag));
        assert_eq!(Locale::parse("ja-日本"), Err(I18nError::InvalidTag));
        assert_eq!(Locale::parse("en-US-u"), Err(I18nError::InvalidTag));
        assert_eq!(
            Locale::parse("en-u-fw-xyz"),
            Err(I18nError::InvalidExtensionValue)
        );
        assert_eq!(
            Locale::parse("en-u-hc-h25"),
            Err(I18nError::InvalidExtensionValue)
        );
    }

    #[test]
    fn the_fallback_chain_drops_extensions_then_truncates() {
        assert_eq!(
            chain("ja-JP-u-ca-japanese"),
            ["ja-JP-u-ca-japanese", "ja-JP", "ja", "und"]
        );
        assert_eq!(chain("zh-Hant-TW"), ["zh-Hant-TW", "zh-Hant", "zh", "und"]);
        assert_eq!(chain("en"), ["en", "und"]);
        assert_eq!(chain("und"), ["und"]);
    }

    #[test]
    fn the_fallback_chain_drops_a_variant_before_the_region() {
        assert_eq!(
            chain("ca-ES-valencia-u-nu-latn"),
            [
                "ca-ES-valencia-u-nu-latn",
                "ca-ES-valencia",
                "ca-ES",
                "ca",
                "und"
            ]
        );
    }

    #[test]
    fn root_has_no_parent() {
        assert_eq!(Locale::ROOT.parent(), None);
        assert!(Locale::ROOT.is_root());
        assert!(!Locale::parse("en").unwrap().is_root());
    }

    #[test]
    fn builders_produce_the_same_locale_as_parsing() {
        let built = Locale::parse("ja-JP")
            .unwrap()
            .with_calendar("japanese")
            .unwrap()
            .with_numbering_system("jpan")
            .unwrap()
            .with_first_day_of_week(Weekday::Sunday)
            .with_hour_cycle(HourCycle::H11);
        let parsed = Locale::parse("ja-JP-u-ca-japanese-fw-sun-hc-h11-nu-jpan").unwrap();
        assert_eq!(built, parsed);
        assert_eq!(built.to_tag(), parsed.to_tag());
    }

    #[test]
    fn matching_a_tag_uses_the_canonical_form() {
        let locale = Locale::parse("ZH_hant_tw").unwrap();
        assert!(locale.matches_tag("zh-Hant-TW"));
        assert!(!locale.matches_tag("zh-Hant"));
    }

    #[test]
    fn hour_cycles_know_whether_they_need_a_day_period() {
        assert!(HourCycle::H11.needs_day_period());
        assert!(HourCycle::H12.needs_day_period());
        assert!(!HourCycle::H23.needs_day_period());
        assert!(!HourCycle::H24.needs_day_period());
        assert_eq!(HourCycle::H23.to_string(), "h23");
    }

    #[test]
    fn the_first_day_of_the_week_is_a_regional_fact() {
        assert_eq!(region_first_day_of_week("US"), Some(Weekday::Sunday));
        assert_eq!(region_first_day_of_week("EG"), Some(Weekday::Saturday));
        assert_eq!(region_first_day_of_week("MV"), Some(Weekday::Friday));
        assert_eq!(region_first_day_of_week("DE"), None);
    }

    #[test]
    fn a_locale_parses_through_the_from_str_trait() {
        let locale: Locale = "fr-CA".parse().unwrap();
        assert_eq!(locale.language(), "fr");
        assert_eq!(locale.region(), Some("CA"));
        assert_eq!(locale.script(), None);
        assert_eq!(Locale::default(), Locale::ROOT);
    }
}
