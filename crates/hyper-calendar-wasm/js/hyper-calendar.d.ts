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
  | "jupiter"
  | "planetary"
  | "relativity"
  | "places"
  | "humanize"
  | "natural"
  | "datetime"
  | "patterns"
  | "zone-names"
  | "uncertainty"
  | "units"
  | "fiscal"
  | "name-days"
  | "attributes";

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

/**
 * The epoch a Level 2 CCSDS code counts from; each part absent is 0. A
 * Level 1 code counts from 1958 January 1, whatever the epoch.
 */
export interface CcsdsEpoch {
  /** A CUC code's count 0, as TAI seconds from 1970-01-01 00:00:00 TAI. */
  taiSeconds?: number | bigint;
  attoseconds?: number | bigint;
  /** A CDS code's day 0, as a POSIX day: −7305 for 1950 January 1. */
  unixDay?: number | bigint;
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

/** The line of `hc_jjy_call_sign_decode`: a call-sign frame's minute and stop notice. */
export interface JjyCallSign {
  /** The POSIX second of the frame's first marker. */
  unixSeconds: number;
  /** The minute's fixed day in JST. */
  fixed: number;
  hour: number;
  /** 15 or 45. */
  minute: number;
  /** ST1–ST3: 0 no stop planned, 1 within seven days, 2 within three to six, 3 within two, 4 within 24 hours, 5 within 12, 6 within 2. */
  stopStart: number;
  /** ST4: a stop by day only. */
  daytimeOnly: boolean;
  /** ST5–ST6: 0 no stop planned, 1 seven days or more or not known, 2 two to six days, 3 less than two. */
  stopSpan: number;
}

/** The stop notice `jjyCallSignEncode` writes; each absent is 0 or false. */
export interface JjyStopNotice {
  stopStart?: 0 | 1 | 2 | 3 | 4 | 5 | 6;
  daytimeOnly?: boolean;
  stopSpan?: 0 | 1 | 2 | 3;
}

/** What `radioEncode` puts in a frame besides the minute. */
export interface RadioEncodeOptions {
  /** 1 for a second inserted, −1 for one omitted (`jjy` and `wwvb-pm` only), 0 for none. */
  leap?: -1 | 0 | 1;
  /**
   * DCF77's zone, required for `dcf77`; WWVB's summer-time state, required for
   * both WWVB codes; absent for `jjy`. Or `zone:` and a zone's name,
   * `zone:Europe/Berlin` or `zone:America/Denver`, in a module built with
   * `tz` too, to read the state, and DCF77's A1, from the rules
   * `fixedFromUnixInZone` reads for that name; a zone the built-in table
   * lacks once `loadZone` has its TZif file.
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
export type Ayanamsa =
  | "lahiri"
  | "lahiri-rashtriya"
  | "lahiri-crc-1955"
  | "lahiri-drik"
  | "raman"
  | "krishnamurti"
  | "reingold-dershowitz"
  | "fagan-bradley";

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

/** A tropical sign, by the lower case of its English name. */
export type TropicalSignId =
  | "aries" | "taurus" | "gemini" | "cancer" | "leo" | "virgo"
  | "libra" | "scorpio" | "sagittarius" | "capricorn" | "aquarius" | "pisces";

/** A nakṣatra, by the lower-case ASCII form of its name, words joined by a hyphen. */
export type NakshatraId =
  | "ashvini" | "bharani" | "krittika" | "rohini" | "mrigashirsha" | "ardra"
  | "punarvasu" | "pushya" | "ashlesha" | "magha" | "purva-phalguni" | "uttara-phalguni"
  | "hasta" | "chitra" | "svati" | "vishakha" | "anuradha" | "jyeshtha" | "mula"
  | "purva-ashadha" | "uttara-ashadha" | "shravana" | "dhanishtha" | "shatabhisha"
  | "purva-bhadrapada" | "uttara-bhadrapada" | "revati";

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
  /** The sign Jupiter must be in, and the Sun, each with its Sanskrit name, `Vṛṣabha`. */
  jupiter: SiderealSignId;
  jupiterName: string;
  sun: SiderealSignId;
  sunName: string;
  atNewMoon: boolean;
  /** The Sun's entry into its sign and into the next, or the new moon twice; `null` when no new moon falls in the stay. */
  from: number | null;
  to: number | null;
  /** Whether the caller's Jupiter meets the condition; `null` when none was given. */
  holds: boolean | null;
}

/** One line of `hc_kumbh_yogas`: what a condition is. */
export interface KumbhYogaInfo {
  id: KumbhYoga;
  site: string;
  siteName: string;
  localeUsed: string;
  river: string;
  jupiter: SiderealSignId;
  jupiterName: string;
  sun: SiderealSignId;
  sunName: string;
  atNewMoon: boolean;
  /** Where the condition comes from. */
  source: string;
}

/** One line of `hc_pushkaram_rivers`. */
export interface PushkaramRiver {
  /** `pushkaram-ganga` to `pushkaram-pranahita`. */
  id: string;
  name: string;
  localeUsed: string;
  /** Where the source keeps the river for the sign, where it names a region. */
  region: string | null;
  sign: SiderealSignId;
  signName: string;
  /** Where the pairing comes from. */
  source: string;
}

/** One line of `hc_pushkaram_rules`. */
export interface PushkaramRule {
  id: PushkaramEntryRule;
  /** Which entry the rule counts. */
  description: string;
}

/** One line of `hc_jupiter_stations`. */
export interface JupiterStation {
  /** POSIX seconds, rounded down. */
  moment: number;
  /** `retrograde` where Jupiter turns back (vakri), `direct` where it resumes (mārgī). */
  kind: "retrograde" | "direct";
  sign: SiderealSignId;
  signName: string;
  /** Degrees, in the zodiac of the ayanāṃśa asked for. */
  siderealLongitude: number;
}

/** One line of `hc_pushkaram`: a river's twelve days. *//** One line of `hc_pushkaram`: a river's twelve days. */
export interface PushkaramDays {
  /** `pushkaram-ganga` to `pushkaram-pranahita`. */
  id: string;
  name: string;
  localeUsed: string;
  /** In English, where the source names one. */
  region: string | null;
  sign: SiderealSignId;
  /** Its Sanskrit name, `Siṃha`. */
  signName: string;
  /** Fixed days; `null` where the Sun does not set on the day of the entry. */
  first: number | null;
  last: number | null;
  missing: MissingSolarEvent | null;
}

/** The one line of `hc_jupiter_at`: Jupiter's position at an instant, tropical and sidereal. */
export interface JupiterPosition {
  /** Apparent geocentric ecliptic longitude in degrees, true equinox of the date: the tropical one. */
  longitude: number;
  latitude: number;
  /** In astronomical units. */
  distance: number;
  /** The longitude less the ayanāṃśa, in degrees. */
  siderealLongitude: number;
  sign: SiderealSignId;
  /** Its Sanskrit name, `Vṛṣabha`. */
  signName: string;
  degreesIntoSign: number;
  /** The longitude's change in degrees a day; negative in retrograde. */
  dailyMotion: number;
  retrograde: boolean;
  /** Geometric, in the mean ecliptic and equinox of the date. */
  heliocentricLongitude: number;
  heliocentricLatitude: number;
  /** In astronomical units. */
  heliocentricDistance: number;
}

/** One line of `hc_jupiter_ingresses`: Jupiter's crossing of a sidereal boundary. */
export interface JupiterIngress {
  /** POSIX seconds of Universal Time, rounded down. */
  moment: number;
  from: SiderealSignId;
  fromName: string;
  to: SiderealSignId;
  toName: string;
  /** `forward` into the next sign, `retrograde` back into the one before. */
  direction: "forward" | "retrograde";
}

/** One line of `hc_jupiter_risings`: a heliacal rising of Jupiter and the year it begins. */
export interface JupiterRising {
  /** POSIX seconds of Universal Time, rounded down. */
  rising: number;
  /** The setting before it, when Jupiter came within 11° east of the Sun. */
  setting: number;
  /** Jupiter's sidereal longitude at the rising, in degrees. */
  siderealLongitude: number;
  /** The nakṣatra, 1 Aśvinī to 27 Revatī, with its identifier and its name. */
  nakshatra: number;
  nakshatraId: string;
  nakshatraName: string;
  /** The year of Jupiter's name by the Bṛhatsaṃhitā, as the twelve-year cycle spells it: `Karttika`. */
  year: string;
  /** From 1, Chaitra, to 12, Phālguna. */
  yearPosition: number;
}

/** The one line of `hc_kumbh_by_sky`: `hc_kumbh`'s, with Jupiter computed. */
export interface KumbhBySky extends Omit<KumbhOccasion, "holds"> {
  /** Whether there is an occasion and Jupiter is in the condition's sign at its first moment; never `null`. */
  holds: boolean;
  /** The sidereal sign Jupiter is in at the occasion's first moment; `null` when there is none. */
  jupiterThen: SiderealSignId | null;
  /** Its sidereal longitude then, in degrees. */
  jupiterLongitudeThen: number | null;
}

/** Which entry of Jupiter into a sign the Pushkaram follows. */
export type PushkaramEntryRule = "pushkaram-final-entry" | "pushkaram-first-entry";

/** One line of `hc_pushkaram_by_sky` and `hc_pushkarams_in_year`: `hc_pushkaram`'s, then the entry found and the rule. */
export interface PushkaramBySky extends PushkaramDays {
  /** POSIX seconds of Universal Time of Jupiter's entry, rounded down. */
  entry: number;
  rule: PushkaramEntryRule;
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

/** The one line of `hc_barhaspatya_year_at`. */
export interface BarhaspatyaNameAt extends BarhaspatyaName {
  /** The twelve-year cycle's saṃvatsara Table XII couples with it, 1 for Chaitra to 12 for Phālguna. */
  twelveYear: number;
  /** Its name as the table spells it, `Asvina`. */
  twelveYearName: string;
  /** The sign of Jupiter's mean longitude while the name is current, and its Sanskrit name, `Meṣa`. */
  meanSign: SiderealSignId;
  meanSignName: string;
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

/** The line of `hc_irig_frame_start`. */
export interface IrigFrameStart {
  secondsOfDay: number;
  hundredths: number;
  frameMicroseconds: number;
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
  | "nayin"
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

/** A god of `hc_almanac_directions`: the 八将神, then 金神, 大金神 and 姫金神. */
export type AlmanacGodId =
  | "taisai"
  | "daishogun"
  | "daion"
  | "saikyo"
  | "saiha"
  | "saisetsu"
  | "oban"
  | "hyobi"
  | "konjin"
  | "dai-konjin"
  | "hime-konjin"
  | WanderingRuleId
  | "konjin-rest-day";

/** A reading of the 遊行 of 大将軍 or 金神. */
export type WanderingRuleId = "daishogun-iinippon" | "konjin-wikipedia-begun-in-season" | "konjin-wikipedia-days-in-season";

/** One line of `hc_almanac_directions`. */
export interface AlmanacDirection {
  id: AlmanacGodId;
  /** The god's name, 太歳神. */
  name: string;
  /** The Hepburn reading the National Diet Library gives; `null` for the three 金神. */
  reading: string | null;
  /** The direction as an earthly branch, 午, or 中央 for a god gone to the middle of the house; `null` for a god at home. */
  branch: string | null;
  /** The branch's azimuth in degrees clockwise from north; `null` with it or for 中央. */
  azimuth: number | null;
  /**
   * What the Library says the god forbids or favours, in English; `null` for the 金神. On a
   * 遊行 line, `home` or `gone`, `null` where the rule does not say.
   */
  meaning: string | null;
  /** The 干支 year in force on the day, turning at 立春: 丙午. */
  year: string;
  /** The branch's number, 1 for 子 to 12 for 亥; `null` with `azimuth`. */
  branchNumber: number | null;
}

/** A reckoning of 臘日 `hc_rounichi` takes. */
export type RounichiRule =
  | "second-dragon-after-minor-cold"
  | "second-dragon-from-minor-cold"
  | "dragon-nearest-major-cold-earlier"
  | "dragon-nearest-major-cold-later"
  | "first-dog-after-major-cold"
  | "first-dog-from-major-cold"
  | "lunar-twelfth-ninth"
  | "ox-month-ninth-from-minor-cold"
  | "ox-month-ninth-after-minor-cold"
  | "third-dog-after-winter-solstice"
  | "third-dog-from-winter-solstice";

/** A publisher's list of the undertakings of each 二十八宿. */
export type UndertakingListId = "saijigoyomi" | "linderabell";

/** A grade of `hc_mansion_undertakings`: 大吉, 吉, 凶, 大凶, or the list's remark. */
export type UndertakingGrade = "best" | "favoured" | "avoided" | "worst" | "note";

/** One line of `hc_mansion_undertakings`. */
export interface MansionUndertaking {
  /** The mansion's number, 1 for 角 to 28 for 軫. */
  mansion: number;
  /** The mansion's name, 角. */
  mansionName: string;
  grade: UndertakingGrade;
  /** The undertaking, or the remark, as the publisher prints it. */
  undertaking: string;
}

/** The kind of a line of `hc_almanac_person_days`. */
export type AlmanacPersonKind = "grave-day" | "three-evil-day";

/** One line of `hc_almanac_person_days`. */
export interface AlmanacPersonDay {
  kind: AlmanacPersonKind;
  /** The reading's identifier, `gomunichi-wikipedia`, or the entry's, `taikanichi`. */
  id: string;
  /** The entry's name, 五墓日 or 大禍日. */
  name: string;
  /** The 干支 of the person's grave day, 乙丑, or the branch of the 節月 of their evil days, 巳. */
  keeps: string;
  /** Whether the day is that entry for the person. */
  applies: boolean;
}

/** A version of the Tibetan calendar, by its registry identifier. */
export type TibetanCalendarId =
  | "tibetan"
  | "tibetan-tsurphu"
  | "tibetan-bhutan"
  | "mongolian"
  | "tibetan-lochen"
  | "tibetan-tsurphu-karana"
  | "tibetan-bhutan-lochen";

/** The kind of a line of `hc_tibetan_almanac_day`. */
export type TibetanAlmanacKind =
  | "weekday"
  | "mansion"
  | "yoga"
  | "karana"
  | "half-day"
  | "sun"
  | "mean-sun"
  | "rahu"
  | "rab-byung"
  | "royal-year"
  | "year-symbol"
  | "month-symbol"
  | "day-symbol"
  | "lunar-day-animal"
  | "lunar-day-element"
  | "lunar-day-trigram"
  | "lunar-day-number"
  | "day-trigram"
  | "day-number-janson"
  | "day-number-henning"
  | "chinese-mansion"
  | "element-pair";

/** One line of `hc_tibetan_almanac_day`. */
export interface TibetanAlmanacEntry {
  kind: TibetanAlmanacKind;
  /**
   * The component's number, the half-day, the rab byung position, the royal year or the animal's number; for
   * the attributes, the animal's, element's, trigram's, number's or Chinese mansion's number, and for
   * `element-pair` the weekday's, 1 for Saturday.
   */
  id: number | null;
  /** The Sanskrit or English name, or for a symbol the element and animal, `Water-Snake`; for a trigram its Tibetan name (`li`), for a number its colour, for `element-pair` the weekday's element. */
  name: string | null;
  /** The Tibetan name in Wylie; `null` for a symbol; for a trigram its Chinese name, for a number its element, for `element-pair` the mansion's element. */
  tibetan: string | null;
  /** The almanac's reading, `2;11,24`, or for a symbol the gender; for a trigram or a number its direction. */
  reading: string | null;
  /** The reading as a decimal, or for a symbol the element's colour; for a trigram the attribute Janson calls its element, for `lunar-day-element` its colour. */
  value: string | null;
}

/** A planet of the Phugpa almanac. */
export type TibetanPlanetId = "mercury" | "venus" | "mars" | "jupiter" | "saturn";

/** One line of `hc_tibetan_planets`. */
export interface TibetanPlanet {
  planet: TibetanPlanetId;
  particularDay: number;
  /** The longitudes in lunar mansions as the almanac reads them, `23;4,36`. */
  meanHeliocentric: string;
  trueSlow: string;
  fast: string;
  meanHeliocentricValue: number;
  trueSlowValue: number;
  fastValue: number;
}

/** The line of `hc_bhutanese_winter_solstice`. */
export interface BhutaneseWinterSolstice {
  fixed: number;
  /** The weekday and time the almanac prints, `2;51,38`. */
  reading: string;
  julianDate: number;
}

/** A rule for the day a festival on a skipped or repeated date is kept. */
export type TibetanFestivalRule = "berzin" | "henning-almanac";

/** A reckoning of the Orthodox fasts. */
export type OrthodoxFastReckoning =
  | "orthodox-fasts"
  | "orthodox-fasts-revised-julian"
  | "armenian-fasts"
  | "armenian-fasts-jerusalem"
  | "armenian-fasts-fifty-days"
  | "coptic-fasts"
  | "ethiopian-fasts";

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

/** The one line of `hc_temporal_hour`. */
export interface TemporalHour {
  reckoning: ZmanimReckoning;
  /** The hour's length in seconds; `null` where the day's start or end does not happen. */
  seconds: number | null;
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
  readonly reading: ReadonlyArray<string>;
  readonly formattedDatetime: ReadonlyArray<string>;
  readonly formattedIsoDate: ReadonlyArray<string>;
  readonly isoDateParts: ReadonlyArray<string>;
  readonly isoDuration: ReadonlyArray<string>;
  readonly formattedIsoDuration: ReadonlyArray<string>;
  readonly isoInterval: ReadonlyArray<string>;
  readonly patternFields: ReadonlyArray<string>;
  readonly localeChain: ReadonlyArray<string>;
  readonly localeInfo: ReadonlyArray<string>;
  readonly pluralCategory: ReadonlyArray<string>;
  readonly names: ReadonlyArray<string>;
  readonly caseText: ReadonlyArray<string>;
  readonly isolated: ReadonlyArray<string>;
  readonly gregorianAdoption: ReadonlyArray<string>;
  readonly holidaysInYear: ReadonlyArray<string>;
  readonly holidaysOn: ReadonlyArray<string>;
  readonly term: ReadonlyArray<string>;
  readonly pentadTraditions: ReadonlyArray<string>;
  readonly pentadInTradition: ReadonlyArray<string>;
  readonly zassetsu: ReadonlyArray<string>;
  readonly seasonalDay: ReadonlyArray<string>;
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
  readonly tithi: ReadonlyArray<string>;
  readonly tithisOfDay: ReadonlyArray<string>;
  readonly ayanamsaTable: ReadonlyArray<string>;
  readonly ayanamsaValue: ReadonlyArray<string>;
  readonly festivalReading: ReadonlyArray<string>;
  readonly janmashtami: ReadonlyArray<string>;
  readonly vaishnavaDay: ReadonlyArray<string>;
  readonly vishtiFreeSpan: ReadonlyArray<string>;
  readonly rahuAt: ReadonlyArray<string>;
  readonly rahuIngress: ReadonlyArray<string>;
  readonly marriageAugury: ReadonlyArray<string>;
  readonly chineseAlmanacSolarTerms: ReadonlyArray<string>;
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
  readonly places: ReadonlyArray<string>;
  readonly relativeTime: ReadonlyArray<string>;
  readonly relativeDayAt: ReadonlyArray<string>;
  readonly duration: ReadonlyArray<string>;
  readonly naturalText: ReadonlyArray<string>;
  readonly localizedNaturalText: ReadonlyArray<string>;
  readonly unitChoice: ReadonlyArray<string>;
  readonly relativeTimeWith: ReadonlyArray<string>;
  readonly approximateDuration: ReadonlyArray<string>;
  readonly zoneName: ReadonlyArray<string>;
  readonly utcFromTai: ReadonlyArray<string>;
  readonly taiMinusUtcExact: ReadonlyArray<string>;
  readonly utcFromTaiExact: ReadonlyArray<string>;
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
  readonly drekkana: ReadonlyArray<string>;
  readonly hjdTt: ReadonlyArray<string>;
  readonly hjdUtc: ReadonlyArray<string>;
  readonly ttBipm: ReadonlyArray<string>;
  readonly namingPeriod: ReadonlyArray<string>;
  readonly asianDay: ReadonlyArray<string>;
  readonly holyYear: ReadonlyArray<string>;
  readonly commonWorship: ReadonlyArray<string>;
  readonly roman1960Office: ReadonlyArray<string>;
  readonly holidayGroups: ReadonlyArray<string>;
  readonly dayPeriod: ReadonlyArray<string>;
  readonly numberingSystems: ReadonlyArray<string>;
  readonly calendarEras: ReadonlyArray<string>;
  readonly eraTable: ReadonlyArray<string>;
  readonly olympicGames: ReadonlyArray<string>;
  readonly ccsdsDecode: ReadonlyArray<string>;
  readonly ccsdsAscii: ReadonlyArray<string>;
  readonly radioDecode: ReadonlyArray<string>;
  readonly jjyCallSign: ReadonlyArray<string>;
  readonly unixFromDotnetTicks: ReadonlyArray<string>;
  readonly sixHourClock: ReadonlyArray<string>;
  readonly frenchDecimalTime: ReadonlyArray<string>;
  readonly civilFromFrenchDecimalTime: ReadonlyArray<string>;
  readonly babylonianRegnalYear: ReadonlyArray<string>;
  readonly equinoxMargin: ReadonlyArray<string>;
  readonly shmuelTekufah: ReadonlyArray<string>;
  readonly dayName: ReadonlyArray<string>;
  readonly kalam: ReadonlyArray<string>;
  readonly almanacCycles: ReadonlyArray<string>;
  readonly almanacDay: ReadonlyArray<string>;
  readonly almanacDirections: ReadonlyArray<string>;
  readonly mansionUndertakings: ReadonlyArray<string>;
  readonly almanacPersonDays: ReadonlyArray<string>;
  readonly tibetanAlmanacDay: ReadonlyArray<string>;
  readonly tibetanPlanets: ReadonlyArray<string>;
  readonly bhutaneseWinterSolstice: ReadonlyArray<string>;
  readonly orthodoxFast: ReadonlyArray<string>;
  readonly orthodoxFastSeasons: ReadonlyArray<string>;
  readonly prayerTimes: ReadonlyArray<string>;
  readonly prayerMethods: ReadonlyArray<string>;
  readonly zmanim: ReadonlyArray<string>;
  readonly temporalHour: ReadonlyArray<string>;
  readonly edoTime: ReadonlyArray<string>;
  readonly choghadiya: ReadonlyArray<string>;
  readonly panchak: ReadonlyArray<string>;
  readonly kumbh: ReadonlyArray<string>;
  readonly pushkaram: ReadonlyArray<string>;
  readonly jupiterAt: ReadonlyArray<string>;
  readonly jupiterIngress: ReadonlyArray<string>;
  readonly jupiterRising: ReadonlyArray<string>;
  readonly kumbhBySky: ReadonlyArray<string>;
  readonly kumbhYoga: ReadonlyArray<string>;
  readonly pushkaramRiver: ReadonlyArray<string>;
  readonly pushkaramRule: ReadonlyArray<string>;
  readonly jupiterStation: ReadonlyArray<string>;
  readonly pushkaramBySky: ReadonlyArray<string>;
  readonly pushkaramsInYear: ReadonlyArray<string>;
  readonly folkDay: ReadonlyArray<string>;
  readonly nightWatch: ReadonlyArray<string>;
  readonly barhaspatyaYear: ReadonlyArray<string>;
  readonly barhaspatyaYearAt: ReadonlyArray<string>;
  readonly muhurtas: ReadonlyArray<string>;
  readonly amritaSiddhi: ReadonlyArray<string>;
  readonly nakshatra: ReadonlyArray<string>;
  readonly planetaryHour: ReadonlyArray<string>;
  readonly gmat: ReadonlyArray<string>;
  readonly irigDecode: ReadonlyArray<string>;
  readonly irigFrameStart: ReadonlyArray<string>;
  readonly irigFormats: ReadonlyArray<string>;
  readonly orbitRateOffset: ReadonlyArray<string>;
  readonly rocket: ReadonlyArray<string>;
  readonly flipAndBurn: ReadonlyArray<string>;
  readonly doppler: ReadonlyArray<string>;
  readonly velocityAdd: ReadonlyArray<string>;
  readonly schwarzschildRadius: ReadonlyArray<string>;
  readonly properTimeUncertain: ReadonlyArray<string>;
  readonly planckUnits: ReadonlyArray<string>;
  readonly bpConvert: ReadonlyArray<string>;
  readonly deepConvert: ReadonlyArray<string>;
  readonly deepCompare: ReadonlyArray<string>;
  readonly dailyInsolation: ReadonlyArray<string>;
  readonly edtfParse: ReadonlyArray<string>;
  readonly edtfRelations: ReadonlyArray<string>;
  readonly significant: ReadonlyArray<string>;
  readonly significantOp: ReadonlyArray<string>;
  readonly uncertain: ReadonlyArray<string>;
  readonly uncertainOp: ReadonlyArray<string>;
  readonly interval: ReadonlyArray<string>;
  readonly units: ReadonlyArray<string>;
  readonly unitConvert: ReadonlyArray<string>;
  readonly rates: ReadonlyArray<string>;
  readonly framePeriod: ReadonlyArray<string>;
  readonly tempo: ReadonlyArray<string>;
  readonly fiscalProfiles: ReadonlyArray<string>;
  readonly fiscalYearOn: ReadonlyArray<string>;
  readonly fiscalYearSpan: ReadonlyArray<string>;
  readonly weekYearSystems: ReadonlyArray<string>;
  readonly weekYearOn: ReadonlyArray<string>;
  readonly nameDayLists: ReadonlyArray<string>;
  readonly nameDaysOn: ReadonlyArray<string>;
  readonly nameDay: ReadonlyArray<string>;
  readonly attributionAuthorities: ReadonlyArray<string>;
  readonly attributions: ReadonlyArray<string>;
  readonly attributionsOn: ReadonlyArray<string>;
  readonly harvestMoon: ReadonlyArray<string>;
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
  /** Where the calendar's day begins: `midnight`, `noon`, `sunset`, `sunrise`, `daybreak` (sunrise in summer and dawn in winter, `icelandic-medieval`'s) or `local-time HH:MM:SS`. */
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
  /** Its parent in the fallback chain, `null` for none. */
  parent: string | null;
  /** The numbering system numbers are written in by default, `latn`, `arab`. */
  numbering: string;
  /** The direction of the locale's script. */
  direction: TextDirection;
}

/** The syntaxes `hc_parse_datetime` reads. */
export type DatetimeSyntax = "iso8601" | "iso8601-full" | "rfc3339" | "rfc2822" | "python" | "auto";

/** The syntaxes `hc_format_datetime` writes. */
export type DatetimeFormat =
  | "iso8601"
  | "iso8601-basic"
  | "iso8601-ordinal"
  | "iso8601-week"
  | "rfc3339"
  | "rfc2822"
  | "imf-fixdate"
  | "python";

/** How much of a time `hc_format_datetime` writes. */
export type DatetimePrecision =
  | "auto" | "hours" | "minutes" | "seconds" | "milliseconds" | "microseconds" | "nanoseconds";

/** What a text said about its zone. */
export type ReadingZone = "none" | "utc" | "offset" | "unknown-local";

/** A date-time as text read it: a local reading, and an instant when the text states a zone. */
export interface Reading {
  /** The local fixed day. */
  localDay: number;
  /** The local second of the day, 0 to 86 400 (`23:59:60` is the 86 400th). */
  localSecond: number;
  /** The attoseconds into that second. */
  attoseconds: bigint;
  /** The zone the text stated. */
  zone: ReadingZone;
  /** The offset in seconds east of UTC, `null` where no zone was stated. */
  offsetSeconds: number | null;
  /** The POSIX second of the instant, floored, `null` where no zone was stated. */
  unixSeconds: number | null;
  /** Whether it is an inserted leap second, which POSIX counts as the second after it. */
  leapSecond: boolean;
  /** Whether the text wrote the end of a day as `24:00`. */
  endOfDay: boolean;
}

/** The line of `hc_format_datetime`. */
export interface FormattedDatetime {
  /** The date-time. */
  text: string;
  /** The syntax it is written in. */
  syntax: DatetimeFormat;
}

/** An ISO 8601 date's form. */
export type IsoDateForm = "calendar" | "ordinal" | "week";

/** An ISO 8601 date's style. */
export type IsoDateStyle = "extended" | "basic";

/** The line of `hc_format_iso_date_as`. */
export interface FormattedIsoDate {
  /** The date. */
  text: string;
  /** Its form. */
  form: IsoDateForm;
  /** Its style. */
  style: IsoDateStyle;
}

/** An ISO 8601 date read into its parts. */
export interface IsoDateParts {
  /** The form. */
  form: IsoDateForm;
  /** The year; the week-numbering year of a week date. */
  year: number;
  /** The month, `null` where the form or the text has none. */
  month: number | null;
  /** The day of the month. */
  day: number | null;
  /** The day of the year. */
  dayOfYear: number | null;
  /** The week. */
  week: number | null;
  /** The weekday, 1 Monday to 7 Sunday. */
  weekday: number | null;
  /** The fixed day, `null` for a date that names none. */
  fixed: number | null;
  /** The style. */
  style: IsoDateStyle;
}

/** A duration's exact length, where it has one. */
export interface DurationLength {
  /** The duration written in canonical form. */
  text: string;
  /** Whether it has years or months, which have no fixed length. */
  nominal: boolean;
  /** Its whole seconds, `null` for a nominal one. */
  exactSeconds: bigint | null;
  /** The attoseconds after them, `null` for a nominal one. */
  exactAttoseconds: bigint | null;
}

/** An ISO 8601 duration read into its components. */
export interface IsoDurationParts extends DurationLength {
  /** Whether it had a leading minus (ISO 8601-2). */
  negative: boolean;
  /** The years, `null` where the text did not write them. */
  years: number | null;
  months: number | null;
  weeks: number | null;
  days: number | null;
  hours: number | null;
  minutes: number | null;
  seconds: number | null;
  /** The digits of the decimal fraction of the lowest component, as written. */
  fraction: string | null;
  /** `designators` (`P1Y2M3D`) or `alternative` (`P0001-02-03T04:05:06`). */
  form: "designators" | "alternative";
}

/** The components of a duration to write; one left out is absent. */
export interface IsoDurationInput {
  negative?: boolean;
  years?: number | bigint;
  months?: number | bigint;
  weeks?: number | bigint;
  days?: number | bigint;
  hours?: number | bigint;
  minutes?: number | bigint;
  seconds?: number | bigint;
  /** The digits of a decimal fraction of the lowest component. */
  fraction?: string;
}

/** The line of `hc_format_iso_duration`. */
export type FormattedIsoDuration = DurationLength;

/** An ISO 8601 interval's shape. */
export type IntervalShape = "start-end" | "start-duration" | "duration-end" | "duration";

/** An ISO 8601 interval read into its ends. */
export interface IsoIntervalParts {
  /** `null` for an interval that does not repeat, a count, or `inf` for `R/`. */
  repetitions: number | "inf" | null;
  /** The shape. */
  shape: IntervalShape;
  /** The start as written back. */
  start: string | null;
  /** The POSIX second of the start, `null` where it states no zone or no time. */
  startUnixSeconds: number | null;
  /** The end as written back. */
  end: string | null;
  /** The POSIX second of the end. */
  endUnixSeconds: number | null;
  /** The duration, `null` for a start and an end. */
  duration: DurationLength | null;
}

/** The pattern languages `hc_parse_pattern` reads. */
export type PatternSyntax = "strftime" | "python" | "cldr";

/** The line of `hc_parse_pattern`: the fields a pattern read, and the reading they resolve to. */
export interface PatternFields {
  year: number | null;
  century: number | null;
  yearOfCentury: number | null;
  month: number | null;
  day: number | null;
  dayOfYear: number | null;
  isoYear: number | null;
  isoWeek: number | null;
  isoWeekday: number | null;
  /** The `%U` week number. */
  weekSunday: number | null;
  /** The `%W` week number. */
  weekMonday: number | null;
  /** The hour on a 24-hour clock. */
  hour: number | null;
  /** The hour on a 12-hour clock. */
  hour12: number | null;
  dayPeriod: "am" | "pm" | null;
  minute: number | null;
  second: number | null;
  attoseconds: bigint | null;
  zone: ReadingZone | null;
  offsetSeconds: number | null;
  /** The POSIX second a `%s` read. */
  unixSeconds: number | null;
  era: "ce" | "bce" | null;
  /** The fixed day CLDR's `g` read. */
  fixed: number | null;
  /** The reading the fields resolve to, `null` where they name no whole date and time. */
  reading: Reading | null;
}

/** `ltr` or `rtl`. */
export type TextDirection = "ltr" | "rtl";

/** Why a fallback chain goes from one step to the next. */
export type LocaleChainRule =
  | "requested"
  | "likely-script"
  | "extensions"
  | "variant"
  | "parent-locales"
  | "region"
  | "script"
  | "root";

/** One step of `hc_locale_chain`. */
export interface LocaleChainStep {
  /** The step, from 0. */
  step: number;
  /** The tag. */
  tag: string;
  /** The rule that led to it from the step before. */
  rule: LocaleChainRule;
  /** Whether `hc-i18n` carries an entry of data for exactly this tag. */
  carried: boolean;
}

/** Which case mappings apply. */
export type CasingStyleName = "standard" | "turkic";

/** The line of `hc_locale_info`. */
export interface LocaleInfo {
  /** The tag, written canonically. */
  tag: string;
  /** The language subtag. */
  language: string;
  /** The script subtag, as given. */
  script: string | null;
  /** The region subtag, as given. */
  region: string | null;
  /** The variant subtag, as given. */
  variant: string | null;
  /** The `-u-ca-` key. */
  calendarKey: string | null;
  /** The `-u-nu-` key. */
  numberingKey: string | null;
  /** The `-u-fw-` key as an ISO weekday number. */
  firstDayKey: number | null;
  /** The `-u-hc-` key. */
  hourCycleKey: string | null;
  /** The tag of the entry of data that answers for the locale. */
  localeUsed: string;
  /** The next step of the fallback chain. */
  parent: string | null;
  /** The rule that gave the parent. */
  parentRule: LocaleChainRule | null;
  /** The numbering system numbers are written in by default. */
  numbering: string;
  /** The ISO weekday number the week begins on. */
  firstDay: number;
  /** CLDR's `minDays`: the fewest days of a year a week needs to be its first. */
  minDays: number;
  /** The direction of the locale's script. */
  direction: TextDirection;
  /** Which case mappings apply. */
  casing: CasingStyleName;
  /** Whether month and weekday names are written with a capital. */
  capitalisesMonthNames: boolean;
  /** The language of the cardinal plural rules that apply, `und` for none. */
  pluralRules: string;
}

/** The kinds of plural rule. `ordinal` is not carried. */
export type PluralKind = "cardinal" | "ordinal";

/** A CLDR plural category. */
export type PluralCategoryName = "zero" | "one" | "two" | "few" | "many" | "other";

/** The line of `hc_plural_category`. */
export interface PluralCategoryAnswer {
  /** The category. */
  category: PluralCategoryName;
  /** The language of the rules that decided it, `und` where none is carried. */
  rules: string;
  /** The operands of UTS #35 read from the number as written. */
  operands: { i: bigint; v: number; w: number; f: bigint; t: bigint };
}

/** The widths of a name. */
export type NameWidth = "wide" | "abbreviated" | "short" | "narrow";

/** Whether a name stands inside a date or on its own. */
export type NameContext = "format" | "standalone";

/** One line of `hc_names`. */
export interface LocaleName {
  /** `month`, `month-in-leap-year`, `weekday`, `quarter`, `day-period` or a cycle's kind. */
  kind: string;
  /** The position from 1; the ISO number for a weekday. */
  position: number;
  /** The name. */
  name: string;
  /** The tag of the entry of data that answered. */
  localeUsed: string;
}

/** How `hc_case` recases a text. */
export type CaseMode =
  | "lower"
  | "upper"
  | "capitalise-first"
  | "lowercase-first"
  | "sentence-start"
  | "in-sentence";

/** The line of `hc_case`. */
export interface CasedText {
  /** The text recased. */
  text: string;
  /** The mode. */
  mode: CaseMode;
  /** `standard` or `turkic`. */
  casing: CasingStyleName;
  /** The tag of the entry of data that answered. */
  localeUsed: string;
}

/** How `hc_isolate` wraps a text. */
export type IsolateMode = "field" | "first-strong" | "strip";

/** The line of `hc_isolate`. */
export interface IsolatedText {
  /** The text, with the isolates around it where the mode says. */
  text: string;
  /** The locale's direction. */
  direction: TextDirection;
  /** The text's own direction by the first-strong rule, `null` where it has no strong character. */
  textDirection: TextDirection | null;
  /** Whether isolates were added. */
  isolated: boolean;
  /** The mode. */
  mode: IsolateMode;
}

export type HolidayKind =
  | "public"
  | "bank"
  | "religious"
  | "observance"
  | "school"
  | "workday"
  | "government"
  | "half-day";
export type Confidence = "exact" | "approximate";

/** One line of `hc_holidays_in_year`. */
export interface HolidayInYear {
  /** The ISO 8601 date, or `null` on a gap. */
  date: string | null;
  name: string;
  localName: string | null;
  /** `gap` is a holiday the table could not place in the year: its calendar's range ended, no announcement was read, the year is before the first its sources were read for, or the subdivision was not read. */
  kind: HolidayKind | "gap";
  /** `null` on a gap. */
  confidence: Confidence | null;
  /** Whether this is a substitute day. */
  substitute: boolean;
  /** The ISO 8601 date a substitute stands in for, or `null`. */
  observedFor: string | null;
  /** The subdivision whose own entry this is, `JP-13`, or `null` for a nationwide one. */
  region: string | null;
  /** The group whose own entry this is, `women`, or `null` for one everyone has. */
  group: string | null;
  /**
   * The holiday's stable identifier within its table, lower-case ASCII and hyphenated:
   * `new-years-day`. The same entry of `hc_holidays_on` and `hc_common_worship_on` carries it,
   * so lines are joined on it and not on the name. Spelling variants of one day share it. A
   * gap carries its rule's, `unread-subdivision` for a subdivision not read, and `unread-weekend` for a year whose weekend law was not read.
   */
  id: string;
  /** The instrument the rule cites, or `null` where the table's own sources speak for it. */
  source: string | null;
  /** Whether a bridge policy made this entry, Japan's 国民の休日 between two holidays. */
  bridged: boolean;
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
  /** The ISO 3166-2 subdivision whose own entry this is, `JP-13`, or `null` for a nationwide one. */
  region: string | null;
  /** The group of people whose own entry this is, `women`, or `null` for one everyone has. */
  group: string | null;
  /** The holiday's stable identifier within its table, `new-years-day`; see {@link HolidayInYear.id}. */
  id: string;
  /** Whether a bridge policy made this entry, Japan's 国民の休日 between two holidays. */
  bridged: boolean;
}

/** One line of `hc_zassetsu_in_year`. */
export interface ZassetsuDay {
  /** `spring-setsubun`, `summer-doyo-entry`; the older rules' days end in `-classical`. */
  id: string;
  /** The name in Japanese, 節分. */
  name: string;
  romaji: string;
  englishName: string;
  /** `solar-longitude`, `offset-from-term`, `nights-from-beginning-of-spring`, `nearest-stem-day` or `classical`. */
  rule: string;
  /** The fixed day. */
  day: number;
  /** The last day of the period it opens: the 土用 of a 土用の入り, the week of a 彼岸入り; else `null`. */
  last: number | null;
  /** The first 丑の日 of a 土用, or `null`. */
  firstOxDay: number | null;
  /** The second 丑の日, `null` where the 土用 has one. */
  secondOxDay: number | null;
}

/** The kind of a line of `hc_seasonal_days_in_year`. */
export type SeasonalDayKind = "san-fu" | "shu-jiu" | "dog-days" | "quarter-day" | "folk-day";

/** One line of `hc_seasonal_days_in_year`. */
export interface SeasonalDay {
  kind: SeasonalDayKind;
  id: string;
  name: string;
  /** The name in the convention's own language, 初伏. */
  localName: string;
  first: number;
  /** The last fixed day; the first for a single day. */
  last: number;
  /** The calendar of the dates (`gregorian`, `julian`) or the tradition of a quarter day; else `null`. */
  group: string | null;
}

/** A tradition that names the 72 pentads. */
export type PentadTraditionId = "chinese" | "japanese" | "jokyo" | "senmyo";

/** One line of `hc_pentad_traditions`. */
export interface PentadTradition {
  id: PentadTraditionId;
  englishName: string;
  /** The text the tradition's names come from. */
  authority: string;
  /** How many of its names carry an alternate reading the text prints beside them. */
  alternates: number;
}

/** The one line of `hc_pentad_in_tradition`. */
export interface PentadInTradition {
  /** The first pentad of 春分 at 0 through 71. */
  index: number;
  name: string;
  /** The English gloss of the name in this tradition. */
  gloss: string;
  /** The alternate reading the tradition's text prints beside the name, `武始交` for 虎始交, or `null`. */
  alternate: string | null;
  /** The fixed day the pentad began at the meridian. */
  begins: number;
  /** The last fixed day before the next pentad begins. */
  ends: number;
  tradition: PentadTraditionId;
  authority: string;
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

/** CLDR's draft level of a name: how far its vetting went. */
export type PlaceDraft = "approved" | "contributed" | "provisional";

/** A code's status in CLDR's validity data. */
export type PlaceStatus = "regular" | "deprecated" | "macroregion" | "special" | "unknown" | "municipal";

/** One line of `hc_territories`, `hc_subdivisions` and `hc_place_name`. */
export interface PlaceName {
  /** The code as ISO writes it: `JP`, `001`, or `JP-13` for CLDR's `jp13`; or a municipality's under its subdivision's, `JP-14-130`. */
  code: string;
  /**
   * The name in the locale, else in English: 東京都 for `JP-13` under `ja`.
   * `null` only for a deprecated subdivision neither names.
   */
  name: string | null;
  /** CLDR's English name, `en.xml`'s; `null` for the deprecated subdivisions it does not name. */
  englishName: string | null;
  /** The tag of the data that named it: `ja`, `pt` for a name `pt-PT` inherits, or `en`. */
  localeUsed: string | null;
  /** The draft level of that value; nearly every subdivision name outside English is `provisional`. */
  draft: PlaceDraft | null;
  /** `regular` for a code in use; `deprecated` for a subdivision CLDR keeps from an earlier ISO list; `municipal` for a municipality, whose names are `hc-i18n`'s and not CLDR's, and whose `draft` is `null`. */
  status: PlaceStatus;
}

/** How wide a relative phrase is: *3 days ago*, *3 d. ago*, *3d ago*. */
export type RelativeStyle = "long" | "short" | "narrow";

/** How a duration is phrased: in words, abbreviated, narrow, or as bare suffixes, *2h30m*. */
export type DurationStyle = "long" | "short" | "narrow" | "compact";

/** The unit a relative phrase is counted in, CLDR's field name. */
export type HumanizeUnit = "second" | "minute" | "hour" | "day" | "week" | "month" | "quarter" | "year";

/** The line of `hc_relative_time` and `hc_relative_day`. */
export interface RelativeTime {
  /** The phrase: *3 hours ago*, *in 2 days*, *yesterday*. */
  phrase: string;
  /** The unit it is counted in. */
  unit: HumanizeUnit;
  /** The signed count, negative in the past. */
  count: number;
  /** The tag of the `hc-humanize` data the locale resolved to. */
  localeUsed: string;
}

/** The line of `hc_relative_day_at`. */
export interface RelativeDayAt extends RelativeTime {
  /** The time of day as it was written into the phrase, `15:05`. */
  time: string;
}

/** The line of `hc_duration`. */
export interface HumanizedDuration {
  /** The phrase: *2 hours and 30 minutes*, *2h30m*. */
  phrase: string;
  /** Whether the span was negative; the phrase is of its length. */
  negative: boolean;
  /** The tag of the `hc-humanize` data the locale resolved to. */
  localeUsed: string;
}

/** The line of `hc_fractional` and `hc_scientific`, which write no word. */
export interface NaturalText {
  /** The text: *five*, *3/10*, *1.50 kV*, *3.0 MB*, *one, two and three*, *1.0 googol*. */
  text: string;
  /** The language of the vocabulary that wrote it, always `en`. */
  language: string;
}

/** The line of the `humanize` functions that write words. */
export interface LocalizedNaturalText {
  /** The text: *fünf*, *12,4 Millionen*, *3.0 MB*, *vor 3 Sekunden*. */
  text: string;
  /** The language of the catalogue that wrote it, `de-DE`, `ru-RU`, or `en`. */
  localeUsed: string;
}

/** A unit `naturalDelta` and `naturalTime` can stop at. */
export type DeltaUnit = "seconds" | "milliseconds" | "microseconds";

/** A unit `preciseDelta` can stop at or suppress. */
export type PreciseUnitName =
  | "microseconds" | "milliseconds" | "seconds" | "minutes" | "hours" | "days" | "months" | "years";

/** The grammatical gender `ordinal` takes. */
export type OrdinalGender = "male" | "female";

/** A table of thresholds `hc-humanize` states. */
export type ThresholdsName = "default" | "exact" | "with-quarters";

/** How a fractional count is made whole. */
export type RoundingName = "ceil" | "floor" | "nearest" | "truncate" | "nearest-half";

/** How a hedge is chosen. */
export type HedgePolicy = "default" | "bounded";

/** What a hedge says about the round number. */
export type Hedge = "exactly" | "about" | "just-over" | "over" | "nearly";

/** The line of `hc_unit_choice`. */
export interface UnitChoice {
  /** The unit the span is said in. */
  unit: HumanizeUnit;
  /** The signed count, its whole part. */
  count: number;
  /** Whether a half is added to the count in the direction of its sign (`nearest-half`). */
  half: boolean;
  /** The thresholds applied. */
  thresholds: ThresholdsName;
  /** The rounding applied. */
  rounding: RoundingName;
}

/** The line of `hc_relative_time_with`. */
export interface RelativeTimeWith extends RelativeTime {
  /** Whether a half is added to the count (*an hour and a half ago*). */
  half: boolean;
}

/** The line of `hc_approximate_duration`. */
export interface ApproximateDuration {
  /** The phrase: *about 3 hours*, *just over a year*. */
  phrase: string;
  /** The hedge. */
  hedge: Hedge;
  /** The unit. */
  unit: HumanizeUnit;
  /** The count, which *nearly* carries up to the next. */
  count: number;
  /** The tag of the `hc-humanize` data the locale resolved to. */
  localeUsed: string;
}

/** The suffixes and base of `naturalSize`. */
export type NaturalSizeStyle = "decimal" | "binary" | "gnu";

/** A CLDR pattern field that writes a time zone's name. */
export type ZoneNameField =
  | "z"
  | "zz"
  | "zzz"
  | "zzzz"
  | "O"
  | "OOOO"
  | "v"
  | "vvvv"
  | "V"
  | "VV"
  | "VVV"
  | "VVVV";

/** The line of `hc_format_pattern`. */
export interface FormattedInZone {
  text: string;
  syntax: "cldr" | "strftime";
  zone: string;
  offset: number;
  daylight: boolean;
}

/** The line of `hc_zone_name`. */
export interface ZoneName {
  /** The name, *Pacific Daylight Time*, or its fallback. */
  name: string;
  field: ZoneNameField;
  zone: string;
  /** Seconds east of UTC at the instant. */
  offset: number;
  daylight: boolean;
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

/** The one line of `hc_tai_minus_utc_exact`: `TAI - UTC` as whole seconds and the attoseconds after them. */
export interface TaiMinusUtc {
  seconds: bigint;
  /** 0 to 10¹⁸ − 1; not zero from 1961 to 1971, when the offset was not whole. */
  attoseconds: bigint;
}

/** The one line of `hc_utc_from_tai_exact`: a TAI instant as a UTC label. */
export interface UtcInstantLabel {
  /** The POSIX second; for a leap second, the one after it. */
  unixSeconds: bigint;
  /** The attoseconds into the second, 0 to 10¹⁸ − 1. */
  attoseconds: bigint;
  /** Whether the instant falls in an inserted `23:59:60`. */
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

/** One line of `hc_tithi_at`. */
export interface Tithi {
  /** 1 for śukla pratipadā through 30 for amāvasyā. */
  number: number;
  paksha: "shukla" | "krishna";
  /** The tithi's day in its fortnight, 1 to 15. */
  pakshaDay: number;
  /** The name in IAST: `Caturdaśī`, `Pūrṇimā`, `Amāvasyā`. */
  name: string;
  /** POSIX seconds, rounded down: the Moon had gained a multiple of 12° on the Sun. */
  began: number;
  ends: number;
  /** The instant read: the one asked for, or the day's sunrise. */
  readAt: number;
  sky: "true" | "surya-siddhanta";
}

/** One line of `hc_tithis_of_day`: a tithi with what the day's sunrises make of it. */
export interface TithiOfDay extends Tithi {
  /** It holds the day's sunrise: the tithi the day carries. */
  atSunrise: boolean;
  /** It holds the next sunrise too: repeated (adhika), the day's again tomorrow. */
  repeated: boolean;
  /** It holds neither sunrise: skipped (kṣaya). */
  skipped: boolean;
}

/** One line of `hc_ayanamsas`. */
export interface AyanamsaInfo {
  id: Ayanamsa;
  name: string;
  /** The Julian date the anchor is quoted for. */
  anchorJulianDate: number;
  /** The anchor in degrees. */
  anchorDegrees: number;
  /** Where the anchor is from. */
  source: string;
  /**
   * What the anchor stands for: `mean`, the precession alone, or `true`,
   * the mean value plus the nutation in longitude of the day (the value
   * carries the nutation of each day besides).
   */
  kind: "mean" | "true";
}

/** The two readings of a festival's day where the sects part: `hc_festival_readings`. */
export type FestivalReadingId = "smarta" | "vaishnava";

/** The historical Indian eras over the lunisolar months, `hc_era_new_year`. */
export type LunarEraId =
  | "vikram-samvat-kartikadi"
  | "rajyabhisheka-saka"
  | "saptarshi"
  | "gupta"
  | "valabhi"
  | "kalachuri"
  | "lakshmana-sena";

/** One line of `hc_festival_readings`. */
export interface FestivalReading {
  id: FestivalReadingId;
  name: string;
  /** How the day is taken, in words. */
  rule: string;
  /** Where the reading is from. */
  source: string;
}

/** The line of `hc_janmashtami`. */
export interface FestivalDay {
  reading: FestivalReadingId;
  /** The Gregorian year asked. */
  year: number;
  /** The fixed day. */
  fixed: number;
  /** Its Gregorian year, month and day. */
  gregorian: [number, number, number];
  /** The tithi the day carries at sunrise there, 1 to 30: 23 or 24 for Janmāṣṭamī. */
  sunriseTithi: number;
  ayanamsa: Ayanamsa;
}

/** The line of `hc_vaishnava_day`. */
export interface VaishnavaDay {
  sakaYear: number;
  /** 1 for Chaitra through 12 for Phālguna. */
  month: number;
  /** 1 for śukla pratipadā through 30 for amāvasyā. */
  tithi: number;
  fixed: number;
  gregorian: [number, number, number];
  sunriseTithi: number;
  ayanamsa: Ayanamsa;
}

/** The line of `hc_vishti_free_span`. */
export interface VishtiFreeSpan {
  sakaYear: number;
  month: number;
  tithi: number;
  /** The fixed day the month's first sunrise falls on. */
  monthFirstDay: number;
  /** POSIX seconds, rounded down; `null` for a tithi Viṣṭi never falls on. */
  begins: number | null;
  ends: number | null;
  ayanamsa: Ayanamsa;
}

/** A node's place: its sidereal longitude and sign. */
export interface NodeSign {
  /** Degrees, 0 up to 360. */
  longitude: number;
  /** The sign by number, 1 for Meṣa through 12. */
  number: number;
  sign: SiderealSignId;
  signName: string;
}

/** The line of `hc_rahu_at`. */
export interface NodePlace {
  /** `mean`: the true node is not carried. */
  node: "mean";
  rahu: NodeSign;
  ketu: NodeSign;
  ayanamsa: Ayanamsa;
  readAt: number;
}

/** One line of `hc_rahu_ingresses`: Rāhu's entry into a sign, Ketu's into the opposite one. */
export interface NodeIngress {
  /** POSIX seconds, rounded down. */
  moment: number;
  from: SiderealSignId;
  fromName: string;
  into: SiderealSignId;
  intoName: string;
  ketuInto: SiderealSignId;
  ketuIntoName: string;
}

/** The line of `hc_ayanamsa_at` and `hc_ayanamsa_from_anchor`. */
export interface AyanamsaValue {
  /** The ayanāṃśa in degrees at the instant. */
  degrees: number;
  /** `custom` for an anchor the caller gave. */
  id: string;
  name: string;
  anchorJulianDate: number;
  anchorDegrees: number;
  readAt: number;
  /** What the anchor stands for, as `AyanamsaInfo.kind`; `mean` for a caller's anchor. */
  kind: "mean" | "true";
}

/** One line of `hc_panchanga_at` or `hc_panchanga_of_day`. *//** One line of `hc_panchanga_at` or `hc_panchanga_of_day`. */
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
  /** The yoga's sky, as `panchangaAt` takes it; `null` for the karaṇa on the true sky. */
  ayanamsa: Ayanamsa | "surya-siddhanta" | null;
  /** Its full name, `Lahiri (Chitrapaksha)` or `Sūrya Siddhānta`; `null` with it. */
  ayanamsaName: string | null;
}

/** One line of `hc_muhurtas`. */
export interface Muhurta {
  half: "day" | "night";
  /** 1 to 15 within the half. */
  number: number;
  /**
   * Its name as English Wikipedia's "Muhurta" tabulates them, `Rudra` to
   * `Bhaga` by day and `Girīśa` to `Samudra` by night; the article marks
   * the table as needing citations.
   */
  name: string;
  /** POSIX seconds, rounded down; `null` where the Sun does not rise or set. */
  start: number | null;
  end: number | null;
  /** What the pañcāṅga prints it as, or `null`. */
  mark: "abhijit" | "dur-muhurtam" | null;
  missing: MissingSolarEvent | null;
}

/** The line of `hc_amrita_siddhi`. */
export interface AmritaSiddhi {
  /** `Amrita Siddhi Yoga`. */
  name: string;
  devanagari: string;
  /** The nakṣatra the weekday pairs with, 1 for Aśvinī to 27 for Revatī. */
  nakshatra: number;
  nakshatraId: NakshatraId;
  /** Its name in IAST, `Aśvinī`. */
  nakshatraName: string;
  /** POSIX seconds, rounded down; `null` on a day the yoga does not fall. */
  start: number | null;
  end: number | null;
  falls: boolean;
  ayanamsa: Ayanamsa;
}

/** The line of `hc_nakshatra_at` and `hc_nakshatra_of_day`. */
export interface NakshatraStay {
  /** 1 for Aśvinī through 27 for Revatī. */
  nakshatra: number;
  nakshatraId: NakshatraId;
  /** Its name in IAST, `Aśvinī`. */
  nakshatraName: string;
  /** POSIX seconds, rounded down. */
  entered: number;
  leaves: number;
  readAt: number;
  ayanamsa: Ayanamsa;
  ayanamsaName: string;
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
  /** The Chinese names the sources give the kind of year; none for `blind` and `bright`. */
  chineseNames: AuguryName[];
}

/** A Chinese name of a kind of year, as a source writes it. */
export interface AuguryName {
  name: string;
  locale: "zh-Hant" | "zh-Hans";
  /** `north` or `south` where the source says where it is used. */
  region: string | null;
}

/** The line of `hc_french_decimal_time`. */
export interface FrenchDecimalTime {
  /** 0 to 9. */
  hour: number;
  /** 0 to 99. */
  minute: number;
  /** 0 to 99. */
  second: number;
  /** The rest of the decimal second, in attoseconds of ordinary time. */
  attoseconds: bigint;
}

/** The line of `hc_civil_from_french_decimal_time`. */
export interface TimeOfDay {
  /** Whole seconds after midnight. */
  secondsOfDay: number;
  attoseconds: bigint;
}

/** The line of `hc_babylonian_regnal_year`. */
export interface RegnalYear {
  king: string;
  year: number;
}

/** A calendar whose new year an equinox decides at a moment of the day. */
export type EquinoxCalendar =
  | "persian"
  | "persian-apparent-noon"
  | "jalali"
  | "bahai-astronomical"
  | "french-republican-equinox";

/** The line of `hc_equinox_new_year_margin`. */
export interface EquinoxMargin {
  /** Positive when the equinox fell before the deciding moment. */
  minutes: number;
  calendar: EquinoxCalendar;
}

/** One of the four seasons of Shmuel's year. */
export type Tekufah = "tishrei" | "tevet" | "nisan" | "tammuz";

/** The line of `hc_shmuel_tekufah`. */
export interface ShmuelTekufah {
  /** The fixed day whose Hebrew day it falls in. */
  fixed: number;
  /** Minutes of Jerusalem mean time since the midnight of `civil`. */
  minutes: number;
  tekufah: Tekufah;
  /** After the reckoning's nightfall, 18:00 of Jerusalem mean time, and before midnight: `fixed` is the next civil day. */
  afterNightfall: boolean;
  /** The fixed day of Jerusalem mean time the moment falls on. */
  civil: number;
}

/** The line of `hc_day_name`. */
export interface DayName {
  name: string;
  /** The naming's identifier: `fr-fabre-1793`, `fr`, `en`, `hy` or `hy-Latn`. */
  naming: string;
  namingName: string;
  authority: string;
}

/** A count of a person's age `hc_chinese_age` takes. */
export type AgeConvention = "chinese-age" | "lichun-age" | "new-year-day-age" | "year-age";

/** One line of `hc_chinese_almanac_solar_terms`. */
export interface AlmanacSolarTerm {
  /** 1 for 小寒 to 24 for 冬至. */
  position: number;
  /** The name in traditional Chinese. */
  name: string;
  fixed: number;
}

/** The part a liturgical day plays in the 1960 ordo of a day. */
export type Roman1960Role = "office" | "commemoration" | "transferred" | "omitted";

/** A class of the 1960 rubrics. */
export type Roman1960Class = "first" | "second" | "third" | "fourth" | "commemoration";

/** One line of `hc_roman_1960_office_on`. */
export interface Roman1960Office {
  role: Roman1960Role;
  title: string;
  class: Roman1960Class;
  /** `I class`. */
  className: string;
  /** On the office's line, the day a feast of the I class transferred here was impeded on. */
  transferredFrom: number | null;
}

/** One line of `hc_holiday_groups`. */
export interface HolidayGroup {
  /** The identifier `hc_holidays_on`'s group column writes: `women`. */
  group: string;
  /** Its name in the locale, where an instrument in the language names it. */
  name: string | null;
  localeUsed: string | null;
  englishName: string;
}

/** One line of `hc_holidays_on_in`. */
export interface HolidayOnIn extends HolidayOn {
  /** What the locale calls the day, where a source in the language names it. */
  nameInLocale: string | null;
  nameLocale: string | null;
}

/** The line of `hc_day_period`. */
export interface DayPeriodReading {
  half: "am" | "pm";
  /** The locale's abbreviated name for the half, *PM*. */
  halfName: string | null;
  /** `midnight`, `noon`, or `morning1` to `night2`; `null` where the language has no rules. */
  period: string | null;
  abbreviated: string | null;
  wide: string | null;
  narrow: string | null;
  localeUsed: string;
}

/** One line of `hc_numbering_systems`. */
export interface NumberingSystemInfo {
  /** The CLDR identifier: `latn`, `hebr`, `grek`. */
  system: string;
  algorithmic: boolean;
  /** A positional system's ten digits, zero first. */
  digits: string | null;
}

/** One line of `hc_calendar_eras`. */
export interface CalendarEra {
  /** The era's code: `reiwa`, `ce`. */
  code: string;
  wide: string | null;
  abbreviated: string | null;
  narrow: string | null;
  calendar: string;
  localeUsed: string;
}

/** A table of eras `hc_era_table` lists. */
export type EraTableId = "japanese" | "chinese-regnal" | "korean-regnal";

/** One line of `hc_era_table`. */
export interface EraTableRow {
  /** The era's code: `reiwa`, `showa-1312`, `kangxi`, `gwangmu`. */
  code: string;
  /** The name in the characters of its source: 令和, 康熙, 光武. */
  name: string;
  /** The reading: hiragana, pinyin or hangul. */
  reading: string;
  romanised: string;
  /** The court (`unified`, `northern`, `southern`) or the dynasty (`ming`, `southern-ming`, `shun`, `qing`, `korean-empire`). */
  group: string;
  /** The Gregorian year of the first year (元年). */
  firstYear: number;
  /** The year of the last, where the table has one: the Chinese and Korean tables. */
  lastYear: number | null;
  /** The fixed day it began; `null` where the table has none. */
  start: number | null;
  /** The last fixed day it was in force; `null` for the era in force and where the table has none. */
  last: number | null;
  /** `attested`, `disputed`, `month-only`, `kept` or `not-kept`. */
  status: string;
  /** The first day under the other reading, 光武 backdated to 1 January 1897; else `null`. */
  otherStart: number | null;
  note: string | null;
}

/** One line of `hc_olympic_games`. */
export interface OlympicGames {
  /** The number; `null` for a Winter Games not held. */
  number: number | null;
  /** The year awarded to: 2020 for the Games held in 2021. */
  year: number;
  /** The host city as Olympedia spells it. */
  host: string;
  status: "celebrated" | "not-held" | "scheduled";
  /** The fixed day of the opening ceremony, or `null`. */
  opening: number | null;
  closing: number | null;
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
  /**
   * The ISO 3166-2 codes of the regions it answers for, in code order: the subdivisions its
   * rules, its weekend laws and its substitution policies are scoped to, `JP-11`, `JP-12`, …,
   * and Kedah `MY-02` and Sharjah `AE-SH`, whose only law of their own is a weekend. Every
   * holiday method accepts each as its `region`, and a region outside the list that a country
   * has has the nationwide days. Empty for a table with none.
   */
  regions: string[];
  /** The groups of people its rules give days to alone, the groups it answers for: `children`, `military`, `women`, `youth`. */
  groups: string[];
  /** Those groups' names in the locale, in the same order, English where `hc-i18n` carries none: `少年儿童` under `zh-CN`. */
  groupNames: string[];
  /**
   * Each subdivision and group a rule is scoped to both of, the scopes whose own days neither
   * the region alone nor the group alone has. Empty for a table with none.
   */
  regionGroups: HolidayRegionGroup[];
  /**
   * The ISO 3166-2 codes of the subdivisions the table's sources were read for, in code order:
   * the `regions` whose days are rules and those read and found to keep no day of their own;
   * a region that has only a weekend law or a substitution policy is not read for its days. A region outside it keeps
   * the nationwide days and has a gap for its own, and so does one in it for a year before the first its
   * sources were read for (`JP-27` before 1989). Empty for a table with no subdivisions.
   */
  readSubdivisions: string[];
  /**
   * The table's weekend laws, in the table's order; empty for a table that
   * states none, whose weekend is Saturday and Sunday. Where several cover a
   * day, the one for the nearest region wins, and a region with none of its
   * own has the one with no regions.
   */
  weekend: WeekendLaw[];
}

/** A pair of column 12 of `hc_holiday_tables`: `CN-XJ:women` is `{ region: "CN-XJ", group: "women" }`. */
export interface HolidayRegionGroup {
  region: string;
  group: string;
}

/** One weekend law of a holiday table: an entry of column 14 of `hc_holiday_tables`. */
export interface WeekendLaw {
  /**
   * The weekend days as ISO 8601 weekday numbers, Monday 1 to Sunday 7
   * (`[5, 6]` for Friday and Saturday); `null` where the weekend law of
   * these years was not read for the regions, which is a gap.
   */
  days: number[] | null;
  /** The first day it is in force, `YYYY-MM-DD`; `null` for no first day. */
  first: string | null;
  /** The last day it is in force, `YYYY-MM-DD`; `null` for no last day. */
  last: string | null;
  /** The ISO 3166-2 codes of the regions it is the weekend of; empty for the whole table. */
  regions: string[];
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
  /** The Roman number of a Sunday in Ordinary Time, 2 to 34; else `null`. */
  sundayInOrdinaryTime: number | null;
  /** The week of Ordinary Time, 1 to 34, on the universal calendar (the Baptism on the Sunday after 6 January); else `null`. */
  weekOfOrdinaryTime: number | null;
  /** The week of Ordinary Time on a calendar that keeps the Epiphany on the Sunday between 2 and 8 January; else `null`. */
  weekOfOrdinaryTimeEpiphanyOnSunday: number | null;
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
  signId: SiderealSignId;
  /** Its Sanskrit name, `Mīna`. */
  signName: string;
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
  signId: TropicalSignId;
  /** Its English name, `Aries`. */
  signName: string;
  /** Which of the sign's three 10° faces, 1 to 3. */
  decan: number;
  /** The face's ruler by al-Bīrūnī's table. */
  ruler: DecanRuler;
  rulerName: string;
  /** From 0 up to 10. */
  degreesIntoDecan: number;
}

/** The one line of `hc_drekkana_at`. */
export interface Drekkana {
  /** The sidereal sign, 1 for Meṣa through 12 for Mīna. */
  sign: number;
  signId: SiderealSignId;
  /** Its Sanskrit name, `Meṣa`. */
  signName: string;
  /** Which of the sign's three 10° drekkāṇas, 1 to 3. */
  drekkana: number;
  /** The ruler of the sign the drekkāṇa is given to. */
  lord: DecanRuler;
  lordName: string;
  /** From 0 up to 10. */
  degreesIntoDrekkana: number;
  /** The sign the drekkāṇa is given to: the sign itself, the fifth or the ninth. */
  lordSign: SiderealSignId;
  /** Its Sanskrit name. */
  lordSignName: string;
  ayanamsa: Ayanamsa;
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
  /** The celebration's identifier, the `id` of its entry in `hc_holidays_on`: `christmas-day`. */
  id: string;
}

/** One line of `hc_orbit_rate_offset`. */
export interface OrbitRateOffset {
  /** the body's identifier */
  id: string;
  /** its standard gravitational parameter GM, in m³ s⁻² */
  gm: number;
  /** the `hc-relativity` constant that holds it */
  gmConstant: string;
  /** √(GM/r) at the orbit radius, in metres per second */
  circularSpeed: number;
  /** GM/c² (1/R − 1/r), the weak-field gravitational part, positive when the orbit is higher */
  gravitationalMicrosecondsPerDay: number;
  /** −GM/(2rc²), the part from the orbital speed, negative */
  kinematicMicrosecondsPerDay: number;
  /** the sum of the two */
  weakFieldMicrosecondsPerDay: number;
  /** the exact Schwarzschild figure, √(1 − 3GM/rc²) / √(1 − 2GM/Rc²) − 1, computed without cancellation */
  exactMicrosecondsPerDay: number;
  /** the constants used, separated by `;` */
  constants: string[];
  /** the functions used and the body's source */
  source: string;
}

/** One line of `hc_rocket`. */
export interface RocketBurn {
  /** the proper acceleration given, in m s⁻² */
  acceleration: number;
  /** the time aboard given, in seconds */
  properSeconds: number;
  /** the time that passes in the frame the burn starts from, (c/a) sinh(aτ/c), in seconds */
  coordinateSeconds: number;
  /** the distance covered in that frame, (c²/a)(cosh(aτ/c) − 1), in metres */
  distance: number;
  /** the same in light-years of `LIGHT_YEAR` */
  distanceLightYears: number;
  /** tanh(aτ/c) */
  beta: number;
  /** 2 / (e^{2aτ/c} + 1), the distance from the speed of light, without cancellation */
  oneMinusBeta: number;
  /** cosh(aτ/c) */
  lorentzFactor: number;
  /** the constants used, separated by `;` */
  constants: string[];
  /** the functions used */
  source: string;
}

/** One line of `hc_flip_and_burn`. */
export interface FlipAndBurn {
  /** the proper acceleration given, in m s⁻² */
  acceleration: number;
  /** the distance given, in metres */
  distance: number;
  /** the time aboard, in seconds */
  properSeconds: number;
  /** the time at home, in seconds */
  coordinateSeconds: number;
  /** the time aboard in Julian years of 31 557 600 s */
  properYears: number;
  /** the time at home in Julian years */
  coordinateYears: number;
  /** β at the turnover */
  peakBeta: number;
  /** 1/(γ²(1 + β)), the distance from the speed of light at the turnover, without cancellation */
  oneMinusPeakBeta: number;
  /** 1 + a(d/2)/c² at the turnover */
  peakLorentzFactor: number;
  /** the constants used, separated by `;` */
  constants: string[];
  /** the functions used */
  source: string;
}

/** One line of `hc_doppler`. */
export interface DopplerShift {
  /** the speed given */
  beta: number;
  /** the cosine given */
  cosTheta: number;
  /** f_observed / f_emitted, above 1 for a blueshift */
  factor: number;
  /** z = λ_observed / λ_emitted − 1, computed without cancellation for a slow source */
  redshift: number;
  /** √((1+β)/(1−β)), the factor for cos θ = 1 */
  headOnFactor: number;
  /** 1/γ, the factor for cos θ = 0 */
  transverseFactor: number;
  /** the functions used */
  source: string;
}

/** One line of `hc_velocity_add`. */
export interface VelocityComposition {
  /** the first velocity given */
  firstBeta: number;
  /** the second velocity given */
  secondBeta: number;
  /** (β₁ + β₂)/(1 + β₁β₂) */
  composedBeta: number;
  /** the composed β times the speed of light, in m s⁻¹ */
  composedSpeed: number;
  /** (1 − β₁)(1 − β₂)/(1 + β₁β₂), without cancellation */
  oneMinusComposedBeta: number;
  /** artanh β₁ */
  firstRapidity: number;
  /** artanh β₂ */
  secondRapidity: number;
  /** the sum of the two */
  composedRapidity: number;
  /** γ₁γ₂(1 + β₁β₂) */
  composedLorentzFactor: number;
  /** the functions used */
  source: string;
}

/** One line of `hc_schwarzschild_radius`. */
export interface SchwarzschildRadius {
  /** the body's identifier */
  id: string;
  /** its standard gravitational parameter GM, in m³ s⁻² */
  gm: number;
  /** the `hc-relativity` constant that holds it */
  gmConstant: string;
  /** 2GM/c², in metres */
  schwarzschildRadius: number;
  /** the constants used, separated by `;` */
  constants: string[];
  /** the function used and the body's source */
  source: string;
}

/** One line of `hc_proper_time_uncertain`. */
export interface UncertainProperTime {
  /** the speed as a fraction of the speed of light */
  beta: number;
  /** its standard deviation */
  betaStdDev: number;
  /** the time the moving clock records, in seconds */
  properSeconds: number;
  /** its standard deviation, in seconds */
  properStdDev: number;
  /** the proper time printed to the figures its standard deviation supports */
  text: string;
  /** `SPEED_OF_LIGHT` */
  constants: string[];
  /** the function used */
  source: string;
}

/** One line of `hc_planck_units`. */
export interface PlanckUnit {
  /** `c`, `hbar`, `G`, `t_P`, `l_P`, `m_P`, `E_P` or `T_P` */
  symbol: string;
  /** the constant's name as CODATA spells it */
  name: string;
  /** the SI unit the value is in, in ASCII */
  unit: string;
  /** the recommended value */
  value: number;
  /** the standard uncertainty; 0 for a defined constant */
  stdDev: number;
  /** how many figures the source prints */
  figures: number;
  /** the standard uncertainty over the value */
  relativeUncertainty: number;
  /** `1` where the constant is exact by definition */
  defined: boolean;
  /** the value printed to exactly those figures */
  text: string;
  /** the source and the CODATA adjustment */
  source: string;
}

/** One line of `hc_bp_convert`. */
export interface DatumConversion {
  /** the datum given */
  from: string;
  /** the datum written */
  to: string;
  /** the number given */
  years: number;
  /** its standard deviation */
  stdDev: number;
  /** the number in the other datum */
  converted: number;
  /** its standard deviation, unchanged */
  convertedStdDev: number;
  /** the number as a label: `11650 cal BP`, `11700 b2k`, `9701 BCE`, `1984 CE` */
  label: string;
  /** where the datums are defined */
  source: string;
}

/** One line of `hc_deep_convert`. */
export interface MagnitudeConversion {
  /** the unit given */
  from: string;
  /** the unit written */
  to: string;
  /** the number given */
  value: number;
  /** its standard deviation */
  stdDev: number;
  /** the number in the other unit */
  converted: number;
  /** its standard deviation */
  convertedStdDev: number;
  /** the converted number printed to the figures its standard deviation supports */
  text: string;
  /** the span in seconds */
  seconds: number;
  /** its standard deviation */
  secondsStdDev: number;
  /** the base-ten logarithm of the seconds; empty for a span of no length */
  log10Seconds: number | null;
  /** its standard deviation, delta method */
  log10StdDev: number | null;
  /** `1` where both units are defined, so the conversion adds no uncertainty */
  exact: boolean;
  /** the functions used */
  source: string;
}

/** One line of `hc_deep_compare`. */
export interface MagnitudeComparison {
  /** the first span in seconds */
  firstSeconds: number;
  /** its standard deviation */
  firstStdDev: number;
  /** the second span in seconds */
  secondSeconds: number;
  /** its standard deviation */
  secondStdDev: number;
  /** the first over the second */
  ratio: number;
  /** its standard deviation */
  ratioStdDev: number;
  /** the base-ten logarithm of the ratio */
  log10Ratio: number;
  /** its standard deviation */
  log10StdDev: number;
  /** the number of decades between them, the absolute value of the logarithm */
  decades: number;
  /** `1` where the one-sigma bars of the two meet */
  overlap: boolean;
  /** −1, 0 or 1 where the first is shorter than, equal to or longer than the second by central value */
  order: number;
  /** the functions used */
  source: string;
}

/** One line of `hc_daily_insolation`. */
export interface DailyInsolation {
  /** the epoch given */
  yearsBefore1950: number;
  /** the latitude given, in degrees, north positive */
  latitude: number;
  /** the Sun's true longitude given, in degrees */
  solarLongitude: number;
  /** the daily mean at the top of the atmosphere, in W m⁻² */
  insolation: number;
  /** the solar constant it was computed with, `SOLAR_CONSTANT_BERGER_LOUTRE_1991`, in W m⁻² */
  solarConstant: number;
  /** the series and the formula */
  source: string;
}

/** One line of `hc_edtf_parse`. */
export interface EdtfPart {
  /** `value` for the whole text, `start` and `end` for the two sides of an interval, `member` for each date or run of a set */
  role: string;
  /** for the value `date`, `interval`, `earlier-than`, `later-than`, `one-of` (a `[...]` set: exactly one holds) or `all-of` (a `{...}` list: all apply); for a part `date`, `open` (`..`), `unknown` (empty), `range`, `earlier-than` or `later-than` */
  kind: string;
  /** the part in its canonical form, which parses to the same value */
  text: string;
  /** `unknown`, `millennium`, `century`, `decade`, `year`, `month` or `day`; empty where the part is not one date */
  precision: string | null;
  /** `certain`, `uncertain` for `?`, `approximate` for `~`, `uncertain-and-approximate` for `%`; empty where the part is not one date */
  qualifier: string | null;
  /** `1` for the long `Y` form of a year; empty where the part is not one date */
  longForm: boolean | null;
  /** the first fixed day the part could denote, inclusive; for the value, the first day of its support, which a `~` or `%` qualifier widens by one unit of the stated precision on each side; empty where it does not stop */
  firstDay: bigint | null;
  /** the last fixed day, inclusive; empty where it does not stop */
  lastDay: bigint | null;
  /** on the value `resolved` (one unit of a scale), `bounded`, `before`, `after` or `unknown`; empty on a part */
  support: string | null;
  /** on the value, the best estimate as seconds from 1970-01-01 00:00:00 counting 86 400-second days, the midpoint; empty for an open or unknown value and on a part */
  estimate: bigint | null;
  /** on the value, the support's width in days; empty where it is open and on a part */
  spanDays: bigint | null;
}

/** One line of `hc_edtf_relations`. */
export interface EdtfRelations {
  /** the relations that are possible, by name, in Allen's order, separated by `;`: `before`, `meets`, `overlaps`, `starts`, `during`, `finishes`, `equals`, `finished by`, `contains`, `started by`, `overlapped by`, `met by`, `after` */
  relations: string[];
  /** the same by Allen's symbols, separated by `;`: `<`, `m`, `o`, `s`, `d`, `f`, `=`, `fi`, `di`, `si`, `oi`, `mi`, `>` */
  symbols: string[];
  /** `1` where the first is before the second whatever the unknown bounds are */
  definitelyBefore: boolean;
  /** `1` where it could be */
  possiblyBefore: boolean;
  /** `1` where the first is after the second whatever the unknown bounds are */
  definitelyAfter: boolean;
  /** `1` where it could be */
  possiblyAfter: boolean;
  /** `1` where the two could overlap in time */
  possiblyConcurrent: boolean;
}

/** One line of `hc_significant`. */
export interface SignificantNumber {
  /** the number given */
  value: number;
  /** the count given */
  figures: number;
  /** the number printed to exactly those figures: `1.38e10`, not `13800000000` */
  text: string;
  /** the number rounded to them, as a double */
  rounded: number;
  /** the decimal exponent of its leading digit as reported */
  exponent: number;
  /** the decimal place of its last significant digit */
  lastPlace: number;
}

/** One line of `hc_significant_op`. */
export interface SignificantResult {
  /** the operation, in lower case */
  operation: string;
  /** the result printed to the figures the rule leaves it */
  text: string;
  /** the result as a double */
  value: number;
  /** its figure count */
  figures: number;
  /** the decimal exponent of its leading digit */
  exponent: number;
  /** the decimal place of its last significant digit */
  lastPlace: number;
}

/** One line of `hc_uncertain`. */
export interface UncertainQuantity {
  /** the value given */
  value: number;
  /** the standard deviation given */
  stdDev: number;
  /** the pair as `value ± σ` */
  text: string;
  /** the value printed to the figures its standard deviation supports; empty where it supports none */
  significant: string | null;
  /** the standard deviation over the value; empty for a value of 0 */
  relative: number | null;
  /** the value less one standard deviation */
  low1σ: number;
  /** the value plus one standard deviation */
  high1σ: number;
  /** the value less two */
  low2σ: number;
  /** the value plus two */
  high2σ: number;
  /** the value less three */
  low3σ: number;
  /** the value plus three */
  high3σ: number;
}

/** One line of `hc_uncertain_op`. */
export interface UncertainResult {
  /** the operation, in lower case */
  operation: string;
  /** the result; for `z-score` the number of standard deviations */
  value: number;
  /** its standard deviation; empty for a z-score */
  stdDev: number | null;
  /** the result as `value ± σ` */
  text: string;
  /** the result printed to the figures its standard deviation supports; empty where it supports none */
  significant: string | null;
}

/** One line of `hc_interval`. */
export interface IntervalResult {
  /** the operation, in lower case */
  operation: string;
  /** `1` where the result is the empty interval; empty for a question */
  empty: boolean | null;
  /** the low bound, whole seconds */
  lowSeconds: bigint | null;
  /** and the attoseconds into the second */
  lowAttoseconds: bigint | null;
  /** the high bound, whole seconds */
  highSeconds: bigint | null;
  /** and the attoseconds */
  highAttoseconds: bigint | null;
  /** the width, whole seconds */
  widthSeconds: bigint | null;
  /** and the attoseconds */
  widthAttoseconds: bigint | null;
  /** the midpoint, whole seconds */
  midpointSeconds: bigint | null;
  /** and the attoseconds */
  midpointAttoseconds: bigint | null;
  /** for `overlaps` and `contains`, `1` or `0`; empty for an interval */
  holds: boolean | null;
}

/** One line of `hc_units`. */
export interface TimeUnit {
  /** the unit's stable identifier, lower-case and hyphenated; match on this */
  id: string;
  /** the English name */
  name: string;
  /** the conventional symbol; empty where there is none */
  symbol: string | null;
  /** the numerator of the length in seconds, in lowest terms */
  secondsNumerator: bigint;
  /** its denominator, positive */
  secondsDenominator: bigint;
  /** `si`, `civil`, `horological`, `decimal`, `hexadecimal`, `media` or `scientific` */
  family: string;
  /** who defines it, specifically enough to check */
  authority: string;
}

/** One line of `hc_unit_convert`. */
export interface UnitConversion {
  /** the unit given */
  from: string;
  /** the unit written */
  to: string;
  /** the count given, in lowest terms */
  countNumerator: bigint;
  /** its denominator */
  countDenominator: bigint;
  /** the count in the other unit */
  convertedNumerator: bigint;
  /** its denominator */
  convertedDenominator: bigint;
  /** `1` where the converted count is a whole number */
  whole: boolean;
  /** the length in seconds */
  secondsNumerator: bigint;
  /** its denominator */
  secondsDenominator: bigint;
  /** `1` where an attosecond count holds that length exactly: a flick, a third of a second and an NTSC frame are lengths no `Duration` holds */
  attosecondExact: boolean;
}

/** One line of `hc_rates`. */
export interface FrameRate {
  /** the rate's identifier: `24`, `29.97`, `44100` */
  id: string;
  /** `frame` or `sample` */
  kind: string;
  /** the numerator of the rate in events per second, in lowest terms */
  hertzNumerator: bigint;
  /** its denominator */
  hertzDenominator: bigint;
}

/** One line of `hc_frame_period`. */
export interface FramePeriod {
  /** the rate's identifier; `custom` for one given as a fraction */
  rate: string;
  /** `frame`, `sample` or `custom` */
  kind: string;
  /** the rate in events per second, in lowest terms */
  hertzNumerator: bigint;
  /** its denominator */
  hertzDenominator: bigint;
  /** the length of one event in seconds */
  periodNumerator: bigint;
  /** its denominator */
  periodDenominator: bigint;
  /** the length counted in the unit */
  countNumerator: bigint;
  /** its denominator */
  countDenominator: bigint;
  /** the unit's identifier */
  unit: string;
  /** `1` where the length is a whole number of that unit */
  whole: boolean;
  /** `1` where it is a whole number of flicks, the unit chosen so that every common frame and sample rate is */
  wholeFlicks: boolean;
}

/** One line of `hc_tempo`. */
export interface NoteAtTempo {
  /** the tempo in beats per minute, in lowest terms */
  bpmNumerator: bigint;
  /** its denominator */
  bpmDenominator: bigint;
  /** the length of one beat in seconds */
  beatNumerator: bigint;
  /** its denominator */
  beatDenominator: bigint;
  /** the note's length as a fraction of a whole note */
  noteFractionNumerator: bigint;
  /** its denominator */
  noteFractionDenominator: bigint;
  /** the note's length in seconds */
  noteNumerator: bigint;
  /** its denominator */
  noteDenominator: bigint;
  /** the microseconds per quarter note a MIDI `Set Tempo` event stores; empty where the tempo does not fit its 24 bits */
  midiMicroseconds: number | null;
  /** `1` where that integer holds the tempo exactly; empty with the cell before */
  midiExact: boolean | null;
}

/** One line of `hc_fiscal_profiles`. */
export interface FiscalSystem {
  /** the ISO 3166-1 alpha-2 code */
  country: string;
  /** the country in English */
  countryName: string;
  /** `fiscal` for a country's government, tax and corporate years, `school` and `university` for an academic profile's */
  table: string;
  /** `government`, `personal-tax`, `corporate-default` or `academic` */
  kind: string;
  /** the English name */
  name: string;
  /** the name in the local language; empty where English is the local one */
  localName: string | null;
  /** `statute`, `regulation`, `convention`, `per-region`, `per-institution` or `unread` */
  authority: string;
  /** `1` where that is a national rule and not a usual choice */
  national: boolean;
  /** the calendar the start is dated in: `gregory`, `persian-arithmetic-33`, `ethiopic`, `buddhist` or `bikram-sambat` */
  startCalendar: string;
  /** the start's month in it */
  startMonth: number;
  /** the start's day in it */
  startDay: number;
  /** `start-year` or `end-year`: Japan's 2024年度 begins in 2024, the United States' FY 2024 began on 1 October 2023 */
  labelConvention: string;
  /** the year label the system was established in, a label of its own calendar, before which it is absent; empty where no source read gives it */
  validFrom: number | null;
  /** the last; empty where it still is */
  validUntil: number | null;
  /** `1` where the start calendar is an approximation of the astronomical rule */
  approximate: boolean;
  /** what the entry deliberately does not claim */
  note: string;
  /** the date the sources were last checked, `YYYY-MM-DD` */
  sourcesChecked: string;
  /** the statute, ministry or publication the entry came from */
  sources: string;
  /** the first year label the sources read reach; every label of the system before it is a gap */
  readFrom: number;
  /** spans of labels after it that no source reaches either, `first-last` separated by `;`; empty where there are none */
  unread: Array<{ first: number; last: number }>;
}

/** One line of `hc_fiscal_year_on`. */
export interface FiscalYearOfDay {
  /** the code, as `hc_fiscal_profiles` writes it */
  country: string;
  /** `fiscal`, `school` or `university` */
  table: string;
  /** the kind */
  kind: string;
  /** the English name */
  name: string;
  /** `in-force`, `outside-validity`, `gap` or `outside-calendar-range` */
  status: string;
  /** the year label of the year the day is in */
  label: number | null;
  /** that year's first fixed day */
  first: number | null;
  /** its last fixed day */
  last: number | null;
  /** the day's number in it, from 1 */
  dayOfYear: number | null;
  /** how many days it has */
  daysInYear: number | null;
  /** the day of the week from Monday = 1 to Sunday = 7 */
  weekday: number | null;
  /** the fiscal month from 1; empty where the start calendar has no twelve equal months */
  month: number | null;
  /** the fiscal quarter from 1 */
  quarter: number | null;
  /** the fiscal half from 1 */
  half: number | null;
  /** the calendar the start is dated in */
  startCalendar: string;
  /** `start-year` or `end-year` */
  labelConvention: string;
  /** `1` where the start calendar is an approximation */
  approximate: boolean;
  /** the date the sources were last checked */
  sourcesChecked: string;
}

/** One line of `hc_fiscal_year_span`. */
export interface FiscalYearSpan {
  /** the code */
  country: string;
  /** `fiscal`, `school` or `university` */
  table: string;
  /** the kind */
  kind: string;
  /** the English name */
  name: string;
  /** `in-force`, `outside-validity`, `gap` or `outside-calendar-range` */
  status: string;
  /** the label given */
  label: number;
  /** the year's first fixed day */
  first: number | null;
  /** its last fixed day */
  last: number | null;
  /** how many days it has */
  days: number | null;
}

/** One line of `hc_week_year_systems`. */
export interface WeekYearSystem {
  /** the system's identifier: `nrf-4-5-4`, `iso-8601-week-year` */
  id: string;
  /** the English name */
  name: string;
  /** the weekday the year ends on, Monday = 1 to Sunday = 7 */
  weekday: number;
  /** the Gregorian month whose end the rule is applied to */
  month: number;
  /** `last-weekday-of-month` or `weekday-nearest-month-end` */
  anchorRule: string;
  /** `start-year` or `end-year` */
  labelConvention: string;
  /** how a quarter's thirteen weeks split into three periods: `4-4-5`, `4-5-4` or `5-4-4`; empty for a convention that numbers weeks and defines no periods */
  shape: string | null;
  /** what the entry does not claim */
  note: string;
  /** the published definition */
  source: string;
  /** the date it was last checked, `YYYY-MM-DD` */
  sourcesChecked: string;
}

/** One line of `hc_week_year_on`. */
export interface WeekYearOfDay {
  /** the system's identifier */
  system: string;
  /** its English name */
  name: string;
  /** the label of the year the day is in */
  label: number;
  /** that year's first fixed day */
  first: number;
  /** its last fixed day */
  last: number;
  /** how many weeks it has, 52 or 53 */
  weeks: number;
  /** `1` where it has 53 */
  long: boolean;
  /** the week the day is in, from 1 */
  week: number;
  /** that week's first fixed day */
  weekFirst: number;
  /** its last fixed day */
  weekLast: number;
  /** the period from 1 to 12; empty for a convention with no periods */
  period: number | null;
  /** that period's first fixed day */
  periodFirst: number | null;
  /** its last fixed day */
  periodLast: number | null;
  /** the quarter from 1 to 4 */
  quarter: number | null;
}

/** One line of `hc_name_day_lists`. */
export interface NameDayList {
  /** `list` or `gap` */
  kind: string;
  /** the list's identifier, such as `lv-traditional-2026`; for a gap its code */
  id: string;
  /** the ISO 3166-1 alpha-2 code of a list; the English name of a gap */
  country: string;
  /** the BCP 47 language of the names */
  language: string | null;
  /** the English name of the list; for a gap what it leaves out */
  name: string;
  /** the body whose list it is */
  authority: string | null;
  /** when it decided this edition, to whatever precision the source gives */
  decided: string | null;
  /** `promulgated`, `recorded`, `vernacular` or `contested` */
  provenance: string | null;
  /** the first year the edition was in force; empty where open */
  validFrom: number | null;
  /** the last year; empty where it still is */
  validUntil: number | null;
  /** the terms it is held under */
  licence: string | null;
  /** what it does with 29 February: `no-names`, `own-names`, `shift-after-24-february` or `leap-years-only` */
  leapDay: string | null;
  /** how many names it holds */
  totalNames: number | null;
  /** the citation; for a gap what was consulted */
  source: string;
  /** when the file was retrieved; for a gap when the survey was made */
  retrieved: string;
  /** for a gap `licensed-for-a-fee`, `licence-unknown`, `rights-reserved`, `no-keeper-found`, `not-yet-read` or `saints-not-names` */
  reason: string | null;
  /** for a gap the reasoning in full */
  explanation: string | null;
}

/** One line of `hc_name_days_on`. */
export interface NameDaysOfDay {
  /** `list`, `outside` or `gap` */
  kind: string;
  /** the list's identifier; for a gap its code */
  id: string;
  /** the English name of the list; for a gap what it leaves out */
  name: string;
  /** the body whose list it is */
  authority: string | null;
  /** the first year the edition was in force; empty where open */
  validFrom: number | null;
  /** the last year; empty where it still is */
  validUntil: number | null;
  /** the terms it is held under */
  licence: string | null;
  /** the names the list gives that day, separated by `;`, in the list's own spelling and script; empty where there are none */
  names: string[];
  /** how many */
  count: number | null;
  /** `1` where the authority reserves the day for names not on its list (Latvia's 22 May) */
  unlistedNamesDay: boolean | null;
  /** what the source prints beside the day that is not a name, separated by `;` */
  notes: string | null;
  /** the citation; for a gap what was consulted */
  source: string;
  /** for a gap the reason, as `hc_name_day_lists` writes it */
  reason: string | null;
  /** for a gap the reasoning in full */
  explanation: string | null;
}

/** One line of `hc_name_day`. */
export interface NameDayDates {
  /** `list`, `outside` or `gap` */
  kind: string;
  /** the list's identifier; for a gap its code */
  id: string;
  /** the English name of the list; for a gap what it leaves out */
  name: string;
  /** the body whose list it is */
  authority: string | null;
  /** the first year the edition was in force; empty where open */
  validFrom: number | null;
  /** the last year; empty where it still is */
  validUntil: number | null;
  /** the terms it is held under */
  licence: string | null;
  /** the days as `MM-DD`, separated by `;` */
  dates: string[];
  /** the same days as fixed days of that year, separated by `;` */
  fixedDays: number[];
  /** how many */
  count: number | null;
  /** the citation; for a gap what was consulted */
  source: string;
  /** for a gap the reason */
  reason: string | null;
  /** for a gap the reasoning in full */
  explanation: string | null;
}

/** One line of `hc_attribution_authorities`. */
export interface AttributionAuthority {
  /** `authority` or `gap` */
  kind: string;
  /** the subject; empty for a gap */
  subject: string | null;
  /** the list's identifier, such as `birthstones-jp-2021`; for a gap its identifier */
  id: string;
  /** the list's English name; for a gap what is missing */
  name: string;
  /** the body that issued it; empty where nobody did, as for the Finnish month names */
  body: string | null;
  /** the identifier of the region the list is in use in */
  region: string | null;
  /** its English name */
  regionName: string | null;
  /** when the list was first adopted, to whatever precision the source gives */
  established: string | null;
  /** when it was last revised */
  revised: string | null;
  /** the first year it was current; empty where open */
  validFrom: number | null;
  /** the last year; empty where it still is */
  validUntil: number | null;
  /** `promulgated`, `recorded`, `vernacular`, `contested` or `modern-invention` */
  provenance: string | null;
  /** what the list is keyed by: `month`, `lunation`, `sign` or `weekday` */
  keyKind: string | null;
  /** the citation; for a gap what was consulted */
  source: string;
  /** what a caller should know before repeating the list; always present for a contested list */
  caveat: string | null;
  /** for a gap `sources-disagree-with-no-authority`, `method-unpublished`, `different-key`, `modern-invention` or `translation-undetermined` */
  reason: string | null;
  /** for a gap the reasoning in full */
  explanation: string | null;
}

/** One line of `hc_attributions`. */
export interface Attribution {
  /** the subject */
  subject: string;
  /** the list's identifier */
  id: string;
  /** the list's English name */
  name: string;
  /** the key given */
  key: number;
  /** `month`, `lunation`, `sign` or `weekday` */
  keyKind: string;
  /** what the list attributes to the key, separated by `;`, in the list's own spelling */
  attributions: string[];
  /** how many */
  count: number;
  /** the English gloss of a month name; empty for other subjects */
  gloss: string | null;
  /** the first year the list was current; empty where open */
  validFrom: number | null;
  /** the last year; empty where it still is */
  validUntil: number | null;
  /** as `hc_attribution_authorities` writes it */
  provenance: string;
  /** what a caller should know before repeating it */
  caveat: string | null;
  /** `1` where every list of the subject says the same for the key */
  agreed: boolean;
}

/** One line of `hc_attributions_on`. */
export interface AttributionOfDay {
  /** the subject */
  subject: string;
  /** the list's identifier */
  id: string;
  /** the list's English name */
  name: string;
  /** the day's month, sign or weekday */
  key: number;
  /** `month`, `sign` or `weekday` */
  keyKind: string;
  /** what the list attributes to the key, separated by `;` */
  attributions: string[];
  /** how many */
  count: number;
  /** the English gloss of a month name */
  gloss: string | null;
  /** the first year the list was current; empty where open */
  validFrom: number | null;
  /** the last year; empty where it still is */
  validUntil: number | null;
  /** as `hc_attribution_authorities` writes it */
  provenance: string;
  /** what a caller should know before repeating it */
  caveat: string | null;
  /** `1` where every list of the subject says the same for the key */
  agreed: boolean;
}

/** One line of `hc_harvest_moon`. */
export interface HarvestMoon {
  /** the year given */
  year: number;
  /** the fixed day of the Harvest Moon */
  harvestMoon: number;
  /** the fixed day of the Hunter's Moon after it */
  huntersMoon: number;
  /** the Gregorian month the Harvest Moon falls in, 9 or 10 */
  month: number;
  /** the name the Old Farmer's Almanac gives September's full moon that year: `Harvest Moon`, or `Corn Moon` when the Harvest Moon is October's */
  septemberMoon: string;
  /** the meridian the days are judged at, as given */
  meridian: string;
  /** the functions used and the rule */
  source: string;
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
  /**
   * `hc_day_has_leap_second`. A day past the announced leap-second table is
   * `no-data` under `strict`, and false otherwise.
   */
  dayHasLeapSecond(unixSeconds: number | bigint, strict?: boolean): boolean;
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
  /**
   * `hc_tai_minus_utc_exact`: `TAI - UTC` at a POSIX instant as whole seconds and attoseconds;
   * not a whole number of seconds from 1961 to 1971. `strict` refuses outside the leap-second
   * table with `no-data`.
   */
  taiMinusUtcExact(unixSeconds: number | bigint, attoseconds?: number | bigint, strict?: boolean): TaiMinusUtc;
  /**
   * `hc_utc_from_tai_exact`: a TAI instant's POSIX second, the attoseconds into it, and whether it
   * is a leap second. `strict` refuses outside the leap-second table with `no-data`.
   */
  utcFromTaiExact(taiSeconds: number | bigint, taiAttoseconds?: number | bigint, strict?: boolean): UtcInstantLabel;
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
  /** `hc_ccsds_decode`: a Level 2, 3 or 4 code is `no-data` (`ccsdsDecodeFromEpoch` reads a Level 2 one), octets that are not a code `malformed`. */
  ccsdsDecode(hex: string, strict?: boolean): CcsdsCode;
  /** `hc_ccsds_encode`: the code in lower-case hexadecimal, P-field first. */
  ccsdsEncode(taiSeconds: number | bigint, attoseconds: number | bigint, pField: string, strict?: boolean): string;
  /** `hc_ccsds_decode_from_epoch`: a Level 2 code read from the caller's epoch; an epoch outside the years 1 to 9999 is `out-of-range`. */
  ccsdsDecodeFromEpoch(hex: string, epoch: CcsdsEpoch, strict?: boolean): CcsdsCode;
  /** `hc_ccsds_encode_from_epoch`: a Level 2 format counted from the caller's epoch. */
  ccsdsEncodeFromEpoch(
    taiSeconds: number | bigint,
    attoseconds: number | bigint,
    pField: string,
    epoch: CcsdsEpoch,
    strict?: boolean,
  ): string;
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
  /** `hc_radio_decode`: JJY's call-sign frame is `no-data`; `jjyCallSignDecode` reads it. */
  radioDecode(code: RadioCode, frame: string, century: number | bigint): RadioMinute;
  /** `hc_radio_encode`: a frame of `0`, `1` and `M`. */
  radioEncode(code: RadioCode, unixSeconds: number | bigint, options?: RadioEncodeOptions): string;
  /** `hc_jjy_call_sign_decode`: an ordinary minute's frame is `malformed`. */
  jjyCallSignDecode(frame: string, year: number | bigint): JjyCallSign;
  /** `hc_jjy_call_sign_encode`: a minute but 15 or 45 of JST is `out-of-range`. */
  jjyCallSignEncode(unixSeconds: number | bigint, notice?: JjyStopNotice): string;
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
  /** `hc_irig_frame_start`: the start of the frame that holds a reading. */
  irigFrameStart(signal: string, secondsOfDay: number, hundredths?: number): IrigFrameStart;
  /** `hc_dotnet_ticks_from_unix`: the ticks, which need not fit a number. */
  dotnetTicksFromUnix(unixSeconds: number | bigint, attoseconds?: number | bigint): bigint;
  /** `hc_unix_from_dotnet_ticks`. */
  unixFromDotnetTicks(ticks: number | bigint): DotnetReading;
  /** `hc_six_hour_clock`: `secondsOfDay` of the caller's wall clock. */
  sixHourClock(reckoning: SixHourReckoning, secondsOfDay: number): SixHourReading;
  /** `hc_civil_from_six_hour_clock`: seconds after civil midnight. */
  civilFromSixHourClock(reckoning: SixHourReckoning, hour: number, minute: number, second: number, night: boolean): number;
  /** `hc_french_decimal_time`: the decimal time of a time of the civil clock. */
  frenchDecimalTime(secondsOfDay: number, attoseconds?: number | bigint): FrenchDecimalTime;
  /** `hc_civil_from_french_decimal_time`: the civil time of a decimal time. */
  civilFromFrenchDecimalTime(hour: number, minute: number, second: number, attoseconds?: number | bigint): TimeOfDay;
  /** `hc_ioc_olympiad_on`: the modern Olympiad of a day, by the Charter in force on it. */
  iocOlympiadOn(fixed: number | bigint): number;
  /** `hc_babylonian_regnal_year`: the king and regnal year of a Seleucid year. */
  babylonianRegnalYear(seleucidYear: number | bigint): RegnalYear;
  /** `hc_equinox_new_year_margin`: how close an equinox came to the moment that decides a new year. */
  equinoxNewYearMargin(calendar: EquinoxCalendar, year: number | bigint): EquinoxMargin;
  /** `hc_shmuel_tekufah`: a tekufah of Shmuel's reckoning. */
  shmuelTekufah(hebrewYear: number | bigint, tekufah: Tekufah): ShmuelTekufah;
  /** `hc_day_name`: a day's name by one of its calendar's namings. */
  dayName(calendar: string, naming: string, fixed: number | bigint): DayName;

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
  /** `hc_parse_datetime`: a date-time read in a syntax, as a reading. */
  parseDatetime(syntax: DatetimeSyntax, text: string): Reading;
  /** `hc_format_datetime`: an instant written as a date-time in a syntax and to a precision. */
  formatDatetime(
    syntax: DatetimeFormat,
    unixSeconds: number | bigint,
    attoseconds?: number | bigint,
    offsetSeconds?: number,
    precision?: DatetimePrecision,
  ): FormattedDatetime;
  /** `hc_format_iso_date_as`: a fixed day as an ISO 8601 calendar, ordinal or week date. */
  formatIsoDateAs(fixed: number | bigint, form?: IsoDateForm, style?: IsoDateStyle): FormattedIsoDate;
  /** `hc_iso_date_parts`: an ISO 8601 date read into its parts, reduced accuracy included. */
  isoDateParts(text: string): IsoDateParts;
  /** `hc_iso_duration`: an ISO 8601 duration read into its components. */
  isoDuration(text: string): IsoDurationParts;
  /** `hc_format_iso_duration`: a duration written from its components. */
  formatIsoDuration(components: IsoDurationInput): FormattedIsoDuration;
  /** `hc_iso_interval`: an ISO 8601 interval or repeating interval read into its ends. */
  isoInterval(text: string): IsoIntervalParts;
  /** `hc_parse_pattern`: a text read against a `strptime` or CLDR pattern in the C locale's names. */
  parsePattern(syntax: PatternSyntax, pattern: string, text: string): PatternFields;
  /** `hc_parse_pattern_in`: `parsePattern` with a locale's names; `python` takes none. */
  parsePatternIn(syntax: Exclude<PatternSyntax, "python">, pattern: string, text: string, locale: string): PatternFields;
  /** `hc_locale_chain`: the fallback chain of a locale, from its tag to `und`. */
  localeChain(locale: string): LocaleChainStep[];
  /** `hc_locale_info`: what a locale is, its week, numbering, direction, casing and plural rules. */
  localeInfo(locale: string): LocaleInfo;
  /** `hc_plural_category`: the plural category a number written as text has in a locale. */
  pluralCategory(locale: string, number: string, kind?: PluralKind): PluralCategoryAnswer;
  /** `hc_names`: the names a locale has for a calendar in a width and a context. */
  names(locale: string, calendar: string, width?: NameWidth, context?: NameContext): LocaleName[];
  /** `hc_case`: a text recased as a locale cases it. */
  caseText(locale: string, mode: CaseMode, text: string): CasedText;
  /** `hc_isolate`: a text made safe to embed in text running a locale's direction. */
  isolate(locale: string, mode: IsolateMode, text: string): IsolatedText;
  /** `hc_gregorian_adoption`: the steps by which a country adopted the Gregorian calendar, by ISO 3166-1 alpha-2 code; none for a code the module does not know. */
  gregorianAdoption(region: string): GregorianAdoption[];
  /** `hc_panchanga_at`: the yoga's line, then the karaṇa's; an ayanamsa nobody knows is `unknown`. */
  panchangaAt(unixSeconds: number | bigint, ayanamsa: Ayanamsa | "surya-siddhanta"): PanchangaLimb[];
  /** `hc_tithi_at`: the tithi in progress, with its span; `sky` is `true`, an ayanāṃśa or `surya-siddhanta`. */
  tithiAt(unixSeconds: number | bigint, sky: Ayanamsa | "true" | "surya-siddhanta"): Tithi;
  /** `hc_tithis_of_day`: the tithis between a day's sunrise and the next, flagged repeated or skipped; no sunrise is `no-data`. */
  tithisOfDay(
    fixed: number | bigint,
    latitude: number,
    longitude: number,
    elevation: number,
    sky: Ayanamsa | "true" | "surya-siddhanta",
  ): TithiOfDay[];
  /** `hc_ayanamsas`: every named ayanāṃśa with its anchor. */
  ayanamsas(): AyanamsaInfo[];
  /** `hc_ayanamsa_at`: a named ayanāṃśa's value in degrees at an instant. */
  ayanamsaAt(unixSeconds: number | bigint, ayanamsa: Ayanamsa): AyanamsaValue;
  /** `hc_ayanamsa_from_anchor`: the value of an ayanāṃśa the caller anchors, `custom`. */
  ayanamsaFromAnchor(unixSeconds: number | bigint, anchorJulianDate: number, degreesAtAnchor: number): AyanamsaValue;
  /** `hc_festival_readings`: the Smārta and Vaiṣṇava readings of a festival's day. */
  festivalReadings(): FestivalReading[];
  /** `hc_janmashtami`: the day of Kṛṣṇa Janmāṣṭamī in a Gregorian year at a place by a reading. */
  janmashtami(
    year: number | bigint,
    reading: FestivalReadingId,
    latitude: number,
    longitude: number,
    elevation: number,
    ayanamsa: Ayanamsa,
  ): FestivalDay;
  /** `hc_vaishnava_day`: the first day whose sunrise carries a tithi of a month of a Śaka year. */
  vaishnavaDay(
    sakaYear: number | bigint,
    month: number,
    tithi: number,
    latitude: number,
    longitude: number,
    elevation: number,
    ayanamsa: Ayanamsa,
  ): VaishnavaDay;
  /** `hc_vishti_free_span`: the part of a tithi Bhadra does not cover. */
  vishtiFreeSpan(
    sakaYear: number | bigint,
    month: number,
    tithi: number,
    latitude: number,
    longitude: number,
    elevation: number,
    ayanamsa: Ayanamsa,
  ): VishtiFreeSpan;
  /** `hc_rahu_at`: the mean Rāhu and Ketu at an instant. */
  rahuAt(unixSeconds: number | bigint, ayanamsa: Ayanamsa): NodePlace;
  /** `hc_rahu_ingresses`: the mean node's entries into the signs in a span. */
  rahuIngresses(fromUnixSeconds: number | bigint, toUnixSeconds: number | bigint, ayanamsa: Ayanamsa): NodeIngress[];
  /** `hc_era_new_year`: the first day of a year of a historical Indian era over the lunisolar months. */
  eraNewYear(calendar: LunarEraId, year: number | bigint): number;
  /** `hc_panchanga_of_day`: read at the day's sunrise at the place; no sunrise is `no-data`. */
  panchangaOfDay(
    fixed: number | bigint,
    latitude: number,
    longitude: number,
    elevation: number,
    ayanamsa: Ayanamsa | "surya-siddhanta",
  ): PanchangaLimb[];
  /** `hc_muhurtas`: the thirty muhūrtas of a day, Abhijit and Dur Muhurtam marked. */
  muhurtas(fixed: number | bigint, latitude: number, longitude: number, elevation: number): Muhurta[];
  /** `hc_amrita_siddhi`: the amṛta siddhi yoga of a day at a place. */
  amritaSiddhi(fixed: number | bigint, latitude: number, longitude: number, elevation: number, ayanamsa: Ayanamsa): AmritaSiddhi;
  /** `hc_nakshatra_at`: the nakṣatra the Moon is in at an instant. */
  nakshatraAt(unixSeconds: number | bigint, ayanamsa: Ayanamsa): NakshatraStay;
  /** `hc_nakshatra_of_day`: the nakṣatra a day carries at a place, read at its sunrise. */
  nakshatraOfDay(fixed: number | bigint, latitude: number, longitude: number, elevation: number, ayanamsa: Ayanamsa): NakshatraStay;
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
  barhaspatyaYearAt(rule: BarhaspatyaRule, unixSeconds: number | bigint, locale?: string): BarhaspatyaNameAt;
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
  /** `hc_chinese_age`: a person's age on a day by a count. */
  chineseAge(convention: AgeConvention, birthFixed: number | bigint, onFixed: number | bigint): number;
  /** `hc_chinese_almanac_solar_terms`: the Qing almanac's term days of a year. */
  chineseAlmanacSolarTerms(year: number | bigint): AlmanacSolarTerm[];
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
  /** `hc_almanac_directions`: the 八将神 and the 金神 of the year in force on a day. */
  almanacDirections(fixed: number | bigint, meridian?: string): AlmanacDirection[];
  /** `hc_rounichi`: the fixed day of 臘日 in the winter ending in `year`. */
  rounichi(rule: RounichiRule, year: number | bigint, meridian?: string): number;
  /** `hc_mansion_undertakings`: what a publisher's list says the day's 二十八宿 favours and forbids. */
  mansionUndertakings(list: UndertakingListId, fixed: number | bigint): MansionUndertaking[];
  /** `hc_almanac_person_days`: a person's own 五墓日 and 三箇の悪日 on a day. */
  almanacPersonDays(fixed: number | bigint, birthFixed: number | bigint, meridian?: string): AlmanacPersonDay[];
  /** `hc_tibetan_almanac_day`: what a version's almanac prints for a day. */
  tibetanAlmanacDay(calendar: TibetanCalendarId, fixed: number | bigint): TibetanAlmanacEntry[];
  /** `hc_tibetan_planets`: the Phugpa planets at the end of a day. */
  tibetanPlanets(fixed: number | bigint): TibetanPlanet[];
  /** `hc_bhutanese_winter_solstice`: the Bhutanese winter solstice of a Gregorian year. */
  bhutaneseWinterSolstice(year: number | bigint): BhutaneseWinterSolstice;
  /** `hc_tibetan_festival_day`: the day a festival on a Tibetan date is kept, by a rule. */
  tibetanFestivalDay(
    rule: TibetanFestivalRule,
    calendar: TibetanCalendarId,
    year: number | bigint,
    month: number,
    leap: boolean,
    day: number,
  ): number;
  /** `hc_choghadiya`: sixteen lines, the day's eight then the night's. */
  choghadiya(fixed: number | bigint, latitude: number, longitude: number, elevation?: number, locale?: string): ChoghadiyaPart[];
  /** `hc_panchak`: the weekday of the opening on a clock `offsetSeconds` ahead of UTC, 0 unless given. */
  panchak(naming: PanchakNaming, unixSeconds: number | bigint, ayanamsa: Ayanamsa, offsetSeconds?: number, locale?: string): PanchakWindow;
  /** `hc_kumbh`: `jupiter` is the caller's, the library having no ephemeris of Jupiter; empty unless given. */
  kumbh(yoga: KumbhYoga, year: number | bigint, ayanamsa: Ayanamsa, jupiter?: SiderealSignId | "", locale?: string): KumbhOccasion;
  /** `hc_kumbh_yogas`: the Mela Adhikari's seven conditions, with the identifiers `kumbh` takes. */
  kumbhYogas(locale?: string): KumbhYogaInfo[];
  /** `hc_pushkaram_rivers`: every river, with the sign Jupiter enters for it. */
  pushkaramRivers(locale?: string): PushkaramRiver[];
  /** `hc_pushkaram_rules`: the two entry rules `pushkaramBySky` takes. */
  pushkaramRules(): PushkaramRule[];
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

  /** `hc_holiday_is_day_off`; `region` may be empty, and `group` left out or empty for everyone. A code naming no table, a group naming no group of `holidayGroups`, or a region the table's country has no subdivision for, is `unknown`; a day a gap of its year leaves open is `no-data`, and one whose region's weekend law was not read `out-of-range`. */
  holidayIsDayOff(code: string, region: string, fixed: number | bigint, group?: string): boolean;
  /** `hc_holiday_add_business_days`: a day moved by business days of a table. */
  holidayAddBusinessDays(code: string, region: string, fixed: number | bigint, count: number | bigint, group?: string): number;
  /** `hc_holiday_is_weekend`; `region` may be empty. A day on which the region's weekend law was not read is `out-of-range`. */
  holidayIsWeekend(code: string, region: string, fixed: number | bigint): boolean;
  /** `hc_holiday_next`: the first holiday after a day, as a row of `holidaysInYear`; `kind` as for `holidaysInYear`, and the kinds that stop work when empty. A gap that could hide a nearer entry, and no entry within sixteen years, are `no-data`. */
  holidayNext(code: string, region: string, fixed: number | bigint, group?: string, kind?: string): HolidayInYear;
  /** `hc_holiday_previous`: the last holiday before a day, as `holidayNext`. */
  holidayPrevious(code: string, region: string, fixed: number | bigint, group?: string, kind?: string): HolidayInYear;
  /** `hc_holiday_business_days_between`: the business days of a table in a half-open interval. */
  holidayBusinessDaysBetween(
    code: string,
    region: string,
    fromFixed: number | bigint,
    toFixed: number | bigint,
    group?: string,
  ): number;
  /** `hc_holidays_in_year`; `region` may be empty, `group` left out or empty for everyone, and `kind` left out or empty for every kind, or a `;`-separated list of `HolidayKind`s, `"public;bank"`, which keeps the entries of those kinds and the gaps of their rules. */
  holidaysInYear(code: string, region: string, year: number | bigint, group?: string, kind?: string): HolidayInYear[];
  /** `hc_holiday_codes`: countries, then exchanges, traditions and the international sets. */
  holidayCodes(): string[];
  /** `hc_holidays_on`: every entry on one day across every table, in `holidayCodes()` order. A day with no Gregorian year is `out-of-range`. */
  holidaysOn(fixed: number | bigint): HolidayOn[];
  /** `hc_holiday_tables`: every table, in `holidayCodes()` order; `und` unless given. */
  holidayTables(locale?: string): HolidayTable[];
  /** `hc_lectionary`; before 1 January 1970 or after the liturgical year 4099 is `out-of-range`. */
  lectionary(fixed: number | bigint): Lectionary;
  /** `hc_astronomical_easter`; outside 1583 to 2150 is `out-of-range`. */
  astronomicalEaster(year: number | bigint): number;
  /** `hc_astronomical_paschal_full_moon`: the day Easter is the Sunday after; outside 1583 to 2150 is `out-of-range`. */
  astronomicalPaschalFullMoon(year: number | bigint): number;
  /** `hc_holy_year_on`: `null` outside a jubilee; a day the table does not reach is `no-data`. */
  holyYearOn(fixed: number | bigint): HolyYear | null;
  /** `hc_common_worship_on`: empty on a day that keeps none. */
  commonWorshipOn(fixed: number | bigint): CommonWorshipCelebration[];
  /** `hc_holiday_groups`: every group a holiday may be given to alone, named in a locale. */
  holidayGroups(locale?: string): HolidayGroup[];
  /** `hc_holidays_on_in`: `holidaysOn`'s entries with the day's name in a locale. */
  holidaysOnIn(fixed: number | bigint, locale?: string): HolidayOnIn[];
  /** `hc_day_period`: the day periods of a time of day in a locale. */
  dayPeriod(secondsOfDay: number, locale?: string): DayPeriodReading;
  /** `hc_format_number`: an integer in a numbering system. */
  formatNumber(system: string, value: number | bigint): string;
  /** `hc_parse_number`: an integer read back out of a numbering system. */
  parseNumber(system: string, text: string): number;
  /** `hc_numbering_systems`: every numbering system `formatNumber` writes. */
  numberingSystems(): NumberingSystemInfo[];
  /** `hc_calendar_eras`: a calendar's eras named in a locale. */
  calendarEras(calendar: string, locale?: string): CalendarEra[];
  /** `hc_era_table`: every era of a table; one not named is `unknown`. */
  eraTable(table: EraTableId): EraTableRow[];
  /** `hc_olympic_games`: the modern Games of a season; one not named is `unknown`. */
  olympicGames(season: "summer" | "winter"): OlympicGames[];
  /** `hc_roman_1960_office_on`: the 1960 ordo of a day. */
  roman1960OfficeOn(fixed: number | bigint): Roman1960Office[];
  /** `hc_orthodox_fast_on`: a day outside the years 326 to 4099, or 1583 to 4099 on the Gregorian reckonings, is `out-of-range`. */
  orthodoxFastOn(reckoning: OrthodoxFastReckoning, fixed: number | bigint): OrthodoxFastDay;
  /** `hc_orthodox_fast_seasons`: the scheme's periods, twelve for the Eastern Orthodox, in the order a day is tested against them. */
  orthodoxFastSeasons(reckoning: OrthodoxFastReckoning, year: number | bigint): OrthodoxFastSeason[];

  /** `hc_term_in_effect`; a meridian nobody knows is `unknown`. */
  termInEffect(fixed: number | bigint, meridian?: Meridian): TermInEffect;
  /** `hc_pentad_in_effect`. */
  pentadInEffect(fixed: number | bigint, meridian?: Meridian): TermInEffect;
  /** `hc_pentad_traditions`: every tradition that names the 72 pentads. */
  pentadTraditions(): PentadTradition[];
  /** `hc_pentad_in_tradition`: the pentad in effect, named by a tradition; one not listed is `unknown`. */
  pentadInTradition(fixed: number | bigint, tradition: PentadTraditionId, meridian?: Meridian): PentadInTradition;
  /** `hc_zassetsu_in_year`: the 21 雑節 and the three days the older rules place elsewhere; a year outside −1000 to 3000 is `out-of-range`. */
  zassetsuInYear(year: number | bigint, meridian?: Meridian): ZassetsuDay[];
  /** `hc_seasonal_days_in_year`: the 伏, the nines, the dog days, the quarter days and the folk days of a year. */
  seasonalDaysInYear(year: number | bigint, meridian?: Meridian): SeasonalDay[];
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
  /** `hc_drekkana_at`: the drekkāṇa the Sun is in, in an ayanamsa's zodiac. */
  drekkanaAt(unixSeconds: number | bigint, ayanamsa: Ayanamsa): Drekkana;
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
  /** `hc_temporal_hour`: the length of the temporal hour a reckoning counts its times in. */
  temporalHour(reckoning: ZmanimReckoning, fixed: number | bigint, latitude: number, longitude: number, elevation?: number): TemporalHour;
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

  /** `hc_jupiter_at`: an instant outside the years −1000 to 3000 is `out-of-range`. */
  jupiterAt(unixSeconds: number | bigint, ayanamsa: Ayanamsa): JupiterPosition;
  /** `hc_jupiter_ingresses`: Jupiter's crossings of the sidereal boundaries in `[from, to)`; a span over a hundred Julian years is `out-of-range`. */
  jupiterIngresses(fromUnixSeconds: number | bigint, toUnixSeconds: number | bigint, ayanamsa: Ayanamsa): JupiterIngress[];
  /** `hc_jupiter_risings`: Jupiter's heliacal risings in `[from, to)`; a span over a hundred Julian years is `out-of-range`. */
  jupiterRisings(fromUnixSeconds: number | bigint, toUnixSeconds: number | bigint, ayanamsa: Ayanamsa): JupiterRising[];
  /** `hc_jupiter_stations`: where Jupiter turns back or resumes in a span; an end outside −1000 to 3000, or a span over a hundred Julian years, is `out-of-range`. */
  jupiterStations(fromUnixSeconds: number | bigint, toUnixSeconds: number | bigint, ayanamsa: Ayanamsa): JupiterStation[];
  /** `hc_kumbh_by_sky`: `kumbh` with Jupiter's sign computed. */
  kumbhBySky(yoga: KumbhYoga, year: number | bigint, ayanamsa: Ayanamsa, locale?: string): KumbhBySky;
  /** `hc_kumbhs_in_year_by_sky`: `kumbhBySky` for each of the seven conditions, in the order `kumbhYogas` lists them. */
  kumbhsInYearBySky(year: number | bigint, ayanamsa: Ayanamsa, locale?: string): KumbhBySky[];
  /** `hc_pushkaram_by_sky`: `pushkaram` for the entry into the sign that falls in the year; empty in a year with none. */
  pushkaramBySky(
    sign: SiderealSignId,
    year: number | bigint,
    ayanamsa: Ayanamsa,
    rule: PushkaramEntryRule,
    latitude: number,
    longitude: number,
    elevation?: number,
    meridian?: string,
    locale?: string,
  ): PushkaramBySky[];
  /** `hc_pushkarams_in_year`: `pushkaramBySky` for every sign Jupiter enters in the year, one sign after another in the order of the entries; empty in a year with none. */
  pushkaramsInYear(
    year: number | bigint,
    ayanamsa: Ayanamsa,
    rule: PushkaramEntryRule,
    latitude: number,
    longitude: number,
    elevation?: number,
    meridian?: string,
    locale?: string,
  ): PushkaramBySky[];

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
  /** `hc_territories`: every territory CLDR 48 names, in code order. */
  territories(locale?: string): PlaceName[];
  /** `hc_subdivisions`: a country's ISO 3166-2 subdivisions, every one for `""`; a code that is not a territory's is `unknown`. */
  subdivisions(country: string, locale?: string): PlaceName[];
  /** `hc_place_name`: one territory or subdivision in ISO form; another code, `jp13` among them, is `unknown`. */
  placeName(code: string, locale?: string): PlaceName;
  /** `hc_relative_time`: how `thenUnix` reads from `nowUnix`, *3 hours ago*. */
  relativeTime(
    thenUnix: number | bigint,
    nowUnix: number | bigint,
    style?: RelativeStyle,
    automatic?: boolean,
    locale?: string,
  ): RelativeTime;
  /** `hc_relative_day`: which calendar day `thenFixed` is, seen from `nowFixed`, *yesterday*. */
  relativeDay(
    thenFixed: number | bigint,
    nowFixed: number | bigint,
    style?: RelativeStyle,
    automatic?: boolean,
    locale?: string,
  ): RelativeTime;
  /** `hc_relative_day_at`: the same with a time of day, *yesterday at 15:05*. */
  relativeDayAt(
    thenFixed: number | bigint,
    nowFixed: number | bigint,
    secondsOfDay: number,
    style?: RelativeStyle,
    automatic?: boolean,
    locale?: string,
  ): RelativeDayAt;
  /** `hc_duration`: a span of seconds in days, hours, minutes and seconds. */
  duration(seconds: number | bigint, style?: DurationStyle, maxComponents?: number, locale?: string): HumanizedDuration;
  /** `hc_apnumber`: *zero* to *nine* spelled out, every other number as its digits, in a catalogue. */
  apnumber(value: number | bigint, locale?: string): LocalizedNaturalText;
  /** `hc_fractional`: a number as a fraction, *3/10*, *1 3/10*. */
  fractional(value: number): NaturalText;
  /** `hc_scientific`: scientific notation with superscript exponent, *3.00 x 10⁻¹*. */
  scientific(value: number, precision?: number): NaturalText;
  /** `hc_metric`: a number with an SI prefix and a unit, *1.50 kV*. */
  metric(value: number, unit?: string, precision?: number, locale?: string): LocalizedNaturalText;
  /** `hc_naturalsize`: a size in bytes, *3.0 MB*, *2.9 KiB*, *2.9K*. */
  naturalSize(value: number, style?: NaturalSizeStyle, decimals?: number, locale?: string): LocalizedNaturalText;
  /** `hc_naturallist`: items joined as a list, *one, two and three*; English in every locale. */
  naturalList(items: string[], locale?: string): LocalizedNaturalText;
  /** `hc_intword`: an integer of any length as a count with a word, *12.4 thousand*, *1.0 googol*. */
  intword(digits: string | number | bigint, decimals?: number, locale?: string): LocalizedNaturalText;
  /** `hc_naturaldelta`: a span without tense, *3 hours*. */
  naturalDelta(seconds: number | bigint, microseconds?: number, months?: boolean, minimumUnit?: DeltaUnit, locale?: string): LocalizedNaturalText;
  /** `hc_naturaltime`: a span with tense, *3 hours ago*. */
  naturalTime(seconds: number | bigint, microseconds?: number, months?: boolean, minimumUnit?: DeltaUnit, locale?: string): LocalizedNaturalText;
  /** `hc_precisedelta`: a span in every unit, *1 year, 2 months and 3 days*. */
  preciseDelta(
    seconds: number | bigint,
    microseconds?: number,
    minimumUnit?: PreciseUnitName,
    suppress?: PreciseUnitName[],
    decimals?: number,
    locale?: string,
  ): LocalizedNaturalText;
  /** `hc_naturalday`: *today*, *tomorrow*, *yesterday*, or the day by a `strftime` pattern. */
  naturalDay(day: number | bigint, today: number | bigint, pattern?: string, locale?: string): LocalizedNaturalText;
  /** `hc_naturaldate`: `naturalDay` with the year from five twelfths of a year away. */
  naturalDate(day: number | bigint, today: number | bigint, locale?: string): LocalizedNaturalText;
  /** `hc_ordinal`: *1st*, *2nd*, *103rd*. */
  ordinal(value: number | bigint, gender?: OrdinalGender, locale?: string): LocalizedNaturalText;
  /** `hc_intcomma`: an integer with thousands separators. */
  intcomma(digits: string | number | bigint, locale?: string): LocalizedNaturalText;
  /** `hc_intcomma_float`: a float with thousands separators, to `ndigits` places or as Python's `repr`. */
  intcommaFloat(value: number, ndigits?: number | null, locale?: string): LocalizedNaturalText;
  /** `hc_unit_choice`: the unit a span is said in under a threshold table and a rounding. */
  unitChoice(seconds: number | bigint, thresholds?: ThresholdsName, rounding?: RoundingName): UnitChoice;
  /** `hc_relative_time_with`: `relativeTime` under a threshold table and a rounding. */
  relativeTimeWith(
    thenUnix: number | bigint,
    nowUnix: number | bigint,
    style?: RelativeStyle,
    automatic?: boolean,
    locale?: string,
    thresholds?: ThresholdsName,
    rounding?: RoundingName,
  ): RelativeTimeWith;
  /** `hc_approximate_duration`: a span hedged as a round number, *about 3 hours*. */
  approximateDuration(
    seconds: number | bigint,
    style?: RelativeStyle,
    locale?: string,
    thresholds?: ThresholdsName,
    policy?: HedgePolicy,
  ): ApproximateDuration;
  /** `hc_zone_name`: a zone's name at an instant, as a CLDR field writes it. */
  zoneName(zone: string, unixSeconds: number | bigint, locale?: string, field?: ZoneNameField): ZoneName;
  /** `hc_format_pattern`: an instant formatted in a zone by a CLDR or strftime pattern. */
  formatPattern(zone: string, unixSeconds: number | bigint, locale: string, syntax: "cldr" | "strftime", pattern: string): FormattedInZone;
  /** `hc_orbit_rate_offset`: a clock on a circular orbit against one held still on the ground. */
  orbitRateOffset(body: string, orbitRadiusMetres: number, groundRadiusMetres: number): OrbitRateOffset;
  /** `hc_rocket`: a rocket of constant proper acceleration burning from rest. */
  rocket(properAcceleration: number, properSeconds: number): RocketBurn;
  /** `hc_flip_and_burn`: a flip-and-burn voyage between two points at rest. */
  flipAndBurn(properAcceleration: number, distanceMetres: number): FlipAndBurn;
  /** `hc_doppler`: the relativistic Doppler shift of a source moving at β, seen at an angle. */
  doppler(beta: number, cosTheta: number): DopplerShift;
  /** `hc_velocity_add`: the composition of two collinear velocities. */
  velocityAdd(firstBeta: number, secondBeta: number): VelocityComposition;
  /** `hc_schwarzschild_radius`: the Schwarzschild radius of a body. */
  schwarzschildRadius(body: string): SchwarzschildRadius;
  /** `hc_proper_time_uncertain`: a clock moving at a constant speed that is not exactly known. */
  properTimeUncertain(speedMetresPerSecond: number, speedStdDev: number, coordinateSeconds: number): UncertainProperTime;
  /** `hc_planck_units`: the CODATA constants the Planck units are built from, and the Planck units. */
  planckUnits(): PlanckUnit[];
  /** `hc_bp_convert`: a calendar age or year in one datum written in another. */
  bpConvert(years: number, stdDevYears: number, from: string, to: string): DatumConversion;
  /** `hc_deep_convert`: a magnitude of time in one unit written in another, with its uncertainty carried through. */
  deepConvert(value: number, stdDev: number, from: string, to: string): MagnitudeConversion;
  /** `hc_deep_compare`: two magnitudes of time compared across the decades between them. */
  deepCompare(firstValue: number, firstStdDev: number, firstUnit: string, secondValue: number, secondStdDev: number, secondUnit: string): MagnitudeComparison;
  /** `hc_daily_insolation`: the daily mean insolation at any latitude and solar longitude, for the orbit of an epoch. */
  dailyInsolation(yearsBeforePresent: number, latitudeDegrees: number, solarLongitudeDegrees: number): DailyInsolation;
  /** `hc_edtf_parse`: an ISO 8601-2 value placed on the timeline. */
  edtfParse(text: string): EdtfPart[];
  /** `hc_edtf_relations`: what can hold between two EDTF values placed on the timeline. */
  edtfRelations(first: string, second: string): EdtfRelations;
  /** `hc_significant`: a number with a count of significant figures. */
  significant(value: number, figures: number): SignificantNumber;
  /** `hc_significant_op`: arithmetic on two numbers with figure counts. */
  significantOp(operation: string, first: number, firstFigures: number, second: number, secondFigures: number): SignificantResult;
  /** `hc_uncertain`: a Gaussian quantity, `value ± σ`. */
  uncertain(value: number, stdDev: number): UncertainQuantity;
  /** `hc_uncertain_op`: arithmetic on Gaussian quantities, with the errors propagated to first order. */
  uncertainOp(operation: string, first: number, firstStdDev: number, second: number, secondStdDev: number): UncertainResult;
  /** `hc_interval`: arithmetic on intervals of time. */
  interval(operation: string, firstLowSeconds: number | bigint, firstHighSeconds: number | bigint, secondLowSeconds: number | bigint, secondHighSeconds: number | bigint): IntervalResult;
  /** `hc_units`: every unit of time with an exactly defined length. */
  units(): TimeUnit[];
  /** `hc_unit_convert`: a count of one unit of time written in another, exactly. */
  unitConvert(countNumerator: number | bigint, countDenominator: number | bigint, from: string, to: string): UnitConversion;
  /** `hc_rates`: every frame rate and sample rate the crate carries as an exact period. */
  rates(): FrameRate[];
  /** `hc_frame_period`: the length of one frame or one sample, exactly. */
  framePeriod(rate: string, inUnit: string): FramePeriod;
  /** `hc_tempo`: a note at a tempo, exactly. */
  tempo(bpmNumerator: number | bigint, bpmDenominator: number | bigint, noteHalvings: number, dots: number, tupletSpace: number, tupletCount: number, beatHalvings: number): NoteAtTempo;
  /** `hc_fiscal_profiles`: every fiscal, tax and academic year system the crate carries, country by country. */
  fiscalProfiles(): FiscalSystem[];
  /** `hc_fiscal_year_on`: what the year systems of a country say a fixed day is. */
  fiscalYearOn(country: string, kind: string, fixed: number | bigint): FiscalYearOfDay[];
  /** `hc_fiscal_year_span`: the span of the year a label names in each year system of a country. */
  fiscalYearSpan(country: string, kind: string, label: number | bigint): FiscalYearSpan[];
  /** `hc_week_year_systems`: every named year of whole weeks. */
  weekYearSystems(): WeekYearSystem[];
  /** `hc_week_year_on`: where a fixed day is in a year of whole weeks. */
  weekYearOn(system: string, fixed: number | bigint): WeekYearOfDay;
  /** `hc_name_day_lists`: every name-day list the crate ships and every country it declines to ship one for. */
  nameDayLists(): NameDayList[];
  /** `hc_name_days_on`: what the lists of a country name on a day. */
  nameDaysOn(country: string, fixed: number | bigint): NameDaysOfDay[];
  /** `hc_name_day`: the days of a year on which the lists of a country give a name. */
  nameDay(country: string, name: string, year: number | bigint): NameDayDates[];
  /** `hc_attribution_authorities`: every attribution list the crate ships, with what it declines to ship. */
  attributionAuthorities(subject?: string): AttributionAuthority[];
  /** `hc_attributions`: what every list of a subject attributes to one key. */
  attributions(subject: string, key: number | bigint): Attribution[];
  /** `hc_attributions_on`: what every list attributes to the month, the weekday and the sign of a day. */
  attributionsOn(fixed: number | bigint, meridian?: string): AttributionOfDay[];
  /** `hc_harvest_moon`: the Harvest Moon of a year. */
  harvestMoon(year: number | bigint, meridian?: string): HarvestMoon;
}

/**
 * Instantiate the module and bind it. `source` may also be a promise of
 * any accepted form.
 */
export function load(source: ModuleSource | Promise<ModuleSource>, options?: LoadOptions): Promise<HyperCalendar>;
