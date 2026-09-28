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

/**
 * The names the binding throws on its own account. `unsafe-integer` is
 * both a result a number cannot hold exactly and an `i64` or `u64`
 * argument given as an integer number past `Number.MAX_SAFE_INTEGER`,
 * which a `BigInt` would carry exactly; a non-integer number, a `BigInt`
 * outside the argument's type or a value of another type is a `TypeError`.
 */
export type BindingErrorName =
  | "not-exported"
  | "unsafe-integer"
  | "allocation-failed"
  | "unrecognised-sentinel";

/** The Cargo features the exports are gated by. */
export type Feature =
  | "civil"
  | "timestamps"
  | "time-codes"
  | "calendars"
  | "holiday"
  | "seasons"
  | "deep-time"
  | "tz"
  | "sky"
  | "orbital"
  | "planetary"
  | "relativity";

/** A binary CCSDS code `hc_ccsds_decode` reads. */
export type CcsdsCodeName = "cuc" | "cds" | "ccs";

/** An instant as a CCSDS line writes it: TAI, and its UTC label. */
export interface CcsdsInstant {
  tai: TaiInstant;
  utc: {
    /** The POSIX second; for a leap second, the one after it. */
    unixSeconds: bigint;
    /** Whether it is an inserted `23:59:60`. */
    leapSecond: boolean;
    attoseconds: bigint;
  };
}

/** The one line of `hc_ccsds_decode`. */
export interface CcsdsCode extends CcsdsInstant {
  code: CcsdsCodeName;
}

/** How far an ASCII code's time part runs. */
export type CcsdsAsciiPrecision = "hour" | "minute" | "second" | "fraction";

/** The one line of `hc_ccsds_ascii_parse`. */
export interface CcsdsAsciiCode extends CcsdsInstant {
  variation: "a" | "b";
  precision: CcsdsAsciiPrecision;
  /** The digits of the fraction, 1 to 18, for `fraction`; else `null`. */
  digits: number | null;
  /** Whether the optional `Z` follows. */
  terminator: boolean;
}

/** A radio time code `hc_radio_decode` and `hc_radio_encode` name. */
export type RadioCode = "jjy" | "dcf77" | "wwvb-am" | "wwvb-pm";

/** The leap second a frame announces. */
export type RadioLeap = "none" | "positive" | "negative";

/** DCF77's zone, or WWVB's summer-time state. */
export type RadioSummer = "cet" | "cest" | "standard" | "begins-today" | "in-effect" | "ends-today";

/** The one line of `hc_radio_decode`. */
export interface RadioMinute {
  /** The POSIX second of the minute the frame names. */
  unixSeconds: number;
  /** That minute's fixed day, hour and minute in the code's own time. */
  fixed: number;
  hour: number;
  minute: number;
  /** The code's time less UTC, in hours. */
  offsetHours: number;
  /** The frame's length, 59 to 61. */
  seconds: number;
  leap: RadioLeap;
  /** `null` for `jjy`. */
  summer: RadioSummer | null;
  /** DCF77's A1; `null` for the other codes. */
  zoneChange: boolean | null;
  /** UT1 − UTC in tenths of a second, for `wwvb-am`; else `null`. */
  dut1Tenths: number | null;
  /** The phase code's six-bit `dst_next` word, for `wwvb-pm`; else `null`. */
  dstNext: number | null;
}

/** What `radioEncode` puts in a frame besides the minute. */
export interface RadioEncodeOptions {
  /** 1 for a second inserted, −1 for one omitted (`jjy` and `wwvb-pm` only), 0 for none. */
  leap?: -1 | 0 | 1;
  /**
   * DCF77's zone, required for `dcf77`; WWVB's summer-time state, required for
   * both WWVB codes; absent for `jjy`. Or `zone:` and a zone's name,
   * `zone:Europe/Berlin` or `zone:America/New_York`, in a module built with
   * `tz` too, to read the state, and DCF77's A1, from the rules
   * `fixedFromUnixInZone` reads for that name; a zone the built-in table lacks,
   * such as `America/Denver`, once `loadZone` has its TZif file.
   */
  summer?: RadioSummer | `zone:${string}`;
  /** DCF77's A1; ignored when `summer` names a zone. */
  zoneChange?: boolean;
  /** WWVB's amplitude UT1 − UTC in tenths, −9 to 9. */
  dut1Tenths?: number;
  /** The phase code's `dst_next` word; ignored when `summer` names a zone, whose next change gives it. */
  dstNext?: number;
}

/** The reading `hc_unix_from_dotnet_ticks` writes. */
export interface DotnetReading {
  unixSeconds: number;
  attoseconds: bigint;
}

/** A six-hour reckoning of the civil clock. */
export type SixHourReckoning = "ethiopian-hours" | "swahili-hours";

/** The one line of `hc_six_hour_clock`. */
export interface SixHourReading {
  /** The hour on the dial, 1 to 12. */
  hour: number;
  minute: number;
  second: number;
  half: "day" | "night";
  /** The part of the day the source names, in its language; `null` for `ethiopian-hours`. */
  period: string | null;
  periodEnglish: string | null;
}

/** One of the seven classical planets, by `hc-seasons`'s identifier. */
export type ClassicalPlanet = "sun" | "moon" | "mercury" | "venus" | "mars" | "jupiter" | "saturn";

/** A kind of choghadiya. */
export type ChoghadiyaId = "udvega" | "chara" | "labha" | "amrita" | "kala" | "shubha" | "roga";

/** One line of `hc_choghadiya`: an eighth of the daylight or of the night. */
export interface ChoghadiyaPart {
  half: "day" | "night";
  /** 1 to 8. */
  part: number;
  id: ChoghadiyaId;
  /** Named in the locale, उद्वेग to रोग under `hi`, else English's. */
  name: string;
  localeUsed: string;
  quality: "auspicious" | "neutral" | "inauspicious";
  ruler: ClassicalPlanet;
  /** POSIX seconds of UTC, rounded down; `null` where the Sun does not rise or set. */
  start: number | null;
  end: number | null;
  missing: MissingSolarEvent | null;
}

/** A Mars surface mission, by the identifier `hc_missions` gives it. */
export type MissionId =
  | "viking-1"
  | "viking-2"
  | "mars-pathfinder"
  | "spirit"
  | "opportunity"
  | "phoenix"
  | "curiosity"
  | "insight"
  | "perseverance"
  | "zhurong";

/** A body `hc-planetary` carries, by the identifier `hc_bodies` gives it. */
export type BodyId =
  | "sun"
  | "mercury"
  | "venus"
  | "earth"
  | "moon"
  | "mars"
  | "phobos"
  | "deimos"
  | "ceres"
  | "jupiter"
  | "io"
  | "europa"
  | "ganymede"
  | "callisto"
  | "saturn"
  | "enceladus"
  | "titan"
  | "uranus"
  | "neptune"
  | "triton"
  | "pluto"
  | "charon";

/** A body `hc-relativity` carries a GM for, by the identifier `hc_gravitating_bodies` gives it. */
export type GravitatingBodyId =
  | "sun"
  | "earth"
  | "moon"
  | "mars"
  | "jupiter"
  | "sagittarius-a-star";

/** An ayanāṃśa, by the identifier `hc-seasons`' `Ayanamsa::by_id` finds it by. */
export type Ayanamsa = "lahiri" | "raman" | "krishnamurti" | "fagan-bradley";

/** A naming table of the Panchak kinds. */
export type PanchakNaming = "panchak-five-kinds" | "panchak-raj-midweek";

/** A kind of Panchak window. */
export type PanchakKind = "rog" | "raj" | "agni" | "chor" | "mrityu";

/** The one line of `hc_panchak`. */
export interface PanchakWindow {
  /** Whether the instant asked of is within the window. */
  within: boolean;
  /** The Moon at 300° and at 360° of sidereal longitude, POSIX seconds of UTC. */
  opens: number;
  closes: number;
  /** The weekday it opens on at the offset given, Monday 1 to Sunday 7. */
  weekday: number;
  /** `null` on a weekday the table names no kind for. */
  kind: PanchakKind | null;
  name: string | null;
  localeUsed: string | null;
}

/** A sidereal sign, by the lower-case ASCII form of its Sanskrit name. */
export type SiderealSignId =
  | "mesha" | "vrishabha" | "mithuna" | "karka" | "simha" | "kanya"
  | "tula" | "vrishchika" | "dhanus" | "makara" | "kumbha" | "mina";

/** A condition of the Kumbh Mela, the Mela Adhikari's seven. */
export type KumbhYoga =
  | "kumbh-haridwar"
  | "kumbh-prayag-vrishabha"
  | "kumbh-prayag-mesha"
  | "kumbh-nashik-simha"
  | "kumbh-nashik-karka"
  | "kumbh-ujjain-simha"
  | "kumbh-ujjain-tula";

/** The one line of `hc_kumbh`. */
export interface KumbhOccasion {
  id: KumbhYoga;
  /** `haridwar`, `prayag`, `nashik` or `ujjain`. */
  site: string;
  siteName: string;
  localeUsed: string;
  /** In English, as the source gives it. */
  river: string;
  /** The sign Jupiter must be in, and the Sun. */
  jupiter: SiderealSignId;
  sun: SiderealSignId;
  atNewMoon: boolean;
  /** The Sun's entry into its sign and into the next, or the new moon twice; `null` when no new moon falls in the stay. */
  from: number | null;
  to: number | null;
  /** Whether the caller's Jupiter meets the condition; `null` when none was given. */
  holds: boolean | null;
}

/** One line of `hc_pushkaram`: a river's twelve days. */
export interface PushkaramDays {
  /** `pushkaram-ganga` to `pushkaram-pranahita`. */
  id: string;
  name: string;
  localeUsed: string;
  /** In English, where the source names one. */
  region: string | null;
  sign: SiderealSignId;
  /** Fixed days; `null` where the Sun does not set on the day of the entry. */
  first: number | null;
  last: number | null;
  missing: MissingSolarEvent | null;
}

/** A kind of line `hc_folk_day` writes. */
export type FolkDayKind = "first-month-count" | "plum-rains" | "vietnamese-day" | "folk-half" | "folk-named-day";

/** One line of `hc_folk_day`. */
export interface FolkDay {
  kind: FolkDayKind;
  /** `dragons`, `ru-mei-bing`, `tam-nuong`, `kasim`, `cemre-air`, … */
  id: string;
  /** Named in the locale, else English, else the kind's own language. */
  name: string;
  localeUsed: string;
  /** The count, the lunar day or the day of the half; `null` for `plum-rains`. */
  count: number | null;
}

/** The one line of `hc_night_watch`. */
export interface NightWatch {
  /** 1 (一更) to 5 (五更). */
  watch: number;
  /** The points struck since it began, 0 to 4. */
  points: number;
  name: string;
  localeUsed: string;
  /** 黃昏 to 平旦, in Chinese as the source writes it. */
  hanName: string;
  /** 戌 to 寅. */
  branch: string;
}

/** A rule of the northern sixty-year cycle, Sewell and Dikshit's Art. 59. */
export type BarhaspatyaRule = "surya-siddhanta-bija" | "surya-siddhanta" | "arya-siddhanta";

/** The name of a year of the northern cycle, and the locale used, as `hc_day_extras` gives it. */
export interface BarhaspatyaName {
  /** 1 for Prabhava through 60 for Kṣaya. */
  position: number;
  name: string;
  localeUsed: string;
}

/** The one line of `hc_barhaspatya_year`. */
export interface BarhaspatyaYear extends BarhaspatyaName {
  /** The name the rule expunges in that solar year, or `null`. */
  expunged: number | null;
  expungedName: string | null;
}

/** One line of `hc_planetary_hour` and `hc_planetary_hours_of_day`. */
export interface PlanetaryHour {
  /** The fixed day of the sunrise the planetary day began at; `null` where a sunrise or sunset is missing. */
  day: number | null;
  /** 1 to 24 from sunrise. */
  hour: number | null;
  ruler: ClassicalPlanet | null;
  name: string | null;
  localeUsed: string | null;
  daytime: boolean | null;
  start: number | null;
  end: number | null;
  missing: MissingSolarEvent | null;
}

/** The one line of `hc_gmat_from_gmt` and `hc_gmt_from_gmat`. */
export interface ClockReading {
  fixed: number;
  /** Whole seconds after the day's start: midnight for GMT, noon for GMAT. */
  secondsOfDay: number;
  /** Up to 10¹⁸, so a `bigint`. */
  attoseconds: bigint;
}

/** The one line of `hc_irig_decode`. */
export interface IrigReading {
  fixed: number;
  dayOfYear: number;
  hour: number;
  minute: number;
  /** 60 for a leap second. */
  second: number;
  hundredths: number;
  /** The year's two digits, `null` for a code without them. */
  year: number | null;
  /** The control bits, control bit 1 lowest; `null` for a code without them. */
  control: number | null;
  straightBinarySeconds: number | null;
}

/** One line of `hc_irig_formats`. */
export interface IrigFormatInfo {
  /** The letter, `A`, `B`, `D`, `E`, `G` or `H`. */
  format: string;
  /** The index count interval: 1 000 for A, 60 000 000 for D. */
  indexCountMicroseconds: number;
  /** The index counts in a frame, 100 or 60. */
  indexCounts: number;
  /** The frame's length; a frame begins at each multiple of it from midnight. */
  frameMicroseconds: number;
  /** The fields of the BCD time of year, most significant first: `days`, `hours`, … */
  fields: string[];
  /** The control bits the format has room for. */
  controlBits: number;
  /** The modulation digits Table 4-1 permits. */
  modulations: number[];
  /** The carrier digits it permits. */
  carriers: number[];
  /** The coded expressions it permits. */
  expressions: number[];
}

/** What `irigEncode` puts in a frame besides the second. */
export interface IrigEncodeOptions {
  /** 0 to 99; the formats A and G send them. */
  hundredths?: number;
  /** Control bit 1 lowest. */
  control?: number;
}

/** A rule of 入梅 or 出梅. */
export type PlumRainRule = "ru-mei-bing" | "ru-mei-ren" | "chu-mei-wei";

/** A convention of the day `hc_kalam` divides. */
export type KalamConvention = "rahu-kalam-sunrise" | "rahu-kalam-fixed";

/** A period `hc_kalam` writes. */
export type KalamId = "rahu-kalam" | "yamaganda" | "gulika-kalam";

/** One line of `hc_kalam`. */
export interface KalamPeriod {
  id: KalamId;
  englishName: string;
  /** Named in the locale, राहुकाल under `hi`, else English's. */
  name: string;
  localeUsed: string;
  /** The eighth of the day, 1 to 8. */
  part: number;
  /** `universal`: POSIX seconds; `local`: seconds after the day's own midnight. */
  clock: "universal" | "local";
  start: number | null;
  end: number | null;
  missing: MissingSolarEvent | null;
}

/** The one line of `hc_almanac_cycles`. */
export interface AlmanacCycles {
  /** 恵方's point of the twenty-four: 甲, 庚, 丙 or 壬. */
  eho: string;
  ehoRomaji: string;
  /** Degrees clockwise from north. */
  azimuth: number;
  /** The nearest of the sixteen points in Japanese, 南南東. */
  sixteenPoint: string;
  /** The same in English, `south-south-east`. */
  direction: string;
  /** The 三元九運 period, 1 to 9. */
  period: number;
  periodName: string;
  era: string;
  star: string;
  ruler: string;
  firstYear: number;
  lastYear: number;
  /** Whether the day is 손 없는 날; `null` outside the `dangi` calendar's years. */
  withoutSon: boolean | null;
}

/** What an `hc_almanac_day` line is about. */
export type AlmanacKind =
  | "sexagenary"
  | "twelve-direct"
  | "mansion"
  | "mansion-27"
  | "year-star"
  | "month-star"
  | "day-star"
  | "rokuyo"
  | "lower-register"
  | "selected-day"
  | "combination";

/** One line of `hc_almanac_day`. */
export interface AlmanacAnnotation {
  kind: AlmanacKind;
  /** The 1-based position in the cycle, `"6"` for 赤口, or the entry's identifier, `tenshanichi`. */
  id: string;
  /** The name in the locale: 赤口 under `ja`, `shakkō` under `en`. */
  name: string;
  /** The tag of the data that named it: `ja`, `en`, or `zh-Hant` for a sexagenary day. */
  localeUsed: string;
  /** The Japanese name the almanac prints. */
  japanese: string;
  /** The Hepburn reading; `null` for a combination, which has none. */
  reading: string | null;
  /** Whether the almanac counts the day auspicious; `null` where it gives no verdict. */
  auspicious: boolean | null;
  /** For the 暦注下段, whether an almanac prints the entry; `null` for every other kind. */
  printed: boolean | null;
}

/** A reckoning of the Orthodox fasts. */
export type OrthodoxFastReckoning = "orthodox-fasts" | "orthodox-fasts-revised-julian";

/** The one line of `hc_orthodox_fast_on`. */
export interface OrthodoxFastDay {
  fastDay: boolean;
  status: "period" | "weekly-fast" | "none";
  /** For `period`, its identifier, name and kind; else `null`. */
  period: string | null;
  periodName: string | null;
  /** `meat-excluded` for the Meatfast, when no day is a fast day and none allows meat. */
  kind: "fast" | "fast-free" | "meat-excluded" | null;
  /** What the day abstains from; a day of the Meatfast is not a fast day but abstains from `meat`. */
  abstinence: "nothing" | "meat" | "fast";
}

/** One line of `hc_orthodox_fast_seasons`. */
export interface OrthodoxFastSeason {
  id: string;
  englishName: string;
  kind: "fast" | "fast-free" | "meat-excluded";
  /** The first and last fixed days, both `null` in a year the period does not happen. */
  first: number | null;
  last: number | null;
}

/** A time `hc_prayer_times` writes. */
export type PrayerTimeName =
  | "fajr"
  | "sunrise"
  | "zuhr"
  | "asr-shafii"
  | "asr-hanafi"
  | "maghrib"
  | "isha"
  | "midnight";

/** One line of `hc_prayer_times`. */
export interface PrayerTime {
  time: PrayerTimeName;
  /** POSIX seconds of Universal Time, rounded down, or `null` when the time does not happen. */
  instant: number | null;
  missing: MissingSolarEvent | null;
}

/** One line of `hc_prayer_methods`. */
export interface PrayerMethod {
  id: string;
  englishName: string;
  fajrArcminutes: number;
  /** `null` for a method that takes sunset. */
  maghribArcminutes: number | null;
  /** `null` for a method of an interval. */
  ishaArcminutes: number | null;
  /** `null` for a method of an angle. */
  ishaMinutes: number | null;
  ishaRamadanMinutes: number | null;
  midnight: "sunset-to-sunrise" | "sunset-to-fajr";
  source: string;
}

/** A reckoning of the Jewish day `hc_zmanim` reads. */
export type ZmanimReckoning = "zmanim-gra" | "mga-72-minutes" | "mga-16-1-degrees";

/** A time `hc_zmanim` writes. */
export type ZmanId =
  | "sof-zman-shma"
  | "sof-zman-tfila"
  | "mincha-gedola"
  | "mincha-ketana"
  | "plag-hamincha"
  | "dawn-16-1-degrees"
  | "dawn-72-minutes"
  | "nightfall-8-5-degrees"
  | "nightfall-72-minutes";

/** One line of `hc_zmanim`. */
export interface Zman {
  id: ZmanId;
  /** As Hebcal prints it; `null` for a dawn or a nightfall. */
  englishName: string | null;
  /** Temporal hours from the start of the day; `null` for a dawn or a nightfall. */
  hours: number | null;
  instant: number | null;
  missing: MissingSolarEvent | null;
}

/** The one line of `hc_edo_time`; every field but `missing` is `null` when a dawn or dusk does not happen. */
export interface EdoTime {
  /** The fixed day whose 明け六つ began the reading's day. */
  day: number | null;
  /** From 明け六つ, 0 to 11. */
  hour: number | null;
  /** 明六つ … 暁七つ. */
  name: string | null;
  romaji: string | null;
  strokes: number | null;
  branch: string | null;
  /** The 天保暦's tenths of the hour gone, 0 to 9. */
  tenths: number | null;
  fraction: number | null;
  missing: MissingSolarEvent | null;
}

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
  readonly parseDate: ReadonlyArray<string>;
  readonly dayExtras: ReadonlyArray<string>;
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
  readonly zoneOffset: ReadonlyArray<string>;
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
  readonly ccsdsDecode: ReadonlyArray<string>;
  readonly ccsdsAscii: ReadonlyArray<string>;
  readonly radioDecode: ReadonlyArray<string>;
  readonly unixFromDotnetTicks: ReadonlyArray<string>;
  readonly sixHourClock: ReadonlyArray<string>;
  readonly kalam: ReadonlyArray<string>;
  readonly almanacCycles: ReadonlyArray<string>;
  readonly almanacDay: ReadonlyArray<string>;
  readonly orthodoxFast: ReadonlyArray<string>;
  readonly orthodoxFastSeasons: ReadonlyArray<string>;
  readonly prayerTimes: ReadonlyArray<string>;
  readonly prayerMethods: ReadonlyArray<string>;
  readonly zmanim: ReadonlyArray<string>;
  readonly edoTime: ReadonlyArray<string>;
  readonly choghadiya: ReadonlyArray<string>;
  readonly panchak: ReadonlyArray<string>;
  readonly kumbh: ReadonlyArray<string>;
  readonly pushkaram: ReadonlyArray<string>;
  readonly folkDay: ReadonlyArray<string>;
  readonly nightWatch: ReadonlyArray<string>;
  readonly barhaspatyaYear: ReadonlyArray<string>;
  readonly barhaspatyaYearAt: ReadonlyArray<string>;
  readonly planetaryHour: ReadonlyArray<string>;
  readonly gmat: ReadonlyArray<string>;
  readonly irigDecode: ReadonlyArray<string>;
  readonly irigFormats: ReadonlyArray<string>;
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

/** One line of `hc_day_extras`: one extra field of a day in one calendar. */
export interface DayExtra {
  /** The calendar's identifier. */
  id: string;
  /** The field's identifier, `samvatsara`, `julian-day-number`: a key, never text for a reader. */
  field: string;
  /** Its value. */
  value: number;
  /** What the locale calls the field, else its English label: `Samvatsara (southern reckoning)`, `Julian Day Number`. */
  label: string;
  /** The value as a reader reads it: the name its value holds where it is named, `Parabhava`, else the number in the locale's digits. */
  valueLabel: string;
  /** Whether `describeDay`'s `formatted` already writes the field. */
  inDate: boolean;
  /** The tag of the locale data that answered. */
  localeUsed: string;
}

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
  /** The calendar's extra fields, `{ baktun: "13", katun: "0", ... }`, as the module writes them: keys for a program; `dayExtras` labels them for a reader. */
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

/** The one line of `hc_parse_date`: a written date read back, as `hc_describe_day` writes the day, and the day. */
export interface ParsedDate extends DescribedDay {
  /**
   * The fixed day the text names, or `null` when it names no one day; then
   * `error` says why: `ambiguous`, `two-digit-year`, `year-not-written`,
   * `weekday-mismatch`, `field-mismatch`, `not-recognised`, `empty`, or
   * the calendar's own.
   */
  fixed: number | null;
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
  | "future-event"
  | GeologicRank
  | "archaeological";
export type DeepTimeUnit =
  | "seconds-since-big-bang"
  | "seconds-before-present"
  | "megayears-before-present"
  | "years-before-1950"
  | "years-from-now"
  | "log10-years-from-now";

/** One bound of a deep-time entry. */
export interface DeepTimeBound {
  value: number;
  /** The standard uncertainty. */
  stdDev: number;
  /** The significant figures claimed, or `null` where the table claims none. */
  figures: number | null;
  /** Whether the table marks it approximate: the chart's `~`, or a source's "about". */
  approximate: boolean;
}

/** One line of any deep-time export. */
export interface DeepTimeRow {
  kind: DeepTimeKind;
  /** The entry's stable lower-case kebab identifier, to match on instead of `name`. */
  id: string;
  /** The English name. */
  name: string;
  /** The `id` of the interval one rank up, the region of an archaeological period, the kind of prediction of a future event (`modelled`, `experimental-bound`, `order-of-magnitude`), or `null`. */
  scope: string | null;
  /** The older bound — for a future event the sooner — or `null` where the chronology has no figure. */
  start: DeepTimeBound | null;
  /** The younger bound; the same as `start` for a point in time. */
  end: DeepTimeBound | null;
  /** What the bounds are in. */
  unit: DeepTimeUnit;
  description: string | null;
  source: string;
  /** The geological chart's own name for an interval in the locale's language, or the established term for a cosmic or archaeological row where one was read, or `null`: always `null` for the future rows. */
  localisedName: string | null;
}

/** One bound of an earliest-evidence claim: a standard uncertainty only where the source states one. */
export interface EarliestEvidenceBound extends Omit<DeepTimeBound, "stdDev"> {
  /** The standard uncertainty, or `null` where the source gives none or does not say what its `±` is; the `±` is then in the description. */
  stdDev: number | null;
  /** Whether the source writes the age as approximate: "about", "ca.". */
  approximate: boolean;
}

/** The landmark an earliest-evidence claim is for. */
export type EarliestEvidenceLandmark =
  | "earliest-life"
  | "earliest-homo-sapiens"
  | "earliest-writing"
  | (string & {});

/** One line of `hc_earliest_evidence`: a published claim, dated in the shape its source gives. */
export interface EarliestEvidenceRow extends Omit<DeepTimeRow, "kind" | "scope" | "start" | "end" | "unit"> {
  kind: "earliest-evidence";
  /** The landmark the claim is for. */
  scope: EarliestEvidenceLandmark;
  /** The older limit, or `null` for a minimum age, which has none. */
  start: EarliestEvidenceBound | null;
  /** The younger limit: the age itself, the minimum, or the younger end of a range. */
  end: EarliestEvidenceBound;
  unit: "years-before-1950";
  /** The established term in the locale's language where one was read, or `null`. */
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
  /** Lower case, a hyphen for each space: `viking-1`; what `missionSol` takes. */
  id: MissionId;
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
  /** The name in lower case: `titan`; what `bodyTime` takes. */
  id: BodyId;
  name: string;
  kind: BodyKind;
  /** The identifier of the body it orbits; `null` for the Sun. */
  primary: BodyId | null;
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
  id: GravitatingBodyId;
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
  /** What `gravitationalDilation` takes. */
  id: GravitatingBodyId;
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
  /** The yoga's ayanamsa, as `panchangaAt` takes it; `null` for the karaṇa. */
  ayanamsa: Ayanamsa | null;
  /** Its full name, `Lahiri (Chitrapaksha)`; `null` for the karaṇa. */
  ayanamsaName: string | null;
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

/** Which rules answered for a zone's name. */
export type ZoneRules = "builtin" | "loaded";

/** The one line of `hc_zone_offset`. */
export interface ZoneOffset {
  /** Seconds east of UTC: 3600 for CET, −21600 for MDT. */
  offsetSeconds: number;
  /** Whether the rules call the time daylight saving or summer time. */
  dst: boolean;
  /** The rules' abbreviation, `CET` or `MDT`; `null` where they give a numeric one such as `+0545`. */
  abbreviation: string | null;
  /** The POSIX second of the next change of offset, flag or abbreviation; `null` where the rules have none. */
  nextTransition: number | null;
  /** The offset after it; `null` with it. */
  nextOffsetSeconds: number | null;
  /** The built-in POSIX footers, or a TZif file given to `loadZone`. */
  rules: ZoneRules;
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
  /**
   * The one ISO 3166-1 code `zone.tab` lists the name under, for a label: `JP`
   * for `Asia/Tokyo`, whose `countries` are `JP` and `AU`; `UA` for
   * `Europe/Simferopol`, whose `countries` are `RU` and `UA`. `null` for a name
   * `zone.tab` has no row for, which no name of release 2026d is.
   */
  country: string | null;
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
  | "italian-zero-hour"
  | "japanese-dawn-kansei"
  | "japanese-dusk-kansei"
  | "japanese-dawn-naoj"
  | "japanese-dusk-naoj";

/** The solar event a reckoning needs and does not have. */
export type MissingSolarEventName = "sunrise" | "sunset" | "depression" | "no-noon-shadow";

export interface MissingSolarEvent {
  event: MissingSolarEventName;
  /** The local day it is missing on, as a fixed day. */
  day: number;
  /**
   * For `depression`, the depression sought in arcminutes; `null` for any
   * other event, and for a depression that is not a whole number of
   * arcminutes, the Japanese dawn and dusk's.
   */
  depressionArcminutes: number | null;
  /**
   * For `depression`, the depression sought in arcseconds, from the lines
   * that carry the cell (`hc_solar_event` and the lines after it); else
   * `null`.
   */
  depressionArcseconds: number | null;
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
  /** A short English name for a label: `geometric dip`, `USNO`, `Calendrical Calculations`. */
  shortName: string;
  /** The name in the locale, where an observatory or almanac office gives one; else `englishName`. */
  name: string;
  /** The tag of the data that named it: `zh-Hant`, `fr`, or `en` for the English name. */
  localeUsed: string;
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
  /**
   * The month's name in the locale, as `describeDay` names it for `hindu-lunar`:
   * `Bhadra`, भाद्रपद under `hi`; an intercalary month with the locale's word
   * before it, `Adhika Sravana`. `null` where the locale's data has no name.
   */
  monthName: string | null;
  /** The locale's word for an intercalary month, `Adhika` or अधिक, where this month is one; else `null`. */
  leapMonthWord: string | null;
  /** The Śaka era's name in the locale, शक under `hi`, else English's `Saka`, as in Sanskrit. */
  sakaEra: string | null;
  /** The Vikrama Saṃvat's name in the locale, else English's `Vikrama Samvat`. */
  vikramaEra: string | null;
  /** The tag of the data that named the month: `en`, `hi`, `sa`. */
  localeUsed: string;
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

/**
 * A crescent-visibility criterion `hc_crescent_visible` names: `shaukat`,
 * the arc of light and altitude with the Sun 4.5° down; `yallop`, his
 * *q*-test, and `odeh`, his *V*, both at Bruin's best time; `saudi-rule`,
 * the Moon past conjunction and setting after the Sun; `istanbul-2016` and
 * `khgt`, an elongation of 8° and an altitude of 5° at sunset, the altitude
 * topocentric for the first and geocentric for the second; and the two
 * readings of Neo-MABIMS's 3° and 6.4° at sunset.
 */
export type CrescentCriterion =
  | "shaukat"
  | "yallop"
  | "saudi-rule"
  | "odeh"
  | "istanbul-2016"
  | "khgt"
  | "mabims-2021-topocentric"
  | "mabims-2021-geocentric-elongation";

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
  /** `hc_ccsds_decode`: a Level 2, 3 or 4 code is `no-data`, octets that are not a code `malformed`. */
  ccsdsDecode(hex: string, strict?: boolean): CcsdsCode;
  /** `hc_ccsds_encode`: the code in lower-case hexadecimal, P-field first. */
  ccsdsEncode(taiSeconds: number | bigint, attoseconds: number | bigint, pField: string, strict?: boolean): string;
  /** `hc_ccsds_ascii_parse`. */
  ccsdsAsciiParse(code: string, strict?: boolean): CcsdsAsciiCode;
  /** `hc_ccsds_ascii_format`: `precision` `hour`, `minute`, `second` or `1` to `18`; `terminator` unless `false`. */
  ccsdsAsciiFormat(
    taiSeconds: number | bigint,
    attoseconds: number | bigint,
    variation: "a" | "b",
    precision: string | number,
    terminator?: boolean,
    strict?: boolean,
  ): string;
  /** `hc_radio_decode`: JJY's call-sign frame is `no-data`. */
  radioDecode(code: RadioCode, frame: string, century: number | bigint): RadioMinute;
  /** `hc_radio_encode`: a frame of `0`, `1` and `M`. */
  radioEncode(code: RadioCode, unixSeconds: number | bigint, options?: RadioEncodeOptions): string;
  /**
   * `hc_irig_decode`: a designation Table 4-1 does not permit is `unknown`,
   * a frame that is not the code's `malformed`; the year's two digits are
   * read in the century of `year`.
   */
  irigDecode(signal: string, frame: string, year: number | bigint): IrigReading;
  /** `hc_irig_encode`: a reading the format has no frame at is `out-of-range`. */
  irigEncode(signal: string, fixed: number | bigint, secondsOfDay: number, options?: IrigEncodeOptions): string;
  /** `hc_irig_formats`: the six formats, A first. */
  irigFormats(): IrigFormatInfo[];
  /** `hc_dotnet_ticks_from_unix`: the ticks, which need not fit a number. */
  dotnetTicksFromUnix(unixSeconds: number | bigint, attoseconds?: number | bigint): bigint;
  /** `hc_unix_from_dotnet_ticks`. */
  unixFromDotnetTicks(ticks: number | bigint): DotnetReading;
  /** `hc_six_hour_clock`: `secondsOfDay` of the caller's wall clock. */
  sixHourClock(reckoning: SixHourReckoning, secondsOfDay: number): SixHourReading;
  /** `hc_civil_from_six_hour_clock`: seconds after civil midnight. */
  civilFromSixHourClock(reckoning: SixHourReckoning, hour: number, minute: number, second: number, night: boolean): number;

  /**
   * `hc_describe_day`: the day in every registered calendar, in registry
   * order. `locale` is a BCP 47 tag, `und` unless given, as the module.
   */
  describeDay(fixed: number | bigint, locale?: string): DescribedDay[];
  /**
   * `hc_day_extras`: the day's extra fields, labelled, in every registered
   * calendar or with `id` in one; `locale` as for `describeDay`.
   */
  dayExtras(fixed: number | bigint, locale?: string, id?: string): DayExtra[];
  /** The days from `from` up to but not including `to` as one calendar's eras, years, months or days. */
  calendarUnits(id: string, unit: Unit | number, from: number | bigint, to: number | bigint, locale?: string): CalendarUnit[];
  /**
   * `hc_parse_date`: a date as `locale` writes it in `calendar`, read back;
   * a calendar the registry does not carry is `unknown`.
   */
  parseDate(calendar: string, locale: string, text: string): ParsedDate;
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
  panchangaAt(unixSeconds: number | bigint, ayanamsa: Ayanamsa): PanchangaLimb[];
  /** `hc_panchanga_of_day`: read at the day's sunrise at the place; no sunrise is `no-data`. */
  panchangaOfDay(fixed: number | bigint, latitude: number, longitude: number, elevation: number, ayanamsa: Ayanamsa): PanchangaLimb[];
  /**
   * `hc_hindu_lunar_date`: `sky` an ayanamsa or `surya-siddhanta`. A
   * place beyond 65° of latitude, or off the globe, is `out-of-range` on
   * either sky, as is a day outside Śaka 1622 through 2221 on the true sky
   * and outside Kali Yuga 1 to 10 000 on the Siddhānta's; on the true sky a
   * day whose sunrise at the place the model does not find is `no-data`. The
   * month and the eras are named in `locale`, `und` unless given, which is
   * English.
   */
  hinduLunarDate(
    sky: Ayanamsa | "surya-siddhanta",
    fixed: number | bigint,
    latitude: number,
    longitude: number,
    elevation?: number,
    locale?: string,
  ): HinduLunarDate;
  /** `hc_surya_siddhanta_at`; an instant outside Kali Yuga 1 to 10 000 is `out-of-range`. */
  suryaSiddhantaAt(unixSeconds: number | bigint): SuryaSiddhantaSky;
  /** `hc_surya_siddhanta_sunrise`: POSIX seconds; a place beyond 65° of latitude is `out-of-range`. */
  suryaSiddhantaSunrise(fixed: number | bigint, latitude: number, longitude: number): number;
  /** `hc_barhaspatya_year`: an expired Śaka year, −3178 to 6821. */
  barhaspatyaYear(rule: BarhaspatyaRule, saka: number | bigint, locale?: string): BarhaspatyaYear;
  /** `hc_barhaspatya_year_at`: the name in progress at an instant. */
  barhaspatyaYearAt(rule: BarhaspatyaRule, unixSeconds: number | bigint, locale?: string): BarhaspatyaName;
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
  /** `hc_kalam`: three lines, Rāhu kālam, Yamaganda and Gulika kālam. */
  kalam(
    convention: KalamConvention,
    fixed: number | bigint,
    latitude: number,
    longitude: number,
    elevation?: number,
    locale?: string,
  ): KalamPeriod[];
  /** `hc_almanac_cycles`: 立春 at `meridian`, as for `termInEffect`. */
  almanacCycles(fixed: number | bigint, meridian?: string): AlmanacCycles;
  /** `hc_almanac_day`: the solar terms and new moons at `meridian`; `und` unless a locale is given. */
  almanacDay(fixed: number | bigint, meridian?: string, locale?: string): AlmanacAnnotation[];
  /** `hc_choghadiya`: sixteen lines, the day's eight then the night's. */
  choghadiya(fixed: number | bigint, latitude: number, longitude: number, elevation?: number, locale?: string): ChoghadiyaPart[];
  /** `hc_panchak`: the weekday of the opening on a clock `offsetSeconds` ahead of UTC, 0 unless given. */
  panchak(naming: PanchakNaming, unixSeconds: number | bigint, ayanamsa: Ayanamsa, offsetSeconds?: number, locale?: string): PanchakWindow;
  /** `hc_kumbh`: `jupiter` is the caller's, the library having no ephemeris of Jupiter; empty unless given. */
  kumbh(yoga: KumbhYoga, year: number | bigint, ayanamsa: Ayanamsa, jupiter?: SiderealSignId | "", locale?: string): KumbhOccasion;
  /** `hc_pushkaram`: Jupiter's sign and the moment it enters it are the caller's. */
  pushkaram(
    sign: SiderealSignId,
    entryUnixSeconds: number | bigint,
    latitude: number,
    longitude: number,
    elevation?: number,
    meridian?: string,
    locale?: string,
  ): PushkaramDays[];
  /** `hc_folk_day`: 入梅 and 出梅 at `meridian`, `china` for the published days. */
  folkDay(fixed: number | bigint, meridian?: string, locale?: string): FolkDay[];
  /** `hc_night_watch`: `null` from 05:00 to 18:59. */
  nightWatch(secondsOfDay: number, locale?: string): NightWatch | null;

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
  /** `hc_orthodox_fast_on`: a day outside the years 326 to 4099 is `out-of-range`. */
  orthodoxFastOn(reckoning: OrthodoxFastReckoning, fixed: number | bigint): OrthodoxFastDay;
  /** `hc_orthodox_fast_seasons`: twelve periods, in the order a day is tested against them. */
  orthodoxFastSeasons(reckoning: OrthodoxFastReckoning, year: number | bigint): OrthodoxFastSeason[];

  /** `hc_term_in_effect`; a meridian nobody knows is `unknown`. */
  termInEffect(fixed: number | bigint, meridian?: Meridian): TermInEffect;
  /** `hc_pentad_in_effect`. */
  pentadInEffect(fixed: number | bigint, meridian?: Meridian): TermInEffect;
  /** `hc_cold_food_day`; a reckoning nobody knows is `unknown`, a year outside −999 to 3000 `out-of-range`. */
  coldFoodDay(convention: ColdFoodConvention, year: number | bigint): number;
  /** `hc_plum_rains`: at `meridian`, `china` for the published days; a year outside −1000 to 3000 is `out-of-range`. */
  plumRains(rule: PlumRainRule, year: number | bigint, meridian?: string): number;

  /** `hc_place_years_ago`; a value the crate refuses is `out-of-range`. `locale` names the rows it has names for, `und` unless given. */
  placeYearsAgo(yearsAgo: number, stdDevYears?: number, locale?: string): DeepTimeRow[];
  /** `hc_cosmic_events`. */
  cosmicEvents(locale?: string): DeepTimeRow[];
  /** `hc_earliest_evidence`. */
  earliestEvidence(locale?: string): EarliestEvidenceRow[];
  /** `hc_archaeological_periods`. */
  archaeologicalPeriods(locale?: string): DeepTimeRow[];
  /** `hc_future_events`. */
  futureEvents(locale?: string): DeepTimeRow[];
  /** `hc_geologic_intervals`; the rank by name or by number from 0. */
  geologicIntervals(rank: GeologicRank | number, locale?: string): DeepTimeRow[];

  /**
   * `hc_fixed_from_unix_in_zone`: the instants of the years −9 999 994 to
   * 9 999 994 by UTC, the ones a zone's rules answer for; outside them,
   * `out-of-range`, and a zone nobody knows is `unknown`.
   */
  fixedFromUnixInZone(unixSeconds: number | bigint, zone: string): number;
  /**
   * `hc_unix_from_fixed_in_zone`: the days of the years −9 999 994 to
   * 9 999 994, fixed days −3 652 423 173 to 3 652 422 808; outside them,
   * `out-of-range`.
   */
  unixFromFixedInZone(fixed: number | bigint, zone: string): number;
  /** `hc_zone_load`; bytes that are not TZif are `malformed`. */
  loadZone(name: string, tzif: Uint8Array | ArrayBuffer): void;
  /** `hc_zones`: every zone of `zone1970.tab`, in its order; `und` unless given. */
  zones(locale?: string): ZoneLocation[];
  /** `hc_zone_location`: a zone or a link; a name that places nothing, such as `UTC`, is `unknown`. */
  zoneLocation(zone: string, locale?: string): ZoneLocation;
  /**
   * `hc_zone_offset`: from the rules `fixedFromUnixInZone` reads, for its
   * instants; outside them, `out-of-range`, and a zone nobody knows is `unknown`.
   */
  zoneOffset(zone: string, unixSeconds: number | bigint): ZoneOffset;

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
  /** `hc_horizons`; `und`, which names every horizon in English, unless a locale is given. */
  horizons(locale?: string): Horizon[];
  /** `hc_sunrise`; a missing sunrise is an answer, not an error. */
  sunrise(horizon: HorizonId, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): SolarCrossing;
  /** `hc_sunset`. */
  sunset(horizon: HorizonId, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): SolarCrossing;
  /** `hc_hjd_tt`: right ascension and declination in degrees, J2000. */
  hjdTt(ttJulianDate: number, rightAscension: number, declination: number): HeliocentricJulianDate;
  /** `hc_hjd_utc`; under `strict` a date outside the leap-second table is `no-data`. */
  hjdUtc(utcJulianDate: number, rightAscension: number, declination: number, strict?: boolean): HeliocentricJulianDateUtc;
  /** `hc_gmat_from_gmt`: 23:59:60 has no reading twelve hours earlier and is `out-of-range`. */
  gmatFromGmt(fixed: number | bigint, secondsOfDay: number, attoseconds?: number | bigint): ClockReading;
  /** `hc_gmt_from_gmat`. */
  gmtFromGmat(fixed: number | bigint, secondsOfDay: number, attoseconds?: number | bigint): ClockReading;
  /** `hc_prayer_times`: eight lines; a time the Sun does not reach is an answer, not an error. */
  prayerTimes(method: string, fixed: number | bigint, latitude: number, longitude: number, elevation?: number, ramadan?: boolean): PrayerTime[];
  /** `hc_prayer_methods`. */
  prayerMethods(): PrayerMethod[];
  /** `hc_zmanim`: nine lines, the five times in temporal hours, then the dawns and nightfalls. */
  zmanim(reckoning: ZmanimReckoning, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): Zman[];
  /** `hc_edo_time`: by the 寛政暦's 明け六つ and 暮れ六つ. */
  edoTime(unixSeconds: number | bigint, latitude: number, longitude: number, elevation?: number): EdoTime;
  /** `hc_unix_from_edo_time`: `hour` 0 to 11 from 明け六つ, `fraction` 0 up to 1. */
  unixFromEdoTime(fixed: number | bigint, hour: number, fraction: number, latitude: number, longitude: number, elevation?: number): SolarEvent;
  /** `hc_planetary_hour`: every cell but `missing` is `null` where a sunrise or sunset is missing. */
  planetaryHour(unixSeconds: number | bigint, latitude: number, longitude: number, elevation?: number, locale?: string): PlanetaryHour;
  /** `hc_planetary_hours_of_day`: twenty-four lines. */
  planetaryHoursOfDay(fixed: number | bigint, latitude: number, longitude: number, elevation?: number, locale?: string): PlanetaryHour[];

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
  missionSol(mission: MissionId, unixSeconds: number): number;
  /** `hc_bodies`. */
  bodies(): Body[];
  /** `hc_body_time`; the Sun is `no-data`. */
  bodyTime(body: BodyId, unixSeconds: number, eastLongitude?: number): BodyTime;
  /** `hc_circad_date`; an instant more than 100 Julian years from J2000.0 is `out-of-range`. */
  circadDate(calendar: CircadCalendar, unixSeconds: number): CircadDate;
  /** `hc_proper_time`; a speed at or beyond light is `out-of-range`. */
  properTime(speedMetresPerSecond: number, coordinateSeconds: number): ProperTime;
  /** `hc_gravitational_dilation`; a radius at or inside the horizon is `out-of-range`. */
  gravitationalDilation(body: GravitatingBodyId, radiusMetres: number): GravitationalDilation;
  /** `hc_gravitating_bodies`. */
  gravitatingBodies(): GravitatingBody[];
}

/**
 * Instantiate the module and bind it. `source` may also be a promise of
 * any accepted form.
 */
export function load(source: ModuleSource | Promise<ModuleSource>, options?: LoadOptions): Promise<HyperCalendar>;
