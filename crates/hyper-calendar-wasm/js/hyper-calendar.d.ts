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
export type Feature =
  | "civil"
  | "timestamps"
  | "calendars"
  | "holiday"
  | "seasons"
  | "deep-time"
  | "tz"
  | "sky"
  | "orbital"
  | "planetary"
  | "relativity";

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
  readonly calendarUnits: ReadonlyArray<string>;
  readonly calendars: ReadonlyArray<string>;
  readonly calendarList: ReadonlyArray<string>;
  readonly locales: ReadonlyArray<string>;
  readonly gregorianAdoption: ReadonlyArray<string>;
  readonly holidaysInYear: ReadonlyArray<string>;
  readonly holidaysOn: ReadonlyArray<string>;
  readonly term: ReadonlyArray<string>;
  readonly deepTime: ReadonlyArray<string>;
  readonly sky: ReadonlyArray<string>;
  readonly skyEvent: ReadonlyArray<string>;
  readonly orbit: ReadonlyArray<string>;
  readonly orbitSeries: ReadonlyArray<string>;
  readonly tai64: ReadonlyArray<string>;
  readonly gnssWeek: ReadonlyArray<string>;
  readonly taiInstant: ReadonlyArray<string>;
  readonly glonassDate: ReadonlyArray<string>;
  readonly oleAutomation: ReadonlyArray<string>;
  readonly excel1900Day: ReadonlyArray<string>;
  readonly panchanga: ReadonlyArray<string>;
  readonly marriageAugury: ReadonlyArray<string>;
  readonly holidayTables: ReadonlyArray<string>;
  readonly lectionary: ReadonlyArray<string>;
  readonly zones: ReadonlyArray<string>;
  readonly value: ReadonlyArray<string>;
  readonly solarTime: ReadonlyArray<string>;
  readonly solarEvent: ReadonlyArray<string>;
  readonly marsTime: ReadonlyArray<string>;
  readonly missions: ReadonlyArray<string>;
  readonly bodies: ReadonlyArray<string>;
  readonly bodyTime: ReadonlyArray<string>;
  readonly properTime: ReadonlyArray<string>;
  readonly gravitationalDilation: ReadonlyArray<string>;
  readonly gravitatingBodies: ReadonlyArray<string>;
  readonly utcFromTai: ReadonlyArray<string>;
  readonly tai64PosixPlus10: ReadonlyArray<string>;
  readonly uuidTimestamp: ReadonlyArray<string>;
  readonly ntpResolve: ReadonlyArray<string>;
  readonly uuidTimestampEncode: ReadonlyArray<string>;
  readonly ntpEncode: ReadonlyArray<string>;
  readonly fatDecode: ReadonlyArray<string>;
  readonly fatEncode: ReadonlyArray<string>;
  readonly epoch: ReadonlyArray<string>;
  readonly ttFromEpoch: ReadonlyArray<string>;
  readonly circadDate: ReadonlyArray<string>;
  readonly horizons: ReadonlyArray<string>;
  readonly solarCrossing: ReadonlyArray<string>;
  readonly hinduLunarDate: ReadonlyArray<string>;
  readonly suryaSiddhanta: ReadonlyArray<string>;
  readonly crescent: ReadonlyArray<string>;
  readonly decan: ReadonlyArray<string>;
  readonly hjdTt: ReadonlyArray<string>;
  readonly hjdUtc: ReadonlyArray<string>;
  readonly ttBipm: ReadonlyArray<string>;
  readonly namingPeriod: ReadonlyArray<string>;
  readonly asianDay: ReadonlyArray<string>;
  readonly holyYear: ReadonlyArray<string>;
  readonly commonWorship: ReadonlyArray<string>;
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

/** The crates whose calendars the module registers. */
export type CalendarCrate =
  | "hc-calendars-solar"
  | "hc-calendars-lunar"
  | "hc-calendars-equinox"
  | "hc-calendars-indic"
  | "hc-calendars-regional";

/** One row of `hc_calendar_list`. */
export interface CalendarListEntry {
  /** The calendar's identifier. */
  id: string;
  /** What the locale calls the calendar, as `CalendarEntry.name` has it, or `null` where it has no name for it. */
  name: string | null;
  /** Its English name. */
  englishName: string;
  /** The tag of the locale data the name came from, or `null` with the name. */
  localeUsed: string | null;
  /** The crate that registers it. */
  crate: CalendarCrate | null;
  /** The languages its sources are written in, as `CalendarEntry.nativeLocales` has them: BCP 47 tags, primary first, empty where there are none. A page that puts a reader's own calendars first matches these against the reader's language. */
  nativeLocales: string[];
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
 * The one line of `hc_mars_time`: Mars at an instant and an east
 * longitude, from the Mars24 restatement of Allison and McEwen.
 */
export interface MarsTime {
  /** The Mars Sol Date. */
  marsSolDate: number;
  /** Coordinated Mars Time, `HH:MM:SS` on the 24-hour Martian clock, truncated. */
  mtc: string;
  /** Coordinated Mars Time in decimal Martian hours. */
  mtcHours: number;
  /** Local mean solar time at the longitude, `HH:MM:SS`. */
  lmst: string;
  lmstHours: number;
  /** Local true solar time at the longitude, `HH:MM:SS`: what a sundial reads. */
  ltst: string;
  ltstHours: number;
  /** True minus mean solar time, in Martian minutes. */
  equationOfTimeMinutes: number;
  /** The areocentric solar longitude `Ls`, in degrees. */
  solarLongitude: number;
  /** The Mars year under the Clancy convention, year 1 from 1955-04-11. */
  marsYear: number;
  /** The Darian date at Airy-0: year, month 1 to 24, sol of the month, and their names. */
  darian: { year: number; month: number; sol: number; monthName: string; solOfWeek: string };
  /** The series and the constants, by name. */
  source: string;
}

/** Where a mission clock's midnight is. */
export type MissionClock = "local-mean-solar-time" | "local-true-solar-time-at-landing";

/** One line of `hc_missions`. */
export interface Mission {
  /** Lower case, a hyphen for each space: `viking-1`. */
  id: string;
  name: string;
  /** The landing instant, UTC, as the table writes it. */
  landingUtc: string;
  /** The landing instant as a POSIX timestamp. */
  landingUnix: number;
  /** 0 or 1, the number of the landing sol; `null` where no convention was published. */
  landingSol: number | null;
  /** `null` where no convention was published. */
  clock: MissionClock | null;
  /** The east longitude the clock was built on; `null` where no convention was published. */
  clockEastLongitude: number | null;
  /** The achieved site's east longitude. */
  siteEastLongitude: number;
  /** Whether the operators published the convention. */
  published: boolean;
  note: string;
  source: string;
}

/** What kind of object a body is. */
export type BodyKind = "star" | "planet" | "dwarf-planet" | "moon";

/** Whether a clock's zero point is a standard or a convention this library declares. */
export type ZeroPoint = "standard" | "convention";

/** One line of `hc_bodies`. */
export interface Body {
  /** The name in lower case: `titan`. */
  id: string;
  name: string;
  kind: BodyKind;
  /** The identifier of the body it orbits; `null` for the Sun. */
  primary: string | null;
  /** The sidereal rotation period in hours, negative for a retrograde rotator. */
  siderealRotationHours: number;
  /** The solar day in SI seconds; `null` for the Sun. */
  solarDaySeconds: number | null;
  /** Whether the solar day is a measured constant or derived from the table. */
  solarDayOrigin: "measured" | "derived" | null;
  /** The year in local solar days; `null` for the Sun. */
  yearInLocalDays: number | null;
  zeroPoint: ZeroPoint;
  /** What the zero point is. */
  zeroPointNote: string;
  source: string;
  /** The status of a standard still being drawn up: the Moon's Coordinated Lunar Time; `null` elsewhere. */
  status: string | null;
}

/** The one line of `hc_body_time`: local mean solar time on a body. */
export interface BodyTime {
  /** The local day number, counted from the body's zero point. */
  day: number;
  /** The fraction of the local day elapsed, 0 to 1. */
  fraction: number;
  /** `HH:MM:SS` on a 24-hour local face, truncated. */
  time: string;
  /** Decimal local hours. */
  hours: number;
  solarDaySeconds: number;
  localHourSeconds: number;
  zeroPoint: ZeroPoint;
  zeroPointNote: string;
}

/** The one line of `hc_proper_time`: a clock at a constant speed. */
export interface ProperTime {
  /** The speed as a fraction of `SPEED_OF_LIGHT`. */
  beta: number;
  /** γ. */
  lorentzFactor: number;
  /** The proper time the moving clock records, in seconds. */
  properSeconds: number;
  /** dτ/dt = 1/γ. */
  rate: number;
  /** The rate's offset from 1 in microseconds per 86 400-second day; negative. */
  microsecondsPerDay: number;
  /** The `hc-relativity` constants used, by name. */
  constants: string[];
  source: string;
}

/** The one line of `hc_gravitational_dilation`: a clock held still at a radius. */
export interface GravitationalDilation {
  id: string;
  /** GM in m³ s⁻². */
  gm: number;
  /** The `hc-relativity` constant that holds `gm`. */
  gmConstant: string;
  /** 2GM/c², in metres. */
  schwarzschildRadius: number;
  /** dτ/dt = √(1 − r_s/r) against a clock far from every mass. */
  factor: number;
  /** The factor's offset from 1 in microseconds per 86 400-second day; negative. */
  microsecondsPerDay: number;
  /** The `hc-relativity` constants used, by name. */
  constants: string[];
  source: string;
}

/** One line of `hc_gravitating_bodies`. */
export interface GravitatingBody {
  id: string;
  name: string;
  /** GM in m³ s⁻². */
  gm: number;
  /** The `hc-relativity` constant that holds `gm`. */
  gmConstant: string;
  source: string;
}

/** A TAI64 label's format. */
export type Tai64Format = "tai64" | "tai64n" | "tai64na";

/**
 * A TAI instant: whole seconds from 1970-01-01 00:00:00 TAI, floored, and
 * the attoseconds into that second, 0 to 10¹⁸ − 1. Both are `BigInt`s:
 * attoseconds do not fit a number, and a TAI64 label's seconds need not.
 */
export interface TaiInstant {
  seconds: bigint;
  attoseconds: bigint;
}

/** The one line of `hc_tai64_decode`: a label's format and the instant it names. */
export interface Tai64Label extends TaiInstant {
  format: Tai64Format;
}

/** The one line of `hc_utc_from_tai`: a whole TAI second as a UTC label. */
export interface UtcLabel {
  /** The POSIX second; for a leap second, the one after it. */
  unixSeconds: bigint;
  /** Whether the TAI second is an inserted `23:59:60`. */
  leapSecond: boolean;
}

/**
 * The one line of `hc_tai64_posix_plus_10_decode`: a label in daemontools'
 * convention on a POSIX clock and the POSIX instant it names, both parts
 * `BigInt`s.
 */
export interface PosixTai64Label {
  format: "tai64" | "tai64n";
  /** Whole POSIX seconds. */
  seconds: bigint;
  /** Attoseconds into that second, a whole number of nanoseconds. */
  attoseconds: bigint;
}

/** The one line of `hc_uuid_timestamp`. */
export interface UuidTimestamp {
  version: 1 | 6;
  /** 100-nanosecond intervals from 1582-10-15 00:00 UTC, 60 bits. */
  timestamp: bigint;
  /** The POSIX seconds of the start of that interval. */
  unixSeconds: bigint;
  /** Attoseconds into that second. */
  attoseconds: bigint;
}

/** The one line of `hc_ntp_resolve`: RFC 5905's 128-bit date and its POSIX instant. */
export interface NtpDate {
  /** 0 for 1900 to 2036, negative before 1900. */
  era: number;
  /** Seconds into the era. */
  offset: number;
  /** The fraction of the second in units of 2⁻⁶⁴ s. */
  fraction: bigint;
  unixSeconds: bigint;
  attoseconds: bigint;
}

/** The one line of `hc_uuid_timestamp_encode`: a POSIX instant's UUID timestamp. */
export interface UuidTimeFields {
  /** 100-nanosecond intervals from 1582-10-15 00:00 UTC, 60 bits. */
  timestamp: bigint;
  /** The first three groups of a version 1 UUID, such as `c232ab00-9414-11ec`. */
  v1: string;
  /** The first three groups of a version 6 UUID, such as `1ec9414c-232a-6b00`. */
  v6: string;
}

/** The one line of `hc_ntp_encode`: RFC 5905's date and timestamp of a POSIX instant. */
export interface NtpEncoding {
  /** 0 for 1900 to 2036, negative before 1900. */
  era: number;
  /** Seconds into the era. */
  offset: number;
  /** The fraction of the second in units of 2⁻⁶⁴ s. */
  fraction: bigint;
  /** The 128-bit date, era, offset and fraction, as 32 lower-case hex digits. */
  date: string;
  /** The 64-bit timestamp, offset and top 32 bits of the fraction, as 16. */
  timestamp: string;
}

/** A reckoning of 寒食 `hc_cold_food_day` knows. */
export type ColdFoodConvention = "hanshi-solstice-105" | "hanshi-eve-of-qingming" | "hansik";

/** The one line of `hc_fat_decode`: a local reading, in no zone. */
export interface FatReading {
  fixed: number;
  /** Always even. */
  secondsOfDay: number;
}

/** The one line of `hc_fat_encode`. */
export interface FatWords {
  date: number;
  time: number;
}

/** The letter of a Julian or a Besselian epoch. */
export type EpochNotation = "J" | "B";

/** What names a notation: its letter or its identifier, in any case. */
export type EpochNotationName = EpochNotation | "julian-epoch" | "besselian-epoch";

/** The one line of `hc_epoch_from_tt`. */
export interface Epoch {
  notation: EpochNotation;
  /** The year with its fraction: 2000 for J2000.0. */
  epoch: number;
}

/**
 * The one line of `hc_tt_from_epoch`: whole seconds from 1970-01-01
 * 00:00:00 TT and attoseconds, with the notation the year was read in.
 */
export interface EpochInstant {
  notation: EpochNotation;
  seconds: bigint;
  attoseconds: bigint;
}

/** A calendar `hc_circad_date` answers for. */
export type CircadCalendar =
  | "darian-titan"
  | "gregorian-io"
  | "gregorian-europa"
  | "gregorian-ganymede"
  | "gregorian-callisto"
  | "martiana";

/** The one line of `hc_circad_date`. */
export interface CircadDate {
  calendar: CircadCalendar;
  year: number;
  month: number;
  /** The circad or sol of the month, from 1. */
  day: number;
  monthName: string;
  /** The day's place in the week; `null` for Martiana's epagomenal sol. */
  weekName: string | null;
  /** The circad number from the calendar's epoch, or Martiana's Darian sol number. */
  count: number;
  /** The fraction of that circad or sol elapsed. */
  fraction: number;
  leap: boolean;
  source: string;
}

/** A GNSS broadcast week field, as `hc-core`'s `gnss` names it. */
export type GnssNumbering = "gps-lnav-week" | "gps-cnav-week" | "galileo-week" | "beidou-week" | "navic-week";

/** How a broadcast week is resolved against a reference. */
export type RolloverRule = "not-before" | "nearest";

/** The one line of `hc_gnss_week`. */
export interface GnssWeek {
  /** The full week since the field's week zero. */
  week: number;
  /** The week as the field broadcasts it: `week` modulo 2 to the field's bits. */
  broadcastWeek: number;
  /** Whole seconds into the week, 0 to 604 799. */
  towSeconds: number;
  /** Attoseconds into that second. */
  towAttoseconds: bigint;
}

/** The one line of `hc_glonass_date`. */
export interface GlonassDate {
  /** *N*4, 1 for 1996–1999. */
  fourYearInterval: number;
  /** *N*T, 1 on 1 January of the interval's leap year. */
  day: number;
}

/** The one line of `hc_fixed_from_ole_automation`. */
export interface OleAutomationDay {
  fixed: number;
  /** The seconds into the day, as the double carries them. */
  secondsOfDay: number;
}

/** The one line of `hc_excel_1900_day`. */
export interface Excel1900Day {
  /** The fixed day, or `null` for serial 60. */
  fixed: number | null;
  /** Serial 60, the 29 February 1900 that Excel counts and no calendar has. */
  phantom: boolean;
}

/** One line of `hc_panchanga_at` or `hc_panchanga_of_day`. */
export interface PanchangaLimb {
  limb: "yoga" | "karana";
  /** The yoga, 1 to 27; the karaṇa's half-tithi, 1 to 60. */
  number: number;
  /** Drik Panchang's English spelling. */
  name: string;
  /** Drik Panchang's Hindi edition's Devanagari. */
  devanagari: string;
  /** POSIX seconds, rounded down. */
  began: number;
  ends: number;
  /** The instant read: the one asked for, or the sunrise. */
  readAt: number;
  /** The yoga's ayanamsa by its full name; `null` for the karaṇa. */
  ayanamsa: string | null;
}

/** The published code's names for where 立春 falls in a Chinese year. */
export type MarriageAuguryName = "widow" | "blind" | "bright" | "double-bright";

/** The one line of `hc_chinese_marriage_augury`. */
export interface MarriageAugury {
  augury: MarriageAuguryName;
  /** Whether the year's first 立春 comes after its New Year. */
  lichunAtStart: boolean;
  /** Whether another 立春 comes before the next New Year. */
  lichunAtEnd: boolean;
}

/** What a holiday table is, by the list it is in. */
export type HolidayTableKind = "country" | "subdivision" | "exchange" | "tradition" | "observance";

/** One line of `hc_holiday_tables`. */
export interface HolidayTable {
  code: string;
  kind: HolidayTableKind;
  /** The name in the locale: a country's CLDR name where the locale has one, else the English name. */
  name: string | null;
  englishName: string;
  /** The tag of the data that named it: `ja` for 日本, `en` for an English name. */
  localeUsed: string | null;
  /** The sources the table names. */
  source: string | null;
  /** The ISO 3166-1 country of a subdivision, or of an exchange whose table records one. */
  country: string | null;
  /**
   * CLDR 48's `alt="short"` name beside a CLDR name, from the same locale's
   * data: `Hong Kong` for `HK` under `en`, 香港 under `ja`; `null` where
   * CLDR has none, which is most countries, and for a table that is not a
   * country.
   */
  shortName: string | null;
}

/** One line of `hc_zones` and `hc_zone_location`. */
export interface ZoneLocation {
  /** The name of the row that answered: `Asia/Kolkata` for `Asia/Calcutta`. */
  zone: string;
  /**
   * Degrees north of the principal location: the table's whole arcseconds
   * written to six decimals, so that `Math.round(latitude * 3600)` gives the
   * arcseconds back.
   */
  latitude: number;
  /** Degrees east of the principal location, written as `latitude` is. */
  longitude: number;
  /** The ISO 3166-1 codes of the countries the zone overlaps, the location's first. */
  countries: string[];
  /** The table's comment, which tells a country's zones apart; `null` where the country has one. */
  comment: string | null;
  /** The zone's CLDR 48 exemplar city in the locale, else in English: 東京 under `ja`, `Tokyo` under `en`. */
  exemplarCity: string;
  /** The tag of the data that named the city: `ja`, `de` for `de-AT`, or `en`. */
  localeUsed: string;
}

/** The one line of `hc_lectionary`. */
export interface Lectionary {
  /** The civil year of the liturgical year's Easter. */
  liturgicalYear: number;
  sundayCycle: "A" | "B" | "C";
  weekdayCycle: "I" | "II";
  /** The RCL Proper, 3 to 29, of a Sunday after Trinity Sunday; else `null`. */
  proper: number | null;
}

/** A local clock `hc_solar_time` reads. */
export type SolarClock = "local-mean" | "local-apparent" | "temporal" | "italian";

/** A named time of day `hc_solar_event` answers. */
export type SolarEventName =
  | "asr-shafii"
  | "asr-hanafi"
  | "jewish-dusk-vilna-gaon"
  | "jewish-sabbath-ends-cohn"
  | "italian-zero-hour";

/** The solar event a reckoning needs and does not have. */
export type MissingSolarEventName = "sunrise" | "sunset" | "depression" | "no-noon-shadow";

export interface MissingSolarEvent {
  event: MissingSolarEventName;
  /** The local day it is missing on, as a fixed day. */
  day: number;
  /** For `depression`, the depression sought in arcminutes; else `null`. */
  depressionArcminutes: number | null;
}

/** The one line of `hc_solar_time`. */
export interface SolarTime {
  /** The fixed day of the local date, or `null` when there is no reading. */
  day: number | null;
  /** The hours into it on the clock, or `null`. */
  hours: number | null;
  missing: MissingSolarEvent | null;
}

/** The one line of `hc_solar_event`. */
export interface SolarEvent {
  /** POSIX seconds of Universal Time, rounded down, or `null` when the time does not happen. */
  instant: number | null;
  missing: MissingSolarEvent | null;
}

/** A horizon `hc_horizons` lists. */
export type HorizonId = "geometric-dip" | "usno" | "calendrical-calculations";

/** One line of `hc_horizons`. */
export interface Horizon {
  id: HorizonId;
  englishName: string;
  /** What the convention takes the visible horizon to be. */
  description: string;
  source: string;
}

/** The one line of `hc_sunrise` or `hc_sunset`. */
export interface SolarCrossing {
  /** POSIX seconds of Universal Time, rounded down, or `null` when the Sun does not cross that day. */
  instant: number | null;
  missing: MissingSolarEvent | null;
  /** The geometric altitude of the Sun's centre at the crossing, which the horizon and the height fix. */
  altitudeDegrees: number;
}

/** The one line of `hc_hindu_lunar_date`. */
export interface HinduLunarDate {
  sakaYear: number;
  vikramaYear: number;
  /** 1 for Chaitra through 12 for Phālguna. */
  month: number;
  /** The intercalary (adhika) month, which precedes the ordinary one. */
  leapMonth: boolean;
  /** 1 through 30. */
  tithi: number;
  /** The second day to carry the tithi. */
  leapDay: boolean;
  /** The sunrise the day was read at, POSIX seconds, rounded down. */
  sunrise: number;
}

/** The one line of `hc_surya_siddhanta_at`. */
export interface SuryaSiddhantaSky {
  /** Degrees, sidereal. */
  sunLongitude: number;
  moonLongitude: number;
  /** The Moon's elongation from the Sun, 0 to 360 degrees. */
  elongation: number;
  /** 1 through 30. */
  tithi: number;
  /** 1 for Meṣa through 12 for Mīna. */
  sign: number;
}

/** A crescent-visibility criterion `hc_crescent_visible` names. */
export type CrescentCriterion = "shaukat" | "yallop" | "saudi-rule";

/** The one line of `hc_crescent_visible`. */
export interface CrescentVisibility {
  visible: boolean;
  /** The moment the evening is judged at, POSIX seconds, or `null` when no observation is possible. */
  evaluatedAt: number | null;
  /** At that moment, the Moon's longitude less the Sun's, 0 to 360 degrees, as `Sky.elongation`. */
  elongation: number | null;
  arcOfLight: number | null;
  /** The Moon's geocentric altitude. */
  altitude: number | null;
  arcOfVision: number | null;
  /** The crescent's topocentric width in arcminutes. */
  widthArcminutes: number | null;
}

/** A decan's ruler: one of the seven classical bodies. */
export type DecanRuler = "saturn" | "jupiter" | "mars" | "sun" | "venus" | "mercury" | "moon";

/** The one line of `hc_decan_at`. */
export interface Decan {
  /** The tropical sign, 1 for Aries through 12 for Pisces. */
  sign: number;
  signName: string;
  /** Which of the sign's three 10° faces, 1 to 3. */
  decan: number;
  /** The face's ruler by al-Bīrūnī's table. */
  ruler: DecanRuler;
  rulerName: string;
  /** From 0 up to 10. */
  degreesIntoDecan: number;
}

/** The one line of `hc_hjd_tt`. */
export interface HeliocentricJulianDate {
  hjd: number;
  /** The light-time correction added to the date; negative when the light reaches the Sun first. */
  correctionSeconds: number;
}

/** The one line of `hc_hjd_utc`. */
export interface HeliocentricJulianDateUtc extends HeliocentricJulianDate {
  /** 32.184 s plus TAI − UTC, at which the Earth's position was taken. */
  ttMinusUtcSeconds: number;
}

/** The one line of `hc_tt_bipm`. */
export interface TtBipmReading {
  /** TT(BIPMxx) − TT(TAI), interpolated between the samples. */
  offsetSeconds: number;
  /** TT(BIPMxx) − TAI: 32.184 s and the offset. */
  minusTai: TaiInstant;
  /** The TT(BIPMxx) reading from that scale's 1970 epoch. */
  reading: TaiInstant;
}

/** A period's names on a day, from `hc_naming_period_on`. */
export interface NamingPeriodNames {
  id: string;
  /** The period's name for the day's month. */
  monthName: string | null;
  /** The period's name for the day's weekday. */
  weekdayName: string | null;
  /** That weekday name's meaning in English, as the source glosses it. */
  weekdayMeaning: string | null;
  /** The first day the names can have been in force. */
  earliest: number;
  /** The first day by which every source read has them in force. */
  inForceBy: number;
  /** The first day the old names were back. */
  ended: number;
  source: string;
}

/** The one line of `hc_naming_period_on`. */
export type NamingPeriodOn =
  | { state: "in-force" | "undecided"; period: NamingPeriodNames }
  | { state: "ordinary"; period: null };

/** The one line of `hc_asian_day`. */
export interface AsianDay {
  /** The Julian year, AD, in which the Asian year began. */
  year: number;
  /** 1 for Kaisar through 12 for Hyperberetaios. */
  month: number;
  monthName: string;
  /** `unnumbered` for Sebaste and, in a leap Xandikos, the intercalary day. */
  written: "unnumbered" | "numbered";
  /** The day's number, or its place among the unnumbered days. */
  number: number;
}

/** The one line of `hc_holy_year_on`, within a jubilee. */
export interface HolyYear {
  title: string;
  kind: "ordinary" | "extraordinary";
  pope: string;
  /** The bull of indiction, by its opening words. */
  bull: string;
  /** The fixed day the bull was given. */
  given: number;
  /** The first and last fixed days in Rome. */
  opens: number;
  closes: number;
  /** The first and last fixed days in the dioceses, where the bull dates them. */
  dioceses: { opens: number; closes: number } | null;
}

/** A rank of the *Common Worship* calendar. */
export type CommonWorshipRank = "principal-feast" | "principal-holy-day" | "festival";

/** One line of `hc_common_worship_on`. */
export interface CommonWorshipCelebration {
  /** The title as the Rules print it, the name `hc_holidays_on` gives it. */
  title: string;
  rank: CommonWorshipRank;
  rankName: string;
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
  /**
   * `hc_unix_from_fixed`: midnight UTC on a fixed day. A day before
   * −104 165 947 503, whose midnight would read as a sentinel, or after
   * 106 751 991 886 463, whose midnight would overflow an `i64`, is
   * `out-of-range`; one whose seconds a number cannot hold exactly is
   * `unsafe-integer`.
   */
  unixFromFixed(fixed: number | bigint): number;
  /** `hc_format_iso_date`. */
  formatIsoDate(fixed: number | bigint): string;
  /** `hc_parse_iso_date`; text that is not a date is `invalid-date`. */
  parseIsoDate(text: string): number;
  /** `hc_tai_from_unix`; under `strict`, outside the leap-second table is `no-data`. */
  taiFromUnix(unixSeconds: number | bigint, strict?: boolean): TaiInstant;
  /** `hc_utc_from_tai`: a whole TAI second's POSIX second, and whether it is a leap second. */
  utcFromTai(taiSeconds: number | bigint, strict?: boolean): UtcLabel;
  /** `hc_tai64_encode`: the label in lower-case hexadecimal; an unknown format is `unknown`. */
  tai64Encode(taiSeconds: number | bigint, attoseconds: number | bigint, format: Tai64Format): string;
  /** `hc_tai64_decode`; text that is not 16, 24 or 32 hex digits is `malformed`. */
  tai64Decode(hex: string): Tai64Label;
  /** `hc_gnss_week`; an instant before week zero is `no-data`. */
  gnssWeek(numbering: GnssNumbering, taiSeconds: number | bigint, attoseconds?: number | bigint): GnssWeek;
  /** `hc_gnss_to_tai`; a time of week from 604 800 s is `out-of-range`. */
  gnssToTai(numbering: GnssNumbering, week: number, towSeconds: number, towAttoseconds?: number | bigint): TaiInstant;
  /** `hc_gnss_resolve_week`: the full week a broadcast week names by a rule and a reference in whole TAI seconds. */
  gnssResolveWeek(numbering: GnssNumbering, broadcast: number, rule: RolloverRule, referenceTaiSeconds: number | bigint): number;
  /** `hc_glonass_date`; before 1996 or from 2100 is `out-of-range`. */
  glonassDate(taiSeconds: number | bigint, attoseconds?: number | bigint, strict?: boolean): GlonassDate;
  /** `hc_fixed_from_ole_automation`. */
  fixedFromOleAutomation(value: number): OleAutomationDay;
  /** `hc_ole_automation_from_fixed`. */
  oleAutomationFromFixed(fixed: number | bigint, secondsOfDay?: number): number;
  /** `hc_excel_1900_day`: serial 60 is `phantom`, with no fixed day. */
  excel1900Day(serial: number | bigint): Excel1900Day;
  /** `hc_tai64_posix_plus_10_encode`: daemontools' label on a POSIX clock; `tai64na` is `unknown`. */
  tai64PosixPlus10Encode(unixSeconds: number | bigint, attoseconds: number | bigint, format: "tai64" | "tai64n"): string;
  /** `hc_tai64_posix_plus_10_decode`; text that is not 16 or 24 hex digits is `malformed`. */
  tai64PosixPlus10Decode(hex: string): PosixTai64Label;
  /** `hc_uuid_timestamp`; a UUID of another version is `no-data`, other text `malformed`. */
  uuidTimestamp(uuid: string): UuidTimestamp;
  /** `hc_ntp_resolve`: the timestamp in the era within 2³¹ s of the reference; zero is `no-data`. */
  ntpResolve(seconds: number, fraction: number, referenceUnix: number | bigint): NtpDate;
  /** `hc_uuid_timestamp_encode`; before 1582-10-15 or after 5236-03-31 is `out-of-range`. */
  uuidTimestampEncode(unixSeconds: number | bigint, attoseconds?: number | bigint): UuidTimeFields;
  /** `hc_ntp_encode`: the era, offset and fraction, and both wire layouts in hexadecimal. */
  ntpEncode(unixSeconds: number | bigint, attoseconds?: number | bigint): NtpEncoding;
  /** `hc_fat_decode`; fields that name no day or time are `invalid-date`. */
  fatDecode(date: number, time: number): FatReading;
  /** `hc_fat_encode`; outside 1980 to 2107 is `out-of-range`. */
  fatEncode(fixed: number | bigint, secondsOfDay: number): FatWords;
  /** `hc_swatch_beat`: 0 through 999. */
  swatchBeat(unixSeconds: number | bigint, attoseconds?: number | bigint): number;
  /** `hc_epoch_from_tt`: the Julian or Besselian epoch of a TT instant. */
  epochFromTt(notation: EpochNotationName, ttSeconds: number | bigint, attoseconds?: number | bigint): Epoch;
  /** `hc_tt_from_epoch`; an empty notation is SOFA's rule for an epoch without a letter. */
  ttFromEpoch(notation: EpochNotationName | "", year: number): EpochInstant;
  /**
   * `hc_tt_bipm`: `series` one line per sample, the MJD and TT(BIPMxx) −
   * TAI − 32.184 s in µs, tab-separated; an instant outside it is `no-data`.
   */
  ttBipm(series: string, taiSeconds: number | bigint, attoseconds?: number | bigint, strict?: boolean): TtBipmReading;

  /**
   * `hc_describe_day`: the day in every registered calendar, in registry
   * order. `locale` is a BCP 47 tag, `und` unless given, as the module.
   */
  describeDay(fixed: number | bigint, locale?: string): DescribedDay[];
  /** The days from `from` up to but not including `to` as one calendar's eras, years, months or days. */
  calendarUnits(id: string, unit: Unit | number, from: number | bigint, to: number | bigint, locale?: string): CalendarUnit[];
  /** Every registered calendar, with what the locale calls it and its standing on `today`. */
  calendars(today: number | bigint, locale?: string): CalendarEntry[];
  /** `hc_calendar_list`: every registered calendar by name alone, with nothing that depends on a day. */
  calendarList(locale?: string): CalendarListEntry[];
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
  /** `hc_panchanga_at`: the yoga's line, then the karaṇa's; an ayanamsa nobody knows is `unknown`. */
  panchangaAt(unixSeconds: number | bigint, ayanamsa: string): PanchangaLimb[];
  /** `hc_panchanga_of_day`: read at the day's sunrise at the place; no sunrise is `no-data`. */
  panchangaOfDay(fixed: number | bigint, latitude: number, longitude: number, elevation: number, ayanamsa: string): PanchangaLimb[];
  /**
   * `hc_hindu_lunar_date`: `sky` an ayanamsa name or `surya-siddhanta`; on
   * the true sky a day without a sunrise at the place is `no-data`.
   */
  hinduLunarDate(sky: string, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): HinduLunarDate;
  /** `hc_surya_siddhanta_at`; an instant outside Kali Yuga 1 to 10 000 is `out-of-range`. */
  suryaSiddhantaAt(unixSeconds: number | bigint): SuryaSiddhantaSky;
  /** `hc_surya_siddhanta_sunrise`: POSIX seconds; a place beyond 65° of latitude is `out-of-range`. */
  suryaSiddhantaSunrise(fixed: number | bigint, latitude: number, longitude: number): number;
  /** `hc_crescent_visible`: on the evening that begins the day. */
  crescentVisible(criterion: CrescentCriterion, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): CrescentVisibility;
  /** `hc_ioc_olympiad`; a year before 1896 is `out-of-range`. */
  iocOlympiad(gregorianYear: number | bigint): number;
  /** `hc_hebrew_yahrzeit`: the date of death as the fixed day that carries it. */
  hebrewYahrzeit(deathFixed: number | bigint, hebrewYear: number | bigint): number;
  /** `hc_hebrew_birthday`. */
  hebrewBirthday(birthFixed: number | bigint, hebrewYear: number | bigint): number;
  /** `hc_chinese_reckoned_age`: one at birth, one more each Chinese New Year; before the birth is `no-data`. */
  chineseReckonedAge(birthFixed: number | bigint, onFixed: number | bigint): number;
  /** `hc_chinese_marriage_augury`: by the Chinese calendar's year count, 4661 from 10 February 2024. */
  chineseMarriageAugury(chineseYear: number | bigint): MarriageAugury;
  /** `hc_naming_period_on`: a calendar the registry does not carry is `unknown`. */
  namingPeriodOn(calendar: string, fixed: number | bigint, locale: string): NamingPeriodOn;
  /** `hc_hebrew_sabbatical_cycle_year`: 7 is *shemittah*. */
  hebrewSabbaticalCycleYear(hebrewYear: number | bigint): number;
  /** `hc_asian_day`; a day outside AD 4 to the Asian year 9999 is `out-of-range`. */
  asianDay(fixed: number | bigint): AsianDay;

  /** `hc_holiday_is_day_off`; `region` may be empty. A code naming no table is `unknown`. */
  holidayIsDayOff(code: string, region: string, fixed: number | bigint): boolean;
  /** `hc_holidays_in_year`. */
  holidaysInYear(code: string, region: string, year: number | bigint): HolidayInYear[];
  /** `hc_holiday_codes`: countries, then exchanges, traditions and the international sets. */
  holidayCodes(): string[];
  /** `hc_holidays_on`: every entry on one day across every table, in `holidayCodes()` order. A day with no Gregorian year is `out-of-range`. */
  holidaysOn(fixed: number | bigint): HolidayOn[];
  /** `hc_holiday_tables`: every table, in `holidayCodes()` order; `und` unless given. */
  holidayTables(locale?: string): HolidayTable[];
  /** `hc_lectionary`; outside the liturgical years 1583 to 4099 is `out-of-range`. */
  lectionary(fixed: number | bigint): Lectionary;
  /** `hc_astronomical_easter`; outside 1583 to 2150 is `out-of-range`. */
  astronomicalEaster(year: number | bigint): number;
  /** `hc_astronomical_paschal_full_moon`: the day Easter is the Sunday after; outside 1583 to 2150 is `out-of-range`. */
  astronomicalPaschalFullMoon(year: number | bigint): number;
  /** `hc_holy_year_on`: `null` outside a jubilee; a day the table does not reach is `no-data`. */
  holyYearOn(fixed: number | bigint): HolyYear | null;
  /** `hc_common_worship_on`: empty on a day that keeps none. */
  commonWorshipOn(fixed: number | bigint): CommonWorshipCelebration[];

  /** `hc_term_in_effect`; a meridian nobody knows is `unknown`. */
  termInEffect(fixed: number | bigint, meridian?: Meridian): TermInEffect;
  /** `hc_pentad_in_effect`. */
  pentadInEffect(fixed: number | bigint, meridian?: Meridian): TermInEffect;
  /** `hc_cold_food_day`; a reckoning nobody knows is `unknown`, a year outside −999 to 3000 `out-of-range`. */
  coldFoodDay(convention: ColdFoodConvention, year: number | bigint): number;

  /** `hc_place_years_ago`; a value the crate refuses is `out-of-range`. `locale` names the geologic rows, `und` unless given. */
  placeYearsAgo(yearsAgo: number, stdDevYears?: number, locale?: string): DeepTimeRow[];
  /** `hc_cosmic_events`. */
  cosmicEvents(locale?: string): DeepTimeRow[];
  /** `hc_geologic_intervals`; the rank by name or by number from 0. */
  geologicIntervals(rank: GeologicRank | number, locale?: string): DeepTimeRow[];

  /** `hc_fixed_from_unix_in_zone`; a zone nobody knows is `unknown`. */
  fixedFromUnixInZone(unixSeconds: number | bigint, zone: string): number;
  /**
   * `hc_unix_from_fixed_in_zone`: `unixFromFixed`'s range, each end moved
   * by at most a day by the zone's offset; outside it, `out-of-range`.
   */
  unixFromFixedInZone(fixed: number | bigint, zone: string): number;
  /** `hc_zone_load`; bytes that are not TZif are `malformed`. */
  loadZone(name: string, tzif: Uint8Array | ArrayBuffer): void;
  /** `hc_zones`: every zone of `zone1970.tab`, in its order; `und` unless given. */
  zones(locale?: string): ZoneLocation[];
  /** `hc_zone_location`: a zone or a link; a name that places nothing, such as `UTC`, is `unknown`. */
  zoneLocation(zone: string, locale?: string): ZoneLocation;

  /** `hc_sky_at`; an instant outside −1000 through 3000 is `out-of-range`. */
  skyAt(unixSeconds: number | bigint): Sky;
  /** `hc_solar_terms_between`: the terms in `[from, to)`; a span outside the era or over 400 years is `out-of-range`. */
  solarTermsBetween(fromUnix: number | bigint, toUnix: number | bigint): SkyEvent[];
  /** `hc_moon_phases_between`: the phases in `[from, to)`, as `solarTermsBetween`. */
  moonPhasesBetween(fromUnix: number | bigint, toUnix: number | bigint): SkyEvent[];
  /** `hc_decan_at`. */
  decanAt(unixSeconds: number | bigint): Decan;
  /** `hc_earth_rotation_angle`: degrees, at UT1 counted as POSIX seconds are; outside −1000 through 3000 is `out-of-range`. */
  earthRotationAngle(ut1UnixSeconds: number): number;
  /** `hc_gmst_iau2006`: degrees. */
  gmstIau2006(ut1UnixSeconds: number): number;
  /** `hc_gmst_iau1982`: degrees. */
  gmstIau1982(ut1UnixSeconds: number): number;
  /** `hc_ut2_minus_ut1`: seconds. */
  ut2MinusUt1(ut1UnixSeconds: number): number;
  /** `hc_solar_time`; a missing solar event is an answer, not an error. */
  solarTime(clock: SolarClock, unixSeconds: number | bigint, latitude: number, longitude: number, elevation?: number): SolarTime;
  /** `hc_solar_event`. */
  solarEvent(event: SolarEventName, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): SolarEvent;
  /** `hc_horizons`. */
  horizons(): Horizon[];
  /** `hc_sunrise`; a missing sunrise is an answer, not an error. */
  sunrise(horizon: HorizonId, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): SolarCrossing;
  /** `hc_sunset`. */
  sunset(horizon: HorizonId, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): SolarCrossing;
  /** `hc_hjd_tt`: right ascension and declination in degrees, J2000. */
  hjdTt(ttJulianDate: number, rightAscension: number, declination: number): HeliocentricJulianDate;
  /** `hc_hjd_utc`; under `strict` a date outside the leap-second table is `no-data`. */
  hjdUtc(utcJulianDate: number, rightAscension: number, declination: number, strict?: boolean): HeliocentricJulianDateUtc;

  /** `hc_orbit_at`; an epoch beyond a million years either side of 1950 is `out-of-range`. */
  orbitAt(yearsBefore1950: number): Orbit;
  /**
   * `hc_orbit_series`: `orbitAt` at `from`, `from + step`, ... up to `to`, at most
   * 10 000 samples; a step that is not positive, an end off the span or more
   * samples is `out-of-range`, and a `to` before `from` is empty.
   */
  orbitSeries(fromYearsBefore1950: number, toYearsBefore1950: number, stepYears: number): OrbitSample[];

  /** `hc_mars_time`; an instant more than 100 Julian years from J2000.0 is `out-of-range`. */
  marsTime(unixSeconds: number, eastLongitude?: number): MarsTime;
  /** `hc_missions`. */
  missions(): Mission[];
  /** `hc_mission_sol`; an unpublished convention is `no-data`, an instant before the landing sol `out-of-range`. */
  missionSol(mission: string, unixSeconds: number): number;
  /** `hc_bodies`. */
  bodies(): Body[];
  /** `hc_body_time`; the Sun is `no-data`. */
  bodyTime(body: string, unixSeconds: number, eastLongitude?: number): BodyTime;
  /** `hc_circad_date`; an instant more than 100 Julian years from J2000.0 is `out-of-range`. */
  circadDate(calendar: CircadCalendar, unixSeconds: number): CircadDate;
  /** `hc_proper_time`; a speed at or beyond light is `out-of-range`. */
  properTime(speedMetresPerSecond: number, coordinateSeconds: number): ProperTime;
  /** `hc_gravitational_dilation`; a radius at or inside the horizon is `out-of-range`. */
  gravitationalDilation(body: string, radiusMetres: number): GravitationalDilation;
  /** `hc_gravitating_bodies`. */
  gravitatingBodies(): GravitatingBody[];
}

/**
 * Instantiate the module and bind it. `source` may also be a promise of
 * any accepted form.
 */
export function load(source: ModuleSource | Promise<ModuleSource>, options?: LoadOptions): Promise<HyperCalendar>;
