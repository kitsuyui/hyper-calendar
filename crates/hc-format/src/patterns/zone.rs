//! The time zone fields of a CLDR pattern: `z`, `O`, `v` and `V`, and the
//! localized GMT format the others fall back to.
//!
//! UTS #35 Part 4, *Dates*, version 48.2, "Using Time Zone Names", gives
//! each field a name and a chain of fallbacks, which this module follows
//! with `hc_i18n::zone_names`' data (`docs/systems/zone-names.md` works
//! them through):
//!
//! | Field | Name | Falls back to |
//! | --- | --- | --- |
//! | `z`…`zzz` | short specific non-location, *PDT* | short localized GMT |
//! | `zzzz` | long specific non-location, *Pacific Daylight Time* | long localized GMT |
//! | `O`, `OOOO` | localized GMT, *GMT-7*, *GMT-07:00* | — |
//! | `v` | short generic non-location, *PT* | generic location, then short localized GMT |
//! | `vvvv` | long generic non-location, *Pacific Time* | generic location, then long localized GMT |
//! | `V` | short zone identifier, `uslax` | `unk` |
//! | `VV` | long zone identifier, the zone as given | — |
//! | `VVV` | exemplar city, *Los Angeles* | the unknown zone's city, *Unknown Location* |
//! | `VVVV` | generic location, *Los Angeles Time* | long localized GMT |
//!
//! A name the caller gives the context, [`FormatContext::with_zone_name`]
//! and [`FormatContext::with_zone_abbreviation`], comes first for `z`, as
//! it always has. The names themselves need the `zone-names` feature, and
//! every locale's but English's `localized-zone-names`; without them the
//! fields take their fallbacks, which the formats alone give.

use core::fmt::{self, Write};

use hc_i18n::Locale;
use hc_i18n::zone_names::{self, ZoneFormats};
use hc_tz::UtcOffset;

use crate::error::{FormatError, FormatResult};
use crate::patterns::FormatContext;
use crate::value::ZoneInfo;

/// The locale a zone field reads its names in: the context's, or English
/// for the POSIX `C` vocabulary, whose names are English.
fn names_locale(context: &FormatContext<'_>) -> Locale {
    context
        .locale
        .copied()
        .unwrap_or_else(hc_i18n::names::english)
}

fn zone_formats(locale: Option<&Locale>) -> ZoneFormats {
    locale.map_or_else(zone_names::root_formats, zone_names::formats)
}

/// The ten digits a locale writes its offsets in: its numbering system's
/// where that is positional, else Latin.
fn digits(locale: Option<&Locale>) -> [char; 10] {
    const LATIN: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    locale
        .and_then(|locale| hc_i18n::NumberingSystem::for_locale(locale).digits())
        .copied()
        .unwrap_or(LATIN)
}

fn write_number<W: Write>(
    out: &mut W,
    value: u32,
    width: usize,
    digits: &[char; 10],
) -> fmt::Result {
    let mut buffer = [0u8; 10];
    let mut length = 0;
    let mut rest = value;
    loop {
        buffer[length] = (rest % 10) as u8;
        length += 1;
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    for _ in length..width {
        out.write_char(digits[0])?;
    }
    for place in buffer[..length].iter().rev() {
        out.write_char(digits[usize::from(*place)])?;
    }
    Ok(())
}

/// The offset by a locale's `hourFormat`, `+HH:mm;-HH:mm`: its positive
/// pattern or its negative one, the hours in two digits for the long form
/// and in as few as they need for the short one, the minutes left out of
/// the short form where they are zero, and seconds after the minutes, by
/// the same separator, where the offset has them.
fn write_hour_format<W: Write>(
    out: &mut W,
    hour_format: &str,
    offset: UtcOffset,
    long: bool,
    digits: &[char; 10],
) -> fmt::Result {
    let (positive, negative) = hour_format
        .split_once(';')
        .unwrap_or((hour_format, hour_format));
    let pattern = if offset.is_negative() {
        negative
    } else {
        positive
    };
    let hours_at = pattern.find('H').unwrap_or(pattern.len());
    let hours_end = pattern[hours_at..]
        .find(|c| c != 'H')
        .map_or(pattern.len(), |end| hours_at + end);
    let minutes_at = pattern[hours_end..]
        .find("mm")
        .map_or(pattern.len(), |at| hours_end + at);
    let separator = &pattern[hours_end..minutes_at];
    let suffix = pattern.get(minutes_at + 2..).unwrap_or("");
    out.write_str(&pattern[..hours_at])?;
    let two = long && hours_end - hours_at >= 2;
    write_number(out, offset.abs_hours(), if two { 2 } else { 1 }, digits)?;
    let (minutes, seconds) = (offset.abs_minutes(), offset.abs_seconds());
    if long || minutes != 0 || seconds != 0 {
        out.write_str(separator)?;
        write_number(out, minutes, 2, digits)?;
    }
    if seconds != 0 {
        out.write_str(separator)?;
        write_number(out, seconds, 2, digits)?;
    }
    out.write_str(suffix)
}

/// The localized GMT format, `O` and `OOOO`: the locale's `gmtFormat`
/// around its `hourFormat`, in its digits, and its `gmtZeroFormat` for a
/// zero offset in the short form; with no locale, `root.xml`'s, `GMT+9`
/// and `GMT+09:00`.
pub(crate) fn write_localized_gmt<W: Write>(
    out: &mut W,
    locale: Option<&Locale>,
    zone: ZoneInfo,
    long: bool,
) -> FormatResult<()> {
    let Some(offset) = zone.offset() else {
        return Err(FormatError::Unrepresentable("a zone field with no zone"));
    };
    let formats = zone_formats(locale);
    if offset.is_utc() && !long {
        out.write_str(formats.gmt_zero)?;
        return Ok(());
    }
    let (before, after) = formats.gmt.split_once("{0}").unwrap_or((formats.gmt, ""));
    out.write_str(before)?;
    write_hour_format(out, formats.hour, offset, long, &digits(locale))?;
    out.write_str(after)?;
    Ok(())
}

/// `z`…`zzzz`: the caller's name, else the specific non-location name,
/// else the localized GMT format.
pub(crate) fn write_specific<W: Write>(
    out: &mut W,
    context: &FormatContext<'_>,
    long: bool,
) -> FormatResult<()> {
    let given = if long {
        context.zone_name.or(context.zone_abbreviation)
    } else {
        context.zone_abbreviation.or(context.zone_name)
    };
    if let Some(name) = given {
        out.write_str(name)?;
        return Ok(());
    }
    #[cfg(feature = "zone-names")]
    if let Some(name) = names::specific(context, long) {
        out.write_str(name)?;
        return Ok(());
    }
    write_localized_gmt(out, context.locale, context.zone, long)
}

/// `v` and `vvvv`: the generic non-location name, else the generic
/// location format, else the localized GMT format.
pub(crate) fn write_generic<W: Write>(
    out: &mut W,
    context: &FormatContext<'_>,
    long: bool,
) -> FormatResult<()> {
    #[cfg(feature = "zone-names")]
    if names::generic(out, context, long)? || names::location(out, context)? {
        return Ok(());
    }
    write_localized_gmt(out, context.locale, context.zone, long)
}

/// `V`…`VVVV`.
pub(crate) fn write_zone_id<W: Write>(
    out: &mut W,
    context: &FormatContext<'_>,
    count: usize,
) -> FormatResult<()> {
    match count {
        1 => {
            #[cfg(feature = "zone-names")]
            if let Some(id) = context.zone_id.and_then(zone_names::zone_id) {
                out.write_str(id.short)?;
                return Ok(());
            }
            out.write_str("unk")?;
            Ok(())
        }
        2 => {
            let Some(id) = context.zone_id else {
                return Err(FormatError::Unrepresentable("a zone field with no zone"));
            };
            out.write_str(id)?;
            Ok(())
        }
        3 => write_city(out, context),
        _ => {
            #[cfg(feature = "zone-names")]
            if names::location(out, context)? {
                return Ok(());
            }
            write_localized_gmt(out, context.locale, context.zone, true)
        }
    }
}

/// `VVV`: the zone's exemplar city, else the unknown zone's.
fn write_city<W: Write>(out: &mut W, context: &FormatContext<'_>) -> FormatResult<()> {
    #[cfg(feature = "zone-names")]
    if let Some(given) = context.zone_id
        && zone_names::zone_id(given).is_some()
    {
        names::write_exemplar_city(out, &names_locale(context), given)?;
        return Ok(());
    }
    let formats = zone_formats(Some(&names_locale(context)));
    out.write_str(formats.unknown_city)?;
    Ok(())
}

#[cfg(feature = "zone-names")]
mod names {
    use core::fmt::Write;

    use hc_i18n::Locale;
    use hc_i18n::exemplar_cities;
    use hc_i18n::zone_names::{self, NameLength, NameType, ZoneId, ZoneName};

    use super::{names_locale, zone_formats};
    use crate::error::FormatResult;
    use crate::patterns::FormatContext;

    const fn length(long: bool) -> NameLength {
        if long {
            NameLength::Long
        } else {
            NameLength::Short
        }
    }

    /// The instant of the reading in minutes since 1970-01-01 00:00 UTC:
    /// the local reading less its offset, or the reading itself where the
    /// context states no offset.
    fn utc_minutes(context: &FormatContext<'_>) -> i64 {
        const UNIX_EPOCH_RD: i64 = 719_163;
        let time = context.date_time.time;
        let local = (context.date_time.day.0 - UNIX_EPOCH_RD) * 1_440
            + i64::from(time.hour()) * 60
            + i64::from(time.minute());
        let offset = context
            .zone
            .offset()
            .map_or(0, |offset| offset.seconds() / 60);
        local - i64::from(offset)
    }

    fn zone(context: &FormatContext<'_>) -> Option<ZoneId> {
        context.zone_id.and_then(zone_names::zone_id)
    }

    /// The specific non-location name: the zone's own, else its
    /// metazone's at the instant; `None` where the context does not say
    /// whether the reading is daylight time.
    ///
    /// A standard reading takes UTS #35's type fallback: where the names
    /// have no daylight one, the generic name, else the standard one. A
    /// daylight reading takes a daylight name or none, and so falls to the localized
    /// GMT format: the fallback's premise, that "the metazone doesn't
    /// require daylight support", does not hold of a zone keeping daylight
    /// time, and its standard name would state another offset. So
    /// `Europe/London` in summer is *British Summer Time* in `zzzz`, from
    /// the zone's own name, and *GMT+1* in `z` in `en`, whose `GMT`
    /// metazone has no daylight name, rather than *GMT*. ICU4J's
    /// `TimeZoneFormat.formatSpecific` asks for the daylight name alone
    /// there too (`icu-zone-format-sources`).
    pub(super) fn specific(context: &FormatContext<'_>, long: bool) -> Option<&'static str> {
        let daylight = context.zone_daylight?;
        let id = zone(context)?;
        let locale = names_locale(context);
        let length = length(long);
        let metazone = || zone_names::metazone_at(id.canonical, utc_minutes(context));
        let found = if daylight {
            zone_names::own_zone_name(&locale, id.canonical, length, NameType::Daylight).or_else(
                || zone_names::metazone_name(&locale, metazone()?, length, NameType::Daylight),
            )
        } else {
            zone_names::with_type_fallback(
                |kind| zone_names::own_zone_name(&locale, id.canonical, length, kind),
                NameType::Standard,
            )
            .or_else(|| {
                let metazone = metazone()?;
                zone_names::with_type_fallback(
                    |kind| zone_names::metazone_name(&locale, metazone, length, kind),
                    NameType::Standard,
                )
            })
        };
        found.map(|found| found.name)
    }

    /// The generic non-location name, written where there is one: the
    /// zone's own, else its metazone's, which UTS #35 qualifies by the
    /// zone's country or city where the zone is not the metazone's
    /// preferred zone for the locale's country (*Pacific Time (Canada)*
    /// for Vancouver in `en`).
    pub(super) fn generic<W: Write>(
        out: &mut W,
        context: &FormatContext<'_>,
        long: bool,
    ) -> FormatResult<bool> {
        let Some(id) = zone(context) else {
            return Ok(false);
        };
        let locale = names_locale(context);
        let length = length(long);
        if let Some(found) = zone_names::with_type_fallback(
            |kind| zone_names::own_zone_name(&locale, id.canonical, length, kind),
            NameType::Generic,
        ) {
            out.write_str(found.name)?;
            return Ok(true);
        }
        let Some(metazone) = zone_names::metazone_at(id.canonical, utc_minutes(context)) else {
            return Ok(false);
        };
        let Some(ZoneName { name, .. }) = zone_names::with_type_fallback(
            |kind| zone_names::metazone_name(&locale, metazone, length, kind),
            NameType::Generic,
        ) else {
            return Ok(false);
        };
        let country = locale
            .region()
            .or_else(|| zone_names::likely_region(locale.language()))
            .unwrap_or("001");
        let preferred = zone_names::preferred_zone(metazone, country)
            .or_else(|| zone_names::preferred_zone(metazone, "001"));
        if preferred == Some(id.canonical) {
            out.write_str(name)?;
            return Ok(true);
        }
        let formats = zone_formats(Some(&locale));
        // `{1} ({0})`: the metazone's name in `{1}`, the place in `{0}`.
        write_substituted(out, formats.fallback, |out, index| {
            if index == 1 {
                out.write_str(name)?;
                Ok(())
            } else if !id.region.is_empty()
                && zone_names::preferred_zone(metazone, id.region) == Some(id.canonical)
            {
                write_country(out, &locale, id.region)
            } else {
                write_exemplar_city(out, &locale, context.zone_id.unwrap_or(id.canonical))
            }
        })?;
        Ok(true)
    }

    /// A pattern with `{0}` and `{1}` filled by `fill`.
    fn write_substituted<W: Write>(
        out: &mut W,
        pattern: &str,
        mut fill: impl FnMut(&mut W, u8) -> FormatResult<()>,
    ) -> FormatResult<()> {
        let mut rest = pattern;
        while let Some(at) = rest.find('{') {
            out.write_str(&rest[..at])?;
            let after = &rest[at..];
            if let Some(tail) = after.strip_prefix("{0}") {
                fill(out, 0)?;
                rest = tail;
            } else if let Some(tail) = after.strip_prefix("{1}") {
                fill(out, 1)?;
                rest = tail;
            } else {
                out.write_char('{')?;
                rest = &after[1..];
            }
        }
        out.write_str(rest)?;
        Ok(())
    }

    /// The generic location format, written where the zone has a region:
    /// the locale's `regionFormat` with the country's name, where the zone
    /// is its region's only zone or its primary zone, else with the zone's
    /// exemplar city.
    pub(super) fn location<W: Write>(
        out: &mut W,
        context: &FormatContext<'_>,
    ) -> FormatResult<bool> {
        let Some(id) = zone(context) else {
            return Ok(false);
        };
        if id.region.is_empty() {
            return Ok(false);
        }
        let locale = names_locale(context);
        let formats = zone_formats(Some(&locale));
        let by_country = zone_names::zones_in_region(id.region) == 1
            || zone_names::primary_zone(id.region) == Some(id.canonical);
        write_substituted(out, formats.region, |out, _| {
            if by_country {
                write_country(out, &locale, id.region)
            } else {
                write_exemplar_city(out, &locale, context.zone_id.unwrap_or(id.canonical))
            }
        })?;
        Ok(true)
    }

    /// A country by its short name, else its name, else its code, as UTS
    /// #35's composition has it.
    fn write_country<W: Write>(out: &mut W, locale: &Locale, region: &str) -> FormatResult<()> {
        match hc_i18n::territories::territory_name(locale, region) {
            Some(found) => {
                let short = hc_i18n::territories::short_name(found.tag, region);
                out.write_str(short.unwrap_or(found.name))?;
            }
            None => out.write_str(region)?,
        }
        Ok(())
    }

    /// The zone's exemplar city in the locale, else English's, else the
    /// last field of its identifier.
    pub(super) fn write_exemplar_city<W: Write>(
        out: &mut W,
        locale: &Locale,
        zone: &str,
    ) -> FormatResult<()> {
        let candidates = zone_names::zone_id(zone)
            .map(|id| [id.canonical, id.iana])
            .unwrap_or(["", ""]);
        let found = core::iter::once(zone)
            .chain(candidates)
            .filter(|name| !name.is_empty())
            .find_map(|name| exemplar_cities::zone_index(name).map(|index| (index, name)));
        match found {
            Some((index, name)) => write!(out, "{}", city(locale, index, name).name)?,
            None => write!(out, "{}", exemplar_cities::CityName::Derived(zone))?,
        }
        Ok(())
    }

    #[cfg(feature = "localized-zone-names")]
    fn city<'a>(locale: &Locale, index: usize, zone: &'a str) -> exemplar_cities::ExemplarCity<'a> {
        exemplar_cities::exemplar_city(locale, index, zone)
    }

    #[cfg(not(feature = "localized-zone-names"))]
    fn city<'a>(_: &Locale, index: usize, zone: &'a str) -> exemplar_cities::ExemplarCity<'a> {
        exemplar_cities::english_city(index, zone)
    }
}
