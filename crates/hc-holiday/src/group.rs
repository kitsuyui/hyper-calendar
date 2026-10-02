//! The groups of people a holiday can be given to alone.
//!
//! Some statutes give a day to part of the population and to no one else.
//! China's 全国年节及纪念日放假办法, Article 3, gives women half of 8 March,
//! youth of fourteen and over half of 4 May, children under fourteen the
//! whole of 1 June and active servicemen half of 1 August; Taiwan's
//! 紀念日及節日實施條例, Article 6, leaves the day off on Police Day, Fire
//! Fighters' Day, Armed Forces Day and Coast Guard Day to the authority
//! of each service; Nepal's Home Ministry gives Teej to women employees
//! and Gai Jatra to the Newar community; Bangladesh's Ministry of Public
//! Administration lists optional holidays for each faith. Such a day is a rule of the
//! country's table scoped to a [`Group`], as a subdivision's day is a rule
//! scoped to its ISO 3166-2 code, and the two scopes are independent: a
//! rule may name a region, a
//! group, both or neither ([ADR 0011](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0011-a-day-for-one-group-is-a-scoped-rule.md)).
//!
//! A calendar asked for no group answers for everyone's days alone, and
//! one asked for a group adds that group's own days, as
//! [`Scope`](crate::rule::Scope) says. A group is named by its identifier,
//! lower-case and hyphenated, matched as every identifier is
//! ([`hc_core::catalogue::matches`]).
//!
//! The groups are data, not an `enum`: the world adds a group whenever a
//! statute names one ([ADR 0007](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0007-sets-the-world-can-extend-are-data.md)).
//! A group is only who the day is for. Who belongs to it — China's youth
//! are those of fourteen and over, its children those under fourteen — is
//! the statute's, and the rule's name and source say so; the same
//! identifier serves every country whose instrument names the same people,
//! whatever its limits. Each group's name in a language other than English
//! is `hc-i18n`'s, from a source written in it.

/// A group of people a holiday may be given to alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Group {
    /// A stable identifier, lower-case and hyphenated: `women`.
    pub id: &'static str,
    /// The English name, plural: `women`, `active servicemen`.
    pub english_name: &'static str,
}

hc_core::catalogue! {
    type: Group,
    id: |group| group.id,
    tests: group_catalogue,

    /// Every group a rule is scoped to, in the order of the first table
    /// that names each: China's four, Taiwan's services, then Nepal's
    /// communities, faiths and employees, then the groups Bangladesh's
    /// optional holidays are for that Nepal's do not name.
    pub const GROUPS;

    /// The group with this identifier.
    pub fn by_id;

    entries: {
        /// Women: China's 妇女 of Article 3 (一).
        pub const WOMEN = Group { id: "women", english_name: "women" };
        /// Young people: China's 14周岁以上的青年 of Article 3 (二), those of
        /// fourteen and over.
        pub const YOUTH = Group { id: "youth", english_name: "youth" };
        /// Children: China's 不满14周岁的少年儿童 of Article 3 (三), those
        /// under fourteen.
        pub const CHILDREN = Group { id: "children", english_name: "children" };
        /// Members of the armed forces: China's 现役军人 of Article 3 (四),
        /// those on active service, and those Taiwan's Ministry of National
        /// Defense gives Armed Forces Day.
        pub const MILITARY = Group { id: "military", english_name: "military personnel" };
        /// The police, whose Police Day Taiwan's Article 6 leaves to their
        /// authority.
        pub const POLICE = Group { id: "police", english_name: "police" };
        /// Firefighters, whose Fire Fighters' Day Taiwan's Article 6 leaves
        /// to their authority.
        pub const FIREFIGHTERS = Group { id: "firefighters", english_name: "firefighters" };
        /// The coast guard, whose Coast Guard Day Taiwan's Article 6 leaves
        /// to the Ocean Affairs Council.
        pub const COAST_GUARD = Group { id: "coast-guard", english_name: "coast guard personnel" };
        /// Indigenous people, each of whom Taiwan's Article 6 lets choose
        /// three days for the ceremonies of their people.
        pub const INDIGENOUS_PEOPLES = Group {
            id: "indigenous-peoples",
            english_name: "indigenous peoples",
        };
        /// The Newar community, to whom Nepal's Home Ministry notices give
        /// Gai Jatra "देशभरका नेवार समुदायका लागि मात्र".
        pub const NEWAR = Group { id: "newar", english_name: "the Newar community" };
        /// The Dura community, to whom the notice for 2083 BS gives Dura
        /// Mhaipru Nakuma.
        pub const DURA = Group { id: "dura", english_name: "the Dura community" };
        /// The Kirat faithful, किराँत धर्मावलम्बी, to whom the notices give
        /// Falgunanda Jayanti.
        pub const KIRAT = Group { id: "kirat", english_name: "the Kirat faithful" };
        /// Muslims, to whom Nepal's notices give the Prophet's birthday and
        /// to whom Bangladesh's give the optional holidays of its Muslim
        /// section.
        pub const MUSLIMS = Group { id: "muslims", english_name: "Muslims" };
        /// Sikhs, to whom the notices give Guru Nanak Jayanti.
        pub const SIKHS = Group { id: "sikhs", english_name: "Sikhs" };
        /// Persons with disabilities, to whose employees the notices give
        /// the International Day of Persons with Disabilities.
        pub const PERSONS_WITH_DISABILITIES = Group {
            id: "persons-with-disabilities",
            english_name: "persons with disabilities",
        };
        /// Hindus, for whom Bangladesh's notifications list the optional
        /// holidays of the Hindu section (হিন্দু পর্ব).
        pub const HINDUS = Group { id: "hindus", english_name: "Hindus" };
        /// Buddhists, for whom Bangladesh's notifications list the optional
        /// holidays of the Buddhist section (বৌদ্ধ পর্ব).
        pub const BUDDHISTS = Group { id: "buddhists", english_name: "Buddhists" };
        /// Christians, for whom Bangladesh's notifications list the optional
        /// holidays of the Christian section (খ্রিষ্টান পর্ব).
        pub const CHRISTIANS = Group { id: "christians", english_name: "Christians" };
        /// The employees of the small ethnic groups, ক্ষুদ্র নৃগোষ্ঠী, in
        /// the Chittagong Hill Tracts and outside them, for whom Bangladesh's
        /// notifications list Boisabi and the like. The notifications use
        /// this term, not "indigenous", so the group is not
        /// [`INDIGENOUS_PEOPLES`], Taiwan's.
        pub const SMALL_ETHNIC_GROUPS = Group {
            id: "small-ethnic-groups",
            english_name: "small ethnic groups",
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_identifier_is_lower_case_kebab_and_every_group_has_a_name() {
        for group in GROUPS {
            assert!(
                group
                    .id
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte == b'-'),
                "{}",
                group.id
            );
            assert!(!group.id.starts_with('-') && !group.id.ends_with('-'));
            assert!(!group.english_name.is_empty());
        }
    }

    #[test]
    fn a_group_is_found_in_either_case() {
        assert_eq!(by_id(" Women "), Some(WOMEN));
        assert_eq!(by_id("COAST-GUARD"), Some(COAST_GUARD));
        assert_eq!(by_id("woman"), None);
    }
}
