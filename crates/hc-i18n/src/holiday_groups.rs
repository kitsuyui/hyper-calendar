//! What a locale calls a group of people a holiday is given to alone, as
//! data, for the lines `hc_holiday_tables` writes.
//!
//! The groups are `hc-holiday`'s, `hc_holiday::group`, each with an
//! English name. A name in another language is carried only where an
//! instrument written in that language names the group as the group a day
//! is given to; nothing here is translated.
//!
//! # What is carried
//!
//! * **`zh-Hans`**: the four groups of Article 3 of the State Council's
//!   全国年节及纪念日放假办法, as it names them [gov-cn-holiday-measures-2024]:
//!   妇女 for `women`, 青年 for `youth` (its 14周岁以上的青年), 少年儿童
//!   for `children` (its 不满14周岁的少年儿童) and 现役军人 for `military`.
//! * **`ne`**: the seven groups of Nepal's Ministry of Home Affairs'
//!   notices of public holidays for 2080 to 2083 BS, as they name them
//!   [np-moha-holidays-2083]: नेवार समुदाय and दुरा समुदाय, the Newar and
//!   Dura communities of section 2.2; महिला, of its महिला कर्मचारी, for
//!   `women`; अपाङ्गता भएका, of its अपाङ्गता भएका कर्मचारी, for
//!   `persons-with-disabilities`; and किराँत धर्मावलम्बी, मुस्लिम
//!   धर्मावलम्बी and सिख धर्मावलम्बी, the Kirat, Muslim and Sikh faithful of
//!   section 7.2. The notices give the days to employees (कर्मचारी), and
//!   the name is of the people, without the word.
//!
//! Left without a name, and so written in English by the line writers:
//! Bangladesh's five, `muslims`, `hindus`, `christians`, `buddhists` and
//! `small-ethnic-groups`, in `bn`. The Ministry of Public Administration's
//! notifications that name them are in Bengali, but their PDFs were not
//! opened, and the newspapers that reproduce them spell the Christian
//! section's name two ways. (`muslims` has a name in `ne`, from Nepal's
//! notices.) And
//! Taiwan's five, `police`, `firefighters`, `military`, `coast-guard` and
//! `indigenous-peoples`, in `zh-Hant`. The 紀念日及節日實施條例 names the
//! services' days and the authorities that set them off — 消防節及警察節
//! 依主管機關規定放假 — and 原住民 for the indigenous days, but it gives no
//! day to a group it names as such, and the authorities' rules, which
//! would, were not read. Every other locale has no table.
//!
//! # Locales
//!
//! [`group_name`] walks [`Locale::fallback`] and takes the first table in
//! the chain that names the group, as [`crate::horizons`] does: `zh-CN`
//! and `zh-SG` find `zh-Hans`.

use crate::locale::Locale;

/// One locale's names for the groups.
#[derive(Debug, Clone, Copy)]
pub struct GroupNames {
    /// The BCP 47 tag, spelled as [`crate::data::LOCALES`] spells it.
    pub tag: &'static str,
    /// Each named group: its `hc_holiday::group` identifier and its name.
    pub names: &'static [(&'static str, &'static str)],
}

impl GroupNames {
    /// The name this table gives the group with an identifier, matched
    /// without regard to ASCII case.
    #[must_use]
    pub fn name_of(&self, id: &str) -> Option<&'static str> {
        self.names
            .iter()
            .find(|(group, _)| group.eq_ignore_ascii_case(id.trim()))
            .map(|(_, name)| *name)
    }
}

/// A group's name and the tag of the table that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupName {
    /// What the locale calls the group.
    pub name: &'static str,
    /// The tag of the table that answered: `zh-Hans` for `zh-CN`.
    pub tag: &'static str,
}

/// What `locale` calls a group, from the first table in its fallback
/// chain that names it; `None` when none does, the root locale included.
#[must_use]
pub fn group_name(locale: &Locale, id: &str) -> Option<GroupName> {
    locale.fallback().find_map(|candidate| {
        let rendered = candidate.rendered()?;
        TABLES
            .iter()
            .filter(|table| rendered.as_str() == table.tag)
            .find_map(|table| {
                table.name_of(id).map(|name| GroupName {
                    name,
                    tag: table.tag,
                })
            })
    })
}

/// Every table, in tag order.
pub static TABLES: &[GroupNames] = &[
    GroupNames {
        tag: "ne",
        names: &[
            ("dura", "दुरा समुदाय"),
            ("kirat", "किराँत धर्मावलम्बी"),
            ("muslims", "मुस्लिम धर्मावलम्बी"),
            ("newar", "नेवार समुदाय"),
            ("persons-with-disabilities", "अपाङ्गता भएका"),
            ("sikhs", "सिख धर्मावलम्बी"),
            ("women", "महिला"),
        ],
    },
    GroupNames {
        tag: "zh-Hans",
        names: &[
            ("children", "少年儿童"),
            ("military", "现役军人"),
            ("women", "妇女"),
            ("youth", "青年"),
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).expect("a tag")
    }

    #[test]
    fn every_table_is_a_carried_locale_and_names_each_group_once() {
        for table in TABLES {
            assert!(
                crate::data::LOCALES
                    .iter()
                    .any(|data| data.tag == table.tag)
            );
            for (index, (id, name)) in table.names.iter().enumerate() {
                assert!(!name.is_empty());
                assert!(
                    !table.names[index + 1..]
                        .iter()
                        .any(|(other, _)| other == id)
                );
            }
        }
    }

    /// `zh-CN` finds `zh-Hans` and `ne-NP` finds `ne`; `zh-TW`, English and
    /// the root locale name nothing, and nothing names Taiwan's services.
    #[test]
    fn a_name_comes_from_the_locales_chain_or_not_at_all() {
        for tag in ["zh-CN", "zh-Hans", "zh"] {
            assert_eq!(
                group_name(&locale(tag), " Women "),
                Some(GroupName {
                    name: "妇女",
                    tag: "zh-Hans"
                }),
                "{tag}"
            );
        }
        assert_eq!(group_name(&locale("zh-TW"), "women"), None);
        assert_eq!(group_name(&locale("en"), "women"), None);
        assert_eq!(group_name(&Locale::ROOT, "women"), None);
        assert_eq!(group_name(&locale("zh-Hans"), "police"), None);
        assert_eq!(
            group_name(&locale("ne-NP"), "women"),
            Some(GroupName {
                name: "महिला",
                tag: "ne"
            })
        );
    }
}
