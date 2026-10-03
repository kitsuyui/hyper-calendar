//! Country tables read from a page or an instrument that states the year and
//! no more: each is one government year with the first year the page reaches.
//!
//! The tables of [`crate::countries`] that are written out by hand carry the
//! history their sources gave. These do not: a country here is a year whose
//! start is on a page, a first year, and an `authority` that says whether the
//! instrument that fixes it was read. Where it was not, the authority is
//! [`Authority::Unread`](crate::Authority::Unread), the entry is read from the
//! year of the page, and every earlier year is a gap.

use crate::countries::FiscalProfile;
use crate::year_system::{
    Authority, LabelConvention, SourceDate, SystemKind, YearStart, YearSystem,
};

/// One government year as a profile of its own.
macro_rules! government_year {
    (
        $(#[$meta:meta])*
        $ident:ident, $code:literal, $name:literal, $local:literal, $system:literal,
        $authority:ident, ($month:literal, $day:literal), $label:ident,
        $valid_from:expr, $read_from:literal, $note:literal, $sources:literal
    ) => {
        $(#[$meta])*
        pub static $ident: FiscalProfile = FiscalProfile {
            code: $code,
            english_name: $name,
            systems: &[YearSystem {
                name: $system,
                local_name: $local,
                kind: SystemKind::Government,
                authority: Authority::$authority,
                start: YearStart::gregorian($month, $day),
                label: LabelConvention::$label,
                valid_from: $valid_from,
                valid_until: None,
                read_from: $read_from,
                unread: &[],
                note: $note,
            }],
            sources_checked: SourceDate::new(2026, 10, 4),
            sources: $sources,
        };
    };
}

government_year! {
/// Angola — the calendar year, as two reference pages say.
    ANGOLA, "AO", "Angola", "", "Angola fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Angola - Corporate - Tax administration\" (last reviewed \
        2026-06-24), gives the tax year as the calendar year, and Wikipedia's \"Economy of Angola\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Angola, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Angola\" (secondary), read 2026-10-04"
}

government_year! {
/// Argentina — the calendar year, by statute: Ley 24.156, art. 10.
    ARGENTINA, "AR", "Argentina", "ejercicio financiero", "Argentine national public sector financial year",
    Statute, (1, 1), LabelledByStartYear,
    None, 1993,
    "\
        Ley 24.156 (sanctioned 30 September 1992, published 29 October 1992) art. 10: the ejercicio \
        financiero begins on 1 January and ends on 31 December. The first whole year after its \
        publication is 1993; its date in force was not shown on the page, and nothing read says what the \
        year was before.",
    "\
        Ley 24.156, Administración Financiera y de los Sistemas de Control del Sector Público Nacional, \
        art. 10 (argentina.gob.ar, texto actualizado), read 2026-10-04"
}

government_year! {
/// Bangladesh — 1 July to 30 June, by the General Clauses Act, and named for the year it begins
/// in.
    BANGLADESH, "BD", "Bangladesh", "", "Bangladesh financial year",
    Statute, (7, 1), LabelledByStartYear,
    None, 1974,
    "\
        The General Clauses Act, 1897, s. 3(19), as substituted by article 4 of the General Clauses \
        (Amendment) Order, 1972 (President's Order No. 147 of 1972; its day in 1972 was not read): \"the \
        year commencing on the first day of July and ending on the 30th day of June\". The Finance Acts \
        name the year by the day it begins in: \"the year beginning on the first day of July, 1974\", \
        1975 and 1981 (the Act of 1974 is dated 29 June 1974). The definition before the Order was not \
        read, so 1974, the first year a dated Act names, is the first year carried.",
    "\
        The General Clauses Act, 1897 (bdlaws.minlaw.gov.bd, act 73); The Finance Act, 1974, 1975 and \
        1981 (bdlaws.minlaw.gov.bd, acts 473, 484 and 617), read 2026-10-04"
}

government_year! {
/// Bahrain — the calendar year, as two reference pages say.
    BAHRAIN, "BH", "Bahrain", "", "Bahrain fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Bahrain - Corporate - Tax administration\" (last reviewed \
        2026-07-26), gives the tax year as the calendar year, and Wikipedia's \"Economy of Bahrain\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Bahrain, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Bahrain\" (secondary), read 2026-10-04"
}

government_year! {
/// The Bahamas — 1 July to 30 June, as the Ministry of Finance says.
    BAHAMAS, "BS", "Bahamas", "", "Bahamian financial year",
    Unread, (7, 1), LabelledByStartYear,
    None, 2025,
    "\
        The Ministry of Finance's budget page says the budget is approved for adoption on 1 July every \
        year and the \"2026/2027\" Annual Budget was enacted on 1 July 2026; its documents are named \
        \"FY2026/2027 Budget\" and \"FY2025/2026 Supplementary Budget\", the span led by the year it \
        begins in. The Public Finance Management Act 2023 is a PDF and was not read, so the earliest \
        year carried is the earliest it names, 2025.",
    "\
        Ministry of Finance, The Bahamas, budget page (bahamasbudget.gov.bs), read 2026-10-04"
}

government_year! {
/// Chile — the calendar year, by Decreto Ley 1263, art. 12.
    CHILE, "CL", "Chile", "ejercicio presupuestario", "Chilean budget year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        Decreto Ley 1263, Ley orgánica de administración financiera del Estado, art. 12: \"El ejercicio \
        presupuestario coincidirá con el año calendario\", read on a third-party copy (iura.cl) that \
        gives no date. A search result titles the decree 28 November 1975; no page read confirms it, so \
        the entry is read from 2026, the year of the copy.",
    "\
        Decreto Ley 1263, art. 12 (iura.cl, a copy; the Biblioteca del Congreso Nacional's page needs \
        scripts and was not read), read 2026-10-04"
}

government_year! {
/// Colombia — the calendar year, by the Estatuto Orgánico del Presupuesto, art. 14.
    COLOMBIA, "CO", "Colombia", "año fiscal", "Colombian fiscal year",
    Statute, (1, 1), LabelledByStartYear,
    None, 1997,
    "\
        Decreto 111 de 1996 (dated 15 January 1996, Diario Oficial 42.692 of 18 January 1996) art. 14, \
        Anualidad: the año fiscal begins on 1 January and ends on 31 December. Published after 1996 \
        began, so the first whole year is 1997. The decree compiles earlier budget laws; what the year \
        was before it was not read.",
    "\
        Decreto 111 de 1996, art. 14 (normas.cra.gov.co), read 2026-10-04"
}

government_year! {
/// Costa Rica — the calendar year, by the Constitution, art. 176.
    COSTA_RICA, "CR", "Costa Rica", "año económico", "Costa Rican budget year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2020,
    "\
        Constitución Política art. 176: the budget of the Republic is issued for one year, from the \
        first of January to the thirty-first of December, read in the English translation of \
        constituteproject.org's \"Costa Rica 1949 (rev. 2020)\". Whether the wording is the original of \
        1949 was not shown, so the entry is read from the revision, 2020.",
    "\
        Constitution of Costa Rica 1949 (rev. 2020), art. 176 (constituteproject.org), read 2026-10-04"
}

government_year! {
/// Cyprus — the calendar year, by Law 20(I)/2014, section 6.
    CYPRUS, "CY", "Cyprus", "", "Cypriot financial year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2015,
    "\
        Law 20(I)/2014, section 6: the financial year commences on 1 January and ends on 31 December \
        (cylaw.org; the two reads of the page gave different English titles for the law, so it is cited \
        by number only). Read from the year of enactment, 2015; the date was not read.",
    "\
        Law 20(I)/2014, section 6 (cylaw.org), read 2026-10-04"
}

government_year! {
/// Czechia — the calendar year, by Act 218/2000 Sb.
    CZECHIA, "CZ", "Czechia", "rozpočtový rok", "Czech budget year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2001,
    "\
        Zákon č. 218/2000 Sb., o rozpočtových pravidlech, § 2(2): \"Rozpočtový rok je shodný s rokem \
        kalendářním.\" The law is of 27 June 2000 and, per the page, in force from 1 January 2001. \
        Nothing read says what the year was before.",
    "\
        Zákon č. 218/2000 Sb., § 2 (zakonyprolidi.cz), read 2026-10-04"
}

government_year! {
/// Spain — the calendar year, by the Ley General Presupuestaria.
    SPAIN, "ES", "Spain", "ejercicio presupuestario", "Spanish budget year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2005,
    "\
        Ley 47/2003, de 26 de noviembre, General Presupuestaria (BOE 27 November 2003), the article \
        defining the Presupuestos Generales del Estado: the ejercicio presupuestario \"será coincidente \
        con el año natural\". Its date in force is 1 January 2005 per the page. The article number \
        differed between reads (30, 32 and 33), so it is not cited.",
    "\
        Ley 47/2003, de 26 de noviembre, General Presupuestaria (boe.es), read 2026-10-04"
}

government_year! {
/// Fiji — 1 August to 31 July, as the Ministry of Economy says.
    FIJI, "FJ", "Fiji", "", "Fijian financial year",
    Unread, (8, 1), LabelledByStartYear,
    None, 2025,
    "\
        The Ministry of Economy's \"Budget Process\": the National Budget is prepared for each financial \
        year, which begins on 1 August of every calendar year and ends on 31 July of the next. Its \
        budget documents are named \"2026-2027\", \"2025-2026\" and so on, the span led by the year it \
        begins in; the Financial Management Act 2004 was not read (a search result gave a conflicting \
        definition and was not used), and the page does not say when the year began, so only 2025, the \
        year of the earliest budget it dates, is carried.",
    "\
        Ministry of Economy, Fiji, \"Budget Process\" and \"Budget Document\" (finance.gov.fj), read \
        2026-10-04"
}

government_year! {
/// Ghana — the calendar year, as two reference pages say.
    GHANA, "GH", "Ghana", "", "Ghana fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Ghana - Corporate - Tax administration\" (last reviewed \
        2026-03-11), gives the tax year as the calendar year, and Wikipedia's \"Economy of Ghana\" gives \
        the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, so the \
        entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Ghana, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Ghana\" (secondary), read 2026-10-04"
}

government_year! {
/// Hungary — the calendar year, by the Act on Public Finances, § 4.
    HUNGARY, "HU", "Hungary", "költségvetési év", "Hungarian budget year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2012,
    "\
        2011. évi CXCV. törvény az államháztartásról § 4(1): the year of budget execution is the same as \
        the calendar year (\"megegyezik a naptári évvel\"), read on net.jogtar.hu. The page gave only \
        the year 2011; the first whole year after enactment is 2012. Nothing read says what the year was \
        before.",
    "\
        2011. évi CXCV. törvény az államháztartásról, § 4 (net.jogtar.hu), read 2026-10-04"
}

government_year! {
/// Indonesia — the calendar year, by the State Finance Law, article 4.
    INDONESIA, "ID", "Indonesia", "tahun anggaran", "Indonesian budget year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2004,
    "\
        Undang-Undang No. 17 Tahun 2003 tentang Keuangan Negara, Pasal 4: \"Tahun Anggaran meliputi masa \
        satu tahun, mulai dari tanggal 1 Januari sampai dengan tanggal 31 Desember\", enacted on 5 April \
        2003, so the first whole year is 2004. Wikipedia's \"Fiscal year\" says the calendar year dates \
        from 2001; it cites no instrument.",
    "\
        Undang-Undang No. 17 Tahun 2003, Pasal 4 (jdih.kemenkeu.go.id), read 2026-10-04"
}

government_year! {
/// Israel — the calendar year, as two reference pages say.
    ISRAEL, "IL", "Israel", "", "Israel fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Israel - Corporate - Tax administration\" (last reviewed \
        2026-06-29), gives the tax year as the calendar year, and Wikipedia's \"Economy of Israel\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Israel, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Israel\" (secondary), read 2026-10-04"
}

government_year! {
/// Iraq — the calendar year, as two reference pages say.
    IRAQ, "IQ", "Iraq", "", "Iraq fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Iraq - Corporate - Tax administration\" (last reviewed \
        2026-06-24), gives the tax year as the calendar year, and Wikipedia's \"Economy of Iraq\" gives \
        the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, so the \
        entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Iraq, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Iraq\" (secondary), read 2026-10-04"
}

government_year! {
/// Iceland — the calendar year, by the Public Finance Act, art. 53.
    ICELAND, "IS", "Iceland", "reikningsár", "Icelandic accounting year of state bodies",
    Statute, (1, 1), LabelledByStartYear,
    None, 2016,
    "\
        Lög um opinber fjármál nr. 123/2015 art. 53: the accounting year of state bodies (\"reikningsár \
        ríkisaðila\") is the calendar year. It is the accounting year, not a clause on the budget year; \
        the law is of 28 December 2015, so the first whole year is 2016.",
    "\
        Lög um opinber fjármál nr. 123/2015, art. 53 (althingi.is), read 2026-10-04"
}

government_year! {
/// Italy — the calendar year, by the Legge di contabilità e finanza pubblica, art. 20.
    ITALY, "IT", "Italy", "anno finanziario", "Italian financial year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2010,
    "\
        Legge 31 dicembre 2009, n. 196, art. 20, Anno finanziario: the unit of time of management is the \
        year that begins on 1 January and ends on 31 December. In force from 1 January 2010 per the \
        page; the law before it was not read.",
    "\
        Legge 31 dicembre 2009, n. 196, art. 20 (normattiva.it), read 2026-10-04"
}

government_year! {
/// Jamaica — 1 April to 31 March, by the Constitution, section 1.
    JAMAICA, "JM", "Jamaica", "", "Jamaican financial year",
    Statute, (4, 1), LabelledByStartYear,
    None, 2015,
    "\
        The Constitution of Jamaica, section 1: \"the financial year\" means the twelve months ending on \
        31 March in any year or another date prescribed by Act, read in constituteproject.org's \
        \"Jamaica 1962 (rev. 2015)\", so the entry is read from the revision, 2015. The Ministry of \
        Finance's page names the budget \"2026/2027\" for the year from April 2026, which is the label \
        by the year it begins in; the Appropriations Acts are named for the calendar year of the Act, \
        which the page does not tie to a year.",
    "\
        Constitution of Jamaica (rev. 2015), s. 1 (constituteproject.org); Ministry of Finance and the \
        Public Service, budget page (mof.gov.jm), read 2026-10-04"
}

government_year! {
/// Jordan — the calendar year, as two reference pages say.
    JORDAN, "JO", "Jordan", "", "Jordan fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Jordan - Corporate - Tax administration\" (last reviewed \
        2026-07-05), gives the tax year as the calendar year, and Wikipedia's \"Economy of Jordan\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Jordan, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Jordan\" (secondary), read 2026-10-04"
}

government_year! {
/// South Korea — the calendar year, by the National Finance Act, article 2.
    SOUTH_KOREA, "KR", "South Korea", "회계연도", "South Korean fiscal year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2007,
    "\
        National Finance Act (Act No. 8050, promulgated 4 October 2006) art. 2: the State's fiscal year \
        commences on 1 January and ends on 31 December, read in the Korea Legislation Research \
        Institute's English text. The page gives no date in force; the first whole year after \
        promulgation is 2007.",
    "\
        National Finance Act, art. 2 (elaw.klri.re.kr), read 2026-10-04"
}

government_year! {
/// Kazakhstan — the calendar year, as two reference pages say.
    KAZAKHSTAN, "KZ", "Kazakhstan", "", "Kazakhstan fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Kazakhstan - Corporate - Tax administration\" (last reviewed \
        2026-07-23), gives the tax year as the calendar year, and Wikipedia's \"Economy of Kazakhstan\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Kazakhstan, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Kazakhstan\" (secondary), read 2026-10-04"
}

government_year! {
/// Lebanon — the calendar year, as two reference pages say.
    LEBANON, "LB", "Lebanon", "", "Lebanon fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Lebanon - Corporate - Tax administration\" (last reviewed \
        2026-07-01), gives the tax year as the calendar year, and Wikipedia's \"Economy of Lebanon\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Lebanon, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Lebanon\" (secondary), read 2026-10-04"
}

government_year! {
/// Latvia — the calendar year, by the Law on Budget and Financial Management, section 4.
    LATVIA, "LV", "Latvia", "finanšu gads", "Latvian state financial year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2003,
    "\
        Likums par budžetu un finanšu vadību (Law on Budget and Financial Management), section 4: the \
        financial year begins on 1 January and ends on 31 December, read in likumi.lv's English text, \
        adopted on 19 December 2002 and amended to 2025. The page also lists an earlier date, 1996, so \
        which text first held section 4 is not known; the first whole year after the adoption is 2003.",
    "\
        Law on Budget and Financial Management, section 4 (likumi.lv, English text), read 2026-10-04"
}

government_year! {
/// Mongolia — the calendar year, by the Budget Law, article 7.
    MONGOLIA, "MN", "Mongolia", "", "Mongolian fiscal year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2012,
    "\
        The Budget Law of Mongolia (dated 23 December 2011) art. 7: the fiscal year starts on 1 January \
        and ends on 31 December. The page gives no date in force, and a law of December 2011 may have \
        started in 2013, so 2012 is the earliest it can be.",
    "\
        Budget Law of Mongolia, art. 7 (legalinfo.mn), read 2026-10-04"
}

government_year! {
/// Mozambique — the calendar year, as two reference pages say.
    MOZAMBIQUE, "MZ", "Mozambique", "", "Mozambique fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Mozambique - Corporate - Tax administration\" (last reviewed \
        2026-08-11), gives the tax year as the calendar year, and Wikipedia's \"Economy of Mozambique\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Mozambique, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Mozambique\" (secondary), read 2026-10-04"
}

government_year! {
/// Nigeria — the calendar year, as two reference pages say.
    NIGERIA, "NG", "Nigeria", "", "Nigeria fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Nigeria - Corporate - Tax administration\" (last reviewed \
        2026-05-29), gives the tax year as the calendar year, and Wikipedia's \"Economy of Nigeria\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Nigeria, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Nigeria\" (secondary), read 2026-10-04"
}

government_year! {
/// Oman — the calendar year, as two reference pages say.
    OMAN, "OM", "Oman", "", "Oman fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Oman - Corporate - Tax administration\" (last reviewed \
        2026-07-07), gives the tax year as the calendar year, and Wikipedia's \"Economy of Oman\" gives \
        the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, so the \
        entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Oman, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Oman\" (secondary), read 2026-10-04"
}

government_year! {
/// The Philippines — the calendar year, by the Administrative Code of 1987.
    PHILIPPINES, "PH", "Philippines", "", "Philippine fiscal year",
    Statute, (1, 1), LabelledByStartYear,
    None, 1988,
    "\
        The Administrative Code of 1987, Book VI, Chapter 1, section 2(8): the fiscal year begins on the \
        first day of January and ends on the thirty-first of December, read on Wikisource's \
        transcription of the Official Gazette Supplement. The page gives no date in force; the first \
        whole year after the Code is 1988.",
    "\
        Administrative Code of 1987, Book VI, ch. 1, s. 2 (en.wikisource.org), read 2026-10-04"
}

government_year! {
/// Qatar — the calendar year since 2016, as a page of the Atlantic Council reports it.
    QATAR, "QA", "Qatar", "", "Qatari fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    Some(2016), 2016,
    "\
        The Atlantic Council's \"EconSource: Qatar Moves Fiscal Year-End, Reforms Budget Policy\" says \
        the year-end moved to 31 December from 31 March and the year ending in March 2015 was extended \
        to the end of 2015, so the first calendar year is 2016. No law is named on the page (a search \
        result named Law No. 2 of 2015, which was not read). The April year before it is not carried: no \
        page read gives how its years were named.",
    "\
        Atlantic Council, \"EconSource: Qatar Moves Fiscal Year-End, Reforms Budget Policy\" \
        (atlanticcouncil.org), read 2026-10-04"
}

government_year! {
/// Tunisia — the calendar year, as two reference pages say.
    TUNISIA, "TN", "Tunisia", "", "Tunisia fiscal year",
    Unread, (1, 1), LabelledByStartYear,
    None, 2026,
    "\
        PwC Worldwide Tax Summaries, \"Tunisia - Corporate - Tax administration\" (last reviewed \
        2026-06-29), gives the tax year as the calendar year, and Wikipedia's \"Economy of Tunisia\" \
        gives the fiscal year as the calendar year. No instrument was read, and both pages are of 2026, \
        so the entry is read from 2026.",
    "\
        PwC Worldwide Tax Summaries, Tunisia, Corporate, Tax administration (taxsummaries.pwc.com); \
        Wikipedia, \"Economy of Tunisia\" (secondary), read 2026-10-04"
}

government_year! {
/// Turkey — the calendar year, by Law No. 5018, article 3.
    TURKEY, "TR", "Turkey", "mali yıl", "Turkish fiscal year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2004,
    "\
        Kamu Mali Yönetimi ve Kontrol Kanunu (Law No. 5018), article 3(o): the mali yıl is the calendar \
        year, read on mevzuat.gov.tr; adopted on 10 December 2003. Its commencement was staged and was \
        not read, so 2004, the first whole year after adoption, is the first year carried.",
    "\
        Kamu Mali Yönetimi ve Kontrol Kanunu No. 5018, art. 3 (mevzuat.gov.tr), read 2026-10-04"
}

government_year! {
/// Taiwan — the calendar year, by the Budget Act, article 12.
    TAIWAN, "TW", "Taiwan", "會計年度", "Taiwanese fiscal year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2022,
    "\
        Budget Act art. 12: the government fiscal year begins on 1 January and ends on 31 December, and \
        is titled with the Republic of China year (the Minguo year, the Gregorian year less 1911, as the \
        repository's Minguo calendar has it; the page does not say so). The Act's text was read as \
        amended on 9 June 2021, so the entry is read from 2022; the original promulgation was not shown.",
    "\
        Budget Act, art. 12 (law.moj.gov.tw, English), read 2026-10-04"
}

government_year! {
/// Tanzania — 1 July to 30 June, as the Ministry of Finance's reports say.
    TANZANIA, "TZ", "Tanzania", "", "Tanzanian financial year",
    Unread, (7, 1), LabelledByStartYear,
    None, 2024,
    "\
        The Ministry of Finance's home page lists the budget execution report for \"the financial year \
        2025/26 (July–September 2025)\" and for \"the second quarter of 2024/25 (July to December \
        2024)\", the span led by the year it begins in; the Public Finance Act was not read (tanzlii.org \
        refused it), so the earliest year carried is 2024.",
    "\
        Ministry of Finance, Tanzania (mof.go.tz), read 2026-10-04"
}

government_year! {
/// Ukraine — the calendar year, by the Budget Code, article 3.
    UKRAINE, "UA", "Ukraine", "бюджетний період", "Ukrainian budget period",
    Statute, (1, 1), LabelledByStartYear,
    None, 2011,
    "\
        Бюджетний кодекс України of 8 July 2010 (No. 2456-VI), article 3: the budget period is one \
        calendar year, from 1 January to 31 December, read on zakon.rada.gov.ua. The date in force was \
        not read; the first whole year after the adoption is 2011.",
    "\
        Бюджетний кодекс України, article 3 (zakon.rada.gov.ua), read 2026-10-04"
}

government_year! {
/// Uzbekistan — the calendar year, by the Budget Code, article 3.
    UZBEKISTAN, "UZ", "Uzbekistan", "", "Uzbek financial year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2014,
    "\
        The Budget Code of the Republic of Uzbekistan (adopted 26 December 2013), article 3: the \
        financial year is the period from 1 January to 31 December inclusive; in force from 1 January \
        2014 per the page.",
    "\
        Budget Code of the Republic of Uzbekistan, art. 3 (lex.uz), read 2026-10-04"
}

government_year! {
/// Vietnam — the calendar year, by the Law on the State Budget, article 14.
    VIETNAM, "VN", "Vietnam", "năm ngân sách", "Vietnamese budget year",
    Statute, (1, 1), LabelledByStartYear,
    None, 2017,
    "\
        Law No. 83/2015/QH13 on the State Budget (25 June 2015), art. 14: the budget year starts on 1 \
        January and ends on 31 December. The page also says the law takes effect on 1 January 2007, \
        which cannot be right for a law of 2015, and no reliable date in force was read, so the entry is \
        read from 2017, the second year after promulgation: a later year is never earlier than the \
        truth.",
    "\
        Law No. 83/2015/QH13, art. 14 (english.luatvietnam.vn), read 2026-10-04"
}

government_year! {
/// Samoa — 1 July to 30 June, as the Ministry of Finance's documents say.
    SAMOA, "WS", "Samoa", "", "Samoan financial year",
    Unread, (7, 1), LabelledByStartYear,
    None, 2025,
    "\
        The Ministry of Finance's home page lists the \"Quarterly Financial Report 2025-2026 (First \
        Quarter: July to September 2025)\" and the \"Budget 2025/2026\", the span led by the year it \
        begins in. The Public Finance Management Act 2001 is a PDF and was not read, so the earliest \
        year carried is 2025.",
    "\
        Ministry of Finance, Samoa (mof.gov.ws), read 2026-10-04"
}
