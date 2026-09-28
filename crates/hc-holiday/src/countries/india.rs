//! The holidays India's states declare under the Negotiable Instruments
//! Act, 1881, each scoped to its state's ISO 3166-2 code, for the thirteen
//! largest states.
//!
//! Each state notifies a list of such holidays every year, for the
//! purposes of section 25 of the Act, and the banks in the state close on
//! them. Neither the Act nor the notifications, which are gazette PDFs,
//! were read. The Reserve Bank of India's "Holidays under Negotiable
//! Instruments Act" (rbi.org.in, Scripts/HolidayMatrixDisplay.aspx) lists
//! every such day at each of its regional offices, year by year, and that
//! list, for 2025 and 2026, is what is carried: each day at the office in
//! the state, or, for a state with several offices, each day at every one
//! of them — Lucknow and Kanpur for Uttar Pradesh; Mumbai, Belapur and
//! Nagpur for Maharashtra; Thiruvananthapuram and Kochi for Kerala. A day
//! at one office of several is a district's, which a state code cannot
//! scope, and is left out: Id-E-Milad 2025, kept at Mumbai on 8 September
//! and at Belapur and Nagpur on the 5th, is carried on neither, and
//! Thiruvananthapuram's days that Kochi does not keep are not carried.
//! `docs/systems/india-state-holidays.md` has the whole table. So is an election day
//! at a state's one office, whose extent in the state the list does not
//! say: the Legislative Assembly polls of Tamil Nadu (23 April 2026) and
//! West Bengal (29 April 2026) and Jaipur's municipal poll (11 September
//! 2026).
//!
//! The kind is [`Kind::Bank`]: the list is of the Act's holidays, a day
//! off for the banks. The state's own list for its offices is its general
//! holidays, which the notification usually declares with it but which was
//! not read. The Reserve Bank lists no day that falls on a Sunday, when the
//! banks are closed in any case, so a state's holiday on a Sunday is not
//! carried.
//!
//! The name is the Reserve Bank's description of the day, which is one for
//! each date across all its offices: it joins the names the day has
//! wherever it is a holiday, and so can name another state's festival
//! beside the state's own — Maharashtra's 1 May 2026 is "Maharashtra
//! Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit
//! Raghunath Murmu". It is not split, because the list does not say which
//! name is whose. A nationwide day the Department of Personnel and
//! Training's list also has, as Republic Day, is the state's entry as well
//! as the nationwide one.
//!
//! The years before 2025 are not carried, and 2027, whose lists the Reserve
//! Bank had not published, is a gap in every state carried. The other
//! states and union territories are not yet carried; the Reserve Bank's list
//! has them all, from its offices at Agartala, Aizawl, Chandigarh,
//! Dehradun, Gangtok, Guwahati, Imphal, Itanagar, Jammu, Kohima, New Delhi,
//! Panaji, Raipur, Ranchi, Shillong, Shimla and Srinagar.

use crate::rule::{HolidayRule, Kind, Rule};

use super::asia::IN_RULES;
use super::joined;

/// The first year of the Reserve Bank's lists carried.
const FIRST: i32 = 2025;
/// The last.
const LAST: i32 = 2026;

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

/// A holiday under the Act on a day of `year`, in `region`.
const fn nia(
    year: i32,
    month: u8,
    day: u8,
    name: &'static str,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, "", Rule::gregorian(month, day))
        .of_kind(Kind::Bank)
        .years(Some(year), Some(year))
        .in_regions(region)
        .cited(source)
}

/// The years after the last list read, a gap for `region`: its days are
/// declared year by year.
const fn not_read(region: &'static [&'static str], source: &'static str) -> HolidayRule {
    HolidayRule::fixed_public(
        "Holidays under the Negotiable Instruments Act",
        "",
        Rule::unlisted(FIRST as i64, LAST as i64),
    )
    .of_kind(Kind::Bank)
    .years(Some(FIRST), None)
    .in_regions(region)
    .cited(source)
}

/// Every state day carried, state by state, each state's in date order.
#[rustfmt::skip]
pub static STATE_DAYS: &[HolidayRule] = &[
    // Uttar Pradesh: Lucknow and Kanpur.
    not_read(UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 1, 14, "Makar Sankranti/Uttarayana Punyakala/Pongal/Maghe Sankranti/Magh Bihu/Birthday of Hazarat Ali", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 2, 12, "Sant Ravidas Jayanti/Guru Ravi Das’s Birthday/General Election to Local Councils 2025", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 3, 13, "Holika Dahan/Attukal Pongala", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 4, 18, "Good Friday", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 5, 12, "Buddha Pournima", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 8, 9, "Raksha Bandhan/Jhulana Purnima", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 10, 1, "Navaratri Ends/Maha Navami/Dussehra/Ayudhapooja, Vijayadasami/Durga Puja (Dasain)", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 10, 22, "Diwali (Bali Pratipada)/Vikram Samvant New Year Day/Govardhan Pooja/Balipadyami, Laxmi Puja (Deepawali)", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 10, 23, "Bhai Bij/Bhaidooj/Chitragupt Jayanti/Laxmi Puja (Deepawali)/Bhratridwitiya/Ningol Chakkouba", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 11, 5, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2025, 12, 25, "Christmas", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 1, 3, "Birthday of Hazrat Ali", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 1, 15, "Uttarayana Punyakala/Pongal/Maghe Sankranti/Makara Sankranti/Election to Municipal Corporations in Maharashtra", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 1, 26, "Republic Day", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 3, 2, "Holika Dahan", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 3, 4, "Holi/Holi 2nd Day – Dhuleti/Yaosang 2nd Day/Dol Jatra", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 3, 26, "Shree Ram Navami", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 3, 31, "Mahavir Janmakalyanak/Mahavir Jayanti", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 4, 3, "Good Friday", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 8, 28, "Raksha Bandhan/Pang-Lhabsol/Friday following Eid-i-Milad-ul-Nabi/Sree Narayana Guru Jayanthi/Ayyankali Jayanthi", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 11, 9, "Diwali/Laxmi Puja (Deepawali)/Govardhan Pooja/Vishwakarma Day", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 11, 11, "Laxmi Puja (Deepawali)/Ningol Chakkouba/Bhratridwitiya/Bhaidooj/Chitragupt Jayanti", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 11, 24, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    nia(2026, 12, 25, "Christmas", UTTAR_PRADESH, UTTAR_PRADESH_SOURCE),
    // Maharashtra: Mumbai and Belapur and Nagpur.
    not_read(MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 4, 18, "Good Friday", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 5, 12, "Buddha Pournima", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 8, 27, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Ganesh Puja/Vinayakar Chathurthi", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 10, 21, "Diwali Amavasya (Laxmi Pujan)/Deepawali/Govardhan Pooja", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 10, 22, "Diwali (Bali Pratipada)/Vikram Samvant New Year Day/Govardhan Pooja/Balipadyami, Laxmi Puja (Deepawali)", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 11, 5, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2025, 12, 25, "Christmas", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 1, 15, "Uttarayana Punyakala/Pongal/Maghe Sankranti/Makara Sankranti/Election to Municipal Corporations in Maharashtra", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 1, 26, "Republic Day", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 3, 19, "Gudhi Padwa/Ugadi Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st Navratra", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 3, 26, "Shree Ram Navami", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 3, 31, "Mahavir Janmakalyanak/Mahavir Jayanti", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 4, 3, "Good Friday", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 9, 14, "Ganesh Chaturthi/Ganesh Puja/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi/Hartalika", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 11, 10, "Diwali (Bali Pratipada)/Deepawali (Govardhan Puja)/Laxmi Puja/Vikram Samvant New Year Day", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 11, 24, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", MAHARASHTRA, MAHARASHTRA_SOURCE),
    nia(2026, 12, 25, "Christmas", MAHARASHTRA, MAHARASHTRA_SOURCE),
    // Bihar: Patna.
    not_read(BIHAR, BIHAR_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", BIHAR, BIHAR_SOURCE),
    nia(2025, 3, 15, "Holi/Yaosang 2nd Day", BIHAR, BIHAR_SOURCE),
    nia(2025, 3, 22, "Bihar Diwas", BIHAR, BIHAR_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", BIHAR, BIHAR_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", BIHAR, BIHAR_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", BIHAR, BIHAR_SOURCE),
    nia(2025, 4, 18, "Good Friday", BIHAR, BIHAR_SOURCE),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", BIHAR, BIHAR_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", BIHAR, BIHAR_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", BIHAR, BIHAR_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", BIHAR, BIHAR_SOURCE),
    nia(2025, 9, 30, "Maha Ashtami/Durga Ashtami/Durga Puja", BIHAR, BIHAR_SOURCE),
    nia(2025, 10, 1, "Navaratri Ends/Maha Navami/Dussehra/Ayudhapooja, Vijayadasami/Durga Puja (Dasain)", BIHAR, BIHAR_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", BIHAR, BIHAR_SOURCE),
    nia(2025, 10, 27, "Chath Puja (Evening Puja)", BIHAR, BIHAR_SOURCE),
    nia(2025, 10, 28, "Chath Puja (Morning Puja)", BIHAR, BIHAR_SOURCE),
    nia(2025, 11, 6, "Nongkrem Dance/Bihar Legislative Assembly General Election, 2025", BIHAR, BIHAR_SOURCE),
    nia(2025, 12, 25, "Christmas", BIHAR, BIHAR_SOURCE),
    nia(2026, 1, 26, "Republic Day", BIHAR, BIHAR_SOURCE),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", BIHAR, BIHAR_SOURCE),
    nia(2026, 3, 4, "Holi/Holi 2nd Day – Dhuleti/Yaosang 2nd Day/Dol Jatra", BIHAR, BIHAR_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", BIHAR, BIHAR_SOURCE),
    nia(2026, 3, 27, "Shree Ram Navami (Chaite Dasain)", BIHAR, BIHAR_SOURCE),
    nia(2026, 3, 31, "Mahavir Janmakalyanak/Mahavir Jayanti", BIHAR, BIHAR_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", BIHAR, BIHAR_SOURCE),
    nia(2026, 4, 3, "Good Friday", BIHAR, BIHAR_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", BIHAR, BIHAR_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", BIHAR, BIHAR_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", BIHAR, BIHAR_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", BIHAR, BIHAR_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", BIHAR, BIHAR_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", BIHAR, BIHAR_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", BIHAR, BIHAR_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", BIHAR, BIHAR_SOURCE),
    nia(2026, 10, 19, "Dussehra (Mahashtami)/Maha Navami/Ayutha Pooja/Durga Puja/Vijaya Dashomi/Durga Ashtami", BIHAR, BIHAR_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", BIHAR, BIHAR_SOURCE),
    nia(2026, 11, 16, "Chhath Puja/Surya Shashti Dala Chhath (Prath Arghya)", BIHAR, BIHAR_SOURCE),
    nia(2026, 12, 25, "Christmas", BIHAR, BIHAR_SOURCE),
    // West Bengal: Kolkata.
    not_read(WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 1, 1, "New Year’s Day/Loosong/Namsoong", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 1, 23, "Birthday of Netaji Subhas Chandra Bose/Vir Surendrasai Jayanti/General Elections to the Municipal Local Bodies", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 4, 15, "Bengali New Year’s Day/Himachal Day/Bohag Bihu", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 4, 18, "Good Friday", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 5, 9, "Birthday of Rabindranath Tagore", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 5, 12, "Buddha Pournima", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 9, 29, "Maha Saptami/Durga Puja", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 9, 30, "Maha Ashtami/Durga Ashtami/Durga Puja", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 10, 1, "Navaratri Ends/Maha Navami/Dussehra/Ayudhapooja, Vijayadasami/Durga Puja (Dasain)", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 10, 6, "Lakshmi Puja", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 10, 23, "Bhai Bij/Bhaidooj/Chitragupt Jayanti/Laxmi Puja (Deepawali)/Bhratridwitiya/Ningol Chakkouba", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 10, 27, "Chath Puja (Evening Puja)", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 11, 5, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2025, 12, 25, "Christmas", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 1, 1, "New Year’s Day/Gaan-Ngai", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 1, 12, "Birth Day of Swami Vivekananda", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 1, 23, "Birthday of Netaji Subhas Chandra Bose/Saraswati Puja (Shree Panchami)/Vir Surendrasai Jayanti/Basanta Panchami", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 1, 26, "Republic Day", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 3, 26, "Shree Ram Navami", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 3, 31, "Mahavir Janmakalyanak/Mahavir Jayanti", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 4, 3, "Good Friday", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 4, 15, "Bengali New Year’s Day (Nababarsha)/Bohag Bihu/Vishu/Himachal Day", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 5, 9, "Birthday of Rabindranath Tagore", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 7, 6, "MHIP Day/Birth Anniversary of Dr. Syama Prasad Mookerjee", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 10, 10, "Mahalaya Amavasye", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 10, 19, "Dussehra (Mahashtami)/Maha Navami/Ayutha Pooja/Durga Puja/Vijaya Dashomi/Durga Ashtami", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 10, 21, "Vijaya Dasami/Durga Puja (Dasain)", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 11, 11, "Laxmi Puja (Deepawali)/Ningol Chakkouba/Bhratridwitiya/Bhaidooj/Chitragupt Jayanti", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 11, 24, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", WEST_BENGAL, WEST_BENGAL_SOURCE),
    nia(2026, 12, 25, "Christmas", WEST_BENGAL, WEST_BENGAL_SOURCE),
    // Madhya Pradesh: Bhopal.
    not_read(MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 4, 18, "Good Friday", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 5, 12, "Buddha Pournima", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 8, 9, "Raksha Bandhan/Jhulana Purnima", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 10, 21, "Diwali Amavasya (Laxmi Pujan)/Deepawali/Govardhan Pooja", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 11, 5, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2025, 12, 25, "Christmas", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 1, 26, "Republic Day", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 3, 4, "Holi/Holi 2nd Day – Dhuleti/Yaosang 2nd Day/Dol Jatra", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 3, 27, "Shree Ram Navami (Chaite Dasain)", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 3, 30, "Mahavir Jayanti", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 4, 3, "Good Friday", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 8, 28, "Raksha Bandhan/Pang-Lhabsol/Friday following Eid-i-Milad-ul-Nabi/Sree Narayana Guru Jayanthi/Ayyankali Jayanthi", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 11, 9, "Diwali/Laxmi Puja (Deepawali)/Govardhan Pooja/Vishwakarma Day", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 11, 24, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    nia(2026, 12, 25, "Christmas", MADHYA_PRADESH, MADHYA_PRADESH_SOURCE),
    // Tamil Nadu: Chennai.
    not_read(TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 1, 1, "New Year’s Day/Loosong/Namsoong", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 1, 14, "Makar Sankranti/Uttarayana Punyakala/Pongal/Maghe Sankranti/Magh Bihu/Birthday of Hazarat Ali", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 1, 15, "Thiruvalluvar Day", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 1, 16, "Uzhavar Thirunal", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 2, 11, "Thai Poosam/Municipal Corporation General Election 2025", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 4, 18, "Good Friday", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 8, 27, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Ganesh Puja/Vinayakar Chathurthi", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 10, 1, "Navaratri Ends/Maha Navami/Dussehra/Ayudhapooja, Vijayadasami/Durga Puja (Dasain)", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2025, 12, 25, "Christmas", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 1, 1, "New Year’s Day/Gaan-Ngai", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 1, 15, "Uttarayana Punyakala/Pongal/Maghe Sankranti/Makara Sankranti/Election to Municipal Corporations in Maharashtra", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 1, 16, "Thiruvalluvar Day/Kanuma", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 1, 17, "Uzhavar Thirunal", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 1, 26, "Republic Day", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 3, 19, "Gudhi Padwa/Ugadi Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st Navratra", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 3, 31, "Mahavir Janmakalyanak/Mahavir Jayanti", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 4, 3, "Good Friday", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 9, 14, "Ganesh Chaturthi/Ganesh Puja/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi/Hartalika", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 10, 19, "Dussehra (Mahashtami)/Maha Navami/Ayutha Pooja/Durga Puja/Vijaya Dashomi/Durga Ashtami", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", TAMIL_NADU, TAMIL_NADU_SOURCE),
    nia(2026, 12, 25, "Christmas", TAMIL_NADU, TAMIL_NADU_SOURCE),
    // Rajasthan: Jaipur.
    not_read(RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 8, 9, "Raksha Bandhan/Jhulana Purnima", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 9, 22, "Navratra Sthapna", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 9, 30, "Maha Ashtami/Durga Ashtami/Durga Puja", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 10, 22, "Diwali (Bali Pratipada)/Vikram Samvant New Year Day/Govardhan Pooja/Balipadyami, Laxmi Puja (Deepawali)", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 11, 5, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2025, 12, 25, "Christmas", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 1, 26, "Republic Day", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 3, 26, "Shree Ram Navami", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 3, 31, "Mahavir Janmakalyanak/Mahavir Jayanti", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 8, 28, "Raksha Bandhan/Pang-Lhabsol/Friday following Eid-i-Milad-ul-Nabi/Sree Narayana Guru Jayanthi/Ayyankali Jayanthi", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 11, 9, "Diwali/Laxmi Puja (Deepawali)/Govardhan Pooja/Vishwakarma Day", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 11, 24, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", RAJASTHAN, RAJASTHAN_SOURCE),
    nia(2026, 12, 25, "Christmas", RAJASTHAN, RAJASTHAN_SOURCE),
    // Karnataka: Bengaluru.
    not_read(KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 1, 14, "Makar Sankranti/Uttarayana Punyakala/Pongal/Maghe Sankranti/Magh Bihu/Birthday of Hazarat Ali", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 4, 18, "Good Friday", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 4, 30, "Basava Jayanti/Akshaya Tritiya", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 8, 27, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Ganesh Puja/Vinayakar Chathurthi", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 10, 1, "Navaratri Ends/Maha Navami/Dussehra/Ayudhapooja, Vijayadasami/Durga Puja (Dasain)", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 10, 7, "Maharshi Valmiki Jayanti/Kumar Purnima", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 10, 22, "Diwali (Bali Pratipada)/Vikram Samvant New Year Day/Govardhan Pooja/Balipadyami, Laxmi Puja (Deepawali)", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 11, 1, "Kannada Rajyothsava/Igas-Bagwal", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 11, 8, "Kanakadasa Jayanthi", KARNATAKA, KARNATAKA_SOURCE),
    nia(2025, 12, 25, "Christmas", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 1, 15, "Uttarayana Punyakala/Pongal/Maghe Sankranti/Makara Sankranti/Election to Municipal Corporations in Maharashtra", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 1, 26, "Republic Day", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 3, 19, "Gudhi Padwa/Ugadi Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st Navratra", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 3, 30, "Mahavir Jayanti", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 4, 3, "Good Friday", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 4, 20, "Basava Jayanti / Akshaya Tritiya", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 9, 14, "Ganesh Chaturthi/Ganesh Puja/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi/Hartalika", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 10, 10, "Mahalaya Amavasye", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 10, 21, "Vijaya Dasami/Durga Puja (Dasain)", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 11, 10, "Diwali (Bali Pratipada)/Deepawali (Govardhan Puja)/Laxmi Puja/Vikram Samvant New Year Day", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 11, 27, "Kanakadasa Jayanthi", KARNATAKA, KARNATAKA_SOURCE),
    nia(2026, 12, 25, "Christmas", KARNATAKA, KARNATAKA_SOURCE),
    // Gujarat: Ahmedabad.
    not_read(GUJARAT, GUJARAT_SOURCE),
    nia(2025, 1, 14, "Makar Sankranti/Uttarayana Punyakala/Pongal/Maghe Sankranti/Magh Bihu/Birthday of Hazarat Ali", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 4, 10, "Mahavir Janmakalyanak/Mahavir Jayanti", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 4, 18, "Good Friday", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 8, 9, "Raksha Bandhan/Jhulana Purnima", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 8, 27, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Ganesh Puja/Vinayakar Chathurthi", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 10, 22, "Diwali (Bali Pratipada)/Vikram Samvant New Year Day/Govardhan Pooja/Balipadyami, Laxmi Puja (Deepawali)", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 10, 23, "Bhai Bij/Bhaidooj/Chitragupt Jayanti/Laxmi Puja (Deepawali)/Bhratridwitiya/Ningol Chakkouba", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 10, 31, "Sardar Vallabhbhai Patel's Birthday", GUJARAT, GUJARAT_SOURCE),
    nia(2025, 12, 25, "Christmas", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 1, 14, "Makar Sankranti/Magh Bihu", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 1, 26, "Republic Day", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 3, 4, "Holi/Holi 2nd Day – Dhuleti/Yaosang 2nd Day/Dol Jatra", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 3, 26, "Shree Ram Navami", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 3, 31, "Mahavir Janmakalyanak/Mahavir Jayanti", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 4, 3, "Good Friday", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 8, 28, "Raksha Bandhan/Pang-Lhabsol/Friday following Eid-i-Milad-ul-Nabi/Sree Narayana Guru Jayanthi/Ayyankali Jayanthi", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 9, 15, "Samvatsari (Chaturthi Paksha)/Nuakhai/Ganesh Chaturthi (2nd Day)", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 10, 31, "Sardar Vallabhbhai Patel's Birthday", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 11, 10, "Diwali (Bali Pratipada)/Deepawali (Govardhan Puja)/Laxmi Puja/Vikram Samvant New Year Day", GUJARAT, GUJARAT_SOURCE),
    nia(2026, 12, 25, "Christmas", GUJARAT, GUJARAT_SOURCE),
    // Andhra Pradesh: Vijayawada.
    not_read(ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 1, 14, "Makar Sankranti/Uttarayana Punyakala/Pongal/Maghe Sankranti/Magh Bihu/Birthday of Hazarat Ali", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 4, 18, "Good Friday", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 8, 27, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Ganesh Puja/Vinayakar Chathurthi", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2025, 12, 25, "Christmas", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 1, 15, "Uttarayana Punyakala/Pongal/Maghe Sankranti/Makara Sankranti/Election to Municipal Corporations in Maharashtra", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 1, 16, "Thiruvalluvar Day/Kanuma", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 1, 26, "Republic Day", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 3, 19, "Gudhi Padwa/Ugadi Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st Navratra", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 3, 27, "Shree Ram Navami (Chaite Dasain)", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 4, 3, "Good Friday", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 9, 14, "Ganesh Chaturthi/Ganesh Puja/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi/Hartalika", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    nia(2026, 12, 25, "Christmas", ANDHRA_PRADESH, ANDHRA_PRADESH_SOURCE),
    // Odisha: Bhubaneswar.
    not_read(ODISHA, ODISHA_SOURCE),
    nia(2025, 1, 14, "Makar Sankranti/Uttarayana Punyakala/Pongal/Maghe Sankranti/Magh Bihu/Birthday of Hazarat Ali", ODISHA, ODISHA_SOURCE),
    nia(2025, 1, 23, "Birthday of Netaji Subhas Chandra Bose/Vir Surendrasai Jayanti/General Elections to the Municipal Local Bodies", ODISHA, ODISHA_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", ODISHA, ODISHA_SOURCE),
    nia(2025, 3, 15, "Holi/Yaosang 2nd Day", ODISHA, ODISHA_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", ODISHA, ODISHA_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", ODISHA, ODISHA_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", ODISHA, ODISHA_SOURCE),
    nia(2025, 4, 18, "Good Friday", ODISHA, ODISHA_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", ODISHA, ODISHA_SOURCE),
    nia(2025, 6, 27, "Ratha Yatra/Kang (Rathajatra)", ODISHA, ODISHA_SOURCE),
    nia(2025, 8, 9, "Raksha Bandhan/Jhulana Purnima", ODISHA, ODISHA_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", ODISHA, ODISHA_SOURCE),
    nia(2025, 8, 27, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Ganesh Puja/Vinayakar Chathurthi", ODISHA, ODISHA_SOURCE),
    nia(2025, 8, 28, "Ganesh Chaturthi (2nd Day)/Nuakhai", ODISHA, ODISHA_SOURCE),
    nia(2025, 9, 30, "Maha Ashtami/Durga Ashtami/Durga Puja", ODISHA, ODISHA_SOURCE),
    nia(2025, 10, 1, "Navaratri Ends/Maha Navami/Dussehra/Ayudhapooja, Vijayadasami/Durga Puja (Dasain)", ODISHA, ODISHA_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", ODISHA, ODISHA_SOURCE),
    nia(2025, 10, 7, "Maharshi Valmiki Jayanti/Kumar Purnima", ODISHA, ODISHA_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", ODISHA, ODISHA_SOURCE),
    nia(2025, 11, 5, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", ODISHA, ODISHA_SOURCE),
    nia(2025, 12, 25, "Christmas", ODISHA, ODISHA_SOURCE),
    nia(2026, 1, 14, "Makar Sankranti/Magh Bihu", ODISHA, ODISHA_SOURCE),
    nia(2026, 1, 23, "Birthday of Netaji Subhas Chandra Bose/Saraswati Puja (Shree Panchami)/Vir Surendrasai Jayanti/Basanta Panchami", ODISHA, ODISHA_SOURCE),
    nia(2026, 1, 26, "Republic Day", ODISHA, ODISHA_SOURCE),
    nia(2026, 3, 4, "Holi/Holi 2nd Day – Dhuleti/Yaosang 2nd Day/Dol Jatra", ODISHA, ODISHA_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", ODISHA, ODISHA_SOURCE),
    nia(2026, 3, 27, "Shree Ram Navami (Chaite Dasain)", ODISHA, ODISHA_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", ODISHA, ODISHA_SOURCE),
    nia(2026, 4, 3, "Good Friday", ODISHA, ODISHA_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", ODISHA, ODISHA_SOURCE),
    nia(2026, 6, 15, "YMA Day/Raja Sankranti", ODISHA, ODISHA_SOURCE),
    nia(2026, 7, 16, "Ratha Yatra/Kang (Rathajatra)/Harela", ODISHA, ODISHA_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", ODISHA, ODISHA_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", ODISHA, ODISHA_SOURCE),
    nia(2026, 9, 14, "Ganesh Chaturthi/Ganesh Puja/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi/Hartalika", ODISHA, ODISHA_SOURCE),
    nia(2026, 9, 15, "Samvatsari (Chaturthi Paksha)/Nuakhai/Ganesh Chaturthi (2nd Day)", ODISHA, ODISHA_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", ODISHA, ODISHA_SOURCE),
    nia(2026, 10, 19, "Dussehra (Mahashtami)/Maha Navami/Ayutha Pooja/Durga Puja/Vijaya Dashomi/Durga Ashtami", ODISHA, ODISHA_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", ODISHA, ODISHA_SOURCE),
    nia(2026, 11, 24, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", ODISHA, ODISHA_SOURCE),
    nia(2026, 12, 25, "Christmas", ODISHA, ODISHA_SOURCE),
    // Telangana: Hyderabad.
    not_read(TELANGANA, TELANGANA_SOURCE),
    nia(2025, 1, 14, "Makar Sankranti/Uttarayana Punyakala/Pongal/Maghe Sankranti/Magh Bihu/Birthday of Hazarat Ali", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 3, 14, "Holi (Second Day) - Dhuleti/Dhulandi/Dol Jatra", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 4, 5, "Babu Jagjivan Ram’s Birthday", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 4, 18, "Good Friday", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 8, 16, "Janmashtami (Shravan Vad-8)/Krishna Jayanthi", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 8, 27, "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Ganesh Puja/Vinayakar Chathurthi", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 11, 5, "Guru Nanak Jayanti/Kartika Purnima/Rahas Purnima", TELANGANA, TELANGANA_SOURCE),
    nia(2025, 12, 25, "Christmas", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 1, 15, "Uttarayana Punyakala/Pongal/Maghe Sankranti/Makara Sankranti/Election to Municipal Corporations in Maharashtra", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 1, 26, "Republic Day", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 3, 3, "Holi (Second Day)/Dol Jatra/Dhulandi/Holika Dahan/Attukal Pongala", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 3, 19, "Gudhi Padwa/Ugadi Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st Navratra", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 3, 21, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan/Sarhul", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 3, 27, "Shree Ram Navami (Chaite Dasain)", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 4, 3, "Good Friday", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 6, 26, "Muharram (Yaom-EShahadath)/Last Day of Moharam/Ashoora", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 9, 14, "Ganesh Chaturthi/Ganesh Puja/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi/Hartalika", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 11, 24, "Guru Nanak Jayanti/Karthika Purnima/Rahas Purnima", TELANGANA, TELANGANA_SOURCE),
    nia(2026, 12, 25, "Christmas", TELANGANA, TELANGANA_SOURCE),
    // Kerala: Thiruvananthapuram and Kochi.
    not_read(KERALA, KERALA_SOURCE),
    nia(2025, 2, 26, "Mahashivratri", KERALA, KERALA_SOURCE),
    nia(2025, 3, 31, "Ramzan-Id (Id-Ul-Fitr) (Shawal-1)/Khutub-E-Ramzan", KERALA, KERALA_SOURCE),
    nia(2025, 4, 1, "To enable to Banks to close their yearly accounts/Sarhul", KERALA, KERALA_SOURCE),
    nia(2025, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Vishu/Biju/Buisu Festival/Maha Vishuva Sankranti/Tamil New Year's Day/Bohag Bihu/Cheiraoba", KERALA, KERALA_SOURCE),
    nia(2025, 4, 18, "Good Friday", KERALA, KERALA_SOURCE),
    nia(2025, 5, 1, "Maharashtra Din/May Day (Labor Day)", KERALA, KERALA_SOURCE),
    nia(2025, 6, 7, "Bakri ID (Id-Uz-Zuha)", KERALA, KERALA_SOURCE),
    nia(2025, 7, 22, "Demise of Shri V.S.Achuthanandan, former Chief Minister of Kerala", KERALA, KERALA_SOURCE),
    nia(2025, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)/Janmashtami", KERALA, KERALA_SOURCE),
    nia(2025, 9, 4, "First Onam", KERALA, KERALA_SOURCE),
    nia(2025, 9, 5, "Id-E-Milad/Milad-un-Nabi or Id-e Milad (Birthday of Prophet Mohammad) (bara vafat)/Thiruvonam/Milad-i-Sherif", KERALA, KERALA_SOURCE),
    nia(2025, 10, 1, "Navaratri Ends/Maha Navami/Dussehra/Ayudhapooja, Vijayadasami/Durga Puja (Dasain)", KERALA, KERALA_SOURCE),
    nia(2025, 10, 2, "Mahatma Gandhi Jayanti/Dasara/Vijaya Dashami/Dussehra/Durga Puja (Dasain)/Janmotsav of Sri Sri Sankardeva", KERALA, KERALA_SOURCE),
    nia(2025, 10, 20, "Diwali (Deepavali)/Naraka Chaturdashi/Kali Puja", KERALA, KERALA_SOURCE),
    nia(2025, 12, 9, "General election to Local Government Institutions 2025", KERALA, KERALA_SOURCE),
    nia(2025, 12, 25, "Christmas", KERALA, KERALA_SOURCE),
    nia(2026, 1, 2, "New Year Celebration/Mannam Jayanthi", KERALA, KERALA_SOURCE),
    nia(2026, 1, 26, "Republic Day", KERALA, KERALA_SOURCE),
    nia(2026, 3, 20, "Eid-Ul-Fitr (Ramzan)", KERALA, KERALA_SOURCE),
    nia(2026, 4, 1, "To enable to Banks to close their yearly accounts", KERALA, KERALA_SOURCE),
    nia(2026, 4, 2, "Maundy Thursday", KERALA, KERALA_SOURCE),
    nia(2026, 4, 3, "Good Friday", KERALA, KERALA_SOURCE),
    nia(2026, 4, 9, "General Election to Kerala Legislative Assembly 2026", KERALA, KERALA_SOURCE),
    nia(2026, 4, 14, "Dr. Babasaheb Ambedkar Jayanti/Maha Vishuva Sankranti/Biju/Buisu Festival/Tamil New Year's Day/Bohag Bihu/Cheiraoba/Baisakhi", KERALA, KERALA_SOURCE),
    nia(2026, 4, 15, "Bengali New Year’s Day (Nababarsha)/Bohag Bihu/Vishu/Himachal Day", KERALA, KERALA_SOURCE),
    nia(2026, 5, 1, "Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit Raghunath Murmu", KERALA, KERALA_SOURCE),
    nia(2026, 5, 27, "Eid-UI-Adha-(Bakri-Eid)/Id-ul-Zuha", KERALA, KERALA_SOURCE),
    nia(2026, 5, 28, "Bakri ID (Id-Uz-Zuha)", KERALA, KERALA_SOURCE),
    nia(2026, 8, 15, "Independence Day/Parsi New Year (Shahenshahi)", KERALA, KERALA_SOURCE),
    nia(2026, 8, 25, "First Onam/Milad-i-Sherif (Birthday of Prophet Muhammed)", KERALA, KERALA_SOURCE),
    nia(2026, 8, 26, "Id-E-Milad/Baravafat/Milad-un-Nabi (Birthday of Prophet Mohammad)/Thiruvonam", KERALA, KERALA_SOURCE),
    nia(2026, 8, 28, "Raksha Bandhan/Pang-Lhabsol/Friday following Eid-i-Milad-ul-Nabi/Sree Narayana Guru Jayanthi/Ayyankali Jayanthi", KERALA, KERALA_SOURCE),
    nia(2026, 9, 4, "Janmashtami (Vaishnva) (Shravan Vad-8)/Krishna Jayanthi", KERALA, KERALA_SOURCE),
    nia(2026, 9, 21, "Janmotsav of Srimanta Sankardeva/Sree Narayana Guru Samadhi", KERALA, KERALA_SOURCE),
    nia(2026, 10, 2, "Mahatma Gandhi Jayanti", KERALA, KERALA_SOURCE),
    nia(2026, 10, 20, "Dasara/Dusshera (Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja", KERALA, KERALA_SOURCE),
    nia(2026, 10, 21, "Vijaya Dasami/Durga Puja (Dasain)", KERALA, KERALA_SOURCE),
    nia(2026, 12, 25, "Christmas", KERALA, KERALA_SOURCE),
];

/// India's nationwide rules and its states' days, the table
/// [`super::INDIA`] evaluates.
pub(super) static RULES: [HolidayRule; IN_RULES.len() + STATE_DAYS.len()] =
    joined(IN_RULES, STATE_DAYS);
