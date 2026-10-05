//! The international observances: the days the United Nations and its
//! agencies have set aside, each citing the instrument that set it.
//!
//! These are secular, worldwide and without a day off anywhere — a
//! [`Kind::Observance`](crate::rule::Kind::Observance) every one — and they belong to no country and no
//! religion, so they are neither a national table nor a tradition. They
//! are here because a calendar that knows Christmas and Diwali and not
//! Human Rights Day is answering "what is today" for some people and not
//! others.
//!
//! # Source
//!
//! The United Nations' own list, *International Days and Weeks*
//! (`un.org/en/observances/list-days-weeks`, retrieved 2026-09-22), which
//! gives each day's date and the General Assembly resolution or the agency
//! that designated it. Each entry cites its resolution or agency in
//! [`HolidayRule::source`](crate::rule::HolidayRule::source): a test asserts
//! that none is blank. The weeks the list also carries — World Space Week,
//! Disarmament Week and the rest — are spans, a table of their own,
//! [`UNITED_NATIONS_WEEKS`], each on every day of its week.
//!
//! # The first years
//!
//! A day is absent before its first observance
//! ([`HolidayRule::valid_from`](crate::rule::HolidayRule::valid_from)). Where
//! a page read gives the date the resolution was adopted, the day begins in
//! the year of the first occurrence of its date on or after it; a day an
//! earlier body observed on the same date (Wetlands, Family Remittances,
//! Elder Abuse, Kiswahili) begins with the proclamation, and the
//! precursor's year is not the day's. A day proclaimed on its own date
//! (Mother Earth Day, 22 April 2009) has that year as a gap. The first
//! observance a source read states for a day the list cites a resolution for
//! but that no resolution created (Detained and Missing Staff Members, 1986)
//! is its own. The detail per day is in `docs/systems/holiday-first-years.md`.
//!
//! # The days that are rules
//!
//! Most fall on a fixed date. Seven do not, and the list shows them on the
//! date of the year it was consulted in; their own pages, and the
//! resolutions, give the rule, which is what is carried:
//!
//! | Day | Rule |
//! | --- | --- |
//! | International Day of Cooperatives | the first Saturday of July (A/RES/47/90) |
//! | World Migratory Bird Day | the second Saturday of May and of October (UNEP, CMS and AEWA) |
//! | World Maritime Day | the last Thursday of September, as the IMO Council fixes it |
//! | World Habitat Day | the first Monday of October (A/RES/40/202) |
//! | World Philosophy Day | the third Thursday of November (UNESCO, 33 C/Resolution 37) |
//! | World Day of Remembrance for Road Traffic Victims | the third Sunday of November (A/RES/60/5) |
//! | Vesak | "the Day of the Full Moon in the month of May" (A/RES/54/115) — the first full moon on or after 1 May, flagged approximate, because the United Nations announces the day it observes |

use hc_calendar::Weekday;
use hc_seasons::Meridian;

use hc_astro::lunar::MoonPhase;

use crate::rule::{HolidayRule, NO_WEEKEND, Rule, RuleSet, SourceDate, Subdivisions};

/// An international day: an observance, nowhere a day off, citing the
/// instrument that set it.
const fn day(name: &'static str, rule: Rule, source: &'static str) -> HolidayRule {
    HolidayRule::observance(name, "", rule).cited(source)
}

static UNITED_NATIONS_RULES: &[HolidayRule] = &[
    day("World Braille Day", Rule::gregorian(1, 4), "A/RES/73/161").years(Some(2019), None),
    day(
        "International Day of Education",
        Rule::gregorian(1, 24),
        "A/RES/73/25",
    ).years(Some(2019), None),
    day(
        "International Day of Clean Energy",
        Rule::gregorian(1, 26),
        "A/RES/77/327",
    ).years(Some(2024), None),
    day(
        "International Day of Commemoration in Memory of the Victims of the Holocaust",
        Rule::gregorian(1, 27),
        "A/RES/60/7",
    ).years(Some(2005), None).read_from(2006),
    day(
        "International Day of Peaceful Coexistence",
        Rule::gregorian(1, 28),
        "A/RES/79/269",
    ).years(Some(2025), None).read_from(2026),
    day("World Wetlands Day", Rule::gregorian(2, 2), "A/RES/75/317").years(Some(2022), None),
    day(
        "International Day of Human Fraternity",
        Rule::gregorian(2, 4),
        "A/RES/75/200",
    ).years(Some(2021), None),
    day(
        "International Day of Zero Tolerance to Female Genital Mutilation",
        Rule::gregorian(2, 6),
        "A/RES/67/146",
    ).years(Some(2012), None).read_from(2013),
    day(
        "International Day of the Arabian Leopard",
        Rule::gregorian(2, 10),
        "A/RES/77/295",
    ).years(Some(2024), None),
    day(
        "World Pulses Day",
        Rule::gregorian(2, 10),
        "FAO, A/RES/73/251",
    ).years(Some(2019), None),
    day(
        "International Day of Women and Girls in Science",
        Rule::gregorian(2, 11),
        "A/RES/70/212",
    ).years(Some(2015), None).read_from(2016),
    day(
        "International Day for the Prevention of Violent Extremism as and when Conducive to Terrorism",
        Rule::gregorian(2, 12),
        "A/RES/77/243",
    ).years(Some(2023), None),
    day("World Radio Day", Rule::gregorian(2, 13), "A/RES/67/124").years(Some(2012), None),
    day(
        "Global Tourism Resilience Day",
        Rule::gregorian(2, 17),
        "A/RES/77/269",
    ).years(Some(2023), None),
    day(
        "World Day of Social Justice",
        Rule::gregorian(2, 20),
        "A/RES/62/10",
    ).years(Some(2009), None),
    day(
        "International Mother Language Day",
        Rule::gregorian(2, 21),
        "A/RES/56/262",
    ).years(Some(2001), None).read_from(2002),
    day("World Seagrass Day", Rule::gregorian(3, 1), "A/RES/76/265").years(Some(2022), None).read_from(2023),
    day("Zero Discrimination Day", Rule::gregorian(3, 1), "UNAIDS").years(Some(2014), None),
    day("World Wildlife Day", Rule::gregorian(3, 3), "A/RES/68/205").years(Some(2013), None).read_from(2014),
    day(
        "International Day for Disarmament and Non-Proliferation Awareness",
        Rule::gregorian(3, 5),
        "A/RES/77/51",
    ).years(Some(2023), None),
    day(
        "International Women's Day",
        Rule::gregorian(3, 8),
        "United Nations, list of international days",
    ).years(Some(1914), None),
    day(
        "International Day of Women Judges",
        Rule::gregorian(3, 10),
        "A/RES/75/274",
    ).years(Some(2022), None),
    day(
        "International Day to Combat Islamophobia",
        Rule::gregorian(3, 15),
        "A/RES/76/254",
    ).years(Some(2022), None).read_from(2023),
    day(
        "French Language Day",
        Rule::gregorian(3, 20),
        "United Nations Department of Global Communications",
    ).years(Some(2010), None).read_from(2011),
    day(
        "International Day of Happiness",
        Rule::gregorian(3, 20),
        "A/RES/66/281",
    ).years(Some(2013), None),
    day(
        "International Day for the Elimination of Racial Discrimination",
        Rule::gregorian(3, 21),
        "A/RES/2142 (XXI)",
    ).years(Some(1966), None).read_from(1967),
    day(
        "International Day of Forests",
        Rule::gregorian(3, 21),
        "FAO, A/RES/67/200",
    ).years(Some(2013), None),
    day(
        "International Day of Nowruz",
        Rule::gregorian(3, 21),
        "A/RES/64/253",
    ).years(Some(2010), None).read_from(2011),
    day(
        "World Day for Glaciers",
        Rule::gregorian(3, 21),
        "A/RES/77/158",
    ).years(Some(2025), None),
    day(
        "World Down Syndrome Day",
        Rule::gregorian(3, 21),
        "A/RES/66/149",
    ).years(Some(2012), None),
    day(
        "World Poetry Day",
        Rule::gregorian(3, 21),
        "UNESCO, 30 C/Resolution 29",
    ).years(Some(2000), None),
    day("World Water Day", Rule::gregorian(3, 22), "A/RES/47/193").years(Some(1993), None),
    day(
        "World Meteorological Day",
        Rule::gregorian(3, 23),
        "WMO, WMO/EC-XII/Res.6",
    ).years(Some(1961), None),
    day(
        "International Day for the Right to the Truth concerning Gross Human Rights Violations and for the Dignity of Victims",
        Rule::gregorian(3, 24),
        "A/RES/65/196",
    ).years(Some(2010), None).read_from(2011),
    day("World Tuberculosis Day", Rule::gregorian(3, 24), "WHO").years(Some(1982), None).read_from(1983),
    day(
        "International Day of Remembrance of the Victims of Slavery and the Transatlantic Slave Trade",
        Rule::gregorian(3, 25),
        "A/RES/62/122",
    ).years(Some(2008), None),
    day(
        "International Day of Solidarity with Detained and Missing Staff Members",
        Rule::gregorian(3, 25),
        "A/RES/49/59",
    ).years(Some(1986), None),
    day(
        "International Day of Zero Waste",
        Rule::gregorian(3, 30),
        "A/RES/77/161",
    ).years(Some(2022), None).read_from(2023),
    day(
        "World Autism Awareness Day",
        Rule::gregorian(4, 2),
        "A/RES/62/139",
    ).years(Some(2008), None),
    day(
        "International Day for Mine Awareness and Assistance in Mine Action",
        Rule::gregorian(4, 4),
        "A/RES/60/97",
    ).years(Some(2006), None),
    day(
        "International Day of Conscience",
        Rule::gregorian(4, 5),
        "A/RES/73/329",
    ).years(Some(2020), None),
    day(
        "International Day of Sport for Development and Peace",
        Rule::gregorian(4, 6),
        "A/RES/67/296",
    ).years(Some(2013), None).read_from(2014),
    day(
        "International Day of Reflection on the 1994 Genocide against the Tutsi in Rwanda",
        Rule::gregorian(4, 7),
        "A/RES/58/234",
    ).years(Some(2004), None),
    day(
        "World Health Day",
        Rule::gregorian(4, 7),
        "WHO, WHA/A.2/Res.35",
    ).years(Some(1950), None),
    day(
        "International Day of Human Space Flight",
        Rule::gregorian(4, 12),
        "A/RES/65/271",
    ).years(Some(2011), None).read_from(2012),
    day("World Chagas Disease Day", Rule::gregorian(4, 14), "WHO").years(Some(2020), None),
    day(
        "International Wellness Day",
        Rule::gregorian(4, 15),
        "A/RES/80/249",
    ).years(Some(2026), None),
    day(
        "Chinese Language Day",
        Rule::gregorian(4, 20),
        "United Nations Department of Global Communications",
    ).years(Some(2011), None),
    day(
        "World Creativity and Innovation Day",
        Rule::gregorian(4, 21),
        "A/RES/71/284",
    ).years(Some(2018), None),
    day(
        "International Mother Earth Day",
        Rule::gregorian(4, 22),
        "A/RES/63/278",
    ).years(Some(2009), None).read_from(2010),
    day(
        "International Day of Greening the Planet",
        Rule::gregorian(4, 22),
        "A/RES/80/300",
    ).years(Some(2027), None),
    day(
        "English Language Day",
        Rule::gregorian(4, 23),
        "United Nations Department of Global Communications",
    ).years(Some(2010), None).read_from(2011),
    day(
        "International Girls in ICT Day",
        Rule::gregorian(4, 23),
        "ITU",
    ).years(Some(2014), None).read_from(2015),
    day(
        "Spanish Language Day",
        Rule::gregorian(4, 23),
        "United Nations Department of Global Communications",
    ).years(Some(2010), None).read_from(2011),
    day(
        "World Book and Copyright Day",
        Rule::gregorian(4, 23),
        "UNESCO, 28 C/Resolution 3.18",
    ).years(Some(1995), None).read_from(1996),
    day(
        "International Day of Multilateralism and Diplomacy for Peace",
        Rule::gregorian(4, 24),
        "A/RES/73/127",
    ).years(Some(2018), None).read_from(2019),
    day(
        "International Delegate's Day",
        Rule::gregorian(4, 25),
        "A/RES/73/286",
    ).years(Some(2019), None).read_from(2020),
    day("World Malaria Day", Rule::gregorian(4, 25), "WHO").years(Some(2007), None).read_from(2008),
    day(
        "International Chernobyl Disaster Remembrance Day",
        Rule::gregorian(4, 26),
        "A/RES/71/125",
    ).years(Some(2016), None).read_from(2017),
    day(
        "World Intellectual Property Day",
        Rule::gregorian(4, 26),
        "WIPO",
    ).years(Some(2000), None).read_from(2001),
    day(
        "World Day for Safety and Health at Work",
        Rule::gregorian(4, 28),
        "ILO",
    ).years(Some(2003), None),
    day(
        "International Day in Memory of the Victims of Earthquakes",
        Rule::gregorian(4, 29),
        "A/RES/79/285",
    ).years(Some(2025), None).read_from(2026),
    day(
        "International Jazz Day",
        Rule::gregorian(4, 30),
        "UNESCO, 36 C/Resolution 39",
    ).years(Some(2012), None),
    day("World Tuna Day", Rule::gregorian(5, 2), "A/RES/71/124").years(Some(2017), None),
    day(
        "World Press Freedom Day",
        Rule::gregorian(5, 3),
        "UNESCO, 26 C/Resolution 4.3",
    ).years(Some(1993), None).read_from(1994),
    day(
        "World Portuguese Language Day",
        Rule::gregorian(5, 5),
        "UNESCO, 40 C/Resolution 75",
    ).years(Some(2019), None).read_from(2020),
    day(
        "Time of Remembrance and Reconciliation for Those Who Lost Their Lives During the Second World War",
        Rule::gregorian(5, 8),
        "A/RES/59/26",
    ).years(Some(2004), None).read_from(2005),
    day(
        "International Day of Argania",
        Rule::gregorian(5, 10),
        "A/RES/75/262",
    ).years(Some(2021), None).read_from(2022),
    day(
        "International Day of Plant Health",
        Rule::gregorian(5, 12),
        "FAO, A/RES/76/256",
    ).years(Some(2022), None),
    day(
        "International Day of Families",
        Rule::gregorian(5, 15),
        "A/RES/47/237",
    ).years(Some(1993), None).read_from(1994),
    day(
        "International Day of Light",
        Rule::gregorian(5, 16),
        "UNESCO, 39 C/Resolution 16",
    ).years(Some(2018), None),
    day(
        "International Day of Living Together in Peace",
        Rule::gregorian(5, 16),
        "A/RES/72/130",
    ).years(Some(2017), None).read_from(2018),
    day(
        "World Telecommunication and Information Society Day",
        Rule::gregorian(5, 17),
        "A/RES/60/252",
    ).years(Some(2006), None).read_from(2007),
    day(
        "World Fair Play Day",
        Rule::gregorian(5, 19),
        "A/RES/78/310",
    ).years(Some(2025), None),
    day("World Bee Day", Rule::gregorian(5, 20), "FAO, A/RES/72/211").years(Some(2017), None).read_from(2018),
    day(
        "International Tea Day",
        Rule::gregorian(5, 21),
        "FAO, A/RES/74/241",
    ).years(Some(2019), None).read_from(2020),
    day(
        "World Day for Cultural Diversity for Dialogue and Development",
        Rule::gregorian(5, 21),
        "A/RES/57/249",
    ).years(Some(2002), None).read_from(2003),
    day(
        "International Day for Biological Diversity",
        Rule::gregorian(5, 22),
        "A/RES/55/201",
    ).years(Some(2001), None),
    day(
        "International Day to End Obstetric Fistula",
        Rule::gregorian(5, 23),
        "A/RES/67/147",
    ).years(Some(2012), None).read_from(2013),
    day(
        "International Day of the Markhor",
        Rule::gregorian(5, 24),
        "A/RES/78/278",
    ).years(Some(2024), None),
    day("World Football Day", Rule::gregorian(5, 25), "A/RES/78/281").years(Some(2024), None).read_from(2025),
    day(
        "International Day of UN Peacekeepers",
        Rule::gregorian(5, 29),
        "A/RES/57/129",
    ).years(Some(2003), None),
    day(
        "International Day of Potato",
        Rule::gregorian(5, 30),
        "FAO, A/RES/78/123",
    ).years(Some(2024), None),
    day(
        "World No-Tobacco Day",
        Rule::gregorian(5, 31),
        "WHO, Resolution 42.19",
    ).years(Some(1988), None).read_from(1989),
    day(
        "Global Day of Parents",
        Rule::gregorian(6, 1),
        "A/RES/66/292",
    ).years(Some(2012), None).read_from(2013),
    day("World Bicycle Day", Rule::gregorian(6, 3), "A/RES/72/272").years(Some(2018), None),
    day(
        "International Day of Innocent Children Victims of Aggression",
        Rule::gregorian(6, 4),
        "A/RES/ES-7/8",
    ).years(Some(1982), None).read_from(1983),
    day(
        "International Day for the Fight against Illegal, Unreported and Unregulated Fishing",
        Rule::gregorian(6, 5),
        "A/RES/72/72",
    ).years(Some(2017), None).read_from(2018),
    day(
        "World Environment Day",
        Rule::gregorian(6, 5),
        "A/RES/2994 (XXVII)",
    ).years(Some(1973), None),
    day(
        "Russian Language Day",
        Rule::gregorian(6, 6),
        "United Nations Department of Global Communications",
    ).years(Some(2010), None).read_from(2011),
    day(
        "World Food Safety Day",
        Rule::gregorian(6, 7),
        "A/RES/73/250",
    ).years(Some(2019), None),
    day("World Oceans Day", Rule::gregorian(6, 8), "A/RES/63/111").years(Some(2008), None).read_from(2009),
    day(
        "International Day for Dialogue among Civilizations",
        Rule::gregorian(6, 10),
        "A/RES/78/286",
    ).years(Some(2024), None).read_from(2025),
    day(
        "International Day of Play",
        Rule::gregorian(6, 11),
        "A/RES/78/268",
    ).years(Some(2024), None),
    day(
        "World Day Against Child Labour",
        Rule::gregorian(6, 12),
        "ILO",
    ).years(Some(2002), None),
    day(
        "International Albinism Awareness Day",
        Rule::gregorian(6, 13),
        "A/RES/69/170",
    ).years(Some(2015), None),
    day(
        "World Blood Donor Day",
        Rule::gregorian(6, 14),
        "WHO, WHA Resolution 58.13",
    ).years(Some(2004), None),
    day(
        "World Elder Abuse Awareness Day",
        Rule::gregorian(6, 15),
        "A/RES/66/127",
    ).years(Some(2012), None),
    day(
        "International Day of Family Remittances",
        Rule::gregorian(6, 16),
        "A/RES/72/281",
    ).years(Some(2018), None),
    day(
        "World Day to Combat Desertification and Drought",
        Rule::gregorian(6, 17),
        "A/RES/49/115",
    ).years(Some(1994), None).read_from(1995),
    day(
        "International Day for Countering Hate Speech",
        Rule::gregorian(6, 18),
        "A/RES/75/309",
    ).years(Some(2021), None).read_from(2022),
    day(
        "Sustainable Gastronomy Day",
        Rule::gregorian(6, 18),
        "FAO, A/RES/71/246",
    ).years(Some(2017), None),
    day(
        "International Day for the Elimination of Sexual Violence in Conflict",
        Rule::gregorian(6, 19),
        "A/RES/69/293",
    ).years(Some(2015), None).read_from(2016),
    day("World Refugee Day", Rule::gregorian(6, 20), "A/RES/55/76").years(Some(2001), None),
    day(
        "International Day of Yoga",
        Rule::gregorian(6, 21),
        "A/RES/69/131",
    ).years(Some(2015), None),
    day(
        "International Day of the Celebration of the Solstice",
        Rule::gregorian(6, 21),
        "A/RES/73/300",
    ).years(Some(2019), None),
    day(
        "International Widows' Day",
        Rule::gregorian(6, 23),
        "A/RES/65/189",
    ).years(Some(2011), None),
    day(
        "United Nations Public Service Day",
        Rule::gregorian(6, 23),
        "A/RES/57/277",
    ).years(Some(2002), None).read_from(2003),
    day(
        "International Day of Women in Diplomacy",
        Rule::gregorian(6, 24),
        "A/RES/76/269",
    ).years(Some(2022), None),
    day(
        "Day of the Seafarer",
        Rule::gregorian(6, 25),
        "IMO, STCW/CONF.2/DC/4",
    ).years(Some(2010), None).read_from(2011),
    day(
        "International Day against Drug Abuse and Illicit Trafficking",
        Rule::gregorian(6, 26),
        "A/RES/42/112",
    ).years(Some(1989), None),
    day(
        "United Nations International Day in Support of Victims of Torture",
        Rule::gregorian(6, 26),
        "A/RES/52/149",
    ).years(Some(1998), None),
    day(
        "International Day of Deafblindness",
        Rule::gregorian(6, 27),
        "A/RES/79/294",
    ).years(Some(2025), None),
    day(
        "Micro-, Small and Medium-sized Enterprises Day",
        Rule::gregorian(6, 27),
        "A/RES/71/279",
    ).years(Some(2017), None),
    day(
        "International Day of the Tropics",
        Rule::gregorian(6, 29),
        "A/RES/70/267",
    ).years(Some(2016), None).read_from(2017),
    day(
        "International Asteroid Day",
        Rule::gregorian(6, 30),
        "A/RES/71/90",
    ).years(Some(2016), None).read_from(2017),
    day(
        "International Day of Parliamentarism",
        Rule::gregorian(6, 30),
        "A/RES/72/278",
    ).years(Some(2018), None).read_from(2019),
    day(
        "World Rural Development Day",
        Rule::gregorian(7, 6),
        "A/RES/78/326",
    ).years(Some(2025), None),
    day(
        "World Kiswahili Language Day",
        Rule::gregorian(7, 7),
        "A/RES/78/312",
    ).years(Some(2024), None),
    day(
        "International Day of Reflection and Commemoration of the 1995 Genocide in Srebrenica",
        Rule::gregorian(7, 11),
        "A/RES/78/282",
    ).years(Some(2024), None).read_from(2025),
    day("World Horse Day", Rule::gregorian(7, 11), "A/RES/79/291").years(Some(2025), None).read_from(2026),
    day(
        "World Population Day",
        Rule::gregorian(7, 11),
        "A/RES/45/216",
    ).years(Some(1990), None).read_from(1991),
    day(
        "International Day of Combating Sand and Dust Storms",
        Rule::gregorian(7, 12),
        "A/RES/77/294",
    ).years(Some(2023), None),
    day(
        "International Day of Hope",
        Rule::gregorian(7, 12),
        "A/RES/79/270",
    ).years(Some(2025), None),
    day(
        "World Youth Skills Day",
        Rule::gregorian(7, 15),
        "A/RES/69/145",
    ).years(Some(2014), None).read_from(2015),
    day(
        "Nelson Mandela International Day",
        Rule::gregorian(7, 18),
        "A/RES/64/13",
    ).years(Some(2010), None),
    day(
        "International Moon Day",
        Rule::gregorian(7, 20),
        "A/RES/76/76",
    ).years(Some(2021), None).read_from(2022),
    day("World Chess Day", Rule::gregorian(7, 20), "A/RES/74/22").years(Some(2019), None).read_from(2020),
    day(
        "International Day for Judicial Well-being",
        Rule::gregorian(7, 25),
        "A/RES/79/266",
    ).years(Some(2025), None).read_from(2026),
    day(
        "International Day of Women and Girls of African Descent",
        Rule::gregorian(7, 25),
        "A/RES/78/323",
    ).years(Some(2024), None).read_from(2025),
    day(
        "World Drowning Prevention Day",
        Rule::gregorian(7, 25),
        "A/RES/75/273",
    ).years(Some(2021), None).read_from(2022),
    day("World Hepatitis Day", Rule::gregorian(7, 28), "WHO").years(Some(2010), None).read_from(2011),
    day(
        "International Day of Friendship",
        Rule::gregorian(7, 30),
        "A/RES/65/275",
    ).years(Some(2011), None).read_from(2012),
    day(
        "World Day against Trafficking in Persons",
        Rule::gregorian(7, 30),
        "A/RES/68/192",
    ).years(Some(2013), None).read_from(2014),
    day(
        "International Day of Awareness of the Special Development Needs and Challenges of Landlocked Developing Countries",
        Rule::gregorian(8, 6),
        "A/79/L.108",
    ).years(Some(2025), None),
    day(
        "International Day of the World's Indigenous Peoples",
        Rule::gregorian(8, 9),
        "A/RES/49/214",
    ).years(Some(1994), None).read_from(1995),
    day("World Steelpan Day", Rule::gregorian(8, 11), "A/RES/77/316").years(Some(2023), None).read_from(2024),
    day(
        "International Youth Day",
        Rule::gregorian(8, 12),
        "A/RES/54/120",
    ).years(Some(2000), None),
    day(
        "World Humanitarian Day",
        Rule::gregorian(8, 19),
        "A/RES/63/139",
    ).years(Some(2009), None),
    day(
        "International Day of Remembrance and Tribute to the Victims of Terrorism",
        Rule::gregorian(8, 21),
        "A/RES/72/165",
    ).years(Some(2017), None).read_from(2018),
    day(
        "International Day Commemorating the Victims of Acts of Violence Based on Religion or Belief",
        Rule::gregorian(8, 22),
        "A/RES/73/296",
    ).years(Some(2019), None).read_from(2020),
    day(
        "International Day for the Remembrance of the Slave Trade and Its Abolition",
        Rule::gregorian(8, 23),
        "UNESCO, 29 C/Resolution 40",
    ).years(Some(1998), None),
    day("World Lake Day", Rule::gregorian(8, 27), "A/RES/79/142").years(Some(2024), None).read_from(2025),
    day(
        "International Day against Nuclear Tests",
        Rule::gregorian(8, 29),
        "A/RES/64/35",
    ).years(Some(2010), None),
    day(
        "International Day of the Victims of Enforced Disappearances",
        Rule::gregorian(8, 30),
        "A/RES/65/209",
    ).years(Some(2010), None).read_from(2011),
    day(
        "International Day for People of African Descent",
        Rule::gregorian(8, 31),
        "A/RES/75/170",
    ).years(Some(2020), None).read_from(2021),
    day(
        "International Day of Safe, Secure and Trustworthy Artificial Intelligence",
        Rule::gregorian(8, 31),
        "A/RES/80/302",
    ).years(Some(2027), None),
    day(
        "International Day of Charity",
        Rule::gregorian(9, 5),
        "A/RES/67/105",
    ).years(Some(2013), None),
    day(
        "International Day of the World's Indigenous Women and Girls",
        Rule::gregorian(9, 5),
        "A/RES/80/10",
    ).years(Some(2025), None).read_from(2026),
    day(
        "International Day of Clean Air for Blue Skies",
        Rule::gregorian(9, 7),
        "UNEP, A/RES/74/212",
    ).years(Some(2020), None),
    day(
        "International Day of Police Cooperation",
        Rule::gregorian(9, 7),
        "A/RES/77/241",
    ).years(Some(2022), None).read_from(2023),
    day(
        "World Duchenne Awareness Day",
        Rule::gregorian(9, 7),
        "A/RES/78/12",
    ).years(Some(2024), None),
    day(
        "International Literacy Day",
        Rule::gregorian(9, 8),
        "UNESCO, 14 C/Resolution 1.441",
    ).years(Some(1967), None),
    day(
        "International Day to Protect Education from Attack",
        Rule::gregorian(9, 9),
        "A/RES/74/275",
    ).years(Some(2020), None).read_from(2021),
    day(
        "United Nations Day for South-South Cooperation",
        Rule::gregorian(9, 12),
        "A/RES/58/220",
    ).years(Some(2003), None).read_from(2004),
    day(
        "International Day of Democracy",
        Rule::gregorian(9, 15),
        "A/RES/62/7",
    ).years(Some(2008), None),
    day(
        "International Day for Interventional Cardiology",
        Rule::gregorian(9, 16),
        "A/RES/76/302",
    ).years(Some(2022), None).read_from(2023),
    day(
        "International Day for the Preservation of the Ozone Layer",
        Rule::gregorian(9, 16),
        "A/RES/49/114",
    ).years(Some(1994), None).read_from(1995),
    day(
        "International Day of Science, Technology and Innovation for the South",
        Rule::gregorian(9, 16),
        "A/RES/78/259",
    ).years(Some(2024), None).read_from(2025),
    day("World Patient Safety Day", Rule::gregorian(9, 17), "WHO").years(Some(2019), None).read_from(2020),
    day(
        "International Equal Pay Day",
        Rule::gregorian(9, 18),
        "A/RES/74/142",
    ).years(Some(2019), None).read_from(2020),
    day("World Cleanup Day", Rule::gregorian(9, 20), "A/RES/78/122").years(Some(2024), None),
    day(
        "International Day of Peace",
        Rule::gregorian(9, 21),
        "A/RES/36/67",
    ).years(Some(1982), None),
    day(
        "International Day of Sign Languages",
        Rule::gregorian(9, 23),
        "A/RES/72/161",
    ).years(Some(2018), None),
    day(
        "International Day for the Total Elimination of Nuclear Weapons",
        Rule::gregorian(9, 26),
        "A/RES/68/32",
    ).years(Some(2014), None),
    day("World Tourism Day", Rule::gregorian(9, 27), "UNWTO").years(Some(1980), None),
    day(
        "International Day for Universal Access to Information",
        Rule::gregorian(9, 28),
        "A/RES/74/5",
    ).years(Some(2019), None).read_from(2020),
    day(
        "International Day of Awareness of Food Loss and Waste",
        Rule::gregorian(9, 29),
        "A/RES/74/209",
    ).years(Some(2020), None),
    day(
        "International Translation Day",
        Rule::gregorian(9, 30),
        "A/RES/71/288",
    ).years(Some(2017), None).read_from(2018),
    day(
        "International Coffee Day",
        Rule::gregorian(10, 1),
        "A/RES/80/248",
    ).years(Some(2026), None).read_from(2027),
    day(
        "International Day of Older Persons",
        Rule::gregorian(10, 1),
        "A/RES/45/106",
    ).years(Some(1991), None),
    day(
        "International Day of Non-Violence",
        Rule::gregorian(10, 2),
        "A/RES/61/271",
    ).years(Some(2007), None).read_from(2008),
    day(
        "World Teachers' Day",
        Rule::gregorian(10, 5),
        "UNESCO, 27 C/INF.7",
    ).years(Some(1994), None),
    day("World Cotton Day", Rule::gregorian(10, 7), "A/RES/75/318").years(Some(2021), None).read_from(2022),
    day(
        "World Post Day",
        Rule::gregorian(10, 9),
        "Universal Postal Union",
    ).years(Some(1969), None).read_from(1970),
    day("World Mental Health Day", Rule::gregorian(10, 10), "WHO").years(Some(1992), None),
    day(
        "International Day of the Girl Child",
        Rule::gregorian(10, 11),
        "A/RES/66/170",
    ).years(Some(2012), None),
    day(
        "International Day for Disaster Risk Reduction",
        Rule::gregorian(10, 13),
        "A/RES/64/200",
    ).years(Some(2009), None).read_from(2010),
    day(
        "International Day of Rural Women",
        Rule::gregorian(10, 15),
        "A/RES/62/136",
    ).years(Some(2008), None),
    day(
        "World Food Day",
        Rule::gregorian(10, 16),
        "FAO, A/RES/35/70",
    ).years(Some(1981), None),
    day(
        "International Day for the Eradication of Poverty",
        Rule::gregorian(10, 17),
        "A/RES/47/196",
    ).years(Some(1992), None).read_from(1993),
    day(
        "International Day of the Snow Leopard",
        Rule::gregorian(10, 23),
        "A/RES/79/143",
    ).years(Some(2024), None).read_from(2025),
    day(
        "United Nations Day",
        Rule::gregorian(10, 24),
        "A/RES/168 (II)",
    ).years(Some(1947), None).read_from(1948),
    day(
        "World Development Information Day",
        Rule::gregorian(10, 24),
        "A/RES/3038 (XXVII)",
    ).years(Some(1973), None),
    day(
        "World Day for Audiovisual Heritage",
        Rule::gregorian(10, 27),
        "UNESCO, 33 C/Resolution 5",
    ).years(Some(2005), None).read_from(2006),
    day(
        "International Day of Care and Support",
        Rule::gregorian(10, 29),
        "A/RES/77/317",
    ).years(Some(2023), None).read_from(2024),
    day("World Cities Day", Rule::gregorian(10, 31), "A/RES/68/239").years(Some(2014), None),
    day(
        "International Day to End Impunity for Crimes against Journalists",
        Rule::gregorian(11, 2),
        "A/RES/68/163",
    ).years(Some(2013), None).read_from(2014),
    day(
        "World Tsunami Awareness Day",
        Rule::gregorian(11, 5),
        "A/RES/70/203",
    ).years(Some(2015), None).read_from(2016),
    day(
        "International Day for Preventing the Exploitation of the Environment in War and Armed Conflict",
        Rule::gregorian(11, 6),
        "A/RES/56/4",
    ).years(Some(2001), None),
    day(
        "World Science Day for Peace and Development",
        Rule::gregorian(11, 10),
        "UNESCO, 31 C/Resolution 20",
    ).years(Some(2002), None),
    day(
        "World Diabetes Day",
        Rule::gregorian(11, 14),
        "A/RES/61/225",
    ).years(Some(2006), None).read_from(2007),
    day(
        "International Day for the Prevention of and Fight against All Forms of Transnational Organized Crime",
        Rule::gregorian(11, 15),
        "A/RES/78/267",
    ).years(Some(2024), None).read_from(2025),
    day(
        "International Day for Tolerance",
        Rule::gregorian(11, 16),
        "UNESCO, 28 C/Resolution 5.61",
    ).years(Some(1995), None),
    day(
        "International Day of the Mediterranean Diet",
        Rule::gregorian(11, 16),
        "A/RES/80/174",
    ).years(Some(2025), None).read_from(2026),
    day(
        "World Day for the Prevention of and Healing from Child Sexual Exploitation, Abuse and Violence",
        Rule::gregorian(11, 18),
        "A/RES/77/8",
    ).years(Some(2022), None).read_from(2023),
    day("World Toilet Day", Rule::gregorian(11, 19), "A/RES/67/291").years(Some(2013), None).read_from(2014),
    day(
        "Africa Industrialization Day",
        Rule::gregorian(11, 20),
        "A/RES/44/237",
    ).years(Some(1989), None).read_from(1990),
    day(
        "World Children's Day",
        Rule::gregorian(11, 20),
        "A/RES/836 (IX)",
    ).years(Some(1945), None).read_from(2026),
    day(
        "World Television Day",
        Rule::gregorian(11, 21),
        "A/RES/51/205",
    ).years(Some(1996), None).read_from(1997),
    day(
        "World Conjoined Twins Day",
        Rule::gregorian(11, 24),
        "A/RES/78/313",
    ).years(Some(2024), None).read_from(2025),
    day(
        "International Day for the Elimination of Violence against Women",
        Rule::gregorian(11, 25),
        "A/RES/54/134",
    ).years(Some(1999), None).read_from(2000),
    day(
        "World Sustainable Transport Day",
        Rule::gregorian(11, 26),
        "A/RES/77/286",
    ).years(Some(2023), None).read_from(2024),
    day(
        "International Day for the Elimination of Child, Early and Forced Marriage",
        Rule::gregorian(11, 27),
        "A/RES/80/309",
    ).years(Some(2026), None).read_from(2027),
    day(
        "International Day of Solidarity with the Palestinian People",
        Rule::gregorian(11, 29),
        "A/RES/32/40 B",
    ).years(Some(1978), None),
    day(
        "Day of Remembrance for all Victims of Chemical Warfare",
        Rule::gregorian(11, 30),
        "OPCW, C-20/DEC.10",
    ).years(Some(2015), None).read_from(2016),
    day("World AIDS Day", Rule::gregorian(12, 1), "WHO and UNAIDS").years(Some(1988), None),
    day(
        "International Day for the Abolition of Slavery",
        Rule::gregorian(12, 2),
        "A/RES/317 (IV)",
    ).years(Some(1945), None).read_from(2026),
    day(
        "International Day of Persons with Disabilities",
        Rule::gregorian(12, 3),
        "A/RES/47/3",
    ).years(Some(1992), None),
    day(
        "International Day Against Unilateral Coercive Measures",
        Rule::gregorian(12, 4),
        "A/RES/79/293",
    ).years(Some(2025), None).read_from(2026),
    day(
        "International Day of Banks",
        Rule::gregorian(12, 4),
        "A/RES/74/245",
    ).years(Some(2019), None).read_from(2020),
    day(
        "International Volunteer Day for Economic and Social Development",
        Rule::gregorian(12, 5),
        "A/RES/40/212",
    ).years(Some(1985), None).read_from(1986),
    day("World Soil Day", Rule::gregorian(12, 5), "A/RES/68/232").years(Some(2013), None).read_from(2014),
    day(
        "International Civil Aviation Day",
        Rule::gregorian(12, 7),
        "A/RES/51/33",
    ).years(Some(1996), None).read_from(1997),
    day(
        "International Anti-Corruption Day",
        Rule::gregorian(12, 9),
        "A/RES/58/4",
    ).years(Some(2003), None).read_from(2004),
    day(
        "International Day of Commemoration and Dignity of the Victims of the Crime of Genocide and of the Prevention of this Crime",
        Rule::gregorian(12, 9),
        "A/RES/69/323",
    ).years(Some(2015), None).read_from(2016),
    day("Human Rights Day", Rule::gregorian(12, 10), "A/RES/423 (V)").years(Some(1950), None).read_from(1951),
    day(
        "International Mountain Day",
        Rule::gregorian(12, 11),
        "A/RES/57/245",
    ).years(Some(2003), None),
    day(
        "International Day of Neutrality",
        Rule::gregorian(12, 12),
        "A/RES/71/275",
    ).years(Some(2017), None).read_from(2018),
    day(
        "International Universal Health Coverage Day",
        Rule::gregorian(12, 12),
        "A/RES/72/138",
    ).years(Some(2017), None).read_from(2018),
    day(
        "International Day against Colonialism in All its Forms and Manifestations",
        Rule::gregorian(12, 14),
        "A/RES/80/106",
    ).years(Some(2025), None).read_from(2026),
    day(
        "World Turkic Language Family Day",
        Rule::gregorian(12, 15),
        "UNESCO, 43 C/57",
    ).years(Some(2025), None).read_from(2026),
    day(
        "Arabic Language Day",
        Rule::gregorian(12, 18),
        "United Nations Department of Global Communications",
    ).years(Some(2012), None),
    day(
        "International Migrants Day",
        Rule::gregorian(12, 18),
        "A/RES/55/93",
    ).years(Some(2000), None),
    day(
        "International Day of Recognition for Women Searchers of Missing Persons",
        Rule::gregorian(12, 19),
        "A/RES/80/301",
    ).years(Some(2026), None).read_from(2027),
    day(
        "International Human Solidarity Day",
        Rule::gregorian(12, 20),
        "A/RES/60/209",
    ).years(Some(2005), None).read_from(2006),
    day(
        "World Basketball Day",
        Rule::gregorian(12, 21),
        "A/RES/77/324",
    ).years(Some(2023), None),
    day(
        "World Meditation Day",
        Rule::gregorian(12, 21),
        "A/RES/79/137",
    ).years(Some(2024), None).read_from(2025),
    day(
        "International Anti-Cybercrime Day",
        Rule::gregorian(12, 24),
        "A/RES/79/243",
    ).years(Some(2024), None).read_from(2025),
    day(
        "International Day of Epidemic Preparedness",
        Rule::gregorian(12, 27),
        "A/RES/75/27",
    ).years(Some(2020), None),
    // The seven days that are rules rather than dates.
    day(
        "Vesak, the Day of the Full Moon",
        Rule::LunarPhase {
            phase: MoonPhase::Full,
            month: 5,
            day: 1,
            meridian: Meridian::UNIVERSAL,
        },
        "A/RES/54/115",
    ).years(Some(2000), None)
    .approximate(),
    day(
        "World Migratory Bird Day",
        Rule::nth(5, 2, Weekday::Saturday),
        "UNEP, CMS and AEWA; the second Saturday of May",
    ).years(Some(2006), None),
    day(
        "International Day of Cooperatives",
        Rule::nth(7, 1, Weekday::Saturday),
        "A/RES/47/90; the first Saturday of July",
    ).years(Some(1995), None),
    day(
        "World Maritime Day",
        Rule::last(9, Weekday::Thursday),
        "IMO Council; the last Thursday of September",
    ).years(Some(1945), None).read_from(2026),
    day(
        "World Habitat Day",
        Rule::nth(10, 1, Weekday::Monday),
        "A/RES/40/202; the first Monday of October",
    ).years(Some(1986), None),
    day(
        "World Migratory Bird Day",
        Rule::nth(10, 2, Weekday::Saturday),
        "UNEP, CMS and AEWA; the second Saturday of October",
    ).years(Some(1945), None).read_from(2026),
    day(
        "World Day of Remembrance for Road Traffic Victims",
        Rule::nth(11, 3, Weekday::Sunday),
        "A/RES/60/5; the third Sunday of November",
    ).years(Some(2005), None).read_from(2006),
    day(
        "World Philosophy Day",
        Rule::nth(11, 3, Weekday::Thursday),
        "UNESCO, 33 C/Resolution 37; the third Thursday of November",
    ).years(Some(2002), None),
];

/// The international days of the United Nations and its agencies.
///
/// Every entry is a [`Kind::Observance`](crate::rule::Kind::Observance) and cites its resolution or
/// designating body; the dates are exact except Vesak's, which the United
/// Nations fixes year by year around the May full moon.
pub static UNITED_NATIONS: RuleSet = RuleSet {
    code: "un-days",
    english_name: "United Nations international days",
    rules: UNITED_NATIONS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 10, 5),
    sources: "United Nations, \"International Days and Weeks\", \
              un.org/en/observances/list-days-weeks, retrieved 2026-09-22 and again \
              2026-10-05 (the citations of A/RES/80/300, 80/301, 80/302 and 80/309 and \
              the days of the 80th session), and each rule-based day's own page; every \
              entry cites its resolution or designating body in its `source`. The first \
              year of a day is the year of its first observance where a page read says \
              so (an observance page, a United Nations or agency press release, a \
              report of the first observance), and otherwise the year of the first \
              occurrence of the day's date on or after the date the resolution was \
              adopted, the date taken from the meetings coverage of the United Nations or a \
              news report of the adoption, as docs/systems/holiday-first-years.md lists; \
              for the days with no such page read, the year Wikipedia's article gives \
              (secondary) or the session of the resolution. The resolutions' own texts, \
              which are PDFs, were not read",
    subdivisions: Subdivisions::Undivided,
};

/// A week of observance: every day from `from` to `to`, an observance
/// citing the instrument that set it.
const fn week(
    name: &'static str,
    from: &'static Rule,
    to: &'static Rule,
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, "", Rule::span(from, to)).cited(source)
}

const FEBRUARY_1: Rule = Rule::gregorian(2, 1);
const FEBRUARY_7: Rule = Rule::gregorian(2, 7);
const MARCH_21: Rule = Rule::gregorian(3, 21);
const MARCH_27: Rule = Rule::gregorian(3, 27);
const APRIL_24: Rule = Rule::gregorian(4, 24);
const APRIL_30: Rule = Rule::gregorian(4, 30);
const MAY_25: Rule = Rule::gregorian(5, 25);
const MAY_31: Rule = Rule::gregorian(5, 31);
const AUGUST_1: Rule = Rule::gregorian(8, 1);
const AUGUST_7: Rule = Rule::gregorian(8, 7);
const OCTOBER_4: Rule = Rule::gregorian(10, 4);
const OCTOBER_10: Rule = Rule::gregorian(10, 10);
const OCTOBER_24: Rule = Rule::gregorian(10, 24);
const OCTOBER_30: Rule = Rule::gregorian(10, 30);
const OCTOBER_31: Rule = Rule::gregorian(10, 31);
const NOVEMBER_18: Rule = Rule::gregorian(11, 18);
const NOVEMBER_24: Rule = Rule::gregorian(11, 24);
/// The Monday of the week, Monday to Sunday, in which 11 November falls.
const MONDAY_OF_11_NOVEMBER: Rule = Rule::WeekdayOnOrBefore {
    month: 11,
    day: 11,
    weekday: Weekday::Monday,
};
/// Its Sunday.
const SUNDAY_OF_11_NOVEMBER: Rule = Rule::WeekdayOnOrAfter {
    month: 11,
    day: 11,
    weekday: Weekday::Sunday,
};

static UNITED_NATIONS_WEEKS_RULES: &[HolidayRule] = &[
    week(
        "World Interfaith Harmony Week",
        &FEBRUARY_1,
        &FEBRUARY_7,
        "A/RES/65/5; the first week of February",
    )
    .years(Some(2010), None)
    .read_from(2011),
    week(
        "Week of Solidarity with the Peoples Struggling against Racism and Racial Discrimination",
        &MARCH_21,
        &MARCH_27,
        "A/RES/34/24",
    )
    .years(Some(1979), None)
    .read_from(1981),
    week("World Immunization Week", &APRIL_24, &APRIL_30, "WHO").years(Some(2012), None),
    week(
        "Week of Solidarity with the Peoples of Non-Self-Governing Territories",
        &MAY_25,
        &MAY_31,
        "A/RES/54/91",
    )
    .years(Some(1999), None)
    .read_from(2001),
    week("World Breastfeeding Week", &AUGUST_1, &AUGUST_7, "WHO").years(Some(1992), None),
    week("World Space Week", &OCTOBER_4, &OCTOBER_10, "A/RES/54/68")
        .years(Some(1999), None)
        .read_from(2000),
    week(
        "Disarmament Week",
        &OCTOBER_24,
        &OCTOBER_30,
        "A/RES/S-10/2, paragraph 102; from 24 October, the anniversary of the United Nations",
    )
    .years(Some(1978), None)
    .read_from(1979),
    // Eight days, one more than a span holds: its first seven and its
    // last.
    week(
        "Global Media and Information Literacy Week",
        &OCTOBER_24,
        &OCTOBER_30,
        "A/RES/75/267",
    )
    .years(Some(2021), None)
    .read_from(2022),
    HolidayRule::observance("Global Media and Information Literacy Week", "", OCTOBER_31)
        .years(Some(2021), None)
        .read_from(2022)
        .cited("A/RES/75/267"),
    week(
        "International Week of Science and Peace",
        &MONDAY_OF_11_NOVEMBER,
        &SUNDAY_OF_11_NOVEMBER,
        "A/RES/43/61; the week in which 11 November falls",
    )
    .years(Some(1988), None)
    .read_from(1989),
    week(
        "World Antimicrobial Resistance Awareness Week",
        &NOVEMBER_18,
        &NOVEMBER_24,
        "WHO",
    )
    .years(Some(1945), None)
    .read_from(2026),
];

/// The international weeks of the United Nations and its agencies, every
/// day of each, as the United Nations' list gives them and the resolutions
/// set them: World Interfaith Harmony Week, "the first week of February"
/// (A/RES/65/5), 1 to 7 February; the Week of Solidarity with the Peoples
/// Struggling against Racism and Racial Discrimination, 21 to 27 March
/// (A/RES/34/24); the Week of Solidarity with the Peoples of Non-Self-
/// Governing Territories, 25 to 31 May (A/RES/54/91); World Space Week,
/// 4 to 10 October (A/RES/54/68); Disarmament Week, "starting on
/// 24 October, the anniversary of the founding of the United Nations",
/// to the 30th (A/RES/S-10/2); the Global Media and Information Literacy
/// Week, 24 to 31 October (A/RES/75/267); the International Week of Science
/// and Peace, "the week in which 11 November falls" (A/RES/43/61), read as
/// the Monday-to-Sunday week, which gives the list's 9 to 15 November of
/// 2026; and the World Health Organization's World Immunization Week,
/// World Breastfeeding Week and World Antimicrobial Resistance Awareness
/// Week, on the dates the list prints for them.
pub static UNITED_NATIONS_WEEKS: RuleSet = RuleSet {
    code: "un-weeks",
    english_name: "United Nations international weeks",
    rules: UNITED_NATIONS_WEEKS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "United Nations, \"International Days and Weeks\", \
              un.org/en/observances/list-days-weeks, retrieved 2026-09-29, for every \
              week and its dates; the observances' own pages for World Interfaith \
              Harmony Week (A/RES/65/5), the International Week of Science and Peace \
              (A/RES/43/61), World Space Week (A/RES/54/68), Disarmament Week (A/RES/S-10/2) \
              and the Global Media and Information Literacy Week (A/RES/75/267), \
              retrieved 2026-09-29; the resolutions themselves, PDFs, were not read",
    subdivisions: Subdivisions::Undivided,
};

/// Every international table in the crate.
pub static ALL: &[&RuleSet] = &[&UNITED_NATIONS, &UNITED_NATIONS_WEEKS];

/// The table for an international set's identifier, by
/// [`hc_core::catalogue::matches`].
#[must_use]
pub fn by_code(code: &str) -> Option<&'static RuleSet> {
    ALL.iter()
        .copied()
        .find(|set| hc_core::catalogue::matches(code, set.code))
}

hc_core::catalogue_tests! {
    type: &'static RuleSet,
    id: |set| set.code,
    provenance: |set| set.sources,
    tests: international_table_tests,
    all: ALL,
    lookup: by_code,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::Kind;

    #[test]
    fn every_entry_is_an_observance_that_cites_its_instrument() {
        assert!(UNITED_NATIONS.rules.len() > 200);
        for rule in UNITED_NATIONS.rules {
            assert_eq!(rule.kind, Kind::Observance, "{}", rule.name);
            assert!(!rule.source.is_empty(), "{} cites nothing", rule.name);
            assert!(
                !rule.name.contains("Week"),
                "{} is a week, not a day",
                rule.name
            );
        }
    }
}
