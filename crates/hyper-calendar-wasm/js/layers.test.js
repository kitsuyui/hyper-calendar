// The binding against a build of one layer: the module built with
// `--no-default-features --features civil` loads, its own methods answer,
// and every other method refuses by name rather than failing at load.

import assert from "node:assert/strict";
import { before, test } from "node:test";

import { HcError, HyperCalendar, METHODS, load } from "./hyper-calendar.js";
import { CIVIL_WASM, moduleBytes } from "./support.js";

const HOW =
  "cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release --no-default-features --features civil" +
  " and copy the result to the path support.js gives, or set HC_WASM_CIVIL";

/** @type {HyperCalendar} */
let hc;

before(async () => {
  hc = await load(moduleBytes(CIVIL_WASM, HOW));
});

test("a civil build loads and names its one layer", () => {
  assert.match(hc.version(), /^\d+\.\d+\.\d+/);
  assert.deepEqual(hc.layers(), ["civil"]);
  for (const entry of METHODS) {
    const expected = entry.feature === null || entry.feature === "civil";
    assert.equal(hc.has(entry.method), expected, entry.method);
    assert.equal(typeof hc.exports[entry.export] === "function", expected, entry.export);
  }
});

test("the civil methods answer", () => {
  const rd = hc.gregorianToFixed(2026, 9, 21);
  assert.equal(rd, 739_880);
  assert.equal(hc.formatIsoDate(rd), "2026-09-21");
  assert.equal(hc.parseIsoDate("2026-09-21"), rd);
  assert.equal(hc.weekday(rd), 1);
  assert.equal(hc.fixedFromUnix(0), 719_163);
  assert.equal(hc.taiMinusUtc(1_700_000_000, true), 37);
});

test("a method of another layer throws not-exported when called, not at load", () => {
  /** @type {Record<string, () => unknown>} */
  const calls = {
    describeDay: () => hc.describeDay(739_880, "en"),
    dayExtras: () => hc.dayExtras(739_880, "en"),
    calendarUnits: () => hc.calendarUnits("gregory", "year", 739_000, 739_880, "en"),
    calendars: () => hc.calendars(739_880, "en"),
    calendarList: () => hc.calendarList("en"),
    locales: () => hc.locales(),
    firstDayOfWeek: () => hc.firstDayOfWeek("en"),
    gregorianAdoption: () => hc.gregorianAdoption("JP"),
    holidayIsDayOff: () => hc.holidayIsDayOff("JP", "", 739_880),
    holidaysInYear: () => hc.holidaysInYear("JP", "", 2026),
    holidayCodes: () => hc.holidayCodes(),
    holidaysOn: () => hc.holidaysOn(739_880),
    termInEffect: () => hc.termInEffect(739_880, "japan"),
    pentadInEffect: () => hc.pentadInEffect(739_880, "japan"),
    coldFoodDay: () => hc.coldFoodDay("hansik", 2026),
    placeYearsAgo: () => hc.placeYearsAgo(66e6, 0, "ja"),
    cosmicEvents: () => hc.cosmicEvents("ja"),
    earliestEvidence: () => hc.earliestEvidence("ja"),
    archaeologicalPeriods: () => hc.archaeologicalPeriods("ja"),
    futureEvents: () => hc.futureEvents("ja"),
    geologicIntervals: () => hc.geologicIntervals("eon", "ja"),
    fixedFromUnixInZone: () => hc.fixedFromUnixInZone(0, "Asia/Tokyo"),
    unixFromFixedInZone: () => hc.unixFromFixedInZone(739_880, "Asia/Tokyo"),
    loadZone: () => hc.loadZone("Asia/Tokyo", new Uint8Array(0)),
    zones: () => hc.zones("en"),
    zoneLocation: () => hc.zoneLocation("Asia/Tokyo", "en"),
    zoneOffset: () => hc.zoneOffset("Asia/Tokyo", 0),
    skyAt: () => hc.skyAt(0),
    solarTermsBetween: () => hc.solarTermsBetween(0, 86_400),
    moonPhasesBetween: () => hc.moonPhasesBetween(0, 86_400),
    orbitAt: () => hc.orbitAt(21_000),
    orbitSeries: () => hc.orbitSeries(0, 21_000, 1_000),
    tai64Encode: () => hc.tai64Encode(0, 0, "tai64"),
    tai64Decode: () => hc.tai64Decode("4000000000000000"),
    gnssWeek: () => hc.gnssWeek("gps-lnav-week", 1_554_595_219),
    gnssToTai: () => hc.gnssToTai("gps-lnav-week", 2048, 0),
    gnssResolveWeek: () => hc.gnssResolveWeek("gps-lnav-week", 0, "not-before", 1_554_595_219),
    glonassDate: () => hc.glonassDate(1_704_067_237),
    fixedFromOleAutomation: () => hc.fixedFromOleAutomation(2.25),
    oleAutomationFromFixed: () => hc.oleAutomationFromFixed(693_596, 0),
    excel1900Day: () => hc.excel1900Day(60),
    panchangaAt: () => hc.panchangaAt(1_735_689_600, "Lahiri"),
    panchangaOfDay: () => hc.panchangaOfDay(739_252, 23.18, 82.5, 0, "Lahiri"),
    hinduLunarDate: () => hc.hinduLunarDate("Lahiri", 739_252, 23.18, 82.5),
    suryaSiddhantaAt: () => hc.suryaSiddhantaAt(1_735_689_600),
    suryaSiddhantaSunrise: () => hc.suryaSiddhantaSunrise(739_252, 23.15, 75.77),
    crescentVisible: () => hc.crescentVisible("shaukat", 739_252, 21.42, 39.82),
    iocOlympiad: () => hc.iocOlympiad(2024),
    hebrewYahrzeit: () => hc.hebrewYahrzeit(739_880, 5790),
    hebrewBirthday: () => hc.hebrewBirthday(739_880, 5790),
    chineseReckonedAge: () => hc.chineseReckonedAge(730_286, 734_525),
    chineseMarriageAugury: () => hc.chineseMarriageAugury(4_661),
    holidayTables: () => hc.holidayTables("en"),
    lectionary: () => hc.lectionary(739_880),
    astronomicalEaster: () => hc.astronomicalEaster(2026),
    astronomicalPaschalFullMoon: () => hc.astronomicalPaschalFullMoon(2026),
    earthRotationAngle: () => hc.earthRotationAngle(0),
    gmstIau2006: () => hc.gmstIau2006(0),
    gmstIau1982: () => hc.gmstIau1982(0),
    ut2MinusUt1: () => hc.ut2MinusUt1(0),
    solarTime: () => hc.solarTime("temporal", 0, 51.5, 0),
    solarEvent: () => hc.solarEvent("asr-shafii", 739_880, 51.5, 0),
    horizons: () => hc.horizons("fr"),
    sunrise: () => hc.sunrise("usno", 739_880, 51.5, 0),
    sunset: () => hc.sunset("usno", 739_880, 51.5, 0),
    marsTime: () => hc.marsTime(947_116_800),
    missions: () => hc.missions(),
    missionSol: () => hc.missionSol("curiosity", 1_700_000_000),
    bodies: () => hc.bodies(),
    bodyTime: () => hc.bodyTime("titan", 947_116_800),
    properTime: () => hc.properTime(7_800, 86_400),
    gravitationalDilation: () => hc.gravitationalDilation("earth", 6_378_137),
    gravitatingBodies: () => hc.gravitatingBodies(),
    taiFromUnix: () => hc.taiFromUnix(1_700_000_000, true),
    utcFromTai: () => hc.utcFromTai(1_700_000_037, true),
    tai64PosixPlus10Encode: () => hc.tai64PosixPlus10Encode(0, 0, "tai64"),
    tai64PosixPlus10Decode: () => hc.tai64PosixPlus10Decode("400000000000000a"),
    uuidTimestamp: () => hc.uuidTimestamp("C232AB00-9414-11EC-B3C8-9F6BDECED846"),
    ntpResolve: () => hc.ntpResolve(63_104, 0, 1_893_456_000),
    uuidTimestampEncode: () => hc.uuidTimestampEncode(1_645_557_742),
    ntpEncode: () => hc.ntpEncode(0),
    fatDecode: () => hc.fatDecode(23_866, 49_021),
    fatEncode: () => hc.fatEncode(739_885, 0),
    swatchBeat: () => hc.swatchBeat(0),
    epochFromTt: () => hc.epochFromTt("J", 946_728_000),
    ttFromEpoch: () => hc.ttFromEpoch("J", 2000),
    ttBipm: () => hc.ttBipm("58479\t27.674\n", 1_545_868_837),
    namingPeriodOn: () => hc.namingPeriodOn("gregory", 732_026, "tk"),
    hebrewSabbaticalCycleYear: () => hc.hebrewSabbaticalCycleYear(5782),
    asianDay: () => hc.asianDay(1_360),
    holyYearOn: () => hc.holyYearOn(739_403),
    commonWorshipOn: () => hc.commonWorshipOn(739_369),
    decanAt: () => hc.decanAt(1_790_125_500),
    hjdTt: () => hc.hjdTt(2_451_545, 0, 0),
    hjdUtc: () => hc.hjdUtc(2_451_545, 0, 0),
    circadDate: () => hc.circadDate("darian-titan", 1_040_208_120),
    ccsdsDecode: () => hc.ccsdsDecode("1c4effa220"),
    ccsdsEncode: () => hc.ccsdsEncode(946_684_832, 0, "1c"),
    ccsdsAsciiParse: () => hc.ccsdsAsciiParse("2000-001T00:00Z"),
    ccsdsAsciiFormat: () => hc.ccsdsAsciiFormat(946_684_832, 0, "a", "second"),
    radioDecode: () => hc.radioDecode("jjy", "M", 2000),
    radioEncode: () => hc.radioEncode("jjy", 1_080_807_900),
    dotnetTicksFromUnix: () => hc.dotnetTicksFromUnix(0),
    unixFromDotnetTicks: () => hc.unixFromDotnetTicks(0),
    sixHourClock: () => hc.sixHourClock("ethiopian-hours", 0),
    civilFromSixHourClock: () => hc.civilFromSixHourClock("ethiopian-hours", 1, 0, 0, false),
    kalam: () => hc.kalam("rahu-kalam-fixed", 739_252, 28.6, 77.2),
    almanacCycles: () => hc.almanacCycles(739_650, "japan"),
    almanacDay: () => hc.almanacDay(739_606, "japan", "ja"),
    orthodoxFastOn: () => hc.orthodoxFastOn("orthodox-fasts", 739_313),
    orthodoxFastSeasons: () => hc.orthodoxFastSeasons("orthodox-fasts", 2025),
    prayerTimes: () => hc.prayerTimes("mwl", 739_617, 21.4, 39.8),
    prayerMethods: () => hc.prayerMethods(),
    zmanim: () => hc.zmanim("zmanim-gra", 739_252, 40.7, -74.0),
    edoTime: () => hc.edoTime(1_584_649_727, 35.0, 135.7),
    unixFromEdoTime: () => hc.unixFromEdoTime(737_504, 0, 0, 35.0, 135.7),
    choghadiya: () => hc.choghadiya(739_252, 28.6, 77.2),
    panchak: () => hc.panchak("panchak-five-kinds", 1_736_078_400, "lahiri", 19_800),
    kumbh: () => hc.kumbh("kumbh-haridwar", 2021, "lahiri"),
    pushkaram: () => hc.pushkaram("simha", 1_436_837_820, 28.6, 77.2),
    folkDay: () => hc.folkDay(739_667, "china", "tr"),
    nightWatch: () => hc.nightWatch(23 * 3_600),
    barhaspatyaYear: () => hc.barhaspatyaYear("surya-siddhanta-bija", 1_946),
    barhaspatyaYearAt: () => hc.barhaspatyaYearAt("surya-siddhanta", 1_743_292_800),
    planetaryHour: () => hc.planetaryHour(0, 51.5, 0),
    planetaryHoursOfDay: () => hc.planetaryHoursOfDay(739_880, 51.5, 0),
    gmatFromGmt: () => hc.gmatFromGmt(702_411, 0),
    gmtFromGmat: () => hc.gmtFromGmat(702_411, 0),
    irigDecode: () => hc.irigDecode("B124", "M", 2026),
    irigEncode: () => hc.irigEncode("B124", 731_388, 76_722),
    plumRains: () => hc.plumRains("ru-mei-bing", 2026, "china"),
  };
  const gated = METHODS.filter((entry) => entry.feature !== null && entry.feature !== "civil");
  assert.deepEqual(Object.keys(calls).sort(), gated.map((entry) => entry.method).sort(), "every gated method is tried");
  for (const entry of gated) {
    let thrown;
    try {
      calls[entry.method]();
    } catch (error) {
      thrown = error;
    }
    assert.ok(thrown instanceof HcError, `${entry.method}: ${String(thrown)}`);
    assert.equal(thrown.name, "not-exported", entry.method);
    assert.equal(thrown.export, entry.export, entry.method);
    assert.equal(thrown.code, null, entry.method);
    assert.equal(thrown.constant, null, entry.method);
  }
});
