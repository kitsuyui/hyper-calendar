// Type declarations for hyper-calendar.js, written by hand beside it.
//
// The line formats are the crate README's, column for column; the tests
// beside this file hold the binding's column lists to the README's tables.

/** What `load()` accepts: the module in any form a page has it. */
export type ModuleSource =
  | WebAssembly.Module
  | WebAssembly.Instance
  | WebAssembly.WebAssemblyInstantiatedSource
  | ArrayBuffer
  | Uint8Array
  | Response
  | URL
  | string;

export interface LoadOptions {
  /**
   * The buffer a text-writing export is first offered, in bytes; 64 KiB
   * unless said otherwise. An answer that does not fit is measured and
   * read again, so this is a cost, not a limit.
   */
  initialCapacity?: number;
}

/** A sentinel the module returns, with the name the binding throws it under. */
export interface Sentinel {
  /** The `i64` value. */
  readonly code: bigint;
  /** Its `HC_ERR_*` name. */
  readonly constant: string;
  /** The name `HcError.name` carries. */
  readonly name: SentinelName;
  readonly message: string;
}

export type SentinelName =
  | "invalid-date"
  | "out-of-range"
  | "buffer-too-small"
  | "no-data"
  | "null-pointer"
  | "unknown"
  | "not-utf8"
  | "malformed";

/** The names the binding throws on its own account. */
export type BindingErrorName =
  | "not-exported"
  | "unsafe-integer"
  | "allocation-failed"
  | "unrecognised-sentinel";

/** The Cargo features the exports are gated by. */
export type Feature = "civil" | "calendars" | "holiday" | "seasons" | "deep-time" | "tz" | "sky" | "orbital";

/** One method of `HyperCalendar` and the export it stands for. */
export interface MethodEntry {
  readonly method: keyof HyperCalendar & string;
  readonly export: string;
  /** The feature the export needs, or `null` for the three in every build. */
  readonly feature: Feature | null;
}

export const SENTINELS: ReadonlyArray<Sentinel>;
export const METHODS: ReadonlyArray<MethodEntry>;
export const COLUMNS: {
  readonly describeDay: ReadonlyArray<string>;
  readonly holidaysInYear: ReadonlyArray<string>;
  readonly holidaysOn: ReadonlyArray<string>;
  readonly term: ReadonlyArray<string>;
  readonly deepTime: ReadonlyArray<string>;
  readonly sky: ReadonlyArray<string>;
  readonly skyEvent: ReadonlyArray<string>;
  readonly orbit: ReadonlyArray<string>;
  readonly orbitSeries: ReadonlyArray<string>;
};
export const UNITS: readonly Unit[];
export const NATIVE: "native";
export const GEOLOGIC_RANKS: ReadonlyArray<GeologicRank>;

/**
 * What the module refused, or what the binding could not do. `name` is the
 * sentinel's name or one of the binding's own; `code` the sentinel as a
 * `BigInt`, or `null` for the binding's own; `constant` its `HC_ERR_*`
 * name, or `null`; `export` the export concerned, where known.
 */
export class HcError extends Error {
  constructor(
    name: SentinelName | BindingErrorName,
    details: { code?: bigint | null; constant?: string | null; export?: string | null; message: string },
  );
  name: SentinelName | BindingErrorName;
  code: bigint | null;
  constant: string | null;
  export: string | null;
}

/** Where a calendar stands on a day. */
export type Standing = "in-use" | "proleptic" | "extended" | "unrecorded";

/**
 * Which civil day names a calendar day that does not begin at midnight:
 * `start` for the one it begins on (the Julian Day, which begins at noon on
 * the civil day it is named after), `end` for the one it ends on (the
 * Hebrew day, which begins at the sunset of the evening before).
 */
export type DayNamedBy = "start" | "end";

/** One line of `hc_describe_day`: a fixed day in one calendar. */
export interface DescribedDay {
  /** The calendar's identifier: `gregory`, `chinese`, `japanese`, ... */
  id: string;
  /** Its English name. */
  name: string;
  /** The era code (`reiwa`, `AD`, `AH`, ...), or `null` for a calendar without eras. */
  era: string | null;
  /** The era's name in the locale, or `null`. */
  eraLabel: string | null;
  /** The year as the calendar counts it, or `null` on a refusal. */
  year: number | null;
  /** The month's ordinal from 1, or `null` for a calendar without months or on a refusal. */
  month: number | null;
  /** Whether the month is intercalary. */
  leapMonth: boolean;
  /** The month's name in the locale, or the calendar's own name for it, or `null`. */
  monthLabel: string | null;
  /** The day of the month, or `null`. */
  day: number | null;
  /** Whether the day is a repeated one. */
  leapDay: boolean;
  /** The calendar's extra fields, `{ baktun: "13", katun: "0", ... }`, as the module writes them. */
  extras: Record<string, string>;
  /** `null` when the day converted; otherwise the refusal's code and name. */
  error: { code: number; name: string } | null;
  /** The standing, or `null` on a refusal. */
  standing: Standing | null;
  /** Where the calendar's day begins: `midnight`, `noon`, `sunset`, `sunrise` or `local-time HH:MM:SS`. */
  dayBoundary: string;
  /** The date as the locale writes it — 令和8年9月21日, 癸卯年闰二月初一 — or `null` on a refusal. */
  formatted: string | null;
  /** The tag of the locale data that answered: `ja`, `he`, `und`. */
  localeUsed: string;
  /** Which civil day names the day, or `null` for a day that begins at midnight. */
  dayNamedBy: DayNamedBy | null;
}

/** A unit of a calendar `calendarUnits` walks, largest first. */
export type Unit = "era" | "year" | "month" | "day";

/** One span of `hc_calendar_units`: a run of fixed days that is one unit, or that the calendar refuses. */
export interface CalendarUnit {
  /** The first fixed day of the span. */
  start: number;
  /** The day after the span's last, so that `end - start` is its length. */
  end: number;
  /** The span's label in the locale — 令和元年, `Adar I`, 閏二月, 初四 — or `null` on a refusal. */
  label: string | null;
  /** Whether the unit is intercalary: a leap year, a leap month, a repeated day. */
  leap: boolean;
  /** The standing of the span's first day, or `null` on a refusal. */
  standing: Standing | null;
  /** `null` for a unit; otherwise the calendar's refusal of every day in the span. */
  error: { code: number; name: string } | null;
  /** The tag of the locale data that answered. */
  localeUsed: string;
}

/** One row of `hc_calendars`. */
export interface CalendarEntry {
  /** The calendar's identifier. */
  id: string;
  /** What the locale calls the calendar, or `null` where it has no name for it. */
  name: string | null;
  /** Its English name. */
  englishName: string;
  /** The earliest fixed day it converts, or `null` where unbounded. */
  earliest: number | null;
  /** The latest fixed day it converts, or `null` where unbounded. */
  latest: number | null;
  /** Whether its dates carry an era. */
  hasEra: boolean;
  /** Whether its dates carry a year. */
  hasYear: boolean;
  /** Whether it has months. */
  hasMonth: boolean;
  /** Whether its dates carry a day of the month. */
  hasDay: boolean;
  /** The languages its sources are written in, as BCP 47 tags, primary first; empty for a day count, a proposal or the Gregorian family. */
  nativeLocales: string[];
  /** Its standing on the day asked about. */
  standing: Standing;
}

/** How far one step of a Gregorian adoption reached. */
export type AdoptionScope = "civil" | "ecclesiastical" | "partial";

/** One row of `hc_gregorian_adoption`: one step of one country's adoption. */
export interface GregorianAdoption {
  /** The last day of the old reckoning, as a fixed day. */
  lastOldDay: number;
  /** The first day of the new reckoning, as a fixed day: the next day. */
  firstDay: number;
  /** The registry identifier of the calendar kept until then: `julian`, `japanese-tenpo`, `dangi`, `chinese`, `islamic-umalqura`, `rumi` or `swedish-1700`. */
  oldCalendar: string;
  /** `civil` for the whole polity's civil calendar; `partial` for part of the country, some purposes or part of the calendar; `ecclesiastical` for a church's alone. */
  scope: AdoptionScope;
  /** The instrument behind the step, with its date, and whether it was read. */
  source: string;
  /** The registry identifier of the calendar kept from then: `gregory`, but `swedish-1700` and then `julian` for Sweden's steps of 1700 and 1712. */
  newCalendar: string;
  /** Who took the step, in English. */
  polity: string;
}

/** One row of `hc_locales`. */
export interface LocaleEntry {
  /** The BCP 47 tag. */
  tag: string;
  /** The language's name in English. */
  englishName: string;
  /** The language's name in itself. */
  nativeName: string;
  /** Whether the locale's own data names the Gregorian months. */
  gregorianMonths: boolean;
  /** Whether it names the weekdays. */
  weekdays: boolean;
  /** Whether it names the Gregorian eras. */
  gregorianEras: boolean;
  /** The calendars it has vocabulary of its own for, beyond the shared Gregorian months. */
  calendars: string[];
}

export type HolidayKind = "public" | "bank" | "religious" | "observance" | "school" | "workday";
export type Confidence = "exact" | "approximate";

/** One line of `hc_holidays_in_year`. */
export interface HolidayInYear {
  /** The ISO 8601 date. */
  date: string;
  name: string;
  localName: string | null;
  kind: HolidayKind;
  confidence: Confidence;
  /** Whether this is a substitute day. */
  substitute: boolean;
  /** The ISO 8601 date a substitute stands in for, or `null`. */
  observedFor: string | null;
}

/** One line of `hc_holidays_on`: one entry of one table on one day. */
export interface HolidayOn {
  /** The table's identifier: `JP`, `XNYS`, `christian-western`, `un-days`. */
  table: string;
  /** Its English name. */
  tableName: string;
  name: string;
  localName: string | null;
  /** `gap` is a holiday the table could not place in the day's year. */
  kind: HolidayKind | "gap";
  /** `null` for a gap. */
  confidence: Confidence | null;
  /** The instrument the rule cites, or `null`. */
  source: string | null;
  substitute: boolean;
  /** The fixed day a substitute stands in for, or `null`. */
  observedFor: number | null;
}

/** The one line of `hc_term_in_effect` or `hc_pentad_in_effect`. */
export interface TermInEffect {
  /** 春分 at 0 through 驚蟄 at 23 for a term; the first pentad of 春分 at 0 through 71 for a pentad. */
  index: number;
  chineseName: string;
  japaneseName: string;
  /** The fixed day it began at the meridian. */
  begins: number;
  /** The last fixed day before the next begins. */
  ends: number;
  /** The authority for the Chinese names, or the text the pentad names come from. */
  chineseAuthority: string;
  /** The authority for the Japanese names, or the text the pentad names come from. */
  japaneseAuthority: string;
}

/** A meridian: a name, or a longitude in degrees east of Greenwich. */
export type Meridian =
  | "universal"
  | "japan"
  | "china"
  | "korea"
  | "india"
  | "china-before-1929"
  | (string & {})
  | number;

export type GeologicRank = "eon" | "era" | "period" | "epoch" | "age";
export type DeepTimeKind =
  | "moment"
  | "cosmic-epoch"
  | "cosmic-event"
  | "future-era"
  | GeologicRank
  | "archaeological";
export type DeepTimeUnit =
  | "seconds-since-big-bang"
  | "seconds-before-present"
  | "megayears-before-present"
  | "years-before-1950"
  | "log10-years-from-now";

/** One bound of a deep-time entry. */
export interface DeepTimeBound {
  value: number;
  /** The standard uncertainty. */
  stdDev: number;
  /** The significant figures claimed, or `null` where the table claims none. */
  figures: number | null;
  /** Whether the chart marks it `~`. */
  approximate: boolean;
}

/** One line of any deep-time export. */
export interface DeepTimeRow {
  kind: DeepTimeKind;
  name: string;
  /** The interval one rank up, or the region of an archaeological period, or `null`. */
  scope: string | null;
  /** The older bound, or `null` where the chronology has no figure. */
  start: DeepTimeBound | null;
  /** The younger bound; the same as `start` for a point in time. */
  end: DeepTimeBound | null;
  /** What the bounds are in. */
  unit: DeepTimeUnit;
  description: string | null;
  source: string;
  /** The geological chart's own name for an interval in the locale's language, or `null`: always `null` for the cosmic, future and archaeological rows. */
  localisedName: string | null;
}

/** Which source ΔT was answered from. */
export type DeltaTRegime = "observed" | "predicted" | "fitted" | "extrapolated";

/** The one line of `hc_sky_at`: the Sun and the Moon at an instant. */
export interface Sky {
  /** The Sun's apparent ecliptic longitude in degrees, 0 at the March equinox. */
  sunLongitude: number;
  /** The Earth–Sun distance in astronomical units. */
  sunDistance: number;
  /** The Moon's apparent ecliptic longitude in degrees. */
  moonLongitude: number;
  /** The Moon's ecliptic latitude in degrees, positive north. */
  moonLatitude: number;
  /** The Earth–Moon distance in kilometres, centre to centre. */
  moonDistance: number;
  /** The Moon's elongation from the Sun in degrees: 0 at new moon, 90, 180, 270. */
  elongation: number;
  /** The lit fraction of the Moon's disc, 0 to 1. */
  illuminatedFraction: number;
  /** The last new moon before the instant, as POSIX seconds. */
  previousNewMoon: number;
  /** The first new moon at or after the instant, as POSIX seconds. */
  nextNewMoon: number;
  /** `TT − UT1` at the instant, in seconds. */
  deltaT: number;
  deltaTRegime: DeltaTRegime;
  /** The series behind the line, as `hc-astro` names them. */
  source: string;
}

/** The name of a moon phase, as `hc_moon_phases_between` writes it. */
export type MoonPhaseName = "new" | "first-quarter" | "full" | "last-quarter";

/** One line of `hc_solar_terms_between` or `hc_moon_phases_between`. */
export interface SkyEvent {
  /** The Sun's longitude that defines the term (0 for 春分, in steps of 15), or the elongation that defines the phase (0, 90, 180, 270). */
  angle: number;
  /** The instant as POSIX seconds, rounded down. */
  instant: number;
  /** The term's name in traditional Chinese, or the phase's fixed English name. */
  name: string | MoonPhaseName;
  /** The term's name in Japanese; `null` for a phase. */
  japaneseName: string | null;
}

/**
 * The one line of `hc_orbit_at`: Earth's orbital elements and the June
 * insolation at 65° N at an epoch, from Berger's 1978 series. Each spread
 * is the measured disagreement with Berger & Loutre's 1991 solution over
 * the tier of the span the epoch falls in, not a Gaussian width.
 */
export interface Orbit {
  /** The eccentricity *e* of Earth's orbit. */
  eccentricity: number;
  eccentricitySpread: number;
  /** The obliquity of the ecliptic in degrees. */
  obliquity: number;
  /** In degrees. */
  obliquitySpread: number;
  /** ϖ, the longitude of perihelion from the moving equinox in degrees, 0 to 360; about 102 at present. */
  longitudeOfPerihelion: number;
  /** In degrees; 180 where the eccentricity is smaller than the precession's spread. */
  longitudeOfPerihelionSpread: number;
  /** *e* sin ϖ, positive when perihelion falls in northern summer. */
  climaticPrecession: number;
  climaticPrecessionSpread: number;
  /** The daily mean insolation at 65° N at the June solstice, in W/m², for `solarConstant`. */
  insolation65NJune: number;
  /** The solar constant the insolation was computed with, in W/m². */
  solarConstant: number;
  /** The series and the constant, by name. */
  source: string;
}

/** One line of `hc_orbit_series`: `Orbit` at an epoch. */
export interface OrbitSample extends Orbit {
  /** The epoch, in years before 1950; negative for the future. */
  yearsBefore1950: number;
}

/**
 * An instantiated module, one method per export. Every `i64` result is a
 * number; an `i64` argument may be a number or a `BigInt`. A method whose
 * export the build lacks throws `HcError` `not-exported` when called.
 */
export class HyperCalendar {
  constructor(instance: WebAssembly.Instance | { exports: Record<string, unknown> }, options?: LoadOptions);

  /** The instance's raw exports. */
  readonly exports: Record<string, unknown>;
  /** The module's linear memory. */
  readonly memory: WebAssembly.Memory;

  /** Whether a method's export is in this build. */
  has(method: keyof HyperCalendar & string): boolean;
  /** The features at least one of whose exports this build carries. */
  layers(): Feature[];

  /** `hc_alloc`: a block of linear memory, as a pointer. */
  alloc(len: number): number;
  /** `hc_free`: return a block with the length it was allocated with. */
  free(pointer: number, len: number): void;
  /** `hc_version`: the library version. */
  version(): string;

  /** `hc_gregorian_to_fixed`. */
  gregorianToFixed(year: number | bigint, month: number, day: number): number;
  /** `hc_gregorian_year`. */
  gregorianYear(fixed: number | bigint): number;
  /** `hc_gregorian_month`: 1 through 12. */
  gregorianMonth(fixed: number | bigint): number;
  /** `hc_gregorian_day`. */
  gregorianDay(fixed: number | bigint): number;
  /** `hc_weekday`: Monday = 1 through Sunday = 7. */
  weekday(fixed: number | bigint): number;
  /** `hc_day_of_year`: from 1. */
  dayOfYear(fixed: number | bigint): number;
  /** `hc_is_leap_year`. */
  isLeapYear(fixed: number | bigint): boolean;
  /** `hc_fixed_from_unix`: the UTC day a POSIX timestamp falls on. */
  fixedFromUnix(unixSeconds: number | bigint): number;
  /**
   * `hc_tai_minus_utc`: `TAI - UTC` in whole seconds. `strict` refuses to
   * answer before 1961 and past the announced leap-second table with
   * `no-data`; otherwise the last published value holds.
   */
  taiMinusUtc(unixSeconds: number | bigint, strict?: boolean): number;
  /** `hc_day_has_leap_second`. */
  dayHasLeapSecond(unixSeconds: number | bigint): boolean;
  /** `hc_unix_from_fixed`: midnight UTC on a fixed day. */
  unixFromFixed(fixed: number | bigint): number;
  /** `hc_format_iso_date`. */
  formatIsoDate(fixed: number | bigint): string;
  /** `hc_parse_iso_date`; text that is not a date is `invalid-date`. */
  parseIsoDate(text: string): number;

  /**
   * `hc_describe_day`: the day in every registered calendar, in registry
   * order. `locale` is a BCP 47 tag, `und` unless given, as the module.
   */
  describeDay(fixed: number | bigint, locale?: string): DescribedDay[];
  /** The days from `from` up to but not including `to` as one calendar's eras, years, months or days. */
  calendarUnits(id: string, unit: Unit | number, from: number | bigint, to: number | bigint, locale?: string): CalendarUnit[];
  /** Every registered calendar, with what the locale calls it and its standing on `today`. */
  calendars(today: number | bigint, locale?: string): CalendarEntry[];
  /** Every locale the module carries. */
  locales(): LocaleEntry[];
  /**
   * `hc_first_day_of_week`: the ISO weekday the locale's week begins on,
   * Monday = 1 through Sunday = 7, by CLDR 48's week data; `und` unless
   * given, and a tag that does not parse is `und`, Monday.
   */
  firstDayOfWeek(locale?: string): number;
  /** `hc_gregorian_adoption`: the steps by which a country adopted the Gregorian calendar, by ISO 3166-1 alpha-2 code; none for a code the module does not know. */
  gregorianAdoption(region: string): GregorianAdoption[];

  /** `hc_holiday_is_day_off`; `region` may be empty. A code naming no table is `unknown`. */
  holidayIsDayOff(code: string, region: string, fixed: number | bigint): boolean;
  /** `hc_holidays_in_year`. */
  holidaysInYear(code: string, region: string, year: number | bigint): HolidayInYear[];
  /** `hc_holiday_codes`: countries, then exchanges, traditions and the international sets. */
  holidayCodes(): string[];
  /** `hc_holidays_on`: every entry on one day across every table, in `holidayCodes()` order. A day with no Gregorian year is `out-of-range`. */
  holidaysOn(fixed: number | bigint): HolidayOn[];

  /** `hc_term_in_effect`; a meridian nobody knows is `unknown`. */
  termInEffect(fixed: number | bigint, meridian?: Meridian): TermInEffect;
  /** `hc_pentad_in_effect`. */
  pentadInEffect(fixed: number | bigint, meridian?: Meridian): TermInEffect;

  /** `hc_place_years_ago`; a value the crate refuses is `out-of-range`. `locale` names the geologic rows, `und` unless given. */
  placeYearsAgo(yearsAgo: number, stdDevYears?: number, locale?: string): DeepTimeRow[];
  /** `hc_cosmic_events`. */
  cosmicEvents(locale?: string): DeepTimeRow[];
  /** `hc_geologic_intervals`; the rank by name or by number from 0. */
  geologicIntervals(rank: GeologicRank | number, locale?: string): DeepTimeRow[];

  /** `hc_fixed_from_unix_in_zone`; a zone nobody knows is `unknown`. */
  fixedFromUnixInZone(unixSeconds: number | bigint, zone: string): number;
  /** `hc_unix_from_fixed_in_zone`. */
  unixFromFixedInZone(fixed: number | bigint, zone: string): number;
  /** `hc_zone_load`; bytes that are not TZif are `malformed`. */
  loadZone(name: string, tzif: Uint8Array | ArrayBuffer): void;

  /** `hc_sky_at`; an instant outside −1000 through 3000 is `out-of-range`. */
  skyAt(unixSeconds: number | bigint): Sky;
  /** `hc_solar_terms_between`: the terms in `[from, to)`; a span outside the era or over 400 years is `out-of-range`. */
  solarTermsBetween(fromUnix: number | bigint, toUnix: number | bigint): SkyEvent[];
  /** `hc_moon_phases_between`: the phases in `[from, to)`, as `solarTermsBetween`. */
  moonPhasesBetween(fromUnix: number | bigint, toUnix: number | bigint): SkyEvent[];

  /** `hc_orbit_at`; an epoch beyond a million years either side of 1950 is `out-of-range`. */
  orbitAt(yearsBefore1950: number): Orbit;
  /**
   * `hc_orbit_series`: `orbitAt` at `from`, `from + step`, ... up to `to`, at most
   * 10 000 samples; a step that is not positive, an end off the span or more
   * samples is `out-of-range`, and a `to` before `from` is empty.
   */
  orbitSeries(fromYearsBefore1950: number, toYearsBefore1950: number, stepYears: number): OrbitSample[];
}

/**
 * Instantiate the module and bind it. `source` may also be a promise of
 * any accepted form.
 */
export function load(source: ModuleSource | Promise<ModuleSource>, options?: LoadOptions): Promise<HyperCalendar>;
