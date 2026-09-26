//! The geological chart's interval names in other languages, as data.
//!
//! [`crate::geologic`] names every interval as the International
//! Chronostratigraphic Chart spells it in English. The International
//! Commission on Stratigraphy also publishes the chart in other languages,
//! and it publishes the names themselves as data: the chart's SKOS
//! vocabulary, `chart.ttl` in the repository
//! <https://github.com/i-c-stratigraphy/chart>, carries a `skos:prefLabel`
//! for every interval in twenty-six languages, built from the ICS's own
//! table of translations (`source/multilang.xlsx` there) [ics-chart-ttl].
//! The tables below are those labels for the fourteen languages that
//! `hc-i18n` also carries a locale for, read from commit
//! `81618a865cdb04998355a302f3e859908a080c0e` (2026-07-27) on 2026-09-26.
//! The data is © International Commission on Stratigraphy and licensed
//! CC BY 4.0; this module is the attribution.
//!
//! Two of the fourteen were checked against the translated charts the ICS
//! publishes, and agree with them label for label:
//!
//! * **Japanese** against the Geological Society of Japan's Japanese chart,
//!   国際年代層序表 v2024/12 (日本地質学会, 2025), which the ICS serves as
//!   `ChronostratChart2024-12Japanese.pdf` and the Society at
//!   <https://geosociety.jp/wp-content/uploads/2025/04/ChronostratChart_jp.pdf>
//!   [gsj-chart-2024-12]. Every Japanese label below is printed on it. The
//!   names are the Society's, in its own notation: `第四系／紀` is the
//!   Quaternary as a system (rock) and as a period (time) at once, and a
//!   stage is written in katakana without the 階／期 suffix.
//! * **Chinese** against the ICS's Chinese chart, 国际年代地层表 v2023/09
//!   (`ChronostratChart2023-09Chinese.pdf` at stratigraphy.org)
//!   [ics-chart-2023-09-zh]. Every Chinese label below is printed on it.
//!   The names are chronostratigraphic, 宇・界・系・统・阶, as the chart's
//!   are. The script is simplified, so the table serves `zh-Hans`; no
//!   traditional-script chart was read, and `zh-Hant` has no table rather
//!   than a converted one. The national stratigraphic commission's own
//!   table (全国地层委员会, 中国地层表) was not reached.
//!
//! The other twelve — Czech, German, Spanish, French, Indonesian, Italian,
//! Korean, Dutch, Polish, Portuguese, Russian and Turkish — are the ICS's
//! labels as published and were not compared with a printed chart.
//!
//! # What was left out, and why
//!
//! * The chart's vocabulary has no label in any language for the ratified
//!   subseries and the series named by position — Upper, Middle and Lower
//!   Cretaceous, Jurassic, Triassic, Pennsylvanian, Mississippian, Devonian
//!   and Ordovician, and the Upper Pleistocene — because the printed chart
//!   writes them as "Upper" in a column beside the period rather than as a
//!   name. Those twenty-one intervals are unnamed here in every language;
//!   composing a name from the parts would be a translation the ICS did not
//!   publish.
//! * A label in Latin letters in a language written in another script is an
//!   untranslated placeholder, not a name: the Russian labels of the
//!   Meghalayan, the Northgrippian and the Greenlandian are the English
//!   words. They are left out, and so is the Russian label of the Hadean,
//!   `Хaдейская`, whose second letter is a Latin `a` among Cyrillic ones.
//! * White space inside a label is collapsed, and a space between two CJK
//!   characters is removed: the chart's Chinese labels carry layout spaces
//!   (`显 生 宇`) that the printed chart does not.
//!
//! # Locales
//!
//! A tag is matched as `hc-i18n` matches one: case-insensitively, `_` read
//! as `-`, and by dropping subtags from the right until a table answers, so
//! `ja-JP` finds `ja` and `zh-Hans-CN` finds `zh-Hans`. There is no
//! likely-subtags expansion: plain `zh` finds nothing, as it finds no
//! Chinese vocabulary in `hc-i18n`.
//!
//! The cosmic epochs and events, the future eras and the archaeological
//! periods have no published translation that this crate has read, and are
//! not translated here.

use crate::geologic::{self, GeologicInterval, GeologicRank};

/// How many intervals the chart has, over every rank: the length of every
/// table here.
pub const INTERVAL_COUNT: usize = geologic::EONS.len()
    + geologic::ERAS.len()
    + geologic::PERIODS.len()
    + geologic::EPOCHS.len()
    + geologic::AGES.len();

/// Where the translations come from, for a `source` cell.
pub const SOURCE: &str = "International Commission on Stratigraphy, chart.ttl (github.com/i-c-stratigraphy/chart, \
     commit 81618a8, 2026-07-27, CC BY 4.0), retrieved 2026-09-26; Japanese checked against \
     日本地質学会 国際年代層序表 v2024/12, Chinese against the ICS 国际年代地层表 v2023/09";

/// One language's names for every interval, in the order of
/// [`geologic::EONS`], [`geologic::ERAS`], [`geologic::PERIODS`],
/// [`geologic::EPOCHS`] and [`geologic::AGES`], each youngest first; `""`
/// where the chart has no name.
#[derive(Debug, Clone, Copy)]
pub struct Translation {
    /// The BCP 47 tag, spelled as `hc-i18n` spells its locale.
    pub tag: &'static str,
    names: &'static [&'static str; INTERVAL_COUNT],
}

impl Translation {
    /// The name of the interval at `index` in chart order, if the chart
    /// has one.
    #[must_use]
    pub fn name_at(&self, index: usize) -> Option<&'static str> {
        self.names
            .get(index)
            .copied()
            .filter(|name| !name.is_empty())
    }

    /// The name of `interval` in this language, if the chart has one.
    #[must_use]
    pub fn name_of(&self, interval: &GeologicInterval) -> Option<&'static str> {
        chart_index(interval).and_then(|index| self.name_at(index))
    }
}

/// Every language with a table, in tag order.
pub static TRANSLATIONS: &[Translation] = &[
    Translation {
        tag: "cs",
        names: &CS,
    },
    Translation {
        tag: "de",
        names: &DE,
    },
    Translation {
        tag: "es",
        names: &ES,
    },
    Translation {
        tag: "fr",
        names: &FR,
    },
    Translation {
        tag: "id",
        names: &ID,
    },
    Translation {
        tag: "it",
        names: &IT,
    },
    Translation {
        tag: "ja",
        names: &JA,
    },
    Translation {
        tag: "ko",
        names: &KO,
    },
    Translation {
        tag: "nl",
        names: &NL,
    },
    Translation {
        tag: "pl",
        names: &PL,
    },
    Translation {
        tag: "pt",
        names: &PT,
    },
    Translation {
        tag: "ru",
        names: &RU,
    },
    Translation {
        tag: "tr",
        names: &TR,
    },
    Translation {
        tag: "zh-Hans",
        names: &ZH_HANS,
    },
];

/// Where `interval` sits in chart order, the order of every table here.
#[must_use]
pub fn chart_index(interval: &GeologicInterval) -> Option<usize> {
    let mut offset = 0;
    for rank in GeologicRank::ALL {
        let entries = geologic::intervals(*rank);
        if *rank == interval.rank {
            return entries
                .iter()
                .position(|entry| entry.name == interval.name)
                .map(|position| offset + position);
        }
        offset += entries.len();
    }
    None
}

/// Whether `tag` names the same locale as `candidate`, compared as
/// `hc-i18n` compares tags: ASCII case ignored, `_` read as `-`.
fn same_tag(tag: &str, candidate: &str) -> bool {
    tag.len() == candidate.len()
        && tag.bytes().zip(candidate.bytes()).all(|(a, b)| {
            let a = if a == b'_' { b'-' } else { a };
            a.eq_ignore_ascii_case(&b)
        })
}

/// The table for a BCP 47 tag, dropping subtags from the right until one
/// answers; `None` for a language with no table.
#[must_use]
pub fn translation(tag: &str) -> Option<&'static Translation> {
    let mut candidate = tag;
    loop {
        if candidate.is_empty() {
            return None;
        }
        if let Some(found) = TRANSLATIONS
            .iter()
            .find(|translation| same_tag(candidate, translation.tag))
        {
            return Some(found);
        }
        candidate = match candidate.rfind(['-', '_']) {
            Some(cut) => &candidate[..cut],
            None => "",
        };
    }
}

/// The name of `interval` in the language of `tag`, if the chart has one.
#[must_use]
pub fn interval_name(interval: &GeologicInterval, tag: &str) -> Option<&'static str> {
    translation(tag).and_then(|found| found.name_of(interval))
}

// --- The tables -----------------------------------------------------------
//
// Generated from chart.ttl by reading, for each interval, the concept whose
// English label is the interval's name (or whose identifier is its name
// without spaces, for the Cambrian's numbered series and stages, which the
// vocabulary labels "Series 2" and "Stage 10"), and taking its prefLabel in
// the language, cleaned as the module documentation says. Order: eons, eras,
// periods, epochs, ages, each youngest first, as in `geologic`.

/// Czech, `cs`: the chart's `@cs` labels.
const CS: [&str; INTERVAL_COUNT] = [
    "fanerozoikum",       // Phanerozoic
    "proterozoikum",      // Proterozoic
    "archaikum",          // Archean
    "hadaikum",           // Hadean
    "kenozoikum",         // Cenozoic
    "mesozoikum",         // Mesozoic
    "paleozoikum",        // Paleozoic
    "neoproterozoikum",   // Neoproterozoic
    "mesoproterozoikum",  // Mesoproterozoic
    "paleoproterozoikum", // Paleoproterozoic
    "neoarchaikum",       // Neoarchean
    "mesoarchaikum",      // Mesoarchean
    "paleoarchaikum",     // Paleoarchean
    "eoarchaikum",        // Eoarchean
    "kvartér",            // Quaternary
    "neogén",             // Neogene
    "paleogén",           // Paleogene
    "křída",              // Cretaceous
    "jura",               // Jurassic
    "trias",              // Triassic
    "perm",               // Permian
    "karbon",             // Carboniferous
    "devon",              // Devonian
    "silur",              // Silurian
    "ordovik",            // Ordovician
    "kambrium",           // Cambrian
    "ediakar",            // Ediacaran
    "kryogén",            // Cryogenian
    "ton",                // Tonian
    "sten",               // Stenian
    "ectas",              // Ectasian
    "calymm",             // Calymmian
    "stather",            // Statherian
    "orosir",             // Orosirian
    "rhyak",              // Rhyacian
    "sider",              // Siderian
    "holocén",            // Holocene
    "pleistocén",         // Pleistocene
    "pliocén",            // Pliocene
    "miocén",             // Miocene
    "oligocén",           // Oligocene
    "eocén",              // Eocene
    "paleocén",           // Paleocene
    "",                   // Upper Cretaceous
    "",                   // Lower Cretaceous
    "",                   // Upper Jurassic
    "",                   // Middle Jurassic
    "",                   // Lower Jurassic
    "",                   // Upper Triassic
    "",                   // Middle Triassic
    "",                   // Lower Triassic
    "loping",             // Lopingian
    "guadalup",           // Guadalupian
    "cisural",            // Cisuralian
    "",                   // Upper Pennsylvanian
    "",                   // Middle Pennsylvanian
    "",                   // Lower Pennsylvanian
    "",                   // Upper Mississippian
    "",                   // Middle Mississippian
    "",                   // Lower Mississippian
    "",                   // Upper Devonian
    "",                   // Middle Devonian
    "",                   // Lower Devonian
    "přídolí",            // Pridoli
    "ludlow",             // Ludlow
    "wenlock",            // Wenlock
    "llandovery",         // Llandovery
    "",                   // Upper Ordovician
    "",                   // Middle Ordovician
    "",                   // Lower Ordovician
    "furong",             // Furongian
    "miaoling",           // Miaolingian
    "oddělení 2",         // Cambrian Series 2
    "terreneuv",          // Terreneuvian
    "meghalayan",         // Meghalayan
    "northgrip",          // Northgrippian
    "greenland",          // Greenlandian
    "",                   // Upper Pleistocene
    "chiban",             // Chibanian
    "kalábr",             // Calabrian
    "gelas",              // Gelasian
    "piacenz",            // Piacenzian
    "zancl",              // Zanclean
    "messin",             // Messinian
    "torton",             // Tortonian
    "serravall",          // Serravallian
    "langh",              // Langhian
    "burdigal",           // Burdigalian
    "akvitán",            // Aquitanian
    "chatt",              // Chattian
    "rupel",              // Rupelian
    "priabon",            // Priabonian
    "barton",             // Bartonian
    "lutet",              // Lutetian
    "ypres",              // Ypresian
    "thanet",             // Thanetian
    "seland",             // Selandian
    "dan",                // Danian
    "maastricht",         // Maastrichtian
    "kampán",             // Campanian
    "santon",             // Santonian
    "coniak",             // Coniacian
    "turon",              // Turonian
    "cenoman",            // Cenomanian
    "alb",                // Albian
    "apt",                // Aptian
    "barrem",             // Barremian
    "hauteriv",           // Hauterivian
    "valangin",           // Valanginian
    "berrias",            // Berriasian
    "tithon",             // Tithonian
    "kimmeridge",         // Kimmeridgian
    "oxford",             // Oxfordian
    "callovian",          // Callovian
    "bathon",             // Bathonian
    "bajok",              // Bajocian
    "aalen",              // Aalenian
    "toark",              // Toarcian
    "pliensbach",         // Pliensbachian
    "sinemur",            // Sinemurian
    "hettang",            // Hettangian
    "rhét",               // Rhaetian
    "norik",              // Norian
    "karn",               // Carnian
    "ladin",              // Ladinian
    "anis",               // Anisian
    "olenek",             // Olenekian
    "ind",                // Induan
    "changhsing",         // Changhsingian
    "wuchiaping",         // Wuchiapingian
    "capitan",            // Capitanian
    "word",               // Wordian
    "road",               // Roadian
    "kungur",             // Kungurian
    "artinsk",            // Artinskian
    "sakmar",             // Sakmarian
    "assel",              // Asselian
    "gžel",               // Gzhelian
    "kasimov",            // Kasimovian
    "moskov",             // Moscovian
    "baškir",             // Bashkirian
    "serpuchov",          // Serpukhovian
    "visé",               // Visean
    "tournai",            // Tournaisian
    "famen",              // Famennian
    "frasn",              // Frasnian
    "givet",              // Givetian
    "eifel",              // Eifelian
    "ems",                // Emsian
    "prag",               // Pragian
    "lochkov",            // Lochkovian
    "ludford",            // Ludfordian
    "gorst",              // Gorstian
    "homer",              // Homerian
    "sheinwood",          // Sheinwoodian
    "telych",             // Telychian
    "aeron",              // Aeronian
    "rhuddan",            // Rhuddanian
    "hirnant",            // Hirnantian
    "katian",             // Katian
    "sandbian",           // Sandbian
    "darriwil",           // Darriwilian
    "daping",             // Dapingian
    "floian",             // Floian
    "tremadok",           // Tremadocian
    "stupeň 10",          // Cambrian Stage 10
    "jiangshan",          // Jiangshanian
    "paibian",            // Paibian
    "guzhang",            // Guzhangian
    "drum",               // Drumian
    "wuliuan",            // Wuliuan
    "stupeň 4",           // Cambrian Stage 4
    "stupeň 3",           // Cambrian Stage 3
    "stupeň 2",           // Cambrian Stage 2
    "fortun",             // Fortunian
];

/// German, `de`: the chart's `@de` labels.
const DE: [&str; INTERVAL_COUNT] = [
    "Phanerozoikum",      // Phanerozoic
    "Proterozoikum",      // Proterozoic
    "Archaikum",          // Archean
    "Hadaikum",           // Hadean
    "Känozoikum",         // Cenozoic
    "Mesozoikum",         // Mesozoic
    "Paläozoikum",        // Paleozoic
    "Neoproterozoikum",   // Neoproterozoic
    "Mesoproterozoikum",  // Mesoproterozoic
    "Paläoproterozoikum", // Paleoproterozoic
    "Neoarchaikum",       // Neoarchean
    "Mesoarchaikum",      // Mesoarchean
    "Paläoarchaikum",     // Paleoarchean
    "Eoarchaikum",        // Eoarchean
    "Quartär",            // Quaternary
    "Neogen",             // Neogene
    "Paläogen",           // Paleogene
    "Kreide",             // Cretaceous
    "Jura",               // Jurassic
    "Trias",              // Triassic
    "Perm",               // Permian
    "Karbon",             // Carboniferous
    "Devon",              // Devonian
    "Silur",              // Silurian
    "Ordovizium",         // Ordovician
    "Kambrium",           // Cambrian
    "Ediacarium",         // Ediacaran
    "Cryogenium",         // Cryogenian
    "Tonium",             // Tonian
    "Stenium",            // Stenian
    "Ectasium",           // Ectasian
    "Calymmium",          // Calymmian
    "Statherium",         // Statherian
    "Orosirium",          // Orosirian
    "Rhyacium",           // Rhyacian
    "Siderium",           // Siderian
    "Holozän",            // Holocene
    "Pleistozän",         // Pleistocene
    "Pliozän",            // Pliocene
    "Miozän",             // Miocene
    "Oligozän",           // Oligocene
    "Eozän",              // Eocene
    "Paläozän",           // Paleocene
    "",                   // Upper Cretaceous
    "",                   // Lower Cretaceous
    "",                   // Upper Jurassic
    "",                   // Middle Jurassic
    "",                   // Lower Jurassic
    "",                   // Upper Triassic
    "",                   // Middle Triassic
    "",                   // Lower Triassic
    "Lopingium",          // Lopingian
    "Guadalupium",        // Guadalupian
    "Cisuralium",         // Cisuralian
    "",                   // Upper Pennsylvanian
    "",                   // Middle Pennsylvanian
    "",                   // Lower Pennsylvanian
    "",                   // Upper Mississippian
    "",                   // Middle Mississippian
    "",                   // Lower Mississippian
    "",                   // Upper Devonian
    "",                   // Middle Devonian
    "",                   // Lower Devonian
    "Pridoli",            // Pridoli
    "Ludlow",             // Ludlow
    "Wenlock",            // Wenlock
    "Llandovery",         // Llandovery
    "",                   // Upper Ordovician
    "",                   // Middle Ordovician
    "",                   // Lower Ordovician
    "Furongium",          // Furongian
    "Serie 3",            // Miaolingian
    "Serie 2",            // Cambrian Series 2
    "Terreneuvium",       // Terreneuvian
    "Meghalayum",         // Meghalayan
    "Northgrippium",      // Northgrippian
    "Grönlandium",        // Greenlandian
    "",                   // Upper Pleistocene
    "Chibanium",          // Chibanian
    "Calabrium",          // Calabrian
    "Gelasium",           // Gelasian
    "Piacenzium",         // Piacenzian
    "Zancleum",           // Zanclean
    "Messinium",          // Messinian
    "Tortonium",          // Tortonian
    "Serravallium",       // Serravallian
    "Langhium",           // Langhian
    "Burdigalium",        // Burdigalian
    "Aquitanium",         // Aquitanian
    "Chattium",           // Chattian
    "Rupelium",           // Rupelian
    "Priabonium",         // Priabonian
    "Bartonium",          // Bartonian
    "Lutetium",           // Lutetian
    "Ypresium",           // Ypresian
    "Thanetium",          // Thanetian
    "Seelandium",         // Selandian
    "Danium",             // Danian
    "Maastrichtium",      // Maastrichtian
    "Campanium",          // Campanian
    "Santonium",          // Santonian
    "Coniacium",          // Coniacian
    "Turonium",           // Turonian
    "Cenomanium",         // Cenomanian
    "Albium",             // Albian
    "Aptium",             // Aptian
    "Barremium",          // Barremian
    "Hauterivium",        // Hauterivian
    "Valanginium",        // Valanginian
    "Berriasium",         // Berriasian
    "Tithonium",          // Tithonian
    "Kimmeridgium",       // Kimmeridgian
    "Oxfordium",          // Oxfordian
    "Callovium",          // Callovian
    "Bathonium",          // Bathonian
    "Bajocium",           // Bajocian
    "Aalenium",           // Aalenian
    "Toarcium",           // Toarcian
    "Pliensbachium",      // Pliensbachian
    "Sinemurium",         // Sinemurian
    "Hettangium",         // Hettangian
    "Rhaetium",           // Rhaetian
    "Norium",             // Norian
    "Karnium",            // Carnian
    "Ladinium",           // Ladinian
    "Anisium",            // Anisian
    "Olenekium",          // Olenekian
    "Indusium",           // Induan
    "Changhsingium",      // Changhsingian
    "Wuchiapingium",      // Wuchiapingian
    "Capitanium",         // Capitanian
    "Wordium",            // Wordian
    "Roadium",            // Roadian
    "Kungurium",          // Kungurian
    "Artinskium",         // Artinskian
    "Sakmarium",          // Sakmarian
    "Asselium",           // Asselian
    "Gzhelium",           // Gzhelian
    "Kasimovium",         // Kasimovian
    "Moskovium",          // Moscovian
    "Bashkirium",         // Bashkirian
    "Serpukhovium",       // Serpukhovian
    "Viseum",             // Visean
    "Tournaisium",        // Tournaisian
    "Famennium",          // Famennian
    "Frasnium",           // Frasnian
    "Givetium",           // Givetian
    "Eifelium",           // Eifelian
    "Emsium",             // Emsian
    "Pragium",            // Pragian
    "Lochkovium",         // Lochkovian
    "Ludfordium",         // Ludfordian
    "Gorstium",           // Gorstian
    "Homerium",           // Homerian
    "Sheinwoodium",       // Sheinwoodian
    "Telychium",          // Telychian
    "Aeronium",           // Aeronian
    "Rhuddanium",         // Rhuddanian
    "Hirnantium",         // Hirnantian
    "Katium",             // Katian
    "Sandbium",           // Sandbian
    "Darriwilium",        // Darriwilian
    "Dapingium",          // Dapingian
    "Floium",             // Floian
    "Tremadocium",        // Tremadocian
    "Stufe 10",           // Cambrian Stage 10
    "Jiangshanium",       // Jiangshanian
    "Paibium",            // Paibian
    "Guzhangium",         // Guzhangian
    "Drumium",            // Drumian
    "Stufe 5",            // Wuliuan
    "Stufe 4",            // Cambrian Stage 4
    "Stufe 3",            // Cambrian Stage 3
    "Stufe 2",            // Cambrian Stage 2
    "Fortunium",          // Fortunian
];

/// Spanish, `es`: the chart's `@es` labels.
const ES: [&str; INTERVAL_COUNT] = [
    "Fanerozoico",       // Phanerozoic
    "Proterozoico",      // Proterozoic
    "Arcaico",           // Archean
    "Hádico",            // Hadean
    "Cenozoico",         // Cenozoic
    "Mesozoico",         // Mesozoic
    "Paleozoico",        // Paleozoic
    "Neoproterozoico",   // Neoproterozoic
    "Mesoproterozoico",  // Mesoproterozoic
    "Paleoproterozoico", // Paleoproterozoic
    "Neoarcaico",        // Neoarchean
    "Mesoarcaico",       // Mesoarchean
    "Paleoarcaico",      // Paleoarchean
    "Eoarcaico",         // Eoarchean
    "Cuaternario",       // Quaternary
    "Neógeno",           // Neogene
    "Paleógeno",         // Paleogene
    "Cretácico",         // Cretaceous
    "Jurásico",          // Jurassic
    "Triásico",          // Triassic
    "Pérmico",           // Permian
    "Carbonífero",       // Carboniferous
    "Devónico",          // Devonian
    "Silúrico",          // Silurian
    "Ordovícico",        // Ordovician
    "Cámbrico",          // Cambrian
    "Ediacárico",        // Ediacaran
    "Criogénico",        // Cryogenian
    "Tónico",            // Tonian
    "Esténico",          // Stenian
    "Ectásico",          // Ectasian
    "Calímico",          // Calymmian
    "Estatérico",        // Statherian
    "Orosírico",         // Orosirian
    "Riácico",           // Rhyacian
    "Sidérico",          // Siderian
    "Holoceno",          // Holocene
    "Pleistocene",       // Pleistocene
    "Plioceno",          // Pliocene
    "Mioceno",           // Miocene
    "Oligoceno",         // Oligocene
    "Eoceno",            // Eocene
    "Paleoceno",         // Paleocene
    "",                  // Upper Cretaceous
    "",                  // Lower Cretaceous
    "",                  // Upper Jurassic
    "",                  // Middle Jurassic
    "",                  // Lower Jurassic
    "",                  // Upper Triassic
    "",                  // Middle Triassic
    "",                  // Lower Triassic
    "Lopingiense",       // Lopingian
    "Guadalupiense",     // Guadalupian
    "Cisuraliense",      // Cisuralian
    "",                  // Upper Pennsylvanian
    "",                  // Middle Pennsylvanian
    "",                  // Lower Pennsylvanian
    "",                  // Upper Mississippian
    "",                  // Middle Mississippian
    "",                  // Lower Mississippian
    "",                  // Upper Devonian
    "",                  // Middle Devonian
    "",                  // Lower Devonian
    "Prídoli",           // Pridoli
    "Ludlow",            // Ludlow
    "Wenlock",           // Wenlock
    "Llandovery",        // Llandovery
    "",                  // Upper Ordovician
    "",                  // Middle Ordovician
    "",                  // Lower Ordovician
    "Furongiense",       // Furongian
    "Miaolingiense",     // Miaolingian
    "Serie 2",           // Cambrian Series 2
    "Terreneuviense",    // Terreneuvian
    "Megalayense",       // Meghalayan
    "Norgripiense",      // Northgrippian
    "Groenlandiense",    // Greenlandian
    "",                  // Upper Pleistocene
    "Chibaniense",       // Chibanian
    "Calabriense",       // Calabrian
    "Gelasiense",        // Gelasian
    "Piacenziense",      // Piacenzian
    "Zancliense",        // Zanclean
    "Messiniense",       // Messinian
    "Tortoniense",       // Tortonian
    "Serravalliense",    // Serravallian
    "Langhiense",        // Langhian
    "Burdigaliense",     // Burdigalian
    "Aquitaniense",      // Aquitanian
    "Chattiense",        // Chattian
    "Rupeliense",        // Rupelian
    "Priaboniense",      // Priabonian
    "Bartoniense",       // Bartonian
    "Luteciense",        // Lutetian
    "Ypresiense",        // Ypresian
    "Thanetiense",       // Thanetian
    "Selandiense",       // Selandian
    "Daniense",          // Danian
    "Maastrichtiense",   // Maastrichtian
    "Campaniense",       // Campanian
    "Santoniense",       // Santonian
    "Coniaciense",       // Coniacian
    "Turoniense",        // Turonian
    "Cenomaniense",      // Cenomanian
    "Albiense",          // Albian
    "Aptiense",          // Aptian
    "Barremiense",       // Barremian
    "Hauteriviense",     // Hauterivian
    "Valanginiense",     // Valanginian
    "Berriasiense",      // Berriasian
    "Titoniense",        // Tithonian
    "Kimmeridgiense",    // Kimmeridgian
    "Oxfordiense",       // Oxfordian
    "Calloviense",       // Callovian
    "Bathoniense",       // Bathonian
    "Bajociense",        // Bajocian
    "Aaleniense",        // Aalenian
    "Toarciense",        // Toarcian
    "Pliensbachiense",   // Pliensbachian
    "Sinemuriense",      // Sinemurian
    "Hettangiense",      // Hettangian
    "Rhaetiense",        // Rhaetian
    "Noriense",          // Norian
    "Carniense",         // Carnian
    "Ladiniense",        // Ladinian
    "Anisiense",         // Anisian
    "Olenekiense",       // Olenekian
    "Induense",          // Induan
    "Changhsingiense",   // Changhsingian
    "Wuchiapingiense",   // Wuchiapingian
    "Capitaniense",      // Capitanian
    "Wordiense",         // Wordian
    "Roadiense",         // Roadian
    "Kunguriense",       // Kungurian
    "Artinskiense",      // Artinskian
    "Sakmariense",       // Sakmarian
    "Asseliense",        // Asselian
    "Gzheliense",        // Gzhelian
    "Kasimoviense",      // Kasimovian
    "Moscoviense",       // Moscovian
    "Bashkiriense",      // Bashkirian
    "Serpukhoviense",    // Serpukhovian
    "Viseense",          // Visean
    "Tournaisiense",     // Tournaisian
    "Fameniense",        // Famennian
    "Frasniense",        // Frasnian
    "Givetiense",        // Givetian
    "Eifeliense",        // Eifelian
    "Emsiense",          // Emsian
    "Pragiense",         // Pragian
    "Lochkoviense",      // Lochkovian
    "Ludfordiense",      // Ludfordian
    "Gorstiense",        // Gorstian
    "Homeriense",        // Homerian
    "Sheinwoodiense",    // Sheinwoodian
    "Telychiense",       // Telychian
    "Aeroniense",        // Aeronian
    "Rhuddaniense",      // Rhuddanian
    "Hirnantiense",      // Hirnantian
    "Katiense",          // Katian
    "Sandbiense",        // Sandbian
    "Darriwiliense",     // Darriwilian
    "Dapingiense",       // Dapingian
    "Floiense",          // Floian
    "Tremadociense",     // Tremadocian
    "Piso 10",           // Cambrian Stage 10
    "Jiangshaniense",    // Jiangshanian
    "Paibiense",         // Paibian
    "Guzhangiense",      // Guzhangian
    "Drumiense",         // Drumian
    "Wuliuense",         // Wuliuan
    "Piso 4",            // Cambrian Stage 4
    "Piso 3",            // Cambrian Stage 3
    "Piso 2",            // Cambrian Stage 2
    "Fortuniense",       // Fortunian
];

/// French, `fr`: the chart's `@fr` labels.
const FR: [&str; INTERVAL_COUNT] = [
    "Phanérozoïque",      // Phanerozoic
    "Protéozoïque",       // Proterozoic
    "Archéen",            // Archean
    "Hadéen",             // Hadean
    "Cénozoïque",         // Cenozoic
    "Mésozoïque",         // Mesozoic
    "Paléozoïque",        // Paleozoic
    "Néoprotérozoïque",   // Neoproterozoic
    "Mésoprotérozoïque",  // Mesoproterozoic
    "Paléoprotérozoïque", // Paleoproterozoic
    "Néoarchéen",         // Neoarchean
    "Mésoarchéen",        // Mesoarchean
    "Paléoarchéen",       // Paleoarchean
    "Éoarchéen",          // Eoarchean
    "Quaternaire",        // Quaternary
    "Néogène",            // Neogene
    "Paléogène",          // Paleogene
    "Crétacé",            // Cretaceous
    "Jurassique",         // Jurassic
    "Trias",              // Triassic
    "Permien",            // Permian
    "Carbonifère",        // Carboniferous
    "Dévonien",           // Devonian
    "Silurien",           // Silurian
    "Ordovicien",         // Ordovician
    "Cambrien",           // Cambrian
    "Édiacarien",         // Ediacaran
    "Cryogénien",         // Cryogenian
    "Tonien",             // Tonian
    "Sténien",            // Stenian
    "Ectasien",           // Ectasian
    "Calymmien",          // Calymmian
    "Stathérien",         // Statherian
    "Orosirien",          // Orosirian
    "Rhyacien",           // Rhyacian
    "Sidérien",           // Siderian
    "Holocène",           // Holocene
    "Pléistocène",        // Pleistocene
    "Pliocène",           // Pliocene
    "Miocène",            // Miocene
    "Oligocène",          // Oligocene
    "Éocène",             // Eocene
    "Paléocène",          // Paleocene
    "",                   // Upper Cretaceous
    "",                   // Lower Cretaceous
    "",                   // Upper Jurassic
    "",                   // Middle Jurassic
    "",                   // Lower Jurassic
    "",                   // Upper Triassic
    "",                   // Middle Triassic
    "",                   // Lower Triassic
    "Lopingien",          // Lopingian
    "Guadalupien",        // Guadalupian
    "Cisuralien",         // Cisuralian
    "",                   // Upper Pennsylvanian
    "",                   // Middle Pennsylvanian
    "",                   // Lower Pennsylvanian
    "",                   // Upper Mississippian
    "",                   // Middle Mississippian
    "",                   // Lower Mississippian
    "",                   // Upper Devonian
    "",                   // Middle Devonian
    "",                   // Lower Devonian
    "Pridoli",            // Pridoli
    "Ludlow",             // Ludlow
    "Wenlock",            // Wenlock
    "Llandovery",         // Llandovery
    "",                   // Upper Ordovician
    "",                   // Middle Ordovician
    "",                   // Lower Ordovician
    "Furongien",          // Furongian
    "Miaolingien",        // Miaolingian
    "Séries 2",           // Cambrian Series 2
    "Terreneuvien",       // Terreneuvian
    "Meghalayen",         // Meghalayan
    "Northgrippien",      // Northgrippian
    "Greenlandien",       // Greenlandian
    "",                   // Upper Pleistocene
    "Chibanien",          // Chibanian
    "Calabrien",          // Calabrian
    "Gélasien",           // Gelasian
    "Plaisancien",        // Piacenzian
    "Zancléen",           // Zanclean
    "Messinien",          // Messinian
    "Tortonien",          // Tortonian
    "Serravallien",       // Serravallian
    "Langhien",           // Langhian
    "Burdigalien",        // Burdigalian
    "Aquitanien",         // Aquitanian
    "Chattien",           // Chattian
    "Rupélien",           // Rupelian
    "Priabonien",         // Priabonian
    "Bartonien",          // Bartonian
    "Lutétien",           // Lutetian
    "Yprésien",           // Ypresian
    "Thanétien",          // Thanetian
    "Sélandien",          // Selandian
    "Danien",             // Danian
    "Maastrichtien",      // Maastrichtian
    "Campanien",          // Campanian
    "Santonien",          // Santonian
    "Coniacien",          // Coniacian
    "Turonien",           // Turonian
    "Cénomanien",         // Cenomanian
    "Albien",             // Albian
    "Aptien",             // Aptian
    "Barrémien",          // Barremian
    "Hauterivien",        // Hauterivian
    "Valanginien",        // Valanginian
    "Berriasien",         // Berriasian
    "Tithonien",          // Tithonian
    "Kimméridgien",       // Kimmeridgian
    "Oxfordien",          // Oxfordian
    "Callovien",          // Callovian
    "Bathonien",          // Bathonian
    "Bajocien",           // Bajocian
    "Aalénien",           // Aalenian
    "Toarcien",           // Toarcian
    "Pliensbachien",      // Pliensbachian
    "Sinémurien",         // Sinemurian
    "Hettangien",         // Hettangian
    "Rhétien",            // Rhaetian
    "Norien",             // Norian
    "Carnien",            // Carnian
    "Ladinien",           // Ladinian
    "Anisien",            // Anisian
    "Olénékien",          // Olenekian
    "Indusien",           // Induan
    "Changhsingien",      // Changhsingian
    "Wuchiapingien",      // Wuchiapingian
    "Capitanien",         // Capitanian
    "Wordien",            // Wordian
    "Roadien",            // Roadian
    "Koungourien",        // Kungurian
    "Artinskien",         // Artinskian
    "Sakmarien",          // Sakmarian
    "Assélien",           // Asselian
    "Gzhélien",           // Gzhelian
    "Kasimovien",         // Kasimovian
    "Moscovien",          // Moscovian
    "Bashkirien",         // Bashkirian
    "Serpukhovien",       // Serpukhovian
    "Viséen",             // Visean
    "Tournaisien",        // Tournaisian
    "Famennien",          // Famennian
    "Frasnien",           // Frasnian
    "Givétien",           // Givetian
    "Eifélien",           // Eifelian
    "Emsien",             // Emsian
    "Pragien",            // Pragian
    "Lochkovien",         // Lochkovian
    "Ludfordien",         // Ludfordian
    "Gorstien",           // Gorstian
    "Homérien",           // Homerian
    "Sheinwoodien",       // Sheinwoodian
    "Télychien",          // Telychian
    "Aéronien",           // Aeronian
    "Rhuddanien",         // Rhuddanian
    "Hirnantien",         // Hirnantian
    "Katien",             // Katian
    "Sandbien",           // Sandbian
    "Darriwilien",        // Darriwilian
    "Dapingien",          // Dapingian
    "Floien",             // Floian
    "Trémadocien",        // Tremadocian
    "Étage 10",           // Cambrian Stage 10
    "Jiangshanien",       // Jiangshanian
    "Paibien",            // Paibian
    "Guzhangien",         // Guzhangian
    "Drumien",            // Drumian
    "Wuliuen",            // Wuliuan
    "Étage 4",            // Cambrian Stage 4
    "Étage 3",            // Cambrian Stage 3
    "Étage 2",            // Cambrian Stage 2
    "Fortunien",          // Fortunian
];

/// Indonesian, `id`: the chart's `@id` labels.
const ID: [&str; INTERVAL_COUNT] = [
    "Fanerozoikum",       // Phanerozoic
    "Proterozoikum",      // Proterozoic
    "Archean",            // Archean
    "Hadean",             // Hadean
    "Kenozoikum",         // Cenozoic
    "Mesozoikum",         // Mesozoic
    "Paleozoikum",        // Paleozoic
    "Neoproterozoikum",   // Neoproterozoic
    "Mesoproterozoikum",  // Mesoproterozoic
    "Paleoproterozoikum", // Paleoproterozoic
    "Neoarchean",         // Neoarchean
    "Mesoarchean",        // Mesoarchean
    "Paleoarchean",       // Paleoarchean
    "Eoarchean",          // Eoarchean
    "Kuarter",            // Quaternary
    "Neogen",             // Neogene
    "Paleogen",           // Paleogene
    "Kapur",              // Cretaceous
    "Jura",               // Jurassic
    "Trias",              // Triassic
    "Perem",              // Permian
    "Karbon",             // Carboniferous
    "Devon",              // Devonian
    "Silur",              // Silurian
    "Ordovisiu",          // Ordovician
    "Kambrium",           // Cambrian
    "Ediacaran",          // Ediacaran
    "Cryogenian",         // Cryogenian
    "Tonian",             // Tonian
    "Stenian",            // Stenian
    "Ectasian",           // Ectasian
    "Calymmian",          // Calymmian
    "Statherian",         // Statherian
    "Orosirian",          // Orosirian
    "Rhyacian",           // Rhyacian
    "Siderian",           // Siderian
    "Holosen",            // Holocene
    "Pleistosen",         // Pleistocene
    "Pliosen",            // Pliocene
    "Miosen",             // Miocene
    "Oligosen",           // Oligocene
    "Eosen",              // Eocene
    "Paleosen",           // Paleocene
    "",                   // Upper Cretaceous
    "",                   // Lower Cretaceous
    "",                   // Upper Jurassic
    "",                   // Middle Jurassic
    "",                   // Lower Jurassic
    "",                   // Upper Triassic
    "",                   // Middle Triassic
    "",                   // Lower Triassic
    "Lopingian",          // Lopingian
    "Guadalupian",        // Guadalupian
    "Cisuralian",         // Cisuralian
    "",                   // Upper Pennsylvanian
    "",                   // Middle Pennsylvanian
    "",                   // Lower Pennsylvanian
    "",                   // Upper Mississippian
    "",                   // Middle Mississippian
    "",                   // Lower Mississippian
    "",                   // Upper Devonian
    "",                   // Middle Devonian
    "",                   // Lower Devonian
    "Pridoli",            // Pridoli
    "Ludlow",             // Ludlow
    "Wenlock",            // Wenlock
    "Llandovery",         // Llandovery
    "",                   // Upper Ordovician
    "",                   // Middle Ordovician
    "",                   // Lower Ordovician
    "Furongian",          // Furongian
    "Miaolingian",        // Miaolingian
    "Seri 2",             // Cambrian Series 2
    "Terreneuvian",       // Terreneuvian
    "Meghalayan",         // Meghalayan
    "Northgrippian",      // Northgrippian
    "Greenlandian",       // Greenlandian
    "",                   // Upper Pleistocene
    "Chibanian",          // Chibanian
    "Calabrian",          // Calabrian
    "Gelasian",           // Gelasian
    "Piacenzian",         // Piacenzian
    "Zanclean",           // Zanclean
    "Messinian",          // Messinian
    "Tortonian",          // Tortonian
    "Serravallian",       // Serravallian
    "Langhian",           // Langhian
    "Burdigalian",        // Burdigalian
    "Aquitanian",         // Aquitanian
    "Chattian",           // Chattian
    "Rupelian",           // Rupelian
    "Priabonian",         // Priabonian
    "Bartonian",          // Bartonian
    "Lutetian",           // Lutetian
    "Ypresian",           // Ypresian
    "Thanetian",          // Thanetian
    "Selandian",          // Selandian
    "Danian",             // Danian
    "Maastrichtian",      // Maastrichtian
    "Campanian",          // Campanian
    "Santonian",          // Santonian
    "Coniacian",          // Coniacian
    "Turonian",           // Turonian
    "Cenomanian",         // Cenomanian
    "Albian",             // Albian
    "Aptian",             // Aptian
    "Barremian",          // Barremian
    "Hauterivian",        // Hauterivian
    "Valanginian",        // Valanginian
    "Berriasian",         // Berriasian
    "Tithonian",          // Tithonian
    "Kimmeridgian",       // Kimmeridgian
    "Oxfordian",          // Oxfordian
    "Callovian",          // Callovian
    "Bathonian",          // Bathonian
    "Bajocian",           // Bajocian
    "Aalenian",           // Aalenian
    "Toarcian",           // Toarcian
    "Pliensbachian",      // Pliensbachian
    "Sinemurian",         // Sinemurian
    "Hettangian",         // Hettangian
    "Rhaetian",           // Rhaetian
    "Norian",             // Norian
    "Carnian",            // Carnian
    "Ladinian",           // Ladinian
    "Anisian",            // Anisian
    "Olenekian",          // Olenekian
    "Induan",             // Induan
    "Changhsingian",      // Changhsingian
    "Wuchiapingian",      // Wuchiapingian
    "Capitanian",         // Capitanian
    "Wordian",            // Wordian
    "Roadian",            // Roadian
    "Kungurian",          // Kungurian
    "Artinskian",         // Artinskian
    "Sakmarian",          // Sakmarian
    "Asselian",           // Asselian
    "Gzhelian",           // Gzhelian
    "Kasimovian",         // Kasimovian
    "Moscovian",          // Moscovian
    "Bashkirian",         // Bashkirian
    "Serpukhovian",       // Serpukhovian
    "Visean",             // Visean
    "Tournaisian",        // Tournaisian
    "Famennian",          // Famennian
    "Frasnian",           // Frasnian
    "Givetian",           // Givetian
    "Eifelian",           // Eifelian
    "Emsian",             // Emsian
    "Pragian",            // Pragian
    "Lochkovian",         // Lochkovian
    "Ludfordian",         // Ludfordian
    "Gorstian",           // Gorstian
    "Homerian",           // Homerian
    "Sheinwoodian",       // Sheinwoodian
    "Telychian",          // Telychian
    "Aeronian",           // Aeronian
    "Rhuddanian",         // Rhuddanian
    "Hirnantian",         // Hirnantian
    "Katian",             // Katian
    "Sandbian",           // Sandbian
    "Darriwilian",        // Darriwilian
    "Dapingian",          // Dapingian
    "Floian",             // Floian
    "Tremadocian",        // Tremadocian
    "Jenjang 10",         // Cambrian Stage 10
    "Jiangshanian",       // Jiangshanian
    "Paibian",            // Paibian
    "Guzhangian",         // Guzhangian
    "Drumian",            // Drumian
    "Wuliuan",            // Wuliuan
    "Jenjang 4",          // Cambrian Stage 4
    "Jenjang 3",          // Cambrian Stage 3
    "Jenjang 2",          // Cambrian Stage 2
    "Fortunian",          // Fortunian
];

/// Italian, `it`: the chart's `@it` labels.
const IT: [&str; INTERVAL_COUNT] = [
    "Fanerozoico",       // Phanerozoic
    "Proterozoico",      // Proterozoic
    "Archeano",          // Archean
    "Adeano",            // Hadean
    "Cenozoico",         // Cenozoic
    "Mesozoico",         // Mesozoic
    "Paleozoico",        // Paleozoic
    "Neoproterozoico",   // Neoproterozoic
    "Mesoproterozoico",  // Mesoproterozoic
    "Paleoproterozoico", // Paleoproterozoic
    "Neoarcheano",       // Neoarchean
    "Mesoarcheano",      // Mesoarchean
    "Paleoarcheano",     // Paleoarchean
    "Eoarcheano",        // Eoarchean
    "Quaternario",       // Quaternary
    "Neogene",           // Neogene
    "Paleogene",         // Paleogene
    "Cretacico",         // Cretaceous
    "Giurassico",        // Jurassic
    "Triassico",         // Triassic
    "Permiano",          // Permian
    "Carbonifero",       // Carboniferous
    "Devoniano",         // Devonian
    "Siluriano",         // Silurian
    "Ordoviciano",       // Ordovician
    "Cambriano",         // Cambrian
    "Ediacarano",        // Ediacaran
    "Criogeniano",       // Cryogenian
    "Toniano",           // Tonian
    "Steniano",          // Stenian
    "Ectasiano",         // Ectasian
    "Calimmiano",        // Calymmian
    "Statheriano",       // Statherian
    "Orosiriano",        // Orosirian
    "Riaciano",          // Rhyacian
    "Sideriano",         // Siderian
    "Olocene",           // Holocene
    "Pleistocene",       // Pleistocene
    "Pliocene",          // Pliocene
    "Miocene",           // Miocene
    "Oligocene",         // Oligocene
    "Eocene",            // Eocene
    "Paleocene",         // Paleocene
    "",                  // Upper Cretaceous
    "",                  // Lower Cretaceous
    "",                  // Upper Jurassic
    "",                  // Middle Jurassic
    "",                  // Lower Jurassic
    "",                  // Upper Triassic
    "",                  // Middle Triassic
    "",                  // Lower Triassic
    "Lopingiano",        // Lopingian
    "Guadalupiano",      // Guadalupian
    "Cisuraliano",       // Cisuralian
    "",                  // Upper Pennsylvanian
    "",                  // Middle Pennsylvanian
    "",                  // Lower Pennsylvanian
    "",                  // Upper Mississippian
    "",                  // Middle Mississippian
    "",                  // Lower Mississippian
    "",                  // Upper Devonian
    "",                  // Middle Devonian
    "",                  // Lower Devonian
    "Pridoli",           // Pridoli
    "Ludlow",            // Ludlow
    "Wenlock",           // Wenlock
    "Llandoveriano",     // Llandovery
    "",                  // Upper Ordovician
    "",                  // Middle Ordovician
    "",                  // Lower Ordovician
    "Furongiano",        // Furongian
    "Miaolingiano",      // Miaolingian
    "Serie 2",           // Cambrian Series 2
    "Terreneuviano",     // Terreneuvian
    "Meghalayano",       // Meghalayan
    "Nordgrippiano",     // Northgrippian
    "Groenlandiano",     // Greenlandian
    "",                  // Upper Pleistocene
    "Chibaniano",        // Chibanian
    "Calabriano",        // Calabrian
    "Gelasiano",         // Gelasian
    "Piacenziano",       // Piacenzian
    "Zancleano",         // Zanclean
    "Messiniano",        // Messinian
    "Tortoniano",        // Tortonian
    "Serravalliano",     // Serravallian
    "Langhiano",         // Langhian
    "Burdigaliano",      // Burdigalian
    "Aquitaniano",       // Aquitanian
    "Cattiano",          // Chattian
    "Rupeliano",         // Rupelian
    "Priaboniano",       // Priabonian
    "Bartoniano",        // Bartonian
    "Luteziano",         // Lutetian
    "Ypresiano",         // Ypresian
    "Thanetiano",        // Thanetian
    "Selandiano",        // Selandian
    "Daniano",           // Danian
    "Maastrichtiano",    // Maastrichtian
    "Campaniano",        // Campanian
    "Santoniano",        // Santonian
    "Coniaciano",        // Coniacian
    "Turoniano",         // Turonian
    "Cenomaniano",       // Cenomanian
    "Albiano",           // Albian
    "Aptiano",           // Aptian
    "Barremiano",        // Barremian
    "Hauteriviano",      // Hauterivian
    "Valanginiano",      // Valanginian
    "Berriasiano",       // Berriasian
    "Titoniano",         // Tithonian
    "Kimmeridgiano",     // Kimmeridgian
    "Oxfordiano",        // Oxfordian
    "Calloviano",        // Callovian
    "Bathoniano",        // Bathonian
    "Bajociano",         // Bajocian
    "Aaleniano",         // Aalenian
    "Toarciano",         // Toarcian
    "Pliensbachiano",    // Pliensbachian
    "Sinemuriano",       // Sinemurian
    "Hettangiano",       // Hettangian
    "Retico",            // Rhaetian
    "Norico",            // Norian
    "Carnico",           // Carnian
    "Ladinico",          // Ladinian
    "Anisico",           // Anisian
    "Olenekiano",        // Olenekian
    "Induano",           // Induan
    "Changhsingiano",    // Changhsingian
    "Wuchiapingiano",    // Wuchiapingian
    "Capitaniano",       // Capitanian
    "Wordiano",          // Wordian
    "Roadiano",          // Roadian
    "Kunguriano",        // Kungurian
    "Artinskiano",       // Artinskian
    "Sakmariano",        // Sakmarian
    "Asseliano",         // Asselian
    "Gzheliano",         // Gzhelian
    "Kasimoviano",       // Kasimovian
    "Moscoviano",        // Moscovian
    "Bashkiriano",       // Bashkirian
    "Serpukhoviano",     // Serpukhovian
    "Viseano",           // Visean
    "Tournaisiano",      // Tournaisian
    "Famenniano",        // Famennian
    "Frasniano",         // Frasnian
    "Givetiano",         // Givetian
    "Eifeliano",         // Eifelian
    "Emsiano",           // Emsian
    "Pragiano",          // Pragian
    "Lochkoviano",       // Lochkovian
    "Ludfordiano",       // Ludfordian
    "Gorstiano",         // Gorstian
    "Homeriano",         // Homerian
    "Sheinwoodiano",     // Sheinwoodian
    "Telychiano",        // Telychian
    "Aeroniano",         // Aeronian
    "Rhuddaniano",       // Rhuddanian
    "Hirnantiano",       // Hirnantian
    "Katiano",           // Katian
    "Sandbiano",         // Sandbian
    "Darriwiliano",      // Darriwilian
    "Dapingiano",        // Dapingian
    "Floiano",           // Floian
    "Tremadociano",      // Tremadocian
    "Piano 10",          // Cambrian Stage 10
    "Jiangshaniano",     // Jiangshanian
    "Paibiano",          // Paibian
    "Guzhangiano",       // Guzhangian
    "Drumiano",          // Drumian
    "Wuliuano",          // Wuliuan
    "Piano 4",           // Cambrian Stage 4
    "Piano 3",           // Cambrian Stage 3
    "Piano 2",           // Cambrian Stage 2
    "Fortuniano",        // Fortunian
];

/// Japanese, `ja`: the chart's `@ja` labels.
const JA: [&str; INTERVAL_COUNT] = [
    "顕生（累）界／代",                     // Phanerozoic
    "原生（累）界／代",                     // Proterozoic
    "太古（累）界／代（始生（累）界／代）", // Archean
    "冥王界／代",                           // Hadean
    "新生界／代",                           // Cenozoic
    "中生界／代",                           // Mesozoic
    "古生界／代",                           // Paleozoic
    "新原生界／代",                         // Neoproterozoic
    "中原生界／代",                         // Mesoproterozoic
    "古原生界／代",                         // Paleoproterozoic
    "新太古界／代（新始生界／代）",         // Neoarchean
    "中太古界／代（中始生界／代）",         // Mesoarchean
    "古太古界／代（古始生界／代）",         // Paleoarchean
    "原太古界／代（原始生界／代）",         // Eoarchean
    "第四系／紀",                           // Quaternary
    "新第三系／紀",                         // Neogene
    "古第三系／紀",                         // Paleogene
    "白亜系／紀",                           // Cretaceous
    "ジュラ系／紀",                         // Jurassic
    "三畳系／紀",                           // Triassic
    "ペルム系／紀",                         // Permian
    "石炭系／紀",                           // Carboniferous
    "デボン系／紀",                         // Devonian
    "シルル系／紀",                         // Silurian
    "オルドビス系／紀",                     // Ordovician
    "カンブリア系／紀",                     // Cambrian
    "エディアカラン",                       // Ediacaran
    "クライオジェニアン",                   // Cryogenian
    "トニアン",                             // Tonian
    "ステニアン",                           // Stenian
    "エクタシアン",                         // Ectasian
    "カリミアン",                           // Calymmian
    "スタテリアン",                         // Statherian
    "オロシリアン",                         // Orosirian
    "リィアキアン",                         // Rhyacian
    "シデリアン",                           // Siderian
    "完新統／世",                           // Holocene
    "更新統／世",                           // Pleistocene
    "鮮新統／世",                           // Pliocene
    "中新統／世",                           // Miocene
    "漸新統／世",                           // Oligocene
    "始新統／世",                           // Eocene
    "暁新統／世",                           // Paleocene
    "",                                     // Upper Cretaceous
    "",                                     // Lower Cretaceous
    "",                                     // Upper Jurassic
    "",                                     // Middle Jurassic
    "",                                     // Lower Jurassic
    "",                                     // Upper Triassic
    "",                                     // Middle Triassic
    "",                                     // Lower Triassic
    "ローピンジアン",                       // Lopingian
    "グアダルピアン",                       // Guadalupian
    "シスウラリアン",                       // Cisuralian
    "",                                     // Upper Pennsylvanian
    "",                                     // Middle Pennsylvanian
    "",                                     // Lower Pennsylvanian
    "",                                     // Upper Mississippian
    "",                                     // Middle Mississippian
    "",                                     // Lower Mississippian
    "",                                     // Upper Devonian
    "",                                     // Middle Devonian
    "",                                     // Lower Devonian
    "プリドリ",                             // Pridoli
    "ラドロー",                             // Ludlow
    "ウェンロック",                         // Wenlock
    "ランドベリ",                           // Llandovery
    "",                                     // Upper Ordovician
    "",                                     // Middle Ordovician
    "",                                     // Lower Ordovician
    "フロンギアン",                         // Furongian
    "ミャオリンギアン",                     // Miaolingian
    "シリーズ 2",                           // Cambrian Series 2
    "テレニュービアン",                     // Terreneuvian
    "メガラヤン",                           // Meghalayan
    "ノースグリッピアン",                   // Northgrippian
    "グリーンランディアン",                 // Greenlandian
    "",                                     // Upper Pleistocene
    "チバニアン",                           // Chibanian
    "カラブリアン",                         // Calabrian
    "ジェラシアン",                         // Gelasian
    "ピアセンジアン",                       // Piacenzian
    "ザンクリアン",                         // Zanclean
    "メッシニアン",                         // Messinian
    "トートニアン",                         // Tortonian
    "サーラバリアン",                       // Serravallian
    "ランギアン",                           // Langhian
    "バーディガリアン",                     // Burdigalian
    "アキタニアン",                         // Aquitanian
    "チャッティアン",                       // Chattian
    "ルペリアン",                           // Rupelian
    "プリアボニアン",                       // Priabonian
    "バートニアン",                         // Bartonian
    "ルテシアン",                           // Lutetian
    "イプレシアン",                         // Ypresian
    "サネティアン",                         // Thanetian
    "セランディアン",                       // Selandian
    "ダニアン",                             // Danian
    "マーストリヒチアン",                   // Maastrichtian
    "カンパニアン",                         // Campanian
    "サントニアン",                         // Santonian
    "コニアシアン",                         // Coniacian
    "チューロニアン",                       // Turonian
    "セノマニアン",                         // Cenomanian
    "アルビアン",                           // Albian
    "アプチアン",                           // Aptian
    "バレミアン",                           // Barremian
    "オーテリビアン",                       // Hauterivian
    "バランジニアン",                       // Valanginian
    "ベリアシアン",                         // Berriasian
    "チトニアン",                           // Tithonian
    "キンメリッジアン",                     // Kimmeridgian
    "オックスフォーディアン",               // Oxfordian
    "カロビアン",                           // Callovian
    "バトニアン",                           // Bathonian
    "バジョシアン",                         // Bajocian
    "アーレニアン",                         // Aalenian
    "トアルシアン",                         // Toarcian
    "プリンスバキアン",                     // Pliensbachian
    "シネムリアン",                         // Sinemurian
    "ヘッタンジアン",                       // Hettangian
    "レーティアン",                         // Rhaetian
    "ノーリアン",                           // Norian
    "カーニアン",                           // Carnian
    "ラディニアン",                         // Ladinian
    "アニシアン",                           // Anisian
    "オレネキアン",                         // Olenekian
    "インドゥアン",                         // Induan
    "チャンシンジアン",                     // Changhsingian
    "ウーチャーピンジアン",                 // Wuchiapingian
    "キャピタニアン",                       // Capitanian
    "ウォーディアン",                       // Wordian
    "ローディアン",                         // Roadian
    "クングリアン",                         // Kungurian
    "アーティンスキアン",                   // Artinskian
    "サクマーリアン",                       // Sakmarian
    "アッセリアン",                         // Asselian
    "グゼリアン",                           // Gzhelian
    "カシモビアン",                         // Kasimovian
    "モスコビアン",                         // Moscovian
    "バシキリアン",                         // Bashkirian
    "サープコビアン",                       // Serpukhovian
    "ビゼーアン",                           // Visean
    "トルネーシアン",                       // Tournaisian
    "ファメニアン",                         // Famennian
    "フラニアン",                           // Frasnian
    "ジベティアン",                         // Givetian
    "アイフェリアン",                       // Eifelian
    "エムシアン",                           // Emsian
    "プラギアン",                           // Pragian
    "ロッコヴィアン",                       // Lochkovian
    "ルドフォーディアン",                   // Ludfordian
    "ゴースティアン",                       // Gorstian
    "ホメリアン",                           // Homerian
    "シェイウッディアン",                   // Sheinwoodian
    "テリチアン",                           // Telychian
    "アエロニアン",                         // Aeronian
    "ラッダニアン",                         // Rhuddanian
    "ヒルナンシアン",                       // Hirnantian
    "カティアン",                           // Katian
    "サンドビアン",                         // Sandbian
    "ダリウィリアン",                       // Darriwilian
    "ダーピンジアン",                       // Dapingian
    "フロイアン",                           // Floian
    "トレマドキアン",                       // Tremadocian
    "ステージ 10",                          // Cambrian Stage 10
    "ジャンシャニアン",                     // Jiangshanian
    "ペイビアン",                           // Paibian
    "ガズハンジアン",                       // Guzhangian
    "ドラミアン",                           // Drumian
    "ウリューアン",                         // Wuliuan
    "ステージ 4",                           // Cambrian Stage 4
    "ステージ 3",                           // Cambrian Stage 3
    "ステージ 2",                           // Cambrian Stage 2
    "フォーチュニアン",                     // Fortunian
];

/// Korean, `ko`: the chart's `@ko` labels.
const KO: [&str; INTERVAL_COUNT] = [
    "현생누대",       // Phanerozoic
    "원생누대",       // Proterozoic
    "시생누대",       // Archean
    "명왕누대",       // Hadean
    "생대",           // Cenozoic
    "중생대",         // Mesozoic
    "고생대",         // Paleozoic
    "신원생대",       // Neoproterozoic
    "중원생대",       // Mesoproterozoic
    "고원생대",       // Paleoproterozoic
    "신시생대",       // Neoarchean
    "중시생대",       // Mesoarchean
    "고시생대",       // Paleoarchean
    "초시생대",       // Eoarchean
    "제4기",          // Quaternary
    "신진기",         // Neogene
    "고진기",         // Paleogene
    "백악기",         // Cretaceous
    "쥐라기",         // Jurassic
    "트라이아스기",   // Triassic
    "페름기",         // Permian
    "석탄기",         // Carboniferous
    "데본기",         // Devonian
    "실루리아기",     // Silurian
    "오르도비스기",   // Ordovician
    "캄브리아기",     // Cambrian
    "에디아카라기",   // Ediacaran
    "크리오스진기",   // Cryogenian
    "토노스기",       // Tonian
    "스테노스기",     // Stenian
    "엑타시스기",     // Ectasian
    "칼리마기",       // Calymmian
    "스타테로스기",   // Statherian
    "오로세이라기",   // Orosirian
    "라이악스기",     // Rhyacian
    "시데로스기",     // Siderian
    "홀로세",         // Holocene
    "플라이스토세",   // Pleistocene
    "플라이오세",     // Pliocene
    "마이오세",       // Miocene
    "올리고세",       // Oligocene
    "에오세",         // Eocene
    "팔레오세",       // Paleocene
    "",               // Upper Cretaceous
    "",               // Lower Cretaceous
    "",               // Upper Jurassic
    "",               // Middle Jurassic
    "",               // Lower Jurassic
    "",               // Upper Triassic
    "",               // Middle Triassic
    "",               // Lower Triassic
    "러핑세",         // Lopingian
    "과달루페세",     // Guadalupian
    "시스우랄세",     // Cisuralian
    "",               // Upper Pennsylvanian
    "",               // Middle Pennsylvanian
    "",               // Lower Pennsylvanian
    "",               // Upper Mississippian
    "",               // Middle Mississippian
    "",               // Lower Mississippian
    "",               // Upper Devonian
    "",               // Middle Devonian
    "",               // Lower Devonian
    "프리돌리세",     // Pridoli
    "러들로세",       // Ludlow
    "웬록세",         // Wenlock
    "란도베리세",     // Llandovery
    "",               // Upper Ordovician
    "",               // Middle Ordovician
    "",               // Lower Ordovician
    "푸롱세",         // Furongian
    "미아오링세",     // Miaolingian
    "제2세",          // Cambrian Series 2
    "테레누브세",     // Terreneuvian
    "메갈라야절",     // Meghalayan
    "노스그립절",     // Northgrippian
    "그린란드절",     // Greenlandian
    "",               // Upper Pleistocene
    "지바절",         // Chibanian
    "칼라브리아절",   // Calabrian
    "젤라절",         // Gelasian
    "피아첸차절",     // Piacenzian
    "장클레절",       // Zanclean
    "메시나절",       // Messinian
    "토르토나절",     // Tortonian
    "세라발레절",     // Serravallian
    "랑게절",         // Langhian
    "부르디갈라절",   // Burdigalian
    "아킨텐절",       // Aquitanian
    "카티절",         // Chattian
    "루펠절",         // Rupelian
    "프리아보나절",   // Priabonian
    "바턴절",         // Bartonian
    "루테티아절",     // Lutetian
    "이퍼르절",       // Ypresian
    "타넷절",         // Thanetian
    "셀란절",         // Selandian
    "다니아절",       // Danian
    "마스트리히트절", // Maastrichtian
    "캄파이나절",     // Campanian
    "산토눔절",       // Santonian
    "코냑절",         // Coniacian
    "투로니아절",     // Turonian
    "세노마눔절",     // Cenomanian
    "알바절",         // Albian
    "압트절",         // Aptian
    "바렘절",         // Barremian
    "오트리브절",     // Hauterivian
    "발랑절",         // Valanginian
    "베리아절",       // Berriasian
    "티토누스절",     // Tithonian
    "킴머리지절",     // Kimmeridgian
    "옥스퍼드절",     // Oxfordian
    "칼로비움절",     // Callovian
    "바토니움절",     // Bathonian
    "바조카에절",     // Bajocian
    "알렌절",         // Aalenian
    "토아르시움절",   // Toarcian
    "플린스바흐절",   // Pliensbachian
    "시네무룸절",     // Sinemurian
    "에탕주절",       // Hettangian
    "래티아절",       // Rhaetian
    "노릭절",         // Norian
    "카닉절",         // Carnian
    "라딘절",         // Ladinian
    "아니수스절",     // Anisian
    "올레네크절",     // Olenekian
    "인더스절",       // Induan
    "창싱절",         // Changhsingian
    "우지아핑절",     // Wuchiapingian
    "캐피탄절",       // Capitanian
    "워드절",         // Wordian
    "로드절",         // Roadian
    "쿤구르절",       // Kungurian
    "아르틴스크절",   // Artinskian
    "사크마라절",     // Sakmarian
    "아셀절",         // Asselian
    "그젤절",         // Gzhelian
    "카시모프절",     // Kasimovian
    "모스코바절",     // Moscovian
    "바시키르절",     // Bashkirian
    "세르푸호프절",   // Serpukhovian
    "비제절",         // Visean
    "투르네절",       // Tournaisian
    "파멘절",         // Famennian
    "프랜절",         // Frasnian
    "지베절",         // Givetian
    "아이펠절",       // Eifelian
    "엠즈절",         // Emsian
    "프라하절",       // Pragian
    "로치코프절",     // Lochkovian
    "로드포드절",     // Ludfordian
    "고스티절",       // Gorstian
    "호머절",         // Homerian
    "셰인우드절",     // Sheinwoodian
    "텔리치절",       // Telychian
    "에어론절",       // Aeronian
    "루단절",         // Rhuddanian
    "허난트절",       // Hirnantian
    "케이티절",       // Katian
    "샌드비절",       // Sandbian
    "다리윌절",       // Darriwilian
    "다핑절",         // Dapingian
    "플로절",         // Floian
    "트레마독절",     // Tremadocian
    "제10절",         // Cambrian Stage 10
    "지앙샨절",       // Jiangshanian
    "파이비절",       // Paibian
    "구장절",         // Guzhangian
    "드럼절",         // Drumian
    "울리우절",       // Wuliuan
    "제4절",          // Cambrian Stage 4
    "제3절",          // Cambrian Stage 3
    "제2절",          // Cambrian Stage 2
    "포츈절",         // Fortunian
];

/// Dutch, `nl`: the chart's `@nl` labels.
const NL: [&str; INTERVAL_COUNT] = [
    "Fanerozoïcum",       // Phanerozoic
    "Proterozoïcum",      // Proterozoic
    "Archeïcum",          // Archean
    "Hadeïcum",           // Hadean
    "Kenozoïcum",         // Cenozoic
    "Mesozoïcum",         // Mesozoic
    "Paleozoïcum",        // Paleozoic
    "Neoproterozoïcum",   // Neoproterozoic
    "Mesoproterozoïcum",  // Mesoproterozoic
    "Paleoproterozoïcum", // Paleoproterozoic
    "Neoarcheïcum",       // Neoarchean
    "Mesoarcheïcum",      // Mesoarchean
    "Paleoarcheïcum",     // Paleoarchean
    "Eoarcheïcum",        // Eoarchean
    "Kwartair",           // Quaternary
    "Neogeen",            // Neogene
    "Paleogeen",          // Paleogene
    "Krijt",              // Cretaceous
    "Jura",               // Jurassic
    "Trias",              // Triassic
    "Perm",               // Permian
    "Carboon",            // Carboniferous
    "Devoon",             // Devonian
    "Siluur",             // Silurian
    "Ordovicium",         // Ordovician
    "Cambrium",           // Cambrian
    "Ediacarium",         // Ediacaran
    "Cryogenium",         // Cryogenian
    "Tonium",             // Tonian
    "Stenium",            // Stenian
    "Ectasium",           // Ectasian
    "Calymmium",          // Calymmian
    "Statherium",         // Statherian
    "Orosirium",          // Orosirian
    "Rhyacium",           // Rhyacian
    "Siderium",           // Siderian
    "Holoceen",           // Holocene
    "Pleistoceen",        // Pleistocene
    "Plioceen",           // Pliocene
    "Mioceen",            // Miocene
    "Oligoceen",          // Oligocene
    "Eoceen",             // Eocene
    "Paleoceen",          // Paleocene
    "",                   // Upper Cretaceous
    "",                   // Lower Cretaceous
    "",                   // Upper Jurassic
    "",                   // Middle Jurassic
    "",                   // Lower Jurassic
    "",                   // Upper Triassic
    "",                   // Middle Triassic
    "",                   // Lower Triassic
    "Lopingien",          // Lopingian
    "Guadalupien",        // Guadalupian
    "Cisuralien",         // Cisuralian
    "",                   // Upper Pennsylvanian
    "",                   // Middle Pennsylvanian
    "",                   // Lower Pennsylvanian
    "",                   // Upper Mississippian
    "",                   // Middle Mississippian
    "",                   // Lower Mississippian
    "",                   // Upper Devonian
    "",                   // Middle Devonian
    "",                   // Lower Devonian
    "Pridoli",            // Pridoli
    "Ludlow",             // Ludlow
    "Wenlock",            // Wenlock
    "Llandovery",         // Llandovery
    "",                   // Upper Ordovician
    "",                   // Middle Ordovician
    "",                   // Lower Ordovician
    "Furongien",          // Furongian
    "Miaolingien",        // Miaolingian
    "Serie 2",            // Cambrian Series 2
    "Terreneuvien",       // Terreneuvian
    "Meghalayen",         // Meghalayan
    "Northgrippien",      // Northgrippian
    "Greenlandien",       // Greenlandian
    "",                   // Upper Pleistocene
    "Chibaien",           // Chibanian
    "Calabrien",          // Calabrian
    "Gelasien",           // Gelasian
    "Piacenzien",         // Piacenzian
    "Zancléen",           // Zanclean
    "Messinien",          // Messinian
    "Tortonien",          // Tortonian
    "Serravallien",       // Serravallian
    "Langhien",           // Langhian
    "Burdigalien",        // Burdigalian
    "Aquitanien",         // Aquitanian
    "Chattien",           // Chattian
    "Rupelien",           // Rupelian
    "Priabonien",         // Priabonian
    "Bartonien",          // Bartonian
    "Lutetien",           // Lutetian
    "Ypresien",           // Ypresian
    "Thanetien",          // Thanetian
    "Selandien",          // Selandian
    "Danien",             // Danian
    "Maastrichtien",      // Maastrichtian
    "Campanien",          // Campanian
    "Santonien",          // Santonian
    "Coniacien",          // Coniacian
    "Turonien",           // Turonian
    "Cenomanien",         // Cenomanian
    "Albien",             // Albian
    "Aptien",             // Aptian
    "Barremien",          // Barremian
    "Hauterivien",        // Hauterivian
    "Valanginien",        // Valanginian
    "Berriasien",         // Berriasian
    "Tithonien",          // Tithonian
    "Kimmeridgien",       // Kimmeridgian
    "Oxfordien",          // Oxfordian
    "Callovien",          // Callovian
    "Bathonien",          // Bathonian
    "Bajocien",           // Bajocian
    "Aalenien",           // Aalenian
    "Toarcien",           // Toarcian
    "Pliensbachien",      // Pliensbachian
    "Sinemurien",         // Sinemurian
    "Hettangien",         // Hettangian
    "Rhaetien",           // Rhaetian
    "Norien",             // Norian
    "Carnien",            // Carnian
    "Ladinien",           // Ladinian
    "Anisien",            // Anisian
    "Olenekien",          // Olenekian
    "Induen",             // Induan
    "Changhsingien",      // Changhsingian
    "Wuchiapingien",      // Wuchiapingian
    "Capitanien",         // Capitanian
    "Wordien",            // Wordian
    "Roadien",            // Roadian
    "Kungurien",          // Kungurian
    "Artinskien",         // Artinskian
    "Sakmarien",          // Sakmarian
    "Asselien",           // Asselian
    "Gzhelien",           // Gzhelian
    "Kasimovien",         // Kasimovian
    "Moscovien",          // Moscovian
    "Bashkirien",         // Bashkirian
    "Serpukhovien",       // Serpukhovian
    "Viséen",             // Visean
    "Tournaisien",        // Tournaisian
    "Famennien",          // Famennian
    "Frasnien",           // Frasnian
    "Givetien",           // Givetian
    "Eifelien",           // Eifelian
    "Emsien",             // Emsian
    "Pragien",            // Pragian
    "Lochkovien",         // Lochkovian
    "Ludfordien",         // Ludfordian
    "Gorstien",           // Gorstian
    "Homerien",           // Homerian
    "Sheinwoodien",       // Sheinwoodian
    "Telychien",          // Telychian
    "Aeronien",           // Aeronian
    "Rhuddanien",         // Rhuddanian
    "Hirnantien",         // Hirnantian
    "Katien",             // Katian
    "Sandbien",           // Sandbian
    "Darriwilien",        // Darriwilian
    "Dapingien",          // Dapingian
    "Floien",             // Floian
    "Tremadocien",        // Tremadocian
    "Etage 10",           // Cambrian Stage 10
    "Jiangshanien",       // Jiangshanian
    "Paibien",            // Paibian
    "Guzhangien",         // Guzhangian
    "Drumien",            // Drumian
    "Wuliuien",           // Wuliuan
    "Etage 4",            // Cambrian Stage 4
    "Etage 3",            // Cambrian Stage 3
    "Etage 2",            // Cambrian Stage 2
    "Fortunien",          // Fortunian
];

/// Polish, `pl`: the chart's `@pl` labels.
const PL: [&str; INTERVAL_COUNT] = [
    "Fanerozoik",       // Phanerozoic
    "Proterozoik",      // Proterozoic
    "Archaik",          // Archean
    "Hadeik",           // Hadean
    "Kenozoik",         // Cenozoic
    "Mezozoik",         // Mesozoic
    "Paleozoik",        // Paleozoic
    "Neoproterozoik",   // Neoproterozoic
    "Mezoproterozoik",  // Mesoproterozoic
    "Paleoproterozoik", // Paleoproterozoic
    "Neoarchaik",       // Neoarchean
    "Mezoarchaik",      // Mesoarchean
    "Paleoarchaik",     // Paleoarchean
    "Eoarchaik",        // Eoarchean
    "Czwartorzęd",      // Quaternary
    "Neogen",           // Neogene
    "Paleogen",         // Paleogene
    "Kreda",            // Cretaceous
    "Jura",             // Jurassic
    "Trias",            // Triassic
    "Perm",             // Permian
    "Karbon",           // Carboniferous
    "Dewon",            // Devonian
    "Sylur",            // Silurian
    "Ordowik",          // Ordovician
    "Kambr",            // Cambrian
    "Ediakar",          // Ediacaran
    "Kriogenian",       // Cryogenian
    "Tonian",           // Tonian
    "Stenian",          // Stenian
    "Ektazjan",         // Ectasian
    "Kalymian",         // Calymmian
    "Staterian",        // Statherian
    "Orozoirian",       // Orosirian
    "Riacjan",          // Rhyacian
    "Syderyjski",       // Siderian
    "Holocen",          // Holocene
    "Plejstocen",       // Pleistocene
    "Pliocen",          // Pliocene
    "Miocen",           // Miocene
    "Oligocen",         // Oligocene
    "Eocen",            // Eocene
    "Paleocen",         // Paleocene
    "",                 // Upper Cretaceous
    "",                 // Lower Cretaceous
    "",                 // Upper Jurassic
    "",                 // Middle Jurassic
    "",                 // Lower Jurassic
    "",                 // Upper Triassic
    "",                 // Middle Triassic
    "",                 // Lower Triassic
    "Loping",           // Lopingian
    "Gwadalup",         // Guadalupian
    "Cisural",          // Cisuralian
    "",                 // Upper Pennsylvanian
    "",                 // Middle Pennsylvanian
    "",                 // Lower Pennsylvanian
    "",                 // Upper Mississippian
    "",                 // Middle Mississippian
    "",                 // Lower Mississippian
    "",                 // Upper Devonian
    "",                 // Middle Devonian
    "",                 // Lower Devonian
    "Przydol",          // Pridoli
    "Ludlow",           // Ludlow
    "Wenlock",          // Wenlock
    "Landower",         // Llandovery
    "",                 // Upper Ordovician
    "",                 // Middle Ordovician
    "",                 // Lower Ordovician
    "Furong",           // Furongian
    "Miaoling",         // Miaolingian
    "Seria 2",          // Cambrian Series 2
    "Terreneuwiański",  // Terreneuvian
    "Megalajski",       // Meghalayan
    "Northgrippski",    // Northgrippian
    "Grenlandzki",      // Greenlandian
    "",                 // Upper Pleistocene
    "Chibański",        // Chibanian
    "Kalabryjski",      // Calabrian
    "Gelaz",            // Gelasian
    "Piacenz",          // Piacenzian
    "Zanklean",         // Zanclean
    "Mesyn",            // Messinian
    "Torton",           // Tortonian
    "Serawal",          // Serravallian
    "Langian",          // Langhian
    "Burdigal",         // Burdigalian
    "Akwitan",          // Aquitanian
    "Chatt",            // Chattian
    "Rupel",            // Rupelian
    "Priabon",          // Priabonian
    "Barton",           // Bartonian
    "Lutet",            // Lutetian
    "Ypresian",         // Ypresian
    "Tenet",            // Thanetian
    "Selan",            // Selandian
    "Dan",              // Danian
    "Mastrycht",        // Maastrichtian
    "Kampan",           // Campanian
    "Santon",           // Santonian
    "Koniak",           // Coniacian
    "Turon",            // Turonian
    "Cenoman",          // Cenomanian
    "Alb",              // Albian
    "Apt",              // Aptian
    "Barrem",           // Barremian
    "Hoteryw",          // Hauterivian
    "Walanżyn",         // Valanginian
    "Berrias",          // Berriasian
    "Tytoń",            // Tithonian
    "Kimerydż",         // Kimmeridgian
    "Oksford",          // Oxfordian
    "Kelowej",          // Callovian
    "Baton",            // Bathonian
    "Bajos",            // Bajocian
    "Aalen",            // Aalenian
    "Toark",            // Toarcian
    "Pliensbach",       // Pliensbachian
    "Synemur",          // Sinemurian
    "Hettang",          // Hettangian
    "Retyk",            // Rhaetian
    "Noryk",            // Norian
    "Karnik",           // Carnian
    "Ladyn",            // Ladinian
    "Anizyk",           // Anisian
    "Olenek",           // Olenekian
    "Induan",           // Induan
    "Changxing",        // Changhsingian
    "Wuchiaping",       // Wuchiapingian
    "Kapitan",          // Capitanian
    "Word",             // Wordian
    "Roadyjski",        // Roadian
    "Kungur",           // Kungurian
    "Artynsk",          // Artinskian
    "Sakmar",           // Sakmarian
    "Assel",            // Asselian
    "Gżel",             // Gzhelian
    "Kasymow",          // Kasimovian
    "Moskow",           // Moscovian
    "Baszkirian",       // Bashkirian
    "Sierpuchów",       // Serpukhovian
    "Wizen",            // Visean
    "Turnej",           // Tournaisian
    "Famen",            // Famennian
    "Frasn",            // Frasnian
    "Żywet",            // Givetian
    "Eifel",            // Eifelian
    "Ems",              // Emsian
    "Prag",             // Pragian
    "Lochków",          // Lochkovian
    "Ludford",          // Ludfordian
    "Gorst",            // Gorstian
    "Homer",            // Homerian
    "Sheinwood",        // Sheinwoodian
    "Telych",           // Telychian
    "Aeron",            // Aeronian
    "Ruddan",           // Rhuddanian
    "Hirnantan",        // Hirnantian
    "Katan",            // Katian
    "Sandbian",         // Sandbian
    "Darriwil",         // Darriwilian
    "Daping",           // Dapingian
    "Flojan",           // Floian
    "Tremadok",         // Tremadocian
    "Piętro 10",        // Cambrian Stage 10
    "Jiangshan",        // Jiangshanian
    "Paib",             // Paibian
    "Gużang",           // Guzhangian
    "Drum",             // Drumian
    "Wuliu",            // Wuliuan
    "Piętro 4",         // Cambrian Stage 4
    "Piętro 3",         // Cambrian Stage 3
    "Piętro 2",         // Cambrian Stage 2
    "Fortunian",        // Fortunian
];

/// Portuguese, `pt`: the chart's `@pt` labels.
const PT: [&str; INTERVAL_COUNT] = [
    "Fanerozoico",       // Phanerozoic
    "Proterozoico",      // Proterozoic
    "Arqueano",          // Archean
    "Hadeano",           // Hadean
    "Cenozoico",         // Cenozoic
    "Mesozoico",         // Mesozoic
    "Paleozoico",        // Paleozoic
    "Neoproterozoico",   // Neoproterozoic
    "Mesoproterozoico",  // Mesoproterozoic
    "Paleoproterozoico", // Paleoproterozoic
    "Neoarqueano",       // Neoarchean
    "Mesoarqueano",      // Mesoarchean
    "Paleoarqueano",     // Paleoarchean
    "Eoarqueano",        // Eoarchean
    "Quaternário",       // Quaternary
    "Neógeno",           // Neogene
    "Paleógeno",         // Paleogene
    "Cretáceo",          // Cretaceous
    "Jurássico",         // Jurassic
    "Triássico",         // Triassic
    "Permiano",          // Permian
    "Carbonifero",       // Carboniferous
    "Devoniano",         // Devonian
    "Siluriano",         // Silurian
    "Ordoviciano",       // Ordovician
    "Cambriano",         // Cambrian
    "Ediacariano",       // Ediacaran
    "Cryogeniano",       // Cryogenian
    "Toniano",           // Tonian
    "Steniano",          // Stenian
    "Ectasiano",         // Ectasian
    "Calymmiano",        // Calymmian
    "Staheriano",        // Statherian
    "Orosiriano",        // Orosirian
    "Rhyaciano",         // Rhyacian
    "Sideriano",         // Siderian
    "Holoceno",          // Holocene
    "Pleistoceno",       // Pleistocene
    "Plioceno",          // Pliocene
    "Mioceno",           // Miocene
    "Oligoceno",         // Oligocene
    "Eoceno",            // Eocene
    "Paleoceno",         // Paleocene
    "",                  // Upper Cretaceous
    "",                  // Lower Cretaceous
    "",                  // Upper Jurassic
    "",                  // Middle Jurassic
    "",                  // Lower Jurassic
    "",                  // Upper Triassic
    "",                  // Middle Triassic
    "",                  // Lower Triassic
    "Lopingiano",        // Lopingian
    "Guadalupiano",      // Guadalupian
    "Cisuraliano",       // Cisuralian
    "",                  // Upper Pennsylvanian
    "",                  // Middle Pennsylvanian
    "",                  // Lower Pennsylvanian
    "",                  // Upper Mississippian
    "",                  // Middle Mississippian
    "",                  // Lower Mississippian
    "",                  // Upper Devonian
    "",                  // Middle Devonian
    "",                  // Lower Devonian
    "Pridoli",           // Pridoli
    "Ludlow",            // Ludlow
    "Wenlock",           // Wenlock
    "Llandovery",        // Llandovery
    "",                  // Upper Ordovician
    "",                  // Middle Ordovician
    "",                  // Lower Ordovician
    "Furongiano",        // Furongian
    "Miaolingiano",      // Miaolingian
    "Série 2",           // Cambrian Series 2
    "Terreneuviano",     // Terreneuvian
    "Megalayano",        // Meghalayan
    "Northgrippiano",    // Northgrippian
    "Greenlandiano",     // Greenlandian
    "",                  // Upper Pleistocene
    "Chibaniano",        // Chibanian
    "Calabriano",        // Calabrian
    "Gelasiano",         // Gelasian
    "Piacenziano",       // Piacenzian
    "Zancleano",         // Zanclean
    "Messiniano",        // Messinian
    "Tortoniano",        // Tortonian
    "Serravalliano",     // Serravallian
    "Langhiano",         // Langhian
    "Burdigaliano",      // Burdigalian
    "Aquitaniano",       // Aquitanian
    "Chattiano",         // Chattian
    "Rupeliano",         // Rupelian
    "Priaboniano",       // Priabonian
    "Bartoniano",        // Bartonian
    "Lutetiano",         // Lutetian
    "Ypresiano",         // Ypresian
    "Thanetiano",        // Thanetian
    "Selandiano",        // Selandian
    "Daniano",           // Danian
    "Maastrichtiano",    // Maastrichtian
    "Campaniano",        // Campanian
    "Santoniano",        // Santonian
    "Coniaciano",        // Coniacian
    "Turoniano",         // Turonian
    "Cenomaniano",       // Cenomanian
    "Albiano",           // Albian
    "Aptiano",           // Aptian
    "Barremiano",        // Barremian
    "Hauteriviano",      // Hauterivian
    "Valanginiano",      // Valanginian
    "Berriasiano",       // Berriasian
    "Tithoniano",        // Tithonian
    "Kimmeridgiano",     // Kimmeridgian
    "Oxfordiano",        // Oxfordian
    "Calloviano",        // Callovian
    "Bathoniano",        // Bathonian
    "Bajociano",         // Bajocian
    "Aaleniano",         // Aalenian
    "Toarciano",         // Toarcian
    "Pliensbachiano",    // Pliensbachian
    "Sinemuriano",       // Sinemurian
    "Hettangiano",       // Hettangian
    "Rhaetiano",         // Rhaetian
    "Noriano",           // Norian
    "Carniano",          // Carnian
    "Ladiniano",         // Ladinian
    "Anisiano",          // Anisian
    "Olenekiano",        // Olenekian
    "Induano",           // Induan
    "Changhsingiano",    // Changhsingian
    "Wuchiapingiano",    // Wuchiapingian
    "Capitaniano",       // Capitanian
    "Wordiano",          // Wordian
    "Roadiano",          // Roadian
    "Kunguriano",        // Kungurian
    "Artinskiano",       // Artinskian
    "Sakmariano",        // Sakmarian
    "Asseliano",         // Asselian
    "Gzheliano",         // Gzhelian
    "Kasimoviano",       // Kasimovian
    "Moscoviano",        // Moscovian
    "Bashkiriano",       // Bashkirian
    "Serpukhoviano",     // Serpukhovian
    "Viseano",           // Visean
    "Tournaisiano",      // Tournaisian
    "Famenniano",        // Famennian
    "Frasniano",         // Frasnian
    "Givetiano",         // Givetian
    "Eifeliano",         // Eifelian
    "Emsiano",           // Emsian
    "Pragiano",          // Pragian
    "Lochkoviano",       // Lochkovian
    "Ludfordiano",       // Ludfordian
    "Gorstiano",         // Gorstian
    "Homeriano",         // Homerian
    "Sheinwoodiano",     // Sheinwoodian
    "Telychiano",        // Telychian
    "Aeroniano",         // Aeronian
    "Rhuddaniano",       // Rhuddanian
    "Hirnantiano",       // Hirnantian
    "Katiano",           // Katian
    "Sandbiano",         // Sandbian
    "Darriwiliano",      // Darriwilian
    "Dapingiano",        // Dapingian
    "Floiano",           // Floian
    "Tremadociano",      // Tremadocian
    "Andar 10",          // Cambrian Stage 10
    "Jiangshaniano",     // Jiangshanian
    "Paibiano",          // Paibian
    "Guzhangiano",       // Guzhangian
    "Drumiano",          // Drumian
    "Wuliuano",          // Wuliuan
    "Andar 4",           // Cambrian Stage 4
    "Andar 3",           // Cambrian Stage 3
    "Andar 2",           // Cambrian Stage 2
    "Fortuniano",        // Fortunian
];

/// Russian, `ru`: the chart's `@ru` labels.
const RU: [&str; INTERVAL_COUNT] = [
    "Фанерозойская",       // Phanerozoic
    "Протерозойская",      // Proterozoic
    "Архейская",           // Archean
    "",                    // Hadean
    "Кайнозойская",        // Cenozoic
    "Мезозойская",         // Mesozoic
    "Палеозойская",        // Paleozoic
    "Неопротерозойская",   // Neoproterozoic
    "Мезопротерозойская",  // Mesoproterozoic
    "Палеопротерозойская", // Paleoproterozoic
    "Неоархейская",        // Neoarchean
    "Мезоархейская",       // Mesoarchean
    "Палеоархейская",      // Paleoarchean
    "Эоархейская",         // Eoarchean
    "Четвертичная",        // Quaternary
    "Неогеновая",          // Neogene
    "Палеогеновая",        // Paleogene
    "Меловая",             // Cretaceous
    "Юрская",              // Jurassic
    "Триасовая",           // Triassic
    "Пермская",            // Permian
    "Каменноугольная",     // Carboniferous
    "Девонская",           // Devonian
    "Силурийская",         // Silurian
    "Ордовикская",         // Ordovician
    "Кембрийская",         // Cambrian
    "Эдиакарская",         // Ediacaran
    "Криогенская",         // Cryogenian
    "Тонская",             // Tonian
    "Стенская",            // Stenian
    "Эктазская",           // Ectasian
    "Калиммская",          // Calymmian
    "Статерская",          // Statherian
    "Орозирская",          // Orosirian
    "Ряская",              // Rhyacian
    "Сидерская",           // Siderian
    "Голоцен",             // Holocene
    "Плейстоцен",          // Pleistocene
    "Плиоцен",             // Pliocene
    "Миоцен",              // Miocene
    "Олигоцен",            // Oligocene
    "Эоцен",               // Eocene
    "Палео- цен",          // Paleocene
    "",                    // Upper Cretaceous
    "",                    // Lower Cretaceous
    "",                    // Upper Jurassic
    "",                    // Middle Jurassic
    "",                    // Lower Jurassic
    "",                    // Upper Triassic
    "",                    // Middle Triassic
    "",                    // Lower Triassic
    "Лопинский",           // Lopingian
    "Гваделупский",        // Guadalupian
    "Приуральский",        // Cisuralian
    "",                    // Upper Pennsylvanian
    "",                    // Middle Pennsylvanian
    "",                    // Lower Pennsylvanian
    "",                    // Upper Mississippian
    "",                    // Middle Mississippian
    "",                    // Lower Mississippian
    "",                    // Upper Devonian
    "",                    // Middle Devonian
    "",                    // Lower Devonian
    "Пржидольский",        // Pridoli
    "Лудловский",          // Ludlow
    "Венлокский",          // Wenlock
    "Лландоверийский",     // Llandovery
    "",                    // Upper Ordovician
    "",                    // Middle Ordovician
    "",                    // Lower Ordovician
    "Фуронгский",          // Furongian
    "Отдел 3",             // Miaolingian
    "Отдел 2",             // Cambrian Series 2
    "Терреновский",        // Terreneuvian
    "",                    // Meghalayan
    "",                    // Northgrippian
    "",                    // Greenlandian
    "",                    // Upper Pleistocene
    "Средний",             // Chibanian
    "Калабрийский",        // Calabrian
    "Гелазский",           // Gelasian
    "Пьяченцский",         // Piacenzian
    "Занклский",           // Zanclean
    "Мессинский",          // Messinian
    "Тортонский",          // Tortonian
    "Серравальский",       // Serravallian
    "Лангийский",          // Langhian
    "Бурдигальский",       // Burdigalian
    "Аквитанский",         // Aquitanian
    "Хаттский",            // Chattian
    "Рюпельский",          // Rupelian
    "Приабонский",         // Priabonian
    "Бартонский",          // Bartonian
    "Лютетский",           // Lutetian
    "Ипрский",             // Ypresian
    "Танетский",           // Thanetian
    "Зеландский",          // Selandian
    "Датский",             // Danian
    "Маастрихтский",       // Maastrichtian
    "Кампанский",          // Campanian
    "Сантонский",          // Santonian
    "Коньякский",          // Coniacian
    "Туронский",           // Turonian
    "Сеноманский",         // Cenomanian
    "Альбский",            // Albian
    "Аптский",             // Aptian
    "Барремский",          // Barremian
    "Готеривский",         // Hauterivian
    "Валанжинский",        // Valanginian
    "Берриасский",         // Berriasian
    "Титонский",           // Tithonian
    "Кимериджский",        // Kimmeridgian
    "Оксфордский",         // Oxfordian
    "Келловейский",        // Callovian
    "Батский",             // Bathonian
    "Байосский",           // Bajocian
    "Ааленский",           // Aalenian
    "Тоарский",            // Toarcian
    "Плинсбахский",        // Pliensbachian
    "Синемюрский",         // Sinemurian
    "Геттангский",         // Hettangian
    "Рэтский",             // Rhaetian
    "Норийский",           // Norian
    "Карнийский",          // Carnian
    "Ладинский",           // Ladinian
    "Анизийский",          // Anisian
    "Оленекский",          // Olenekian
    "Индский",             // Induan
    "Чансинский",          // Changhsingian
    "Вучапинский",         // Wuchiapingian
    "Кептенский",          // Capitanian
    "Вордский",            // Wordian
    "Роудский",            // Roadian
    "Кунгурский",          // Kungurian
    "Артинский",           // Artinskian
    "Сакмарский",          // Sakmarian
    "Ассельский",          // Asselian
    "Гжельский",           // Gzhelian
    "Касимовский",         // Kasimovian
    "Московский",          // Moscovian
    "Башкирский",          // Bashkirian
    "Серпуховский",        // Serpukhovian
    "Визейский",           // Visean
    "Турнейский",          // Tournaisian
    "Фаменский",           // Famennian
    "Франский",            // Frasnian
    "Живетский",           // Givetian
    "Эйфельский",          // Eifelian
    "Эмсский",             // Emsian
    "Пражский",            // Pragian
    "Лохковский",          // Lochkovian
    "Лудфордский",         // Ludfordian
    "Горстийский",         // Gorstian
    "Гомерский",           // Homerian
    "Шейнвудский",         // Sheinwoodian
    "Теличский",           // Telychian
    "Аэронский",           // Aeronian
    "Рудданский",          // Rhuddanian
    "Хирнантский",         // Hirnantian
    "Катийский",           // Katian
    "Сандбийский",         // Sandbian
    "Дарривильский",       // Darriwilian
    "Дапинский",           // Dapingian
    "Флоский",             // Floian
    "Тремадокский",        // Tremadocian
    "Ярус 10",             // Cambrian Stage 10
    "Цзяншаньский",        // Jiangshanian
    "Паибский",            // Paibian
    "Гужанский",           // Guzhangian
    "Друмский",            // Drumian
    "Ярус 5",              // Wuliuan
    "Ярус 4",              // Cambrian Stage 4
    "Ярус 3",              // Cambrian Stage 3
    "Ярус 2",              // Cambrian Stage 2
    "Фортунский",          // Fortunian
];

/// Turkish, `tr`: the chart's `@tr` labels.
const TR: [&str; INTERVAL_COUNT] = [
    "Fanerozoyik",        // Phanerozoic
    "Proterozoyik",       // Proterozoic
    "Arkeen",             // Archean
    "Hadeen",             // Hadean
    "Senozoyik",          // Cenozoic
    "Mezozoyik",          // Mesozoic
    "Paleozoyik",         // Paleozoic
    "Neo proterozoyik",   // Neoproterozoic
    "Mezo proterozoyik",  // Mesoproterozoic
    "Paleo proterozoyik", // Paleoproterozoic
    "Neo arkeen",         // Neoarchean
    "Meso arkeen",        // Mesoarchean
    "Paleo arkeen",       // Paleoarchean
    "Eo arkeen",          // Eoarchean
    "Kuvaterner",         // Quaternary
    "Neojen",             // Neogene
    "Paleojen",           // Paleogene
    "Kretase",            // Cretaceous
    "Jura",               // Jurassic
    "Triyas",             // Triassic
    "Permiyen",           // Permian
    "Karbonifer",         // Carboniferous
    "Devoniyen",          // Devonian
    "Siluriyen",          // Silurian
    "Ordovisiyen",        // Ordovician
    "Kambriyen",          // Cambrian
    "Ediyakaran",         // Ediacaran
    "Kriyojeniyen",       // Cryogenian
    "Toniyen",            // Tonian
    "Steniyen",           // Stenian
    "Ektasiyen",          // Ectasian
    "Kalimiyen",          // Calymmian
    "Stateriyen",         // Statherian
    "Orosiriyen",         // Orosirian
    "Riyasiyen",          // Rhyacian
    "Sideriyen",          // Siderian
    "Holosen",            // Holocene
    "Pleyistosen",        // Pleistocene
    "Pliyosen",           // Pliocene
    "Miyosen",            // Miocene
    "Oligosen",           // Oligocene
    "Eosen",              // Eocene
    "Paleosen",           // Paleocene
    "",                   // Upper Cretaceous
    "",                   // Lower Cretaceous
    "",                   // Upper Jurassic
    "",                   // Middle Jurassic
    "",                   // Lower Jurassic
    "",                   // Upper Triassic
    "",                   // Middle Triassic
    "",                   // Lower Triassic
    "Lopingiyen",         // Lopingian
    "Guadalupiyen",       // Guadalupian
    "Sisuraliyen",        // Cisuralian
    "",                   // Upper Pennsylvanian
    "",                   // Middle Pennsylvanian
    "",                   // Lower Pennsylvanian
    "",                   // Upper Mississippian
    "",                   // Middle Mississippian
    "",                   // Lower Mississippian
    "",                   // Upper Devonian
    "",                   // Middle Devonian
    "",                   // Lower Devonian
    "Pridoli",            // Pridoli
    "Ludlov",             // Ludlow
    "Venlok",             // Wenlock
    "Landoveri",          // Llandovery
    "",                   // Upper Ordovician
    "",                   // Middle Ordovician
    "",                   // Lower Ordovician
    "Frongiyen",          // Furongian
    "Miaolingiyen",       // Miaolingian
    "Seri 2",             // Cambrian Series 2
    "Terrenöviyen",       // Terreneuvian
    "Meghaliyen",         // Meghalayan
    "Nortgripiyen",       // Northgrippian
    "Grönlandiyen",       // Greenlandian
    "",                   // Upper Pleistocene
    "Çibaniyen",          // Chibanian
    "Kalabriyen",         // Calabrian
    "Gelasiyen",          // Gelasian
    "Piasenziyen",        // Piacenzian
    "Zankliyen",          // Zanclean
    "Messiniyen",         // Messinian
    "Tortoniyen",         // Tortonian
    "Serravaliyen",       // Serravallian
    "Langiyen",           // Langhian
    "Burdigaliyen",       // Burdigalian
    "Akitaniyen",         // Aquitanian
    "Şattiyen",           // Chattian
    "Rupeliyen",          // Rupelian
    "Priaboniyen",        // Priabonian
    "Bartoniyen",         // Bartonian
    "Lütesiyen",          // Lutetian
    "İpresiyen",          // Ypresian
    "Tanesiyen",          // Thanetian
    "Selandiyen",         // Selandian
    "Daniyen",            // Danian
    "Maastrihtiyen",      // Maastrichtian
    "Kampaniyen",         // Campanian
    "Santoniyen",         // Santonian
    "Koniasiyen",         // Coniacian
    "Turoniyen",          // Turonian
    "Senomaniyen",        // Cenomanian
    "Albiyen",            // Albian
    "Apsiyen",            // Aptian
    "Barremiyen",         // Barremian
    "Hotriviyen",         // Hauterivian
    "Valanjiniyen",       // Valanginian
    "Berriaziyen",        // Berriasian
    "Titoniyen",          // Tithonian
    "Kimmericiyen",       // Kimmeridgian
    "Oksfordiyen",        // Oxfordian
    "Kalloviyen",         // Callovian
    "Batoniyen",          // Bathonian
    "Bajosiyen",          // Bajocian
    "Aaleniyen",          // Aalenian
    "Toarsiyen",          // Toarcian
    "Pliyensbahiyen",     // Pliensbachian
    "Sinemuriyen",        // Sinemurian
    "Hettanjiyen",        // Hettangian
    "Resiyen",            // Rhaetian
    "Noriyen",            // Norian
    "Karniyen",           // Carnian
    "Ladiniyen",          // Ladinian
    "Aniziyen",           // Anisian
    "Olenekiyen",         // Olenekian
    "İnduyen",            // Induan
    "Çangsingiyen",       // Changhsingian
    "Vuçepingiyen",       // Wuchiapingian
    "Kapitaniyen",        // Capitanian
    "Vordiyen",           // Wordian
    "Rodiyen",            // Roadian
    "Kunguriyen",         // Kungurian
    "Artinskiyen",        // Artinskian
    "Sakmariyen",         // Sakmarian
    "Asseliyen",          // Asselian
    "Gijeliyen",          // Gzhelian
    "Kasımoviyen",        // Kasimovian
    "Moskoviyen",         // Moscovian
    "Başkiriyen",         // Bashkirian
    "Serpukoviyen",       // Serpukhovian
    "Vizeyen",            // Visean
    "Turneziyen",         // Tournaisian
    "Fameniyen",          // Famennian
    "Frasniyen",          // Frasnian
    "Jivesiyen",          // Givetian
    "Eyfeliyen",          // Eifelian
    "Emsiyen",            // Emsian
    "Pragiyen",           // Pragian
    "Lohkoviyen",         // Lochkovian
    "Ludfordiyen",        // Ludfordian
    "Gorstiyen",          // Gorstian
    "Homeriyen",          // Homerian
    "Şenvudiyen",         // Sheinwoodian
    "Telisiyen",          // Telychian
    "Aroniyen",           // Aeronian
    "Ruddaniyen",         // Rhuddanian
    "Hirnansiyen",        // Hirnantian
    "Katiyen",            // Katian
    "Sandbiyen",          // Sandbian
    "Darriviliyen",       // Darriwilian
    "Dapingiyen",         // Dapingian
    "Floyen",             // Floian
    "Tremadosiyen",       // Tremadocian
    "Kat 10",             // Cambrian Stage 10
    "Jiyangşaniyen",      // Jiangshanian
    "Payibiyen",          // Paibian
    "Guzhangiyen",        // Guzhangian
    "Drumiyen",           // Drumian
    "Vuliuyan",           // Wuliuan
    "Kat 4",              // Cambrian Stage 4
    "Kat 3",              // Cambrian Stage 3
    "Kat 2",              // Cambrian Stage 2
    "Fortuniyen",         // Fortunian
];

/// Chinese, simplified script, `zh-Hans`: the chart's `@zh` labels.
const ZH_HANS: [&str; INTERVAL_COUNT] = [
    "显生宇",         // Phanerozoic
    "元古宇",         // Proterozoic
    "太古宇",         // Archean
    "冥古宇",         // Hadean
    "新生界",         // Cenozoic
    "中生界",         // Mesozoic
    "古生界",         // Paleozoic
    "新元古界",       // Neoproterozoic
    "中元古界",       // Mesoproterozoic
    "古元古界",       // Paleoproterozoic
    "新太古界",       // Neoarchean
    "中太古界",       // Mesoarchean
    "古太古界",       // Paleoarchean
    "始太古界",       // Eoarchean
    "第四系",         // Quaternary
    "新近系",         // Neogene
    "古近系",         // Paleogene
    "白垩系",         // Cretaceous
    "侏罗系",         // Jurassic
    "三叠系",         // Triassic
    "二叠系",         // Permian
    "石炭系",         // Carboniferous
    "泥盆系",         // Devonian
    "志留系",         // Silurian
    "奥陶系",         // Ordovician
    "寒武系",         // Cambrian
    "埃迪卡拉系",     // Ediacaran
    "成冰系",         // Cryogenian
    "拉伸系",         // Tonian
    "狭带系",         // Stenian
    "延展系",         // Ectasian
    "盖层系",         // Calymmian
    "固结系",         // Statherian
    "造山系",         // Orosirian
    "层侵系",         // Rhyacian
    "成铁系",         // Siderian
    "全新统",         // Holocene
    "更新统",         // Pleistocene
    "上新统",         // Pliocene
    "中新统",         // Miocene
    "渐新统",         // Oligocene
    "始新统",         // Eocene
    "古新统",         // Paleocene
    "",               // Upper Cretaceous
    "",               // Lower Cretaceous
    "",               // Upper Jurassic
    "",               // Middle Jurassic
    "",               // Lower Jurassic
    "",               // Upper Triassic
    "",               // Middle Triassic
    "",               // Lower Triassic
    "乐平统",         // Lopingian
    "瓜德鲁普统",     // Guadalupian
    "乌拉尔统",       // Cisuralian
    "",               // Upper Pennsylvanian
    "",               // Middle Pennsylvanian
    "",               // Lower Pennsylvanian
    "",               // Upper Mississippian
    "",               // Middle Mississippian
    "",               // Lower Mississippian
    "",               // Upper Devonian
    "",               // Middle Devonian
    "",               // Lower Devonian
    "普里道利统",     // Pridoli
    "罗德洛统",       // Ludlow
    "温洛克统",       // Wenlock
    "兰多维列统",     // Llandovery
    "",               // Upper Ordovician
    "",               // Middle Ordovician
    "",               // Lower Ordovician
    "芙蓉统",         // Furongian
    "苗岭统",         // Miaolingian
    "第二统",         // Cambrian Series 2
    "纽芬兰统",       // Terreneuvian
    "梅加拉亚阶",     // Meghalayan
    "诺斯格瑞比阶",   // Northgrippian
    "格陵兰阶",       // Greenlandian
    "",               // Upper Pleistocene
    "千叶阶",         // Chibanian
    "卡拉布里雅阶",   // Calabrian
    "杰拉阶",         // Gelasian
    "皮亚琴察阶",     // Piacenzian
    "赞克勒阶",       // Zanclean
    "墨西拿阶",       // Messinian
    "托尔托纳阶",     // Tortonian
    "塞拉瓦莱阶",     // Serravallian
    "兰盖阶",         // Langhian
    "波尔多阶",       // Burdigalian
    "阿基坦阶",       // Aquitanian
    "夏特阶",         // Chattian
    "吕珀尔阶",       // Rupelian
    "普利亚本阶",     // Priabonian
    "巴顿阶",         // Bartonian
    "卢泰特阶",       // Lutetian
    "伊普里斯阶",     // Ypresian
    "坦尼特阶",       // Thanetian
    "塞兰特阶",       // Selandian
    "丹麦阶",         // Danian
    "马斯特里赫特阶", // Maastrichtian
    "坎潘阶",         // Campanian
    "圣通阶",         // Santonian
    "康尼亚克阶",     // Coniacian
    "土伦阶",         // Turonian
    "塞诺曼阶",       // Cenomanian
    "阿尔布阶",       // Albian
    "阿普特阶",       // Aptian
    "巴雷姆阶",       // Barremian
    "欧特里夫阶",     // Hauterivian
    "瓦兰今阶",       // Valanginian
    "贝里阿斯阶",     // Berriasian
    "提塘阶",         // Tithonian
    "钦莫利阶",       // Kimmeridgian
    "牛津阶",         // Oxfordian
    "卡洛夫阶",       // Callovian
    "巴通阶",         // Bathonian
    "巴柔阶",         // Bajocian
    "阿林阶",         // Aalenian
    "托阿尔阶",       // Toarcian
    "普林斯巴阶",     // Pliensbachian
    "辛涅缪尔阶",     // Sinemurian
    "赫塘阶",         // Hettangian
    "瑞替阶",         // Rhaetian
    "诺利阶",         // Norian
    "卡尼阶",         // Carnian
    "拉丁阶",         // Ladinian
    "安尼阶",         // Anisian
    "奥伦尼克阶",     // Olenekian
    "印度阶",         // Induan
    "长兴阶",         // Changhsingian
    "吴家坪阶",       // Wuchiapingian
    "卡匹敦阶",       // Capitanian
    "沃德阶",         // Wordian
    "罗德阶",         // Roadian
    "空谷阶",         // Kungurian
    "亚丁斯克阶",     // Artinskian
    "萨克马尔阶",     // Sakmarian
    "阿瑟尔阶",       // Asselian
    "格舍尔阶",       // Gzhelian
    "卡西莫夫阶",     // Kasimovian
    "莫斯科阶",       // Moscovian
    "巴什基尔阶",     // Bashkirian
    "谢尔普霍夫阶",   // Serpukhovian
    "维宪阶",         // Visean
    "杜内阶",         // Tournaisian
    "法门阶",         // Famennian
    "弗拉阶",         // Frasnian
    "吉维特阶",       // Givetian
    "艾菲尔阶",       // Eifelian
    "埃姆斯阶",       // Emsian
    "布拉格阶",       // Pragian
    "洛赫考夫阶",     // Lochkovian
    "卢德福特阶",     // Ludfordian
    "高斯特阶",       // Gorstian
    "侯墨阶",         // Homerian
    "申伍德阶",       // Sheinwoodian
    "特列奇阶",       // Telychian
    "埃隆阶",         // Aeronian
    "鲁丹阶",         // Rhuddanian
    "赫南特阶",       // Hirnantian
    "凯迪阶",         // Katian
    "桑比阶",         // Sandbian
    "达瑞威尔阶",     // Darriwilian
    "大坪阶",         // Dapingian
    "弗洛阶",         // Floian
    "特马豆克阶",     // Tremadocian
    "第十阶",         // Cambrian Stage 10
    "江山阶",         // Jiangshanian
    "排碧阶",         // Paibian
    "古丈阶",         // Guzhangian
    "鼓山阶",         // Drumian
    "乌溜阶",         // Wuliuan
    "第四阶",         // Cambrian Stage 4
    "第三阶",         // Cambrian Stage 3
    "第二阶",         // Cambrian Stage 2
    "幸运阶",         // Fortunian
];

#[cfg(test)]
mod tests {
    use super::*;

    fn by_name(name: &str) -> &'static GeologicInterval {
        geologic::by_name(name).unwrap()
    }

    #[test]
    fn every_interval_has_a_place_in_chart_order() {
        let mut seen = [false; INTERVAL_COUNT];
        for rank in GeologicRank::ALL {
            for interval in geologic::intervals(*rank) {
                let index = chart_index(interval).unwrap();
                assert!(!seen[index], "{} twice", interval.name);
                seen[index] = true;
            }
        }
        assert!(seen.iter().all(|seen| *seen));
    }

    #[test]
    fn the_tables_are_in_tag_order_and_hold_names_only() {
        for pair in TRANSLATIONS.windows(2) {
            assert!(pair[0].tag < pair[1].tag);
        }
        for translation in TRANSLATIONS {
            for name in translation.names {
                assert_eq!(name.trim(), *name, "{}", translation.tag);
                assert!(
                    !name.contains(['\t', '\n', '\r']),
                    "{}: {name}",
                    translation.tag
                );
            }
        }
    }

    #[test]
    fn the_japanese_names_are_the_geological_society_of_japans() {
        // 国際年代層序表 v2024/12, 日本地質学会.
        assert_eq!(
            interval_name(by_name("Quaternary"), "ja"),
            Some("第四系／紀")
        );
        assert_eq!(interval_name(by_name("Holocene"), "ja"), Some("完新統／世"));
        assert_eq!(
            interval_name(by_name("Meghalayan"), "ja"),
            Some("メガラヤン")
        );
        assert_eq!(
            interval_name(by_name("Chibanian"), "ja"),
            Some("チバニアン")
        );
        assert_eq!(
            interval_name(by_name("Phanerozoic"), "ja"),
            Some("顕生（累）界／代")
        );
        assert_eq!(
            interval_name(by_name("Neoarchean"), "ja"),
            Some("新太古界／代（新始生界／代）")
        );
    }

    #[test]
    fn the_chinese_names_are_the_ics_chinese_charts_without_layout_spaces() {
        // 国际年代地层表 v2023/09.
        assert_eq!(
            interval_name(by_name("Phanerozoic"), "zh-Hans"),
            Some("显生宇")
        );
        assert_eq!(interval_name(by_name("Archean"), "zh-Hans"), Some("太古宇"));
        assert_eq!(
            interval_name(by_name("Quaternary"), "zh-Hans"),
            Some("第四系")
        );
        assert_eq!(
            interval_name(by_name("Meghalayan"), "zh-Hans"),
            Some("梅加拉亚阶")
        );
        assert_eq!(
            interval_name(by_name("Chibanian"), "zh-Hans-CN"),
            Some("千叶阶")
        );
        // No traditional-script table, and no likely-subtags expansion.
        assert_eq!(interval_name(by_name("Quaternary"), "zh-Hant"), None);
        assert_eq!(interval_name(by_name("Quaternary"), "zh"), None);
    }

    #[test]
    fn tags_are_matched_as_hc_i18n_matches_them() {
        let quaternary = by_name("Quaternary");
        assert_eq!(interval_name(quaternary, "JA_jp"), Some("第四系／紀"));
        assert_eq!(interval_name(quaternary, "de-AT"), Some("Quartär"));
        assert_eq!(interval_name(quaternary, "en"), None);
        assert_eq!(interval_name(quaternary, ""), None);
        assert_eq!(interval_name(quaternary, "am"), None);
    }

    #[test]
    fn what_the_chart_does_not_name_is_left_unnamed() {
        for translation in TRANSLATIONS {
            assert_eq!(translation.name_of(by_name("Upper Cretaceous")), None);
            assert_eq!(translation.name_of(by_name("Upper Pleistocene")), None);
        }
        // Placeholders in the wrong script are not names.
        assert_eq!(interval_name(by_name("Meghalayan"), "ru"), None);
        assert_eq!(interval_name(by_name("Hadean"), "ru"), None);
        assert_eq!(
            interval_name(by_name("Quaternary"), "ru"),
            Some("Четвертичная")
        );
    }

    #[test]
    fn the_cambrian_numbered_units_are_named_by_their_identifiers() {
        assert_eq!(
            interval_name(by_name("Cambrian Stage 10"), "zh-Hans"),
            Some("第十阶")
        );
        assert_eq!(
            interval_name(by_name("Cambrian Series 2"), "ko"),
            Some("제2세")
        );
    }
}
