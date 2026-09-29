//! The holidays India's states and union territories declare under the
//! Negotiable Instruments Act, 1881, each scoped to its ISO 3166-2 code,
//! for the twenty-eight that have a regional office of the Reserve Bank of
//! India of their own, Chandigarh apart.
//!
//! Each state notifies a list of such holidays every year, for the
//! purposes of section 25 of the Act, and the banks in the state close on
//! them. Neither the Act nor the notifications, which are gazette PDFs,
//! were read. The Reserve Bank of India's "Holidays under Negotiable
//! Instruments Act" (rbi.org.in, Scripts/HolidayMatrixDisplay.aspx) lists
//! every such day at each of its regional offices, year by year, and that
//! list, for 2019 to 2026, is what is carried: each day at the office in
//! the state, or, for a state with several offices, each day at every one
//! of them — Lucknow and Kanpur for Uttar Pradesh; Mumbai, Belapur and
//! Nagpur for Maharashtra; Thiruvananthapuram and Kochi for Kerala; Jammu
//! and Srinagar for Jammu and Kashmir. A day at one office of several is a
//! district's, which a state code cannot scope, and is left out: Id-E-Milad
//! 2025, kept at Mumbai on 8 September and at Belapur and Nagpur on the
//! 5th, is carried on neither. So is an election day at a state's one
//! office, whose extent in the state the list does not say: the
//! Legislative Assembly polls of Tamil Nadu (23 April 2026) and West Bengal
//! (29 April 2026) and Jaipur's municipal poll (11 September 2026).
//! `docs/systems/india-state-holidays.md` has the whole table.
//!
//! `STATE_DAYS` is generated: `python3 scripts/india-rbi-holidays.py`
//! reads the Reserve Bank's lists over HTTP and rewrites it, and `--check`
//! fails when it is stale. The script needs the network, so CI does not run
//! it. The rest of the file is written by hand.
//!
//! The kind is [`Kind::Bank`]: the list is of the Act's holidays, a day
//! off for the banks. The state's own list for its offices is its general
//! holidays, which the notification usually declares with it but which was
//! not read. The Reserve Bank lists no day that falls on a Sunday, when the
//! banks are closed in any case, so a state's holiday on a Sunday is not
//! carried.
//!
//! The name is the Reserve Bank's description of the day at the state's
//! office. The description is one for each date across nearly all the
//! offices: it joins with `/` the names the day has wherever it is a
//! holiday. It is split only at a `/` outside brackets, never at a comma or
//! a dash, and a part is left out of the state's name only where the list
//! itself shows that the state does not keep it:
//!
//! - in some year the part is the whole description of a date, and the
//!   state's office lists no date that year that bears it, nor the day
//!   before or after that date unless every office keeping the date keeps
//!   that day too, for a moon-sighted feast can fall a day apart;
//! - in no year is it the whole description of a date the office lists;
//! - and a part left in the name is the whole description of some date the
//!   office lists.
//!
//! Assam's 15 April 2022, "Good Friday/Bengali New Year’s Day
//! (Nababarsha)/Himachal Day/Vishu/Bohag Bihu", is "Bengali New Year’s Day
//! (Nababarsha)/Himachal Day/Vishu/Bohag Bihu": "Good Friday" is the whole
//! description of 19 April 2019, which Guwahati does not list, nor any
//! other day that year that bears it, and "Bohag Bihu" is the whole
//! description of 16 April 2022, which it does.
//!
//! Where the list shows nothing, the description is carried whole, and can
//! still name another state's festival beside the state's own. Maharashtra's
//! own list for 2026, as The Live Nagpur reports it, names the state's
//! days, and on them only the parts it names are carried: 1 May 2026,
//! "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary
//! of Pandit Raghunath Murmu", is "Maharashtra Din/Buddha Pournima". A day
//! several states keep under the same name is one rule scoped to all of
//! them. A nationwide day the Department of Personnel and Training's list
//! also has, as Republic Day, is the state's entry as well as the
//! nationwide one.
//!
//! The years before 2019 are a gap in every state carried, and so are those
//! before 2023 in Andhra Pradesh, Arunachal Pradesh and Nagaland, whose
//! offices at Vijayawada, Itanagar and Kohima have no list earlier; so is
//! 2027, whose lists the Reserve Bank had not published. Chandigarh's office
//! is not carried: whose days its list gives, the union territory's,
//! Punjab's or Haryana's, was not established. Nor are the union
//! territories with no office: Andaman and Nicobar, Dadra and Nagar Haveli
//! and Daman and Diu, Ladakh, Lakshadweep and Puducherry.

use crate::rule::{HolidayRule, Kind, Rule};

use super::asia::IN_RULES;
use crate::rule::joined;

/// The first year of the Reserve Bank's lists carried, the first for
/// which its form gives each office's list whole; the lists of the offices
/// at Vijayawada, Itanagar and Kohima begin in 2023.
const FIRST: i32 = 2019;
/// The first year of the lists of the offices at Vijayawada, Itanagar and
/// Kohima.
const FIRST_NEW_OFFICES: i32 = 2023;
/// The last.
const LAST: i32 = 2026;

/// The source of every day: the list, at the offices of the states the
/// day's rule names.
const SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its regional offices in the states named";

/// Uttar Pradesh.
const UTTAR_PRADESH: &[&str] = &["IN-UP"];
/// Uttar Pradesh's days, as the list of the Reserve Bank's offices at Lucknow and Kanpur give them.
const UTTAR_PRADESH_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Lucknow and Kanpur regional offices";
/// Maharashtra.
const MAHARASHTRA: &[&str] = &["IN-MH"];
/// Maharashtra's days, as the list of the Reserve Bank's offices at Mumbai and Belapur and Nagpur give them.
const MAHARASHTRA_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Mumbai and Belapur and Nagpur regional offices";
/// Bihar.
const BIHAR: &[&str] = &["IN-BR"];
/// Bihar's days, as the list of the Reserve Bank's office at Patna gives them.
const BIHAR_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Patna regional office";
/// West Bengal.
const WEST_BENGAL: &[&str] = &["IN-WB"];
/// West Bengal's days, as the list of the Reserve Bank's office at Kolkata gives them.
const WEST_BENGAL_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Kolkata regional office";
/// Madhya Pradesh.
const MADHYA_PRADESH: &[&str] = &["IN-MP"];
/// Madhya Pradesh's days, as the list of the Reserve Bank's office at Bhopal gives them.
const MADHYA_PRADESH_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Bhopal regional office";
/// Tamil Nadu.
const TAMIL_NADU: &[&str] = &["IN-TN"];
/// Tamil Nadu's days, as the list of the Reserve Bank's office at Chennai gives them.
const TAMIL_NADU_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Chennai regional office";
/// Rajasthan.
const RAJASTHAN: &[&str] = &["IN-RJ"];
/// Rajasthan's days, as the list of the Reserve Bank's office at Jaipur gives them.
const RAJASTHAN_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Jaipur regional office";
/// Karnataka.
const KARNATAKA: &[&str] = &["IN-KA"];
/// Karnataka's days, as the list of the Reserve Bank's office at Bengaluru gives them.
const KARNATAKA_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Bengaluru regional office";
/// Gujarat.
const GUJARAT: &[&str] = &["IN-GJ"];
/// Gujarat's days, as the list of the Reserve Bank's office at Ahmedabad gives them.
const GUJARAT_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Ahmedabad regional office";
/// Andhra Pradesh.
const ANDHRA_PRADESH: &[&str] = &["IN-AP"];
/// Andhra Pradesh's days, as the list of the Reserve Bank's office at Vijayawada gives them.
const ANDHRA_PRADESH_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Vijayawada regional office";
/// Odisha.
const ODISHA: &[&str] = &["IN-OD"];
/// Odisha's days, as the list of the Reserve Bank's office at Bhubaneswar gives them.
const ODISHA_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Bhubaneswar regional office";
/// Telangana.
const TELANGANA: &[&str] = &["IN-TS"];
/// Telangana's days, as the list of the Reserve Bank's office at Hyderabad gives them.
const TELANGANA_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Hyderabad regional office";
/// Kerala.
const KERALA: &[&str] = &["IN-KL"];
/// Kerala's days, as the list of the Reserve Bank's offices at Thiruvananthapuram and Kochi give them.
const KERALA_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Thiruvananthapuram and Kochi regional offices";
/// Assam.
const ASSAM: &[&str] = &["IN-AS"];
/// Assam's days, as the list of the Reserve Bank's office at Guwahati gives them.
const ASSAM_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Guwahati regional office";
/// Jharkhand.
const JHARKHAND: &[&str] = &["IN-JH"];
/// Jharkhand's days, as the list of the Reserve Bank's office at Ranchi gives them.
const JHARKHAND_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Ranchi regional office";
/// Delhi.
const DELHI: &[&str] = &["IN-DL"];
/// Delhi's days, as the list of the Reserve Bank's office at New Delhi gives them.
const DELHI_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its New Delhi regional office";
/// Jammu and Kashmir.
const JAMMU_AND_KASHMIR: &[&str] = &["IN-JK"];
/// Jammu and Kashmir's days, as the list of the Reserve Bank's offices at Jammu and Srinagar give them.
const JAMMU_AND_KASHMIR_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Jammu and Srinagar regional offices";
/// Uttarakhand.
const UTTARAKHAND: &[&str] = &["IN-UK"];
/// Uttarakhand's days, as the list of the Reserve Bank's office at Dehradun gives them.
const UTTARAKHAND_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Dehradun regional office";
/// Chhattisgarh.
const CHHATTISGARH: &[&str] = &["IN-CG"];
/// Chhattisgarh's days, as the list of the Reserve Bank's office at Raipur gives them.
const CHHATTISGARH_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Raipur regional office";
/// Himachal Pradesh.
const HIMACHAL_PRADESH: &[&str] = &["IN-HP"];
/// Himachal Pradesh's days, as the list of the Reserve Bank's office at Shimla gives them.
const HIMACHAL_PRADESH_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Shimla regional office";
/// Tripura.
const TRIPURA: &[&str] = &["IN-TR"];
/// Tripura's days, as the list of the Reserve Bank's office at Agartala gives them.
const TRIPURA_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Agartala regional office";
/// Meghalaya.
const MEGHALAYA: &[&str] = &["IN-ML"];
/// Meghalaya's days, as the list of the Reserve Bank's office at Shillong gives them.
const MEGHALAYA_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Shillong regional office";
/// Manipur.
const MANIPUR: &[&str] = &["IN-MN"];
/// Manipur's days, as the list of the Reserve Bank's office at Imphal gives them.
const MANIPUR_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Imphal regional office";
/// Nagaland.
const NAGALAND: &[&str] = &["IN-NL"];
/// Nagaland's days, as the list of the Reserve Bank's office at Kohima gives them.
const NAGALAND_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Kohima regional office";
/// Goa.
const GOA: &[&str] = &["IN-GA"];
/// Goa's days, as the list of the Reserve Bank's office at Panaji gives them.
const GOA_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Panaji regional office";
/// Arunachal Pradesh.
const ARUNACHAL_PRADESH: &[&str] = &["IN-AR"];
/// Arunachal Pradesh's days, as the list of the Reserve Bank's office at Itanagar gives them.
const ARUNACHAL_PRADESH_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Itanagar regional office";
/// Mizoram.
const MIZORAM: &[&str] = &["IN-MZ"];
/// Mizoram's days, as the list of the Reserve Bank's office at Aizawl gives them.
const MIZORAM_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Aizawl regional office";
/// Sikkim.
const SIKKIM: &[&str] = &["IN-SK"];
/// Sikkim's days, as the list of the Reserve Bank's office at Gangtok gives them.
const SIKKIM_SOURCE: &str = "Reserve Bank of India, holidays under the Negotiable Instruments Act at its Gangtok regional office";

/// A holiday under the Act on a day of `year`, in the states `regions`,
/// under the name the state's office gives it.
const fn nia(
    year: i32,
    month: u8,
    day: u8,
    name: &'static str,
    regions: &'static [&'static str],
) -> HolidayRule {
    HolidayRule::fixed_public(name, "", Rule::gregorian(month, day))
        .of_kind(Kind::Bank)
        .years(Some(year), Some(year))
        .in_regions(regions)
        .cited(SOURCE)
}

/// The years of `region`'s lists: none of its days in the years `first`
/// to [`LAST`], whose lists are its [`nia`] rules, and a gap in any other
/// year from `established` — before `first`, because the Reserve Bank's
/// form gives no earlier list whole and the state's own notifications were
/// not read, and after [`LAST`], because the days are declared year by
/// year. Before `established` the holidays are absent: the state did not
/// exist, or the Act was not yet in force ([`NI_ACT_IN_FORCE`], and the
/// reorganisation years of [`STATES_REORGANISATION`],
/// [`BOMBAY_REORGANISATION`] and [`ANDHRA_PRADESH_REORGANISATION`]).
const fn lists_read(
    first: i32,
    established: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(
        "Holidays under the Negotiable Instruments Act",
        "",
        Rule::unlisted(first as i64, LAST as i64),
    )
    .of_kind(Kind::Bank)
    .years(Some(established), None)
    .in_regions(region)
    .cited(source)
}

/// The Negotiable Instruments Act, 1881, commenced 1 March 1882
/// (Wikipedia, "Negotiable Instruments Act, 1881", secondary, retrieved
/// 2026-09-29): the first year a state's holidays under it can fall in.
const NI_ACT_IN_FORCE: i32 = 1882;
/// The States Reorganisation Act, 1956, effective 1 November 1956, which
/// formed Kerala and Andhra Pradesh (Wikipedia, "States Reorganisation Act,
/// 1956", secondary, retrieved 2026-09-29).
const STATES_REORGANISATION: i32 = 1956;
/// The Bombay Reorganisation Act, 1960, in effect 1 May 1960, which divided
/// Bombay State into Gujarat and Maharashtra (Wikipedia, "Maharashtra Day",
/// secondary, retrieved 2026-09-29).
const BOMBAY_REORGANISATION: i32 = 1960;
/// The Andhra Pradesh Reorganisation Act, 2014, commenced 2 June 2014,
/// which formed Telangana (Wikipedia, "Andhra Pradesh Reorganisation Act,
/// 2014", secondary, retrieved 2026-09-29).
const ANDHRA_PRADESH_REORGANISATION: i32 = 2014;

/// Assam, Delhi, Himachal Pradesh, Jammu and Kashmir, Manipur and Tripura
/// were provinces or princely states, not formed by a reorganisation of
/// states, and the year each first had holidays under the Act was not
/// established: their gap runs back to [`NI_ACT_IN_FORCE`].
const NOT_ESTABLISHED: i32 = NI_ACT_IN_FORCE;
/// Goa came under Indian rule on 19 December 1961 (Wikipedia, "Annexation
/// of Goa", secondary, retrieved 2026-09-29).
const GOA_ANNEXATION: i32 = 1961;
/// The state of Nagaland was inaugurated on 1 December 1963, under the
/// State of Nagaland Act, 1962 (Wikipedia, "Nagaland", secondary,
/// retrieved 2026-09-29).
const NAGALAND_STATEHOOD: i32 = 1963;
/// The North-Eastern Areas (Reorganisation) Act, 1971, which established
/// the state of Meghalaya and the union territories of Mizoram and
/// Arunachal Pradesh, in effect from 30 December 1971 (Wikipedia,
/// "North-Eastern Areas (Reorganisation) Act, 1971", secondary, retrieved
/// 2026-09-29).
const NORTH_EASTERN_AREAS_REORGANISATION: i32 = 1971;
/// Sikkim became a state of India under the Thirty-sixth Amendment of the
/// Constitution, in force from 26 April 1975 (Wikipedia, "Thirty-sixth
/// Amendment of the Constitution of India", secondary, retrieved
/// 2026-09-29).
const SIKKIM_STATEHOOD: i32 = 1975;
/// The Madhya Pradesh, Uttar Pradesh and Bihar Reorganisation Acts, 2000,
/// which formed Chhattisgarh on 1 November, Uttarakhand (as Uttaranchal)
/// on 9 November and Jharkhand on 15 November 2000 (Wikipedia,
/// "Chhattisgarh", "Uttar Pradesh Reorganisation Act, 2000" and "Bihar
/// Reorganisation Act, 2000", secondary, retrieved 2026-09-29).
const REORGANISATION_2000: i32 = 2000;

/// Every state's years, then every state day carried, in date order;
/// generated by `scripts/india-rbi-holidays.py`, not edited by hand.
#[rustfmt::skip]
pub static STATE_DAYS: &[HolidayRule] = &[
    lists_read(FIRST, NI_ACT_IN_FORCE, UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    lists_read(FIRST, BOMBAY_REORGANISATION, MAHARASHTRA, MAHARASHTRA_SOURCE),
    lists_read(FIRST, NI_ACT_IN_FORCE, BIHAR, BIHAR_SOURCE),
    lists_read(FIRST, NI_ACT_IN_FORCE, WEST_BENGAL, WEST_BENGAL_SOURCE),
    lists_read(FIRST, NI_ACT_IN_FORCE, MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    lists_read(FIRST, NI_ACT_IN_FORCE, TAMIL_NADU, TAMIL_NADU_SOURCE),
    lists_read(FIRST, NI_ACT_IN_FORCE, RAJASTHAN, RAJASTHAN_SOURCE),
    lists_read(FIRST, NI_ACT_IN_FORCE, KARNATAKA, KARNATAKA_SOURCE),
    lists_read(FIRST, BOMBAY_REORGANISATION, GUJARAT, GUJARAT_SOURCE),
    lists_read(FIRST_NEW_OFFICES, STATES_REORGANISATION, ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    lists_read(FIRST, NI_ACT_IN_FORCE, ODISHA, ODISHA_SOURCE),
    lists_read(FIRST, ANDHRA_PRADESH_REORGANISATION, TELANGANA, TELANGANA_SOURCE),
    lists_read(FIRST, STATES_REORGANISATION, KERALA, KERALA_SOURCE),
    lists_read(FIRST, NOT_ESTABLISHED, ASSAM, ASSAM_SOURCE),
    lists_read(FIRST, REORGANISATION_2000, JHARKHAND, JHARKHAND_SOURCE),
    lists_read(FIRST, NOT_ESTABLISHED, DELHI, DELHI_SOURCE),
    lists_read(FIRST, NOT_ESTABLISHED, JAMMU_AND_KASHMIR, JAMMU_AND_KASHMIR_SOURCE),
    lists_read(FIRST, REORGANISATION_2000, UTTARAKHAND, UTTARAKHAND_SOURCE),
    lists_read(FIRST, REORGANISATION_2000, CHHATTISGARH, CHHATTISGARH_SOURCE),
    lists_read(FIRST, NOT_ESTABLISHED, HIMACHAL_PRADESH, HIMACHAL_PRADESH_SOURCE),
    lists_read(FIRST, NOT_ESTABLISHED, TRIPURA, TRIPURA_SOURCE),
    lists_read(FIRST, NORTH_EASTERN_AREAS_REORGANISATION, MEGHALAYA, MEGHALAYA_SOURCE),
    lists_read(FIRST, NOT_ESTABLISHED, MANIPUR, MANIPUR_SOURCE),
    lists_read(FIRST_NEW_OFFICES, NAGALAND_STATEHOOD, NAGALAND, NAGALAND_SOURCE),
    lists_read(FIRST, GOA_ANNEXATION, GOA, GOA_SOURCE),
    lists_read(FIRST_NEW_OFFICES, NORTH_EASTERN_AREAS_REORGANISATION, ARUNACHAL_PRADESH, ARUNACHAL_PRADESH_SOURCE),
    lists_read(FIRST, NORTH_EASTERN_AREAS_REORGANISATION, MIZORAM, MIZORAM_SOURCE),
    lists_read(FIRST, SIKKIM_STATEHOOD, SIKKIM, SIKKIM_SOURCE),
    nia(2019, 1, 1, "New Year’s Day", &["IN-TN", "IN-ML", "IN-MN", "IN-MZ", "IN-SK"]),
    nia(2019, 1, 2, "New Year Celebration", &["IN-MZ"]),
    nia(2019, 1, 11, "Missionary Day", &["IN-MZ"]),
    nia(2019, 1, 12, "Birthday of Swami Vivekananda", &["IN-WB"]),
    nia(2019, 1, 14, "Makar Sankranti", &["IN-GJ"]),
    nia(2019, 1, 15, "Uttarayaana Punya kaala Makara Sankranti Festival/Pongal/Maghe Sankranti/Magh Bihu & Tusu Puja", &["IN-TN", "IN-KA", "IN-TS", "IN-AS", "IN-SK"]),
    nia(2019, 1, 16, "Thiruvalluvar Day", &["IN-TN"]),
    nia(2019, 1, 17, "Uzhavar Thirunal", &["IN-TN"]),
    nia(2019, 1, 18, "Imoinu Iratpa", &["IN-MN"]),
    nia(2019, 1, 19, "Gaan-Ngai", &["IN-MN"]),
    nia(2019, 1, 22, "Demise of Dr. Shri Shri Shri Shivakumara Mahaswamiji", &["IN-KA"]),
    nia(2019, 1, 23, "Netaji's Birthday", &["IN-WB", "IN-TR"]),
    nia(2019, 1, 26, "Republic Day", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2019, 2, 5, "Losar", &["IN-SK"]),
    nia(2019, 2, 15, "Lui Ngai-Ni", &["IN-MN"]),
    nia(2019, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti/Guru Ravidas’s Birthday", &["IN-MH", "IN-HP"]),
    nia(2019, 2, 20, "State Day", &["IN-MZ"]),
    nia(2019, 3, 1, "Chapchar Kut", &["IN-MZ"]),
    nia(2019, 3, 4, "Mahashivratri", &["IN-UP", "IN-MH", "IN-MP", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP"]),
    nia(2019, 3, 18, "Sad demise of Shri Manohar Parrikar, Chief Minister of Goa", &["IN-GA"]),
    nia(2019, 3, 20, "Holika Dahan", &["IN-UP", "IN-JH", "IN-UK"]),
    nia(2019, 3, 21, "Holi (Second Day)/Dhuleti/Birthday of Md. Hazarat Ali/Dol Jatra/Dhulandi/Holi (Jammu Province only)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-RJ", "IN-GJ", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-GA", "IN-SK"]),
    nia(2019, 3, 22, "Holi/Bihar Divas/Yaosang 2nd Day/Dol Jatra", &["IN-BR", "IN-OD", "IN-AS", "IN-MN"]),
    nia(2019, 4, 1, "Annual closing of Accounts of Commercial and Co-operative Banks", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-MN", "IN-GA", "IN-SK"]),
    nia(2019, 4, 5, "Babu Jagjivan Ram’s Birthday", &["IN-TS"]),
    nia(2019, 4, 6, "Gudhi Padwa/Ugadi Festival/Sajibu Nongmapanba (Cheiraoba)/1st Navratra/Telugu New Year’s Day", &["IN-MH", "IN-TN", "IN-KA", "IN-TS", "IN-JK", "IN-MN", "IN-GA"]),
    nia(2019, 4, 8, "Sarhul", &["IN-JH"]),
    nia(2019, 4, 13, "Ram Navami (Chaite Dasain)", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-RJ", "IN-JH", "IN-UK", "IN-SK"]),
    nia(2019, 4, 15, "Bengali New Year's Day/Vishu/Bohag Bihu", &["IN-WB", "IN-KL", "IN-AS", "IN-TR"]),
    nia(2019, 4, 16, "Bohag Bihu", &["IN-AS"]),
    nia(2019, 4, 17, "Mahavir Jayanti/Mahavir Janma Kalyanak", &["IN-UP", "IN-MH", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-JH", "IN-DL", "IN-CG"]),
    nia(2019, 4, 19, "Good Friday", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2019, 5, 1, "Maharashtra Din/May Day/Mazdoor Diwas", &["IN-MH", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-TS", "IN-KL", "IN-AS", "IN-MN", "IN-GA"]),
    nia(2019, 5, 7, "Basava Jayanthi", &["IN-KA"]),
    nia(2019, 5, 9, "Birthday of Rabindranath Tagore", &["IN-WB"]),
    nia(2019, 5, 16, "State Day", &["IN-SK"]),
    nia(2019, 5, 18, "Buddha Pournima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP"]),
    nia(2019, 5, 31, "Jumat-ul-Veda", &["IN-JK"]),
    nia(2019, 6, 5, "Ramzan Id (Id-ul-Fitr)(Shawal-1)/Khutub-E-Ramzan", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2019, 6, 6, "Maharana Pratap Jayanti", &["IN-HP"]),
    nia(2019, 6, 10, "Demise of Dr. Girish Karnad, Jnanpith Award Winner", &["IN-KA"]),
    nia(2019, 6, 15, "Y.M.A. Day/Raja Sankranti", &["IN-OD", "IN-MZ"]),
    nia(2019, 6, 17, "Saga Dawa", &["IN-SK"]),
    nia(2019, 7, 4, "Rath Yatra", &["IN-OD"]),
    nia(2019, 7, 5, "Guru Hargobindji’s Birthday", &["IN-JK"]),
    nia(2019, 7, 10, "Kharchi Puja", &["IN-TR"]),
    nia(2019, 7, 13, "Bhanu Jayanti/Martyr’s Day", &["IN-JK", "IN-SK"]),
    nia(2019, 7, 14, "Beh Dienkhlam", &["IN-ML"]),
    nia(2019, 7, 17, "U Tirot Sing Day", &["IN-ML"]),
    nia(2019, 7, 23, "Ker Puja", &["IN-TR"]),
    nia(2019, 8, 12, "Bakri Id (Id-ul-Zuha)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ"]),
    nia(2019, 8, 13, "Patriots’ Day/Bakri Id (Id-ul-Zuha)", &["IN-JK", "IN-MN"]),
    nia(2019, 8, 15, "Independence Day/Rakshabandhan", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2019, 8, 17, "Parsi New Year(Shahenshahi)/Pateti", &["IN-MH", "IN-GJ"]),
    nia(2019, 8, 23, "Krishna Jayanthi/Sri Krishna Janmashtmi", &["IN-UP", "IN-BR", "IN-TN", "IN-RJ", "IN-OD", "IN-TS", "IN-JH", "IN-UK"]),
    nia(2019, 8, 24, "Krishna Janmashtmi (Shravan Vad-8)", &["IN-GJ", "IN-TS", "IN-JK", "IN-CG", "IN-HP", "IN-ML", "IN-SK"]),
    nia(2019, 9, 2, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi", &["IN-MH", "IN-TN", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-GA"]),
    nia(2019, 9, 3, "Ganesh Ghaturthi (2nd day)", &["IN-GA"]),
    nia(2019, 9, 3, "Nuakhai", &["IN-OD"]),
    nia(2019, 9, 9, "Moharrum/Karma Puja", &["IN-OD", "IN-JH"]),
    nia(2019, 9, 10, "Moharram (Tajiya)/Ashoora/First Onam", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-RJ", "IN-KA", "IN-GJ", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-JK", "IN-CG", "IN-HP", "IN-TR", "IN-MZ"]),
    nia(2019, 9, 11, "Moharram (Ashoora)/Thiruvonam", &["IN-TN", "IN-KL"]),
    nia(2019, 9, 13, "Indrajatra/Pang-Lhabsol", &["IN-SK"]),
    nia(2019, 9, 13, "Pang-Lhabsol/Sree Narayana Guru Jayanthi", &["IN-KL"]),
    nia(2019, 9, 21, "Sree Narayana Guru Samadhi Day", &["IN-KL"]),
    nia(2019, 9, 28, "Mahalaya Amavasye", &["IN-WB", "IN-KA"]),
    nia(2019, 10, 1, "Half Yearly Closing of Bank Accounts", &["IN-SK"]),
    nia(2019, 10, 2, "Mahatma Gandhi Jayanti", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2019, 10, 5, "Durga Puja (Mahasaptami)", &["IN-WB", "IN-TR"]),
    nia(2019, 10, 7, "Maha Navami/Ayudhapooja/Durga Puja (Dasain)", &["IN-UP", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-OD", "IN-KL", "IN-AS", "IN-JH", "IN-TR", "IN-ML", "IN-SK"]),
    nia(2019, 10, 8, "Dasara/Vijaya Dashmi/Durga Puja (Dasain)/Dussehra", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2019, 10, 18, "Kati Bihu", &["IN-AS"]),
    nia(2019, 10, 21, "Maharashtra Legislative Assembly General Election 2019/Bye-Elections", &["IN-MH"]),
    nia(2019, 10, 28, "Diwali (Balipratipada)/Vikram Samvant New Year Day/Laxmi Puja/Govardhan Puja", &["IN-UP", "IN-MH", "IN-RJ", "IN-GJ", "IN-AS", "IN-UK", "IN-TR", "IN-MN", "IN-SK"]),
    nia(2019, 10, 29, "Bhai Tika/Bhaidooj/Chitragupt Puja/Balipadyami, Deepavali", &["IN-UP", "IN-KA", "IN-SK"]),
    nia(2019, 10, 30, "Ningol Chakkouba", &["IN-MN"]),
    nia(2019, 10, 31, "Sardar Vallabhbhai Patel’s Birthday", &["IN-GJ"]),
    nia(2019, 11, 1, "Kannada Rajyostsava/Kut", &["IN-KA", "IN-MN"]),
    nia(2019, 11, 2, "Chhath Puja/Chath (Evening Ardhya)", &["IN-BR", "IN-JH"]),
    nia(2019, 11, 8, "Wangala Festival", &["IN-ML"]),
    nia(2019, 11, 9, "Milad-i- Sherif (Birthday of Prophet Muhammed)", &["IN-KL"]),
    nia(2019, 11, 12, "Guru Nanak Jayanti/Rahasa Purnima/Kartika Purnima", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-RJ", "IN-OD", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP"]),
    nia(2019, 11, 15, "Friday following Eid-I-Milad-ul-Nabi", &["IN-JK"]),
    nia(2019, 11, 15, "Kanakadasa Jayanthi", &["IN-KA"]),
    nia(2019, 11, 19, "Lhabab Duechen", &["IN-SK"]),
    nia(2019, 11, 23, "Seng Kut snem", &["IN-ML"]),
    nia(2019, 12, 3, "Feast of St. Francis Xavier", &["IN-GA"]),
    nia(2019, 12, 5, "Birthday of Sheikh Mohammad Abdullah/Bye-Elections", &["IN-KA", "IN-JK"]),
    nia(2019, 12, 12, "Pa-Togan Nengminza Sangma/Legislative Assembly General Elections 2019", &["IN-JH", "IN-ML"]),
    nia(2019, 12, 18, "Death Anniversary of U SoSo Tham", &["IN-ML"]),
    nia(2019, 12, 19, "Goa Liberation Day", &["IN-GA"]),
    nia(2019, 12, 24, "Christmas Eve/Christmas Festival", &["IN-ML", "IN-MZ"]),
    nia(2019, 12, 25, "Christmas", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2019, 12, 26, "Christmas Festival", &["IN-ML", "IN-MZ"]),
    nia(2019, 12, 27, "Losoong/Namsoong", &["IN-SK"]),
    nia(2019, 12, 28, "Losoong/Namsoong", &["IN-SK"]),
    nia(2019, 12, 30, "U Kiang Nangbah", &["IN-ML"]),
    nia(2019, 12, 31, "New Year’s Eve", &["IN-MZ"]),
    nia(2020, 1, 1, "New Year’s Day", &["IN-TN", "IN-ML", "IN-MN", "IN-MZ", "IN-SK"]),
    nia(2020, 1, 2, "Year Celebration/Guru Govind Singh Ji Birthday", &["IN-MZ"]),
    nia(2020, 1, 7, "Imoinu Iratpa", &["IN-MN"]),
    nia(2020, 1, 8, "Gaan-Ngai", &["IN-MN"]),
    nia(2020, 1, 14, "Makar Sankranti", &["IN-GJ"]),
    nia(2020, 1, 15, "Uttarayaana Punyakaala Makara Sankranti Festival/Pongal/Magh Bihu and Tusu Puja", &["IN-TN", "IN-KA", "IN-TS", "IN-AS"]),
    nia(2020, 1, 16, "Thiruvalluvar Day", &["IN-TN"]),
    nia(2020, 1, 17, "Uzhavar Thirunal", &["IN-TN"]),
    nia(2020, 1, 23, "Birthday of Netaji Subhas Chandra Bose", &["IN-WB", "IN-TR"]),
    nia(2020, 1, 30, "Saraswati Puja/Basanta Panchami", &["IN-WB", "IN-OD", "IN-TR"]),
    nia(2020, 2, 15, "Lui-Ngai-Ni", &["IN-MN"]),
    nia(2020, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti", &["IN-MH"]),
    nia(2020, 2, 20, "State Day", &["IN-MZ"]),
    nia(2020, 2, 21, "Mahashivratri", &["IN-UP", "IN-MH", "IN-MP", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP"]),
    nia(2020, 2, 24, "Losar", &["IN-SK"]),
    nia(2020, 3, 6, "Chapchar Kut", &["IN-MZ"]),
    nia(2020, 3, 9, "Holika/Doljatra/Birthday of Md. Hazarat Ali", &["IN-UP"]),
    nia(2020, 3, 9, "Holika/Doljatra/Holi/Birthday of Md. Hazarat Ali/Attukal Pongala", &["IN-WB", "IN-TS", "IN-AS", "IN-JH", "IN-UK"]),
    nia(2020, 3, 10, "Holi (Second Day)/Dol Jatra/Dhuleti/Yaosang 2nd Day", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-RJ", "IN-GJ", "IN-OD", "IN-AS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2020, 3, 11, "Holi", &["IN-BR"]),
    nia(2020, 3, 25, "Gudhi Padwa/Ugadi Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st Navratra", &["IN-MH", "IN-TN", "IN-KA", "IN-TS", "IN-JK", "IN-MN", "IN-GA"]),
    nia(2020, 3, 27, "Sarhul", &["IN-JH"]),
    nia(2020, 4, 1, "Annual closing of banks", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2020, 4, 2, "Ram Navami", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-RJ", "IN-GJ", "IN-OD", "IN-TS", "IN-JH", "IN-UK", "IN-HP"]),
    nia(2020, 4, 6, "Mahavir Jayanti", &["IN-UP", "IN-MH", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-JH", "IN-DL", "IN-CG"]),
    nia(2020, 4, 10, "Good Friday", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2020, 4, 13, "Biju Festival/Bohag Bihu/Cheiraoba/Baisakhi", &["IN-AS", "IN-JK", "IN-TR", "IN-MN"]),
    nia(2020, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Bengali New Year’s Day/Tamil New Year's Day/Bohag Bihu/Vishu", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-GA", "IN-SK"]),
    nia(2020, 4, 15, "Bohag Bihu/Himachal Day", &["IN-AS", "IN-HP"]),
    nia(2020, 4, 20, "Garia Puja", &["IN-TR"]),
    nia(2020, 4, 25, "Bhagvan Shree Parshuram Jayanti", &["IN-GJ"]),
    nia(2020, 5, 1, "Maharashtra Din/May Day (Labour Day)", &["IN-MH", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-TS", "IN-KL", "IN-AS", "IN-MN", "IN-GA"]),
    nia(2020, 5, 7, "Buddha Pournima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-MZ"]),
    nia(2020, 5, 8, "Birthday of Rabindranath Tagore", &["IN-WB"]),
    nia(2020, 5, 21, "Shab-I-Qadr", &["IN-JK"]),
    nia(2020, 5, 22, "Jumat-ul-Vida", &["IN-JK"]),
    nia(2020, 5, 25, "Ramzan Id (Id-Ul-Fitr) (Shawal-1)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2020, 6, 5, "Saga Dawa", &["IN-SK"]),
    nia(2020, 6, 15, "Y.M.A. Day/Raja Sankranti", &["IN-OD", "IN-MZ"]),
    nia(2020, 6, 18, "Guru Hargobind Ji's Birthday", &["IN-JK"]),
    nia(2020, 6, 23, "Ratha Yatra", &["IN-OD"]),
    nia(2020, 6, 30, "Remna Ni", &["IN-MZ"]),
    nia(2020, 7, 8, "Beh Dienkhlam", &["IN-ML"]),
    nia(2020, 7, 13, "Bhanu Jayanti", &["IN-SK"]),
    nia(2020, 7, 14, "Ker Puja", &["IN-TR"]),
    nia(2020, 7, 16, "Harela", &["IN-UK"]),
    nia(2020, 7, 17, "U Tirot Sing Day", &["IN-ML"]),
    nia(2020, 7, 18, "COVID-19 containment measure", &["IN-KA", "IN-UK"]),
    nia(2020, 7, 24, "Drukpa Tshechi", &["IN-SK"]),
    nia(2020, 7, 27, "COVID-19 containment measure", &["IN-ML"]),
    nia(2020, 7, 28, "COVID-19 containment measure", &["IN-ML"]),
    nia(2020, 7, 29, "COVID-19 containment measure", &["IN-ML"]),
    nia(2020, 7, 31, "Bakrid", &["IN-KL", "IN-JK"]),
    nia(2020, 8, 1, "Bakri ID (Id-Ul-Zuha)/COVID-19 containment measure", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-MZ"]),
    nia(2020, 8, 3, "Raksha Bandhan", &["IN-UP", "IN-RJ", "IN-GJ", "IN-UK"]),
    nia(2020, 8, 11, "Sri Krishna Janmastami", &["IN-BR", "IN-TN", "IN-OD", "IN-TS"]),
    nia(2020, 8, 12, "Janmashtami", &["IN-UP", "IN-MP", "IN-RJ", "IN-GJ", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML"]),
    nia(2020, 8, 13, "Patriot’s Day", &["IN-MN"]),
    nia(2020, 8, 15, "Independence Day/COVID-19 containment measure", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2020, 8, 20, "Tithi of Srimanta Sankardeva", &["IN-AS"]),
    nia(2020, 8, 21, "Teej (Haritalika)", &["IN-SK"]),
    nia(2020, 8, 22, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Vinayakar Chathurthi", &["IN-MH", "IN-TN", "IN-GJ", "IN-TS", "IN-GA"]),
    nia(2020, 8, 29, "Karma Puja/Ashoora/COVID-19 containment measure", &["IN-JH", "IN-JK"]),
    nia(2020, 8, 31, "Indrajatra", &["IN-SK"]),
    nia(2020, 8, 31, "Thiruvonam", &["IN-KL"]),
    nia(2020, 9, 2, "Pang-Lhabsol/Sree Narayana Guru Jayanthi", &["IN-KL", "IN-SK"]),
    nia(2020, 9, 17, "Mahalaya Amavasye", &["IN-WB", "IN-KA", "IN-TR"]),
    nia(2020, 9, 21, "Sree Narayana Guru Samadhi Day", &["IN-KL"]),
    nia(2020, 10, 2, "Mahatma Gandhi Jayanti", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2020, 10, 14, "Due to unprecedented incessant rains for the last 12 hours and imminent forecast of heavy rains in Telangana State", &["IN-TS"]),
    nia(2020, 10, 15, "Due to unprecedented incessant rains for the last 12 hours and imminent forecast of heavy rains in Telangana State", &["IN-TS"]),
    nia(2020, 10, 17, "Kati Bihu", &["IN-AS"]),
    nia(2020, 10, 17, "Mera Chaoren Houba of Lainingthou Sanamahi", &["IN-MN"]),
    nia(2020, 10, 23, "Durga Puja/Dussehra (Maha Saptami)", &["IN-WB", "IN-TR"]),
    nia(2020, 10, 24, "Durga Puja/Dussehra (Mahanavami)/(Maha asthmi)", &["IN-UP", "IN-BR", "IN-WB", "IN-TS", "IN-KL", "IN-AS", "IN-TR", "IN-ML", "IN-MN"]),
    nia(2020, 10, 26, "Durga Puja (Vijayadashami)/Acession Day/Dussehra (Maha Saptami)", &["IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-OD", "IN-KL", "IN-AS", "IN-JH", "IN-JK", "IN-TR", "IN-ML", "IN-SK"]),
    nia(2020, 10, 27, "Durga Puja", &["IN-SK"]),
    nia(2020, 10, 28, "Durga Puja", &["IN-SK"]),
    nia(2020, 10, 29, "Durga Puja/Milad-i-Sherif (Birthday of Prophet Muhammed)", &["IN-KL", "IN-JK", "IN-SK"]),
    nia(2020, 10, 30, "Id-E-Milad (Milad-un-Nabi)/Baravafat/Lakshmi Puja", &["IN-WB", "IN-TR"]),
    nia(2020, 10, 30, "Id-E-Milad (Milad-un-Nabi)/Friday following Eid-i-Milad-ul-Nabi/Baravafat", &["IN-JK"]),
    nia(2020, 10, 30, "Id-E-Milad (Milad-un-Nabi)/Friday following Eid-i-Milad-ul-Nabi/Baravafat/Lakshmi Puja", &["IN-UP", "IN-MH", "IN-MP", "IN-TN", "IN-KA", "IN-GJ", "IN-TS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-MN"]),
    nia(2020, 10, 31, "Sardar Vallabhbhai Patel’s Birthday/Maharishi Valmiki Jayanthi/Kumar Purnima", &["IN-KA", "IN-GJ", "IN-OD", "IN-HP"]),
    nia(2020, 11, 13, "Wangala Festival", &["IN-ML"]),
    nia(2020, 11, 14, "Diwali Amavasaya (Laxmi Pujan)/Kali Puja", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-GA"]),
    nia(2020, 11, 16, "Diwali (Balipratipada)/Laxmi Puja/Bhaidooj/Chitragupt Jayanti/Vikram Samvat New Year Day", &["IN-UP", "IN-MH", "IN-KA", "IN-GJ", "IN-SK"]),
    nia(2020, 11, 17, "Laxmi Puja/Deepawali/Ningol Chakkouba", &["IN-MN", "IN-SK"]),
    nia(2020, 11, 18, "Laxmi Puja/Deepawali", &["IN-SK"]),
    nia(2020, 11, 20, "Chhath Puja", &["IN-BR", "IN-JH"]),
    nia(2020, 11, 21, "Chhath Puja", &["IN-BR"]),
    nia(2020, 11, 23, "Seng Kutsnem", &["IN-ML"]),
    nia(2020, 11, 30, "Guru Nanak Jayanti/Kartika Purnima/Rahasa Purnima", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-RJ", "IN-OD", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-MZ"]),
    nia(2020, 12, 3, "Feast of St. Francis Xavier", &["IN-GA"]),
    nia(2020, 12, 3, "Kanakadasa Jayanthi", &["IN-KA"]),
    nia(2020, 12, 12, "Pa-Togan Nengminza Sangma", &["IN-ML"]),
    nia(2020, 12, 17, "Losoong/Namsoong", &["IN-SK"]),
    nia(2020, 12, 18, "Death Anniversary of U SoSo Tham/Losoong/Namsoong", &["IN-ML"]),
    nia(2020, 12, 18, "Losoong/Namsoong", &["IN-SK"]),
    nia(2020, 12, 19, "Goa Liberation Day", &["IN-GA"]),
    nia(2020, 12, 24, "Christmas Festival", &["IN-ML", "IN-MZ"]),
    nia(2020, 12, 25, "Christmas", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2020, 12, 26, "Christmas Festival", &["IN-ML"]),
    nia(2020, 12, 30, "U Kiang Nangbah", &["IN-ML"]),
    nia(2020, 12, 31, "Year’s Eve", &["IN-MZ"]),
    nia(2021, 1, 1, "New Year's Day", &["IN-TN", "IN-ML", "IN-MN", "IN-MZ", "IN-SK"]),
    nia(2021, 1, 2, "New Year’s Celebration", &["IN-MZ"]),
    nia(2021, 1, 12, "Birthday of Swami Vivekananda", &["IN-WB"]),
    nia(2021, 1, 14, "Uttarayaana Punyakaala Makar Sankranti Festival/Pongal/Maghe Sankranti//Magh Bihu and Tusu Puja", &["IN-TN", "IN-KA", "IN-GJ", "IN-TS", "IN-AS", "IN-SK"]),
    nia(2021, 1, 15, "Thiruvalluvar Day", &["IN-TN"]),
    nia(2021, 1, 16, "Uzhavar Thirunal", &["IN-TN"]),
    nia(2021, 1, 23, "Birthday of Netaji Subhas Chandra Bose", &["IN-WB", "IN-TR"]),
    nia(2021, 1, 25, "Imoinu Iratpa", &["IN-MN"]),
    nia(2021, 1, 26, "Republic Day", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2021, 1, 26, "Republic Day/Gaan-Ngai", &["IN-MN"]),
    nia(2021, 1, 28, "Thaipoosam Festival", &["IN-TN"]),
    nia(2021, 2, 12, "Losar/Sonam Lochhar", &["IN-SK"]),
    nia(2021, 2, 15, "Lui-Ngai-Ni", &["IN-MN"]),
    nia(2021, 2, 16, "Saraswati Puja (Shree Panchami)/Basanta Panchami", &["IN-WB", "IN-OD", "IN-TR"]),
    nia(2021, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti", &["IN-MH"]),
    nia(2021, 2, 20, "State Day", &["IN-MZ"]),
    nia(2021, 2, 26, "Birthday of Md. Hazarat Ali", &["IN-UP"]),
    nia(2021, 3, 5, "Chapchar Kut", &["IN-MZ"]),
    nia(2021, 3, 11, "Mahashivratri (Maha Vad-13)", &["IN-UP", "IN-MH", "IN-MP", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP"]),
    nia(2021, 3, 22, "Bihar Divas", &["IN-BR"]),
    nia(2021, 3, 29, "Holi (Second Day) - Dhuleti/Yaosang 2nd Day", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-RJ", "IN-GJ", "IN-OD", "IN-TS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-MN", "IN-GA", "IN-SK"]),
    nia(2021, 3, 30, "Holi", &["IN-BR"]),
    nia(2021, 4, 1, "To enable Banks to close their yearly accounts", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-MN", "IN-GA", "IN-SK"]),
    nia(2021, 4, 2, "Good Friday", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2021, 4, 5, "Babu Jagjivan Ram’s Birthday", &["IN-TS"]),
    nia(2021, 4, 13, "Gudhi Padwa/Telugu New Year's Day/Ugadi Festival/Sajibu Nongmapanba (Cheiraoba)/1st Navratra/Baisakhi", &["IN-MH", "IN-TN", "IN-KA", "IN-TS", "IN-JK", "IN-MN", "IN-GA"]),
    nia(2021, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Tamil New Year's Day/Vishu/Biju Festival/Cheiraoba/Bohag Bihu", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-SK"]),
    nia(2021, 4, 15, "Himachal Day/Bengali New Year’s Day/Bohag Bihu/Sarhul/Village Panchayat and District Panchayat election", &["IN-WB", "IN-JH", "IN-HP", "IN-TR"]),
    nia(2021, 4, 15, "Himachal Day/Bengali New Year’s Day/Bohag Bihu/Village Panchayat and District Panchayat election", &["IN-AS"]),
    nia(2021, 4, 16, "Bohag Bihu", &["IN-AS"]),
    nia(2021, 4, 21, "Shree Ram Navmi (Chaite Dashain)/Garia Puja", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-RJ", "IN-GJ", "IN-OD", "IN-TS", "IN-JH", "IN-UK", "IN-HP", "IN-TR", "IN-SK"]),
    nia(2021, 5, 1, "Maharashtra Din/May Day (Labour Day)", &["IN-MH", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-TS", "IN-KL", "IN-AS", "IN-MN", "IN-GA"]),
    nia(2021, 5, 7, "Jumat-ul-Vida", &["IN-JK"]),
    nia(2021, 5, 10, "Shab-i-Qadr", &["IN-JK"]),
    nia(2021, 5, 13, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)", &["IN-MH", "IN-KL", "IN-JK"]),
    nia(2021, 5, 14, "Bhagvan Shree Parshuram Jayanti/Ramjan-Eid (Eid-UI-Fitra)/Basava Jayanti/Akshaya Tritiya", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-OD", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2021, 5, 14, "Ramjan-Eid (Eid-UI-Fitra)/Basava Jayanti/Akshaya Tritiya", &["IN-KA"]),
    nia(2021, 5, 26, "Buddha Pournima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR"]),
    nia(2021, 6, 15, "Y.M.A. Day/Raja Sankranti", &["IN-OD", "IN-MZ"]),
    nia(2021, 6, 17, "Extension of lockdown due COVID-19 pandemic", &["IN-KL"]),
    nia(2021, 6, 19, "Extension of lockdown due COVID-19 pandemic", &["IN-KL"]),
    nia(2021, 6, 22, "Extension of lockdown due COVID-19 pandemic", &["IN-KL"]),
    nia(2021, 6, 25, "Guru Hargobind Ji's Birthday", &["IN-JK"]),
    nia(2021, 6, 30, "Remna Ni", &["IN-MZ"]),
    nia(2021, 7, 12, "Kang (Rathajatra)/Ratha Yatra", &["IN-OD", "IN-MN"]),
    nia(2021, 7, 13, "Bhanu Jayanti", &["IN-SK"]),
    nia(2021, 7, 14, "Drukpa Tshechi", &["IN-SK"]),
    nia(2021, 7, 16, "Harela", &["IN-UK"]),
    nia(2021, 7, 17, "Kharchi Puja", &["IN-TR"]),
    nia(2021, 7, 17, "U Tirot Sing Day", &["IN-ML"]),
    nia(2021, 7, 19, "Guru Rimpoche’s Thungkar Tshechu", &["IN-SK"]),
    nia(2021, 7, 21, "Bakri Id (Id-Ul-Zuha) (Eid-Ul-Azha)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA"]),
    nia(2021, 7, 22, "Eid-Ul-Azha", &["IN-JK"]),
    nia(2021, 7, 31, "Ker Puja", &["IN-TR"]),
    nia(2021, 8, 13, "Patriot’s Day", &["IN-MN"]),
    nia(2021, 8, 16, "Parse New Year (Shahenshahi)", &["IN-MH"]),
    nia(2021, 8, 19, "Muharram (Ashoora)", &["IN-MH", "IN-BR", "IN-WB", "IN-JK", "IN-CG", "IN-TR"]),
    nia(2021, 8, 20, "Muharram/First Onam", &["IN-UP", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-MZ"]),
    nia(2021, 8, 21, "Thiruvonam", &["IN-KL"]),
    nia(2021, 8, 23, "Sree Narayana Guru Jayanthi/Demise of Shri Kalyan Singh, Former Chief Minister and Former Governor, Rajasthan & Himachal Pradesh", &["IN-UP", "IN-KL"]),
    nia(2021, 8, 30, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", &["IN-UP", "IN-BR", "IN-TN", "IN-RJ", "IN-GJ", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-SK"]),
    nia(2021, 8, 31, "Sri Krishna Ashtami", &["IN-TS"]),
    nia(2021, 9, 8, "Tithi of Srimanta Sankardeva", &["IN-AS"]),
    nia(2021, 9, 9, "Teej (Haritalika)", &["IN-SK"]),
    nia(2021, 9, 10, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Vinayakar Chathurthi/Varasiddhi Vinayaka Vrata", &["IN-MH", "IN-TN", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-GA"]),
    nia(2021, 9, 11, "Ganesh Chaturthi (2nd day)", &["IN-GA"]),
    nia(2021, 9, 17, "Karma Puja", &["IN-JH"]),
    nia(2021, 9, 20, "Indrajatra", &["IN-SK"]),
    nia(2021, 9, 21, "Sree Narayana Guru Samadhi Day", &["IN-KL"]),
    nia(2021, 10, 1, "Half Yearly Closing of Bank Accounts", &["IN-SK"]),
    nia(2021, 10, 2, "Mahatma Gandhi Jayanti", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2021, 10, 6, "Mahalaya Amavasye", &["IN-WB", "IN-KA", "IN-TR"]),
    nia(2021, 10, 7, "Mera Chaoren Houba of Lainingthou Sanamahi", &["IN-MN"]),
    nia(2021, 10, 12, "Durga Puja (Maha Saptami)", &["IN-WB", "IN-TR"]),
    nia(2021, 10, 13, "Durga Puja (Maha Ashtami)", &["IN-BR", "IN-WB", "IN-OD", "IN-AS", "IN-JH", "IN-TR", "IN-MN", "IN-SK"]),
    nia(2021, 10, 14, "Durga Puja/Dussehra (Maha Navami)/Ayutha Pooja", &["IN-UP", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-KL", "IN-AS", "IN-JH", "IN-TR", "IN-ML", "IN-SK"]),
    nia(2021, 10, 15, "Durga Puja/Dasara/Dusshera (Vijaya Dashmi)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-ML", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2021, 10, 16, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2021, 10, 18, "Kati Bihu", &["IN-AS"]),
    nia(2021, 10, 19, "Id-E-Milad/Eid-e-Miladunnabi/Milad-i-Sherif (Prophet Mohammad's Birthday)/Baravafat", &["IN-UP", "IN-MH", "IN-MP", "IN-TN", "IN-KA", "IN-GJ", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-MN"]),
    nia(2021, 10, 20, "Maharishi Valmiki’s Birthday/Lakshmi Puja", &["IN-WB", "IN-KA", "IN-HP", "IN-TR"]),
    nia(2021, 10, 22, "Friday following Eid-i-Milad-ul-Nabi", &["IN-JK"]),
    nia(2021, 10, 26, "Accession Day", &["IN-JK"]),
    nia(2021, 11, 1, "Kannada Rajyostsava/Kut", &["IN-KA", "IN-MN"]),
    nia(2021, 11, 3, "Naraka Chaturdashi", &["IN-KA"]),
    nia(2021, 11, 4, "Diwali Amavasaya (Laxmi Pujan)/Deepavali/Kali Puja", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2021, 11, 5, "Diwali (Bali Pratipada)/Vikram Samvant New Year Day/Govardhan Pooja", &["IN-UP", "IN-MH", "IN-RJ", "IN-KA", "IN-GJ", "IN-UK", "IN-SK"]),
    nia(2021, 11, 6, "Bhai Duj/Chitragupt Jayanti/Laxmi Puja/Deepawali/Ningol Chakkouba", &["IN-UP", "IN-HP", "IN-MN", "IN-SK"]),
    nia(2021, 11, 10, "Chhath Puja/Surya Pashti Dala Chhath (Sayan ardhya)/Chhath Parv", &["IN-BR", "IN-JH", "IN-CG"]),
    nia(2021, 11, 11, "Chhath Puja/Morning Ardhya of Chath", &["IN-BR", "IN-JH"]),
    nia(2021, 11, 12, "Wangala Festival", &["IN-ML"]),
    nia(2021, 11, 19, "Guru Nanak Jayanti/Karthika Purnima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-RJ", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-MZ"]),
    nia(2021, 11, 22, "Kanakadasa Jayanthi", &["IN-KA"]),
    nia(2021, 11, 23, "Seng Kutsnem", &["IN-ML"]),
    nia(2021, 12, 3, "Feast of St. Francis Xavier", &["IN-GA"]),
    nia(2021, 12, 18, "Death Anniversary of U SoSo Tham", &["IN-ML"]),
    nia(2021, 12, 24, "Christmas Festival (Christmas Eve)/General Election to the Municipal Corporation", &["IN-ML", "IN-MZ"]),
    nia(2021, 12, 25, "Christmas", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2021, 12, 27, "Christmas Celebration", &["IN-MZ"]),
    nia(2021, 12, 30, "U Kiang Nangbah", &["IN-ML"]),
    nia(2021, 12, 31, "New Year’s Eve", &["IN-MZ"]),
    nia(2022, 1, 1, "New Year’s Day", &["IN-TN", "IN-ML", "IN-MN", "IN-MZ", "IN-SK"]),
    nia(2022, 1, 3, "Losoong", &["IN-SK"]),
    nia(2022, 1, 3, "New Year’s Celebration/Losoong", &["IN-MZ"]),
    nia(2022, 1, 4, "Losoong", &["IN-SK"]),
    nia(2022, 1, 11, "Missionary Day", &["IN-MZ"]),
    nia(2022, 1, 12, "Birthday of Swami Vivekananda", &["IN-WB"]),
    nia(2022, 1, 14, "Makar Sankranti/Pongal", &["IN-GJ"]),
    nia(2022, 1, 14, "Makar Sankranti/Pongal/Imoinu Iratpa", &["IN-TN"]),
    nia(2022, 1, 14, "Pongal/Imoinu Iratpa", &["IN-MN"]),
    nia(2022, 1, 15, "Uttarayaana Punyakaala Makar Sankranti Festival/Maghe Sankranti/Sankranti/Pongal/Gaan-Ngai/Magh Bihu", &["IN-MN"]),
    nia(2022, 1, 15, "Uttarayaana Punyakaala Makar Sankranti Festival/Maghe Sankranti/Sankranti/Pongal/Thiruvalluvar Day/Gaan-Ngai/Magh Bihu", &["IN-KA", "IN-TS", "IN-AS", "IN-SK"]),
    nia(2022, 1, 15, "Uttarayaana Punyakaala Makar Sankranti Festival/Maghe Sankranti/Sankranti/Pongal/Thiruvalluvar Day/Magh Bihu", &["IN-TN"]),
    nia(2022, 1, 18, "Thai Poosam", &["IN-TN"]),
    nia(2022, 1, 26, "Republic Day", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2022, 2, 2, "Sonam Lochhar", &["IN-SK"]),
    nia(2022, 2, 5, "Saraswati Puja (Shree Panchami)/Basanta Panchami", &["IN-WB", "IN-OD", "IN-TR"]),
    nia(2022, 2, 7, "Homage to Bharat Ratna late Ku. Lata Mangeshkar", &["IN-MH"]),
    nia(2022, 2, 15, "Birthday of Md. Hazrat Ali/Lui-Ngai-Ni", &["IN-UP", "IN-MN"]),
    nia(2022, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti/Ordinary Elections to the Urban Local Bodies, 2022", &["IN-MH", "IN-TN"]),
    nia(2022, 3, 1, "Mahashivratri (Maha Vad-14)", &["IN-UP", "IN-MH", "IN-MP", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP"]),
    nia(2022, 3, 3, "Losar", &["IN-SK"]),
    nia(2022, 3, 4, "Chapchar Kut", &["IN-MZ"]),
    nia(2022, 3, 17, "Holika Dahan", &["IN-UP", "IN-JH", "IN-UK"]),
    nia(2022, 3, 18, "Holi/Holi 2nd Day – Dhuleti/Doljatra", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-RJ", "IN-GJ", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2022, 3, 19, "Holi/Yaosang 2nd Day", &["IN-BR", "IN-OD", "IN-JH", "IN-MN"]),
    nia(2022, 3, 22, "Bihar Divas", &["IN-BR"]),
    nia(2022, 4, 1, "Yearly Closing of Bank Account", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-MN", "IN-GA", "IN-SK"]),
    nia(2022, 4, 2, "Gudi Padwa/Ugadi Festival/1st Navratra/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)", &["IN-MH", "IN-TN", "IN-KA", "IN-TS", "IN-JK", "IN-MN", "IN-GA"]),
    nia(2022, 4, 4, "Sarhul", &["IN-JH"]),
    nia(2022, 4, 5, "Babu Jagjivan Ram’s Birthday", &["IN-TS"]),
    nia(2022, 4, 11, "Shad Suk Mynsiem", &["IN-ML"]),
    nia(2022, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Baisakhi/Vaisakhi/Tamil New Year's Day/Cheiraoba/Biju Festival/Bohag Bihu", &["IN-AS"]),
    nia(2022, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Mahavir Jayanti/Baisakhi/Vaisakhi/Tamil New Year's Day/Cheiraoba/Biju Festival", &["IN-MP", "IN-CG", "IN-MZ"]),
    nia(2022, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Mahavir Jayanti/Baisakhi/Vaisakhi/Tamil New Year's Day/Cheiraoba/Biju Festival/Bohag Bihu", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-HP", "IN-TR", "IN-MN", "IN-GA", "IN-SK"]),
    nia(2022, 4, 15, "Bengali New Year’s Day (Nababarsha)/Himachal Day/Vishu/Bohag Bihu", &["IN-AS"]),
    nia(2022, 4, 15, "Good Friday/Bengali New Year’s Day (Nababarsha)/Himachal Day/Vishu", &["IN-BR", "IN-MP", "IN-OD", "IN-TS", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2022, 4, 15, "Good Friday/Bengali New Year’s Day (Nababarsha)/Himachal Day/Vishu/Bohag Bihu", &["IN-UP", "IN-MH", "IN-WB", "IN-TN", "IN-KA", "IN-GJ", "IN-KL", "IN-JH", "IN-DL", "IN-TR"]),
    nia(2022, 4, 16, "Bohag Bihu", &["IN-AS"]),
    nia(2022, 4, 21, "Garia Puja", &["IN-TR"]),
    nia(2022, 4, 29, "Shab-I-Qadr/Jumat-ul-Vida", &["IN-JK"]),
    nia(2022, 5, 2, "Ramjan-Eid (Eid-UI-Fitra)", &["IN-KA", "IN-KL"]),
    nia(2022, 5, 3, "Bhagvan Shree Parshuram Jayanti/Ramjan-Eid (Eid-UI-Fitra)/Basava Jayanti/Akshaya Tritiya", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-OD", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2022, 5, 3, "Ramjan-Eid (Eid-UI-Fitra)/Basava Jayanti/Akshaya Tritiya", &["IN-KA", "IN-KL"]),
    nia(2022, 5, 9, "Birthday of Rabindranath Tagore", &["IN-WB"]),
    nia(2022, 5, 16, "Buddha Purnima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR"]),
    nia(2022, 6, 2, "Maharana Pratap Jayanti", &["IN-HP"]),
    nia(2022, 6, 15, "Y.M.A. Day/Guru Hargobind Ji’s Birthday/Raja Sankranti", &["IN-OD", "IN-JK", "IN-MZ"]),
    nia(2022, 6, 30, "Remna Ni", &["IN-MZ"]),
    nia(2022, 7, 1, "Kang (Rathajatra)/Ratha Yatra", &["IN-OD", "IN-MN"]),
    nia(2022, 7, 7, "Kharchi Puja", &["IN-TR"]),
    nia(2022, 7, 9, "ld-Ul-Ad'ha (Bakrid)", &["IN-KL"]),
    nia(2022, 7, 11, "Eid-ul-Azha", &["IN-JK"]),
    nia(2022, 7, 13, "Bhanu Jayanti", &["IN-SK"]),
    nia(2022, 7, 14, "Beh Dienkhlam", &["IN-ML"]),
    nia(2022, 7, 16, "Harela", &["IN-UK"]),
    nia(2022, 7, 26, "Ker Puja", &["IN-TR"]),
    nia(2022, 8, 1, "Drukpa Tshe-zi", &["IN-SK"]),
    nia(2022, 8, 9, "Muharram (Ashoora)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-CG", "IN-TR", "IN-MZ"]),
    nia(2022, 8, 11, "Raksha Bandhan", &["IN-MP", "IN-RJ", "IN-GJ", "IN-UK", "IN-HP"]),
    nia(2022, 8, 12, "Raksha Bandhan", &["IN-UP"]),
    nia(2022, 8, 13, "Patriot’s Day", &["IN-MN"]),
    nia(2022, 8, 15, "Independence Day", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2022, 8, 16, "Parsi New Year (Shahenshahi)", &["IN-MH"]),
    nia(2022, 8, 18, "Janmashtami", &["IN-OD"]),
    nia(2022, 8, 19, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", &["IN-UP", "IN-BR", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-SK"]),
    nia(2022, 8, 20, "Sri Krishna Ashtami", &["IN-TS"]),
    nia(2022, 8, 29, "Tithi of Srimanta Sankardeva", &["IN-AS"]),
    nia(2022, 8, 31, "Samvatsari (Chaturthi Paksha)/Ganesh Chaturthi/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi", &["IN-MH", "IN-TN", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-GA"]),
    nia(2022, 9, 1, "Ganesh Ghaturthi (2nd day)", &["IN-GA"]),
    nia(2022, 9, 6, "Karma Puja", &["IN-JH"]),
    nia(2022, 9, 7, "First Onam", &["IN-KL"]),
    nia(2022, 9, 8, "Thiruvonam", &["IN-KL"]),
    nia(2022, 9, 9, "Indrajatra", &["IN-SK"]),
    nia(2022, 9, 10, "Sree Naravana Guru Javanthi", &["IN-KL"]),
    nia(2022, 9, 21, "Sree Narayana Guru Samadhi Day", &["IN-KL"]),
    nia(2022, 9, 26, "Navtatri Sthapna/Mera Chaoren Houba of Lainingthou Sanamahi/Agarsain Jayanti", &["IN-RJ", "IN-MN"]),
    nia(2022, 10, 1, "Half Yearly Closing of Bank Accounts", &["IN-SK"]),
    nia(2022, 10, 3, "Durga Puja (Maha Ashtami)", &["IN-BR", "IN-WB", "IN-OD", "IN-AS", "IN-JH", "IN-TR", "IN-MN"]),
    nia(2022, 10, 4, "Durga Puja/Dussehra (Maha Navami)/Ayudha pooja/Janmotsav of Srimanta Sankardeva", &["IN-UP", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-OD", "IN-KL", "IN-AS", "IN-JH", "IN-TR", "IN-ML", "IN-SK"]),
    nia(2022, 10, 5, "Durga Puja/Dussehra (Vijaya Dashmi)/Janmotsav of Srimanta Sankardeva", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2022, 10, 6, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2022, 10, 7, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2022, 10, 8, "Milad-i-Sherif/Eid-i-Milad-ul-Nabi (Birthday of Prophet Muhammed)", &["IN-MP", "IN-KL", "IN-JK"]),
    nia(2022, 10, 13, "Karva Chauth", &["IN-HP"]),
    nia(2022, 10, 14, "Friday following Eid-i-Milad-ul-Nabi", &["IN-JK"]),
    nia(2022, 10, 18, "Kati Bihu", &["IN-AS"]),
    nia(2022, 10, 24, "Kali Puja/Deepavali/Diwali (Laxmi Pujan)/Naraka Chaturdashi", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-GA", "IN-MZ"]),
    nia(2022, 10, 25, "Laxmi Puja/Deepawali/Govardhan Pooja", &["IN-RJ", "IN-MN", "IN-SK"]),
    nia(2022, 10, 26, "Govardhan Pooja/Vikram Samvant New Year Day/Bhai Bij/Bhai Duj/Diwali (Bali Pratipada)/Laxmi Puja/Accession Day", &["IN-UP", "IN-MH", "IN-KA", "IN-GJ", "IN-JK", "IN-UK", "IN-HP", "IN-SK"]),
    nia(2022, 10, 27, "Bhaidooj/Chitragupt Jayanti/Laxmi Puja/Deepawali/Ningol Chakkouba", &["IN-UP", "IN-MN", "IN-SK"]),
    nia(2022, 10, 31, "Sardar Vallabhbhai Patel’s Birthday/Surya Pashti Dala Chhath (Morning ardhya)", &["IN-GJ"]),
    nia(2022, 10, 31, "Surya Pashti Dala Chhath (Morning ardhya)/Chhath Puja", &["IN-BR", "IN-JH"]),
    nia(2022, 11, 1, "Kannada rajyotsava/Kut", &["IN-KA", "IN-MN"]),
    nia(2022, 11, 8, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-RJ", "IN-OD", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-MZ"]),
    nia(2022, 11, 11, "Kanakadasa Jayanthi", &["IN-KA"]),
    nia(2022, 11, 11, "Wangala Festival", &["IN-ML"]),
    nia(2022, 11, 23, "Seng Kutsnem", &["IN-ML"]),
    nia(2022, 12, 3, "Feast of St. Francis Xavier", &["IN-GA"]),
    nia(2022, 12, 12, "Pa-Togan Nengminja Sangma", &["IN-ML"]),
    nia(2022, 12, 19, "Goa Liberation Day", &["IN-GA"]),
    nia(2022, 12, 24, "Christmas Festival", &["IN-ML"]),
    nia(2022, 12, 26, "Christmas Celebration/Losoong/Namsoong", &["IN-ML", "IN-MZ"]),
    nia(2022, 12, 26, "Losoong/Namsoong", &["IN-SK"]),
    nia(2022, 12, 30, "U Kiang Nangbah", &["IN-ML"]),
    nia(2022, 12, 31, "New Year’s Eve", &["IN-MZ"]),
    nia(2023, 1, 2, "New Year’s Celebration", &["IN-MZ"]),
    nia(2023, 1, 3, "Imoinu Iratpa", &["IN-MN"]),
    nia(2023, 1, 4, "Gaan-Ngai", &["IN-MN"]),
    nia(2023, 1, 11, "Missionary Day", &["IN-MZ"]),
    nia(2023, 1, 12, "Birthday of Swami Vivekananda", &["IN-WB"]),
    nia(2023, 1, 14, "Makar Sankranti", &["IN-AR"]),
    nia(2023, 1, 16, "Thiruvalluvar Day", &["IN-TN"]),
    nia(2023, 1, 17, "Uzhavar Thirunal", &["IN-TN"]),
    nia(2023, 1, 23, "Netaji's Birth day", &["IN-WB"]),
    nia(2023, 1, 26, "Republic Day/Saraswati Puja (Shree Panchami)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2023, 2, 15, "Lui-Ngai-Ni", &["IN-MN"]),
    nia(2023, 2, 18, "Mahashivratri (Maha Vad-14)/Sivarathri", &["IN-UP", "IN-MH", "IN-MP", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP"]),
    nia(2023, 2, 20, "State Day/Statehood Day", &["IN-AR", "IN-MZ"]),
    nia(2023, 2, 21, "Losar", &["IN-SK"]),
    nia(2023, 3, 3, "Chapchar Kut", &["IN-MZ"]),
    nia(2023, 3, 7, "Holi (Second Day)/Holika Dahan/Dhulandi/Dol Jatra", &["IN-UP", "IN-JH", "IN-UK"]),
    nia(2023, 3, 7, "Holi/Holi (Second Day)/Holika Dahan/Dhulandi/Dol Jatra/Attukal Pongala", &["IN-MH", "IN-WB", "IN-RJ", "IN-TS", "IN-AS", "IN-JK", "IN-GA"]),
    nia(2023, 3, 8, "Holi/Holi 2nd Day - Dhuleti/Yaosang 2nd Day", &["IN-UP", "IN-BR", "IN-MP", "IN-GJ", "IN-OD", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2023, 3, 9, "Holi", &["IN-BR"]),
    nia(2023, 3, 22, "Gudi Padwa/Ugadi Festival/Bihar Divas/Sajibu Nongmapanba (Cheiraoba)/Telugu New Year's Day/1st Navratra", &["IN-MH", "IN-BR", "IN-TN", "IN-KA", "IN-TS", "IN-JK", "IN-MN", "IN-GA"]),
    nia(2023, 3, 24, "Sarhul", &["IN-JH"]),
    nia(2023, 3, 30, "Shree Ram Navami (Chaite Dashain)", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-RJ", "IN-GJ", "IN-OD", "IN-TS", "IN-JH", "IN-UK", "IN-HP", "IN-SK"]),
    nia(2023, 4, 1, "To enable Banks to close their yearly accounts", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-SK"]),
    nia(2023, 4, 3, "Mahavir Jayanti", &["IN-MP", "IN-RJ"]),
    nia(2023, 4, 4, "Mahavir Jayanti", &["IN-UP", "IN-MH", "IN-WB", "IN-TN", "IN-KA", "IN-GJ", "IN-JH", "IN-DL", "IN-CG", "IN-MZ"]),
    nia(2023, 4, 5, "Babu Jagjivan Ram’s Birthday", &["IN-TS"]),
    nia(2023, 4, 7, "Good Friday", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2023, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Bohag Bihu/Cheiraoba/Vaisakhi/Baisakhi/Tamil New Year's Day/Maha Bisubha Sankranti/Biju Festival/Buisu Festival", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2023, 4, 15, "Vishu/Bohag Bihu/Himachal Day/Bengali New Year’s Day (Nababarsha)", &["IN-WB", "IN-KL", "IN-AS", "IN-HP", "IN-TR", "IN-AR"]),
    nia(2023, 4, 18, "Shab-l-Qadr", &["IN-JK"]),
    nia(2023, 4, 21, "Id-Ul-Fitr (Ramzan Eid)/Garia Puja", &["IN-TR"]),
    nia(2023, 4, 21, "Id-Ul-Fitr (Ramzan Eid)/Garia Puja/Jumat-ul-Vida", &["IN-KL"]),
    nia(2023, 4, 21, "Id-Ul-Fitr (Ramzan Eid)/Jumat-ul-Vida", &["IN-JK"]),
    nia(2023, 4, 22, "Ramzan Eid (Eid-Ul-Fitr)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-AP", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR"]),
    nia(2023, 5, 1, "Maharashtra Day/May Day", &["IN-MH", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-KL", "IN-AS", "IN-MN", "IN-GA"]),
    nia(2023, 5, 5, "Buddha Purnima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-AR", "IN-MZ"]),
    nia(2023, 5, 9, "Birthday of Rabindranath Tagore", &["IN-WB"]),
    nia(2023, 5, 22, "Maharana Pratap Jayanti", &["IN-HP"]),
    nia(2023, 6, 15, "Y.M.A. Day/Raja Sankranti", &["IN-OD", "IN-MZ"]),
    nia(2023, 6, 20, "Kang (Rathajatra)/Ratha Yatra", &["IN-OD", "IN-MN"]),
    nia(2023, 6, 26, "Kharchi Puja", &["IN-TR"]),
    nia(2023, 6, 28, "Bakri Eid (Eid-Ul-Zuha)", &["IN-KL"]),
    nia(2023, 6, 29, "Bakri Eid (Eid-Ul-Adha)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-MZ"]),
    nia(2023, 6, 30, "Remna Ni/Id-Ul-Zuha", &["IN-OD", "IN-JK", "IN-MZ"]),
    nia(2023, 7, 5, "Guru Hargobind Ji's Birthday", &["IN-JK"]),
    nia(2023, 7, 6, "MHIP Day", &["IN-MZ"]),
    nia(2023, 7, 11, "Ker Puja", &["IN-TR"]),
    nia(2023, 7, 13, "Bhanu Jayanti", &["IN-SK"]),
    nia(2023, 7, 17, "U Tirot Sing Day", &["IN-ML"]),
    nia(2023, 7, 21, "Drukpa Tshe-zi", &["IN-SK"]),
    nia(2023, 7, 29, "Muharram (Tajiya)/Ashoora", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-AP", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-CG", "IN-HP", "IN-TR", "IN-MZ"]),
    nia(2023, 8, 8, "Tendong Lho Rum Faat", &["IN-SK"]),
    nia(2023, 8, 15, "Independence Day", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2023, 8, 16, "Parsi New Year (Shahenshahi)", &["IN-MH"]),
    nia(2023, 8, 18, "Tithi of Srimanta Sankardeva", &["IN-AS"]),
    nia(2023, 8, 28, "First Onam", &["IN-KL"]),
    nia(2023, 8, 29, "Thiruvonam", &["IN-KL"]),
    nia(2023, 8, 30, "Raksha Bandhan", &["IN-MP", "IN-RJ", "IN-GJ", "IN-HP"]),
    nia(2023, 8, 31, "Raksha Bandhan/Sree Narayana Guru Jayanthi/Pang-Lhabsol", &["IN-UP", "IN-UK", "IN-SK"]),
    nia(2023, 8, 31, "Sree Narayana Guru Jayanthi/Pang-Lhabsol", &["IN-KL"]),
    nia(2023, 9, 6, "Sri Krishna Janmashtami", &["IN-BR", "IN-TN", "IN-AP", "IN-OD"]),
    nia(2023, 9, 7, "Janmashtami (Shravan Vad-8)/Sri Krishna Ashtami", &["IN-UP", "IN-RJ", "IN-GJ", "IN-TS", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-SK"]),
    nia(2023, 9, 8, "G-20 Summit", &["IN-DL"]),
    nia(2023, 9, 18, "Varasiddhi Vinayaka Vrata/Vinayaka Chathurthi", &["IN-TN", "IN-KA", "IN-TS"]),
    nia(2023, 9, 19, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)", &["IN-MH", "IN-GJ", "IN-OD", "IN-GA"]),
    nia(2023, 9, 20, "Ganesh Chaturthi (2nd day)", &["IN-GA"]),
    nia(2023, 9, 20, "Nuakhai", &["IN-OD"]),
    nia(2023, 9, 22, "Sree Narayana Guru Samadhi Day", &["IN-KL"]),
    nia(2023, 9, 23, "Birthday of Maharaja Hari Singh Ji", &["IN-JK"]),
    nia(2023, 9, 25, "Janmotsav of Srimanta Sankardeva/Karma Puja", &["IN-AS", "IN-JH"]),
    nia(2023, 9, 28, "Eid-E-Milad/Eid-e-Meeladunnabi - (Prophet Mohammad's Birthday) (Bara Vafat)", &["IN-UP", "IN-TN", "IN-KA", "IN-GJ", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-MN", "IN-MZ"]),
    nia(2023, 9, 29, "Friday following Eid-i-Milad-ul-Nabi/Eid-E-Milad", &["IN-JK"]),
    nia(2023, 9, 29, "Indrajatra/Eid-E-Milad", &["IN-SK"]),
    nia(2023, 9, 29, "Indrajatra/Friday following Eid-i-Milad-ul-Nabi/Eid-E-Milad", &["IN-MH"]),
    nia(2023, 10, 2, "Mahatma Gandhi Jayanti", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2023, 10, 14, "Mahalaya", &["IN-WB"]),
    nia(2023, 10, 18, "Kati Bihu", &["IN-AS"]),
    nia(2023, 10, 21, "Durga Puja (Maha Saptami)", &["IN-WB", "IN-AS", "IN-TR", "IN-MN"]),
    nia(2023, 10, 23, "Dusshera (Mahanavami)/Ayudha Pooja/Durga Puja/Vijaya Dasami", &["IN-UP", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-TR", "IN-ML", "IN-NL", "IN-AR"]),
    nia(2023, 10, 24, "Dussehra/Dusshera (Vijaya Dashmi)/Durga Puja", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-OD", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2023, 10, 25, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2023, 10, 26, "Accession Day", &["IN-JK"]),
    nia(2023, 10, 26, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2023, 10, 27, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2023, 10, 28, "Lakshmi Puja", &["IN-WB"]),
    nia(2023, 10, 31, "Sardar Vallabhbhai Patel’s Birthday", &["IN-GJ"]),
    nia(2023, 11, 1, "Kannada Rajyothsava/Kut/Karva Chauth", &["IN-KA", "IN-HP", "IN-MN"]),
    nia(2023, 11, 2, "Nongkrem Dance", &["IN-ML"]),
    nia(2023, 11, 10, "Wangala Festival", &["IN-ML"]),
    nia(2023, 11, 13, "Govardhan Pooja/Laxmi Puja (Deepawali)/Diwali", &["IN-UP", "IN-RJ", "IN-AS", "IN-UK", "IN-CG", "IN-TR", "IN-MN", "IN-SK"]),
    nia(2023, 11, 14, "Diwali (Bali Pratipada)/Deepavali/Vikram Samvant New Year Day/Laxmi Puja", &["IN-MH", "IN-KA", "IN-GJ", "IN-SK"]),
    nia(2023, 11, 15, "Bhaidooj/Chitragupt Jayanti/Laxmi Puja (Deepawali)/Ningol Chakkouba/Bhratridwitiya", &["IN-UP", "IN-WB", "IN-HP", "IN-MN", "IN-SK"]),
    nia(2023, 11, 20, "Chhath (Morning Arghya)", &["IN-BR", "IN-JH"]),
    nia(2023, 11, 23, "Seng Kutsnem/Egaas-Bagwaal", &["IN-UK", "IN-ML"]),
    nia(2023, 11, 27, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-RJ", "IN-OD", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-NL", "IN-AR", "IN-MZ"]),
    nia(2023, 11, 30, "Kanakadasa Jayanthi/Telangana Legislative Assembly General Elections 2023", &["IN-KA", "IN-TS"]),
    nia(2023, 12, 1, "State Inauguration Day/Indigenous Faith day", &["IN-NL", "IN-AR"]),
    nia(2023, 12, 4, "Cyclone Michaung", &["IN-TN"]),
    nia(2023, 12, 4, "Feast of St. Francis Xavier/Cyclone Michaung", &["IN-GA"]),
    nia(2023, 12, 5, "Cyclone Michaung", &["IN-TN"]),
    nia(2023, 12, 12, "Pa-Togan Nengminja Sangma", &["IN-ML"]),
    nia(2023, 12, 13, "Losoong/Namsoong", &["IN-SK"]),
    nia(2023, 12, 14, "Losoong/Namsoong", &["IN-SK"]),
    nia(2023, 12, 18, "Death Anniversary of U SoSo Tham", &["IN-ML"]),
    nia(2023, 12, 19, "Goa Liberation Day", &["IN-GA"]),
    nia(2023, 12, 25, "Christmas", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2023, 12, 26, "Christmas Celebration", &["IN-ML", "IN-NL", "IN-MZ"]),
    nia(2023, 12, 27, "Christmas", &["IN-NL"]),
    nia(2023, 12, 30, "U Kiang Nangbah", &["IN-ML"]),
    nia(2024, 1, 1, "New Year’s Day", &["IN-WB", "IN-TN", "IN-ML", "IN-MN", "IN-NL", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2024, 1, 2, "New Year Celebration", &["IN-MZ"]),
    nia(2024, 1, 11, "Missionary Day", &["IN-MZ"]),
    nia(2024, 1, 12, "Birth Day of Swami Vivekananda", &["IN-WB"]),
    nia(2024, 1, 15, "Uttarayana Punyakala/Makara Sankranti Festival/Maghe Sankranti/Pongal/Magh Bihu", &["IN-TN", "IN-KA", "IN-AP", "IN-OD", "IN-TS", "IN-AS", "IN-SK"]),
    nia(2024, 1, 16, "Thiruvalluvar Day", &["IN-TN"]),
    nia(2024, 1, 17, "Uzhavar Thirunal/Sri Guru Gobind Singh Ji Birthday", &["IN-TN"]),
    nia(2024, 1, 22, "Consecration ceremony of Lord Shri Ram in the temple built in Shri Ram Janmabhoomi complex/Imoinu Iratpa", &["IN-UP", "IN-HP", "IN-MN", "IN-GA"]),
    nia(2024, 1, 23, "Birthday of Netaji Subhas Chandra Bose", &["IN-WB", "IN-TR"]),
    nia(2024, 1, 23, "Gaan-Ngai", &["IN-MN"]),
    nia(2024, 1, 25, "Birthday of Md. Hazarat Ali", &["IN-UP"]),
    nia(2024, 1, 25, "Thai Poosam", &["IN-TN"]),
    nia(2024, 1, 26, "Republic Day", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2024, 2, 10, "Losar", &["IN-SK"]),
    nia(2024, 2, 14, "Basanta Panchami/Saraswati Puja (Shree Panchami)", &["IN-WB", "IN-OD", "IN-TR"]),
    nia(2024, 2, 15, "Lui-Ngai-Ni", &["IN-MN"]),
    nia(2024, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti", &["IN-MH"]),
    nia(2024, 2, 20, "State Day/Statehood Day", &["IN-AR", "IN-MZ"]),
    nia(2024, 2, 26, "Nyokum", &["IN-AR"]),
    nia(2024, 3, 1, "Chapchar Kut", &["IN-MZ"]),
    nia(2024, 3, 8, "Mahashivratri (Maha vad-13)/Sivarathri", &["IN-UP", "IN-MH", "IN-MP", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP"]),
    nia(2024, 3, 22, "Bihar Divas", &["IN-BR"]),
    nia(2024, 3, 25, "Holi (Second Day) - Dhuleti/Dol Jatra/Dhulandi", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-RJ", "IN-GJ", "IN-AP", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2024, 3, 26, "Yaosang 2nd Day/Holi", &["IN-BR", "IN-OD", "IN-MN"]),
    nia(2024, 3, 27, "Holi", &["IN-BR"]),
    nia(2024, 3, 29, "Good Friday", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2024, 4, 1, "To enable to Banks to close their yearly accounts", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-MN", "IN-NL", "IN-GA", "IN-AR"]),
    nia(2024, 4, 5, "Babu Jagjivan Ram’s Birthday", &["IN-TS"]),
    nia(2024, 4, 5, "Jumat-ul-Vida", &["IN-JK"]),
    nia(2024, 4, 9, "Gudhi Padwa/Ugadi Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st Navratra", &["IN-MH", "IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-JK", "IN-MN", "IN-GA"]),
    nia(2024, 4, 10, "Ramzan-Id (Id-Ul-Fitr)", &["IN-KL"]),
    nia(2024, 4, 11, "Ramzan-Id (Id-Ul-Fitr) (1st Shawaal)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ"]),
    nia(2024, 4, 13, "Bohag Bihu/Cheiraoba/Baisakhi/Biju Festival", &["IN-AS", "IN-JK", "IN-TR", "IN-MN"]),
    nia(2024, 4, 15, "Bohag Bihu/Himachal Day", &["IN-AS"]),
    nia(2024, 4, 15, "Bohag Bihu/Himachal Day/Shad Suk Mynsiem", &["IN-HP"]),
    nia(2024, 4, 15, "Himachal Day/Shad Suk Mynsiem", &["IN-ML"]),
    nia(2024, 4, 17, "Shree Ram Navami (Chaite Dasain)", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-RJ", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-JH", "IN-UK", "IN-CG", "IN-HP", "IN-SK"]),
    nia(2024, 4, 20, "Garia Puja", &["IN-TR"]),
    nia(2024, 4, 26, "Lok Sabha General Elections 2024", &["IN-KL"]),
    nia(2024, 5, 1, "Maharashtra Din/May Day (Labour Day)", &["IN-MH", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-KL", "IN-AS", "IN-MN", "IN-GA"]),
    nia(2024, 5, 8, "Birthday of Rabindranath Tagore", &["IN-WB"]),
    nia(2024, 5, 10, "Basava Jayanti/Akshaya Tritiya", &["IN-KA"]),
    nia(2024, 5, 16, "State Day", &["IN-SK"]),
    nia(2024, 5, 23, "Buddha Pournima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-AR", "IN-MZ"]),
    nia(2024, 5, 25, "Nazrul Jayanti/Lok Sabha General Elections 2024", &["IN-OD", "IN-TR"]),
    nia(2024, 5, 30, "Incessant rains due to the effect of Cyclone Remal and subsequent flood", &["IN-MN"]),
    nia(2024, 5, 31, "Incessant rains due to the effect of Cyclone Remal and subsequent flood", &["IN-MN"]),
    nia(2024, 6, 15, "YMA Day/Raja Sankranti", &["IN-OD", "IN-MZ"]),
    nia(2024, 6, 17, "Bakri ID (Id-Uz-Zuha)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA"]),
    nia(2024, 6, 18, "Bakri ID (Id-Uz-Zuha)", &["IN-JK"]),
    nia(2024, 6, 22, "Guru Hargobind Ji’s Birthday", &["IN-JK"]),
    nia(2024, 7, 3, "Beh Dienkhlam/Incessant rains", &["IN-ML"]),
    nia(2024, 7, 3, "Incessant rains", &["IN-MN"]),
    nia(2024, 7, 5, "Incessant rains", &["IN-MN"]),
    nia(2024, 7, 6, "MHIP Day", &["IN-MZ"]),
    nia(2024, 7, 8, "Kang (Rathajatra)/Ratha Yatra", &["IN-OD", "IN-MN"]),
    nia(2024, 7, 9, "Drukpa Tshe-zi", &["IN-SK"]),
    nia(2024, 7, 16, "Harela", &["IN-UK"]),
    nia(2024, 7, 17, "Muharram/Ashoora/U Tirot Sing Day", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-AP", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MZ"]),
    nia(2024, 7, 20, "Kharchi Puja", &["IN-TR"]),
    nia(2024, 8, 3, "Ker Puja", &["IN-TR"]),
    nia(2024, 8, 8, "Tendong Lho Rum Faat", &["IN-SK"]),
    nia(2024, 8, 13, "Patriot’s Day", &["IN-MN"]),
    nia(2024, 8, 15, "Independence Day", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2024, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", &["IN-MH"]),
    nia(2024, 8, 19, "Raksha Bandhan/Jhulana Purnima/Birthday of Bir Bikram Kishore Manikya Bahadur", &["IN-UP", "IN-MP", "IN-RJ", "IN-GJ", "IN-OD", "IN-UK", "IN-HP", "IN-TR"]),
    nia(2024, 8, 20, "Sree Narayana Guru Jayanthi", &["IN-KL"]),
    nia(2024, 8, 26, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-SK"]),
    nia(2024, 9, 4, "Tirubhav Tithi of Srimanta Sankardeva", &["IN-AS"]),
    nia(2024, 9, 7, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi", &["IN-MH", "IN-TN", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-GA"]),
    nia(2024, 9, 9, "Nuakhai", &["IN-OD"]),
    nia(2024, 9, 14, "First Onam", &["IN-KL"]),
    nia(2024, 9, 14, "Karma Puja", &["IN-JH"]),
    nia(2024, 9, 16, "Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)", &["IN-UP", "IN-KA", "IN-GJ", "IN-AP", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-MN", "IN-MZ"]),
    nia(2024, 9, 17, "Indrajatra/Id-e-Milad (Milad-Un-Nabi)", &["IN-TN", "IN-SK"]),
    nia(2024, 9, 18, "Pang-Lhabsol/Id-E-Milad/Unitarian Anniversary Day", &["IN-SK"]),
    nia(2024, 9, 18, "Pang-Lhabsol/Unitarian Anniversary Day", &["IN-ML"]),
    nia(2024, 9, 20, "Friday following Eid-i-Milad-ul-Nabi", &["IN-JK"]),
    nia(2024, 9, 21, "Sree Narayana Guru Samadhi Day", &["IN-KL"]),
    nia(2024, 9, 23, "Birthday of Maharaja Hari Singh Ji", &["IN-JK"]),
    nia(2024, 10, 2, "Mahatma Gandhi Jayanti", &["IN-UP", "IN-MH", "IN-BR", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2024, 10, 2, "Mahatma Gandhi Jayanti/Mahalaya Amavasye", &["IN-WB", "IN-KA", "IN-TR"]),
    nia(2024, 10, 3, "Navratra Sthapna/Maharaja Agrasen Jayanti", &["IN-RJ"]),
    nia(2024, 10, 10, "Durga Puja/Dussehra (Maha Saptami)", &["IN-WB", "IN-AS", "IN-TR", "IN-NL"]),
    nia(2024, 10, 11, "Dusshera (Mahashtami/Mahanavami)/Ayudha Pooja/Durga Puja (Dasain)/Durga Ashtami", &["IN-UP", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-OD", "IN-KL", "IN-AS", "IN-JH", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-AR", "IN-SK"]),
    nia(2024, 10, 12, "Dasara/Dussehra (Mahanavami/Vijayadashmi)/Durga Puja (Dasain)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-GA", "IN-AR", "IN-SK"]),
    nia(2024, 10, 14, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2024, 10, 15, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2024, 10, 16, "Lakshmi Puja", &["IN-WB", "IN-TR"]),
    nia(2024, 10, 17, "Maharshi Valmiki Jayanti/Kati Bihu", &["IN-KA", "IN-AS", "IN-HP"]),
    nia(2024, 10, 24, "*The office of Reserve Bank of India, New Delhi will close at 1400 hours on October 24, 2024 (Thursday)", &["IN-DL"]),
    nia(2024, 10, 26, "Accession Day", &["IN-JK"]),
    nia(2024, 10, 31, "Diwali (Deepavali)/Kali Puja/Naraka Chaturdashi", &["IN-KA"]),
    nia(2024, 10, 31, "Diwali (Deepavali)/Kali Puja/Sardar Vallabhbhai Patel's Birthday/Naraka Chaturdashi", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-NL", "IN-GA", "IN-AR", "IN-MZ"]),
    nia(2024, 11, 1, "Diwali Amavasya (Laxmi Pujan)/Deepawali/Kut/Kannada Rajyothsava/Govardhan Pooja", &["IN-UP", "IN-MH", "IN-MP", "IN-KA", "IN-AS", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-ML", "IN-MN", "IN-SK"]),
    nia(2024, 11, 2, "Diwali (Bali Pratipada)/Balipadyami/Laxmi Puja (Deepawali)/Govardhan Pooja/Vikram Samvant New Year Day", &["IN-UP", "IN-MH", "IN-RJ", "IN-KA", "IN-GJ", "IN-UK", "IN-SK"]),
    nia(2024, 11, 7, "Chhath (Evening Arghya)", &["IN-BR", "IN-WB", "IN-JH"]),
    nia(2024, 11, 8, "Chhath (Morning Arghya)", &["IN-BR", "IN-JH"]),
    nia(2024, 11, 8, "Wangala Festival", &["IN-ML"]),
    nia(2024, 11, 12, "Egaas-Bagwaal/Nongkrem Dance", &["IN-UK", "IN-ML"]),
    nia(2024, 11, 15, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-RJ", "IN-OD", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-NL", "IN-AR", "IN-MZ"]),
    nia(2024, 11, 18, "Kanakadasa Jayanti", &["IN-KA"]),
    nia(2024, 11, 20, "Assembly General Election, 2024", &["IN-MH"]),
    nia(2024, 11, 23, "Seng Kutsnem", &["IN-ML"]),
    nia(2024, 12, 3, "Feast of St. Francis Xavier", &["IN-GA"]),
    nia(2024, 12, 12, "Pa-Togan Nengminja Sangma", &["IN-ML"]),
    nia(2024, 12, 18, "Death Anniversary of U SoSo Tham", &["IN-ML"]),
    nia(2024, 12, 19, "Goa Liberation Day", &["IN-GA"]),
    nia(2024, 12, 24, "Christmas Eve", &["IN-ML", "IN-NL", "IN-MZ"]),
    nia(2024, 12, 25, "Christmas", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2024, 12, 26, "Christmas Celebration", &["IN-ML", "IN-NL", "IN-MZ"]),
    nia(2024, 12, 27, "As a mark of respect to Late Dr. Manmohan Singh, the former Prime Minister of India", &["IN-KA"]),
    nia(2024, 12, 27, "Christmas Celebration", &["IN-NL"]),
    nia(2024, 12, 30, "U Kiang Nangbah", &["IN-ML"]),
    nia(2024, 12, 31, "New Year’s Eve/Lossong/Namsoong", &["IN-MZ", "IN-SK"]),
    nia(2025, 1, 1, "New Year’s Day/Loosong/Namsoong", &["IN-WB", "IN-TN", "IN-ML", "IN-MN", "IN-NL", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2025, 1, 2, "Loosong/Namsoong/New Year Celebration", &["IN-MZ", "IN-SK"]),
    nia(2025, 1, 11, "Imoinu Iratpa", &["IN-MN"]),
    nia(2025, 1, 11, "Missionary Day", &["IN-MZ"]),
    nia(2025, 1, 14, "Makar Sankranti/Uttarayana Punyakala/Pongal/Maghe Sankranti/Magh Bihu/Birthday of Hazarat Ali", &["IN-UP", "IN-TN", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-AS", "IN-AR", "IN-SK"]),
    nia(2025, 1, 15, "Thiruvalluvar Day", &["IN-TN"]),
    nia(2025, 1, 16, "Uzhavar Thirunal", &["IN-TN"]),
    nia(2025, 1, 23, "Birthday of Netaji Subhas Chandra Bose/Vir Surendrasai Jayanti/General Elections to the Municipal Local Bodies", &["IN-WB", "IN-OD", "IN-UK", "IN-TR"]),
    nia(2025, 2, 3, "Saraswati Puja", &["IN-TR"]),
    nia(2025, 2, 11, "Thai Poosam/Municipal Corporation General Election 2025", &["IN-TN", "IN-CG"]),
    nia(2025, 2, 12, "Sant Ravidas Jayanti/Guru Ravi Das’s Birthday/General Election to Local Councils 2025", &["IN-UP", "IN-HP", "IN-MZ"]),
    nia(2025, 2, 15, "Lui-Ngai-Ni", &["IN-MN"]),
    nia(2025, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti", &["IN-MH"]),
    nia(2025, 2, 20, "Statehood Day/State Day", &["IN-AR", "IN-MZ"]),
    nia(2025, 2, 26, "Mahashivratri", &["IN-UP", "IN-MH", "IN-MP", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-MZ"]),
    nia(2025, 2, 28, "Losar", &["IN-SK"]),
    nia(2025, 3, 7, "Chapchar Kut", &["IN-MZ"]),
    nia(2025, 3, 13, "Holika Dahan", &["IN-UP", "IN-JH", "IN-UK"]),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-RJ", "IN-GJ", "IN-AP", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2025, 3, 15, "Holi/Yaosang 2nd Day", &["IN-BR", "IN-OD", "IN-JH", "IN-TR", "IN-MN"]),
    nia(2025, 3, 22, "Bihar Diwas", &["IN-BR"]),
    nia(2025, 3, 27, "Shab-I-Qadr", &["IN-JK"]),
    nia(2025, 3, 28, "Jumat-ul-Vida", &["IN-JK"]),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-SK"]),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-DL", "IN-JK", "IN-UK", "IN-TR", "IN-MN", "IN-NL", "IN-GA", "IN-AR"]),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", &["IN-JH", "IN-SK"]),
    nia(2025, 4, 5, "Babu Jagjivan Ram’s Birthday", &["IN-TS"]),
    nia(2025, 4, 7, "Shad Suk Mynsiem", &["IN-ML"]),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-JH", "IN-DL", "IN-CG", "IN-MZ"]),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-MN", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2025, 4, 15, "Bengali New Year’s Day/Himachal Day/Bohag Bihu", &["IN-WB", "IN-AS", "IN-HP", "IN-TR", "IN-AR"]),
    nia(2025, 4, 16, "Bohag Bihu", &["IN-AS"]),
    nia(2025, 4, 18, "Good Friday", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2025, 4, 21, "Garia Puja", &["IN-TR"]),
    nia(2025, 4, 29, "Bhagvan Shri Parshuram Jayanti", &["IN-HP"]),
    nia(2025, 4, 30, "Basava Jayanti/Akshaya Tritiya", &["IN-KA"]),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", &["IN-MH", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-KL", "IN-AS", "IN-MN", "IN-GA"]),
    nia(2025, 5, 9, "Birthday of Rabindranath Tagore", &["IN-WB"]),
    nia(2025, 5, 12, "Buddha Pournima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-AR", "IN-MZ"]),
    nia(2025, 5, 16, "State Day", &["IN-SK"]),
    nia(2025, 5, 26, "Birthday of Kazi Nazrul Islam", &["IN-TR"]),
    nia(2025, 5, 29, "Maharana Pratap Jayanti", &["IN-HP"]),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-MZ"]),
    nia(2025, 6, 11, "Saga Dawa", &["IN-SK"]),
    nia(2025, 6, 11, "Sant Guru Kabir Jayanti", &["IN-HP"]),
    nia(2025, 6, 27, "Ratha Yatra/Kang (Rathajatra)", &["IN-OD", "IN-MN"]),
    nia(2025, 6, 30, "Remna Ni", &["IN-MZ"]),
    nia(2025, 7, 3, "Kharchi Puja", &["IN-TR"]),
    nia(2025, 7, 5, "Guru Hargobind Ji’s Birthday", &["IN-JK"]),
    nia(2025, 7, 14, "Beh Deinkhlam", &["IN-ML"]),
    nia(2025, 7, 16, "Harela", &["IN-UK"]),
    nia(2025, 7, 17, "Death Anniversary of U Tirot Singh", &["IN-ML"]),
    nia(2025, 7, 19, "Ker Puja", &["IN-TR"]),
    nia(2025, 7, 22, "Demise of Shri V.S.Achuthanandan, former Chief Minister of Kerala", &["IN-KL"]),
    nia(2025, 7, 28, "Drukpa Tshe-zi", &["IN-SK"]),
    nia(2025, 8, 8, "Tendong Lho Rum Faat", &["IN-SK"]),
    nia(2025, 8, 9, "Raksha Bandhan/Jhulana Purnima", &["IN-UP", "IN-MP", "IN-RJ", "IN-GJ", "IN-OD", "IN-UK", "IN-HP"]),
    nia(2025, 8, 13, "Patriot’s Day", &["IN-MN"]),
    nia(2025, 8, 15, "Independence Day", &["IN-WB", "IN-KA", "IN-TS", "IN-KL", "IN-AS", "IN-DL", "IN-TR", "IN-MN", "IN-GA", "IN-MZ", "IN-SK"]),
    nia(2025, 8, 15, "Independence Day/Janmashtami", &["IN-UP", "IN-BR", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-AP", "IN-OD", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-NL", "IN-AR"]),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", &["IN-MH"]),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-AP", "IN-TS", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-ML", "IN-MZ", "IN-SK"]),
    nia(2025, 8, 19, "Birthday of Maharaja Bir Bikram Kishore Manikya Bahadur", &["IN-TR"]),
    nia(2025, 8, 25, "Tirubhav Tithi of Srimanta Sankardeva", &["IN-AS"]),
    nia(2025, 8, 27, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Ganesh Puja/Vinayakar Chathurthi", &["IN-MH", "IN-TN", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-GA"]),
    nia(2025, 8, 28, "Ganesh Chaturthi (2nd Day)", &["IN-GA"]),
    nia(2025, 8, 28, "Nuakhai", &["IN-OD"]),
    nia(2025, 9, 3, "Karma Puja", &["IN-JH"]),
    nia(2025, 9, 4, "First Onam", &["IN-KL"]),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Milad-i-Sherif", &["IN-UP", "IN-KA", "IN-GJ", "IN-AP", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-MN", "IN-MZ"]),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", &["IN-MP", "IN-TN", "IN-KL"]),
    nia(2025, 9, 6, "Id-e-Milad (Milad-Un-Nabi)/Indrajatra", &["IN-SK"]),
    nia(2025, 9, 12, "Friday following Eid-i-Milad-ul-Nabi", &["IN-JK"]),
    nia(2025, 9, 18, "Unitarian Anniversary Day", &["IN-ML"]),
    nia(2025, 9, 22, "Navratra Sthapna", &["IN-RJ"]),
    nia(2025, 9, 23, "Birthday of Maharaja Hari Singh Ji", &["IN-JK"]),
    nia(2025, 9, 29, "Maha Saptami/Durga Puja", &["IN-WB", "IN-AS", "IN-TR"]),
    nia(2025, 9, 30, "Maha Ashtami/Durga Ashtami/Durga Puja", &["IN-BR", "IN-WB", "IN-RJ", "IN-OD", "IN-AS", "IN-JH", "IN-TR", "IN-MN"]),
    nia(2025, 10, 1, "Navaratri Ends/Maha Navami/Dussehra/Ayudhapooja, Vijayadasami/Durga Puja (Dasain)", &["IN-UP", "IN-BR", "IN-WB", "IN-TN", "IN-KA", "IN-OD", "IN-KL", "IN-AS", "IN-JH", "IN-UK", "IN-TR", "IN-ML", "IN-NL", "IN-AR", "IN-SK"]),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", &["IN-SK"]),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Janmotsav of Sri Sri Sankardeva", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ"]),
    nia(2025, 10, 3, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2025, 10, 4, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2025, 10, 6, "Lakshmi Puja", &["IN-WB", "IN-TR"]),
    nia(2025, 10, 7, "Maharshi Valmiki Jayanti/Kumar Purnima", &["IN-KA", "IN-OD", "IN-HP"]),
    nia(2025, 10, 10, "Karva Chauth", &["IN-HP"]),
    nia(2025, 10, 18, "Kati Bihu", &["IN-AS"]),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", &["IN-UP", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2025, 10, 21, "Diwali Amavasya (Laxmi Pujan)/Deepawali/Govardhan Pooja", &["IN-MH", "IN-MP", "IN-AS", "IN-JK", "IN-CG", "IN-MN", "IN-SK"]),
    nia(2025, 10, 22, "Diwali (Bali Pratipada)/Vikram Samvant New Year Day/Govardhan Pooja/Balipadyami, Laxmi Puja (Deepawali)", &["IN-UP", "IN-MH", "IN-RJ", "IN-KA", "IN-GJ", "IN-UK", "IN-SK"]),
    nia(2025, 10, 23, "Bhai Bij/Bhaidooj/Chitragupt Jayanti/Laxmi Puja (Deepawali)/Bhratridwitiya/Ningol Chakkouba", &["IN-UP", "IN-WB", "IN-GJ", "IN-HP", "IN-MN", "IN-SK"]),
    nia(2025, 10, 27, "Chath Puja (Evening Puja)", &["IN-BR", "IN-WB", "IN-JH"]),
    nia(2025, 10, 28, "Chath Puja (Morning Puja)", &["IN-BR", "IN-JH"]),
    nia(2025, 10, 31, "Sardar Vallabhbhai Patel's Birthday", &["IN-GJ"]),
    nia(2025, 11, 1, "Kannada Rajyothsava/Igas-Bagwal", &["IN-KA", "IN-UK"]),
    nia(2025, 11, 5, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", &["IN-UP", "IN-MH", "IN-WB", "IN-MP", "IN-RJ", "IN-OD", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-NL", "IN-AR", "IN-MZ"]),
    nia(2025, 11, 6, "Nongkrem Dance/Bihar Legislative Assembly General Election, 2025", &["IN-BR", "IN-ML"]),
    nia(2025, 11, 7, "Wangala Festival", &["IN-ML"]),
    nia(2025, 11, 8, "Kanakadasa Jayanthi", &["IN-KA"]),
    nia(2025, 11, 15, "Birsa Munda Birth Anniversary / State Formation Day", &["IN-JH"]),
    nia(2025, 12, 1, "State Inauguration Day/Indigenous Faith Day", &["IN-NL", "IN-AR"]),
    nia(2025, 12, 3, "Feast of St. Francis Xavier", &["IN-GA"]),
    nia(2025, 12, 9, "General election to Local Government Institutions 2025", &["IN-KL"]),
    nia(2025, 12, 12, "Death Anniversary of Pa Togan Nengminja Sangma", &["IN-ML"]),
    nia(2025, 12, 18, "Death Anniversary of U SoSo Tham", &["IN-ML"]),
    nia(2025, 12, 19, "Goa Liberation Day", &["IN-GA"]),
    nia(2025, 12, 20, "Losoong / Namsoong", &["IN-SK"]),
    nia(2025, 12, 22, "Losoong / Namsoong", &["IN-SK"]),
    nia(2025, 12, 23, "Holiday for Bank employees in Nagaland (notification No. GAB/GEN/16/2010)", &["IN-NL"]),
    nia(2025, 12, 24, "Christmas Eve", &["IN-ML", "IN-NL", "IN-MZ"]),
    nia(2025, 12, 25, "Christmas", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2025, 12, 26, "Christmas Celebration", &["IN-ML", "IN-NL", "IN-MZ"]),
    nia(2025, 12, 27, "Christmas", &["IN-NL"]),
    nia(2025, 12, 30, "Death Anniversary of U Kiang Nangbah", &["IN-ML"]),
    nia(2025, 12, 31, "Imoinu Iratpa", &["IN-MN"]),
    nia(2025, 12, 31, "New Year's Eve", &["IN-MZ"]),
    nia(2026, 1, 1, "New Year’s Day", &["IN-WB", "IN-TN", "IN-ML", "IN-NL", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 1, 1, "New Year’s Day/Gaan-Ngai", &["IN-MN"]),
    nia(2026, 1, 2, "New Year Celebration/Mannam Jayanthi", &["IN-KL", "IN-MZ"]),
    nia(2026, 1, 3, "Birthday of Hazrat Ali", &["IN-UP"]),
    nia(2026, 1, 12, "Birth Day of Swami Vivekananda", &["IN-WB"]),
    nia(2026, 1, 14, "Makar Sankranti/Magh Bihu", &["IN-GJ", "IN-OD", "IN-AS", "IN-AR"]),
    nia(2026, 1, 15, "Election to Municipal Corporations in Maharashtra", &["IN-MH"]),
    nia(2026, 1, 15, "Uttarayana Punyakala/Pongal/Maghe Sankranti/Makara Sankranti/Election to Municipal Corporations in Maharashtra", &["IN-UP", "IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-SK"]),
    nia(2026, 1, 16, "Thiruvalluvar Day/Kanuma", &["IN-TN", "IN-AP"]),
    nia(2026, 1, 17, "Uzhavar Thirunal", &["IN-TN"]),
    nia(2026, 1, 23, "Birthday of Netaji Subhas Chandra Bose/Saraswati Puja (Shree Panchami)/Vir Surendrasai Jayanti/Basanta Panchami", &["IN-WB", "IN-OD", "IN-TR"]),
    nia(2026, 1, 26, "Republic Day", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 2, 18, "Losar", &["IN-SK"]),
    nia(2026, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti", &["IN-MH"]),
    nia(2026, 2, 20, "State Day/Statehood Day", &["IN-AR", "IN-MZ"]),
    nia(2026, 3, 2, "Holika Dahan", &["IN-UP"]),
    nia(2026, 3, 3, "Holi (Second Day)", &["IN-MH"]),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan", &["IN-UP", "IN-JH", "IN-UK"]),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", &["IN-BR", "IN-WB", "IN-MP", "IN-RJ", "IN-AP", "IN-TS", "IN-AS", "IN-GA"]),
    nia(2026, 3, 4, "Holi/Holi 2nd Day – Dhuleti/Yaosang 2nd Day/Dol Jatra", &["IN-UP", "IN-BR", "IN-MP", "IN-GJ", "IN-OD", "IN-AS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 3, 13, "Chapchar Kut", &["IN-MZ"]),
    nia(2026, 3, 13, "Jumat-ul-Vida", &["IN-JK"]),
    nia(2026, 3, 17, "Shab-I-Qadr", &["IN-JK"]),
    nia(2026, 3, 19, "Gudhi Padwa", &["IN-MH"]),
    nia(2026, 3, 19, "Gudhi Padwa/Ugadi Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st Navratra", &["IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-JK", "IN-MN", "IN-GA"]),
    nia(2026, 3, 20, "Eid-Ul-Fitr (Ramzan)", &["IN-KL"]),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)", &["IN-MH"]),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", &["IN-JK"]),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-AS", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 3, 26, "Shree Ram Navami", &["IN-UP", "IN-MH", "IN-WB", "IN-RJ", "IN-GJ", "IN-UK", "IN-HP", "IN-MZ"]),
    nia(2026, 3, 27, "Shree Ram Navami (Chaite Dasain)", &["IN-BR", "IN-MP", "IN-AP", "IN-OD", "IN-TS", "IN-JH", "IN-SK"]),
    nia(2026, 3, 30, "Mahavir Jayanti", &["IN-MP", "IN-KA"]),
    nia(2026, 3, 31, "Mahavir Janmakalyanak", &["IN-MH"]),
    nia(2026, 3, 31, "Mahavir Janmakalyanak/Mahavir Jayanti", &["IN-UP", "IN-BR", "IN-WB", "IN-TN", "IN-RJ", "IN-GJ", "IN-JH", "IN-DL", "IN-CG"]),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-TR", "IN-MN", "IN-GA", "IN-AR"]),
    nia(2026, 4, 2, "Maundy Thursday", &["IN-KL"]),
    nia(2026, 4, 3, "Good Friday", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 4, 9, "General Election to Kerala Legislative Assembly 2026", &["IN-KL"]),
    nia(2026, 4, 13, "Shad Suk Mynsiem", &["IN-ML"]),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti", &["IN-MH"]),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-HP", "IN-TR", "IN-MN", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 4, 15, "Bengali New Year’s Day (Nababarsha)/Bohag Bihu/Vishu/Himachal Day", &["IN-WB", "IN-KL", "IN-AS", "IN-HP", "IN-TR", "IN-AR"]),
    nia(2026, 4, 16, "Bohag Bihu", &["IN-AS"]),
    nia(2026, 4, 20, "Basava Jayanti / Akshaya Tritiya", &["IN-KA"]),
    nia(2026, 4, 21, "Garia Puja/General Election to Aizawl Municipal Corporation, 2026", &["IN-TR", "IN-MZ"]),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima", &["IN-MH"]),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-MN", "IN-GA", "IN-AR", "IN-MZ"]),
    nia(2026, 5, 9, "Birthday of Rabindranath Tagore", &["IN-WB"]),
    nia(2026, 5, 16, "State Day", &["IN-SK"]),
    nia(2026, 5, 26, "Birthday of Kazi Nazrul Islam", &["IN-TR"]),
    nia(2026, 5, 27, "Eid-UI-Adha-(Bakri-Eid)/Id-ul-Zuha", &["IN-KL", "IN-AS", "IN-JK"]),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-MZ"]),
    nia(2026, 6, 15, "YMA Day/Raja Sankranti", &["IN-OD", "IN-MZ"]),
    nia(2026, 6, 22, "Rev Thomas Jones", &["IN-ML"]),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-CG", "IN-TR", "IN-MZ"]),
    nia(2026, 6, 29, "Sant Guru Kabir Jayanti", &["IN-HP"]),
    nia(2026, 6, 30, "Remna Ni", &["IN-MZ"]),
    nia(2026, 7, 6, "MHIP Day/Birth Anniversary of Dr. Syama Prasad Mookerjee", &["IN-WB", "IN-MZ"]),
    nia(2026, 7, 9, "Beh Deinkhlam", &["IN-ML"]),
    nia(2026, 7, 16, "Kang (Rathajatra)/Harela", &["IN-UK"]),
    nia(2026, 7, 16, "Ratha Yatra/Kang (Rathajatra)", &["IN-OD"]),
    nia(2026, 7, 16, "Ratha Yatra/Kang (Rathajatra)/Harela", &["IN-MN"]),
    nia(2026, 7, 17, "Death Anniversary of U Tirot Singh", &["IN-ML"]),
    nia(2026, 7, 18, "Drukpa Tshe-zi", &["IN-SK"]),
    nia(2026, 7, 22, "Kharchi Puja", &["IN-TR"]),
    nia(2026, 8, 4, "Ker Puja", &["IN-TR"]),
    nia(2026, 8, 8, "Tendong Lho Rum Faat", &["IN-SK"]),
    nia(2026, 8, 13, "Patriot’s Day", &["IN-MN"]),
    nia(2026, 8, 15, "Independence Day", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", &["IN-MH"]),
    nia(2026, 8, 19, "Birthday of Maharaja Bir Bikram Kishore Manikya Bahadur", &["IN-TR"]),
    nia(2026, 8, 25, "First Onam/Milad-i-Sherif (Birthday of Prophet Muhammed)", &["IN-KL"]),
    nia(2026, 8, 26, "Id-E-Milad", &["IN-MH"]),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", &["IN-UP", "IN-BR", "IN-MP", "IN-TN", "IN-KA", "IN-AP", "IN-TS", "IN-KL", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-MN"]),
    nia(2026, 8, 28, "Pang-Lhabsol/Friday following Eid-i-Milad-ul-Nabi/Ayyankali Jayanthi", &["IN-JK"]),
    nia(2026, 8, 28, "Pang-Lhabsol/Sree Narayana Guru Jayanthi/Ayyankali Jayanthi", &["IN-KL"]),
    nia(2026, 8, 28, "Raksha Bandhan/Pang-Lhabsol/Friday following Eid-i-Milad-ul-Nabi/Sree Narayana Guru Jayanthi/Ayyankali Jayanthi", &["IN-CG", "IN-SK"]),
    nia(2026, 8, 28, "Raksha Bandhan/Pang-Lhabsol/Sree Narayana Guru Jayanthi/Ayyankali Jayanthi", &["IN-UP", "IN-MP", "IN-RJ", "IN-GJ", "IN-UK", "IN-HP"]),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-JH", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-ML", "IN-MZ", "IN-SK"]),
    nia(2026, 9, 12, "Tirubhav Tithi of Srimanta Sankardeva", &["IN-AS"]),
    nia(2026, 9, 14, "Ganesh Chaturthi", &["IN-MH"]),
    nia(2026, 9, 14, "Ganesh Chaturthi/Ganesh Puja/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi/Hartalika", &["IN-TN", "IN-KA", "IN-AP", "IN-OD", "IN-TS", "IN-GA"]),
    nia(2026, 9, 15, "Samvatsari (Chaturthi Paksha)/Ganesh Chaturthi (2nd Day)", &["IN-GA"]),
    nia(2026, 9, 15, "Samvatsari (Chaturthi Paksha)/Nuakhai", &["IN-OD"]),
    nia(2026, 9, 15, "Samvatsari (Chaturthi Paksha)/Nuakhai/Ganesh Chaturthi (2nd Day)", &["IN-GJ"]),
    nia(2026, 9, 18, "Unitarian Anniversary Day", &["IN-ML"]),
    nia(2026, 9, 21, "Janmotsav of Srimanta Sankardeva/Sree Narayana Guru Samadhi", &["IN-KL", "IN-AS"]),
    nia(2026, 9, 22, "Karma Puja", &["IN-JH"]),
    nia(2026, 9, 23, "Birthday of Maharaja Hari Singh Ji", &["IN-JK"]),
    nia(2026, 9, 25, "Indrajatra", &["IN-SK"]),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 10, 10, "Mahalaya Amavasye", &["IN-WB", "IN-KA"]),
    nia(2026, 10, 17, "Maha Saptami", &["IN-TR"]),
    nia(2026, 10, 19, "Dussehra (Mahashtami)/Maha Navami/Ayutha Pooja/Durga Puja/Vijaya Dashomi/Durga Ashtami", &["IN-BR", "IN-WB", "IN-TN", "IN-OD", "IN-AS", "IN-JH", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-AR", "IN-MZ"]),
    nia(2026, 10, 20, "Dasara", &["IN-MH"]),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", &["IN-UP", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-NL", "IN-GA", "IN-AR", "IN-MZ"]),
    nia(2026, 10, 21, "Vijaya Dasami/Durga Puja (Dasain)", &["IN-WB", "IN-KA", "IN-KL", "IN-AS", "IN-TR", "IN-SK"]),
    nia(2026, 10, 22, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2026, 10, 23, "Durga Puja (Dasain)", &["IN-SK"]),
    nia(2026, 10, 26, "Laxmi Puja/Accession Day/Maharishi Valmiki’s Birthday", &["IN-JK", "IN-HP", "IN-TR"]),
    nia(2026, 10, 29, "Karva Chauth", &["IN-HP"]),
    nia(2026, 10, 31, "Sardar Vallabhbhai Patel's Birthday", &["IN-GJ"]),
    nia(2026, 11, 9, "Diwali/Laxmi Puja (Deepawali)/Govardhan Pooja/Vishwakarma Day", &["IN-UP", "IN-MP", "IN-RJ", "IN-TR", "IN-MN", "IN-SK"]),
    nia(2026, 11, 10, "Diwali (Bali Pratipada)", &["IN-MH"]),
    nia(2026, 11, 10, "Diwali (Bali Pratipada)/Deepawali (Govardhan Puja)/Laxmi Puja/Vikram Samvant New Year Day", &["IN-KA", "IN-GJ", "IN-UK", "IN-SK"]),
    nia(2026, 11, 11, "Laxmi Puja (Deepawali)/Ningol Chakkouba/Bhratridwitiya/Bhaidooj/Chitragupt Jayanti", &["IN-UP", "IN-WB", "IN-HP", "IN-MN", "IN-SK"]),
    nia(2026, 11, 13, "Wangala Festival", &["IN-ML"]),
    nia(2026, 11, 16, "Chhath Puja/Surya Shashti Dala Chhath (Prath Arghya)", &["IN-BR", "IN-JH"]),
    nia(2026, 11, 20, "Igas-Bagwal", &["IN-UK"]),
    nia(2026, 11, 23, "Seng Kut Snem", &["IN-ML"]),
    nia(2026, 11, 24, "Guru Nanak Jayanti", &["IN-MH"]),
    nia(2026, 11, 24, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", &["IN-UP", "IN-WB", "IN-MP", "IN-RJ", "IN-OD", "IN-TS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-NL", "IN-AR", "IN-MZ"]),
    nia(2026, 11, 27, "Kanakadasa Jayanthi", &["IN-KA"]),
    nia(2026, 12, 1, "Indigenous Faith Day/State Inauguration Day", &["IN-NL", "IN-AR"]),
    nia(2026, 12, 3, "Feast of St. Francis Xavier", &["IN-GA"]),
    nia(2026, 12, 9, "Losoong / Namsoong", &["IN-SK"]),
    nia(2026, 12, 10, "Losoong / Namsoong", &["IN-SK"]),
    nia(2026, 12, 11, "Losoong / Namsoong", &["IN-SK"]),
    nia(2026, 12, 12, "Death Anniversary of Pa Togan Nengminja Sangma", &["IN-ML"]),
    nia(2026, 12, 18, "Death Anniversary of U SoSo Tham", &["IN-ML"]),
    nia(2026, 12, 19, "Goa Liberation Day", &["IN-GA"]),
    nia(2026, 12, 24, "Christmas Eve", &["IN-ML", "IN-NL", "IN-MZ"]),
    nia(2026, 12, 25, "Christmas", &["IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP", "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP", "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK"]),
    nia(2026, 12, 26, "Christmas", &["IN-ML", "IN-NL"]),
    nia(2026, 12, 30, "Death Anniversary of U Kiang Nangbah", &["IN-ML"]),
    nia(2026, 12, 31, "New Year's Eve", &["IN-MZ"]),
];

/// India's nationwide rules and its states' days, the table
/// [`super::INDIA`] evaluates.
pub(super) static RULES: [HolidayRule; IN_RULES.len() + STATE_DAYS.len()] =
    joined(&[IN_RULES, STATE_DAYS]);
