//! The date, time and date-time formats of every carried locale, generated.
//!
//! **Do not edit.** `scripts/formats-cldr.py` writes this file from Unicode
//! CLDR 48 (`release-48`, read 2026-10-04, `cldr48-date-formats`): each carried
//! locale's `common/main/<file>.xml` over `root.xml`, the `dateFormats`,
//! `timeFormats`, standard `dateTimeFormats` and the `availableFormats`
//! items `hms`, `Hms`, `Gy`, `d`, `yMMMMd` and `yMMMd` of the calendars
//! `gregorian generic buddhist chinese coptic dangi ethiopic hebrew indian islamic iso8601 japanese persian roc`, resolved through `root.xml`'s aliases, as the
//! script's documentation says.

/// Each carried locale's and root's (`und`) formats, one row per calendar,
/// sorted by tag and calendar: the eighteen fields of `super::Field` in
/// order, `|` between them, empty where the lookup finds the value further
/// on.
pub(super) static FORMATS: &[(&str, &str, &str)] = &[
    (
        "am",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "am",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "am",
        "generic",
        "EEEE፣ d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG|||||||||h:mm:ss a|||||d MMM y",
    ),
    (
        "am",
        "gregorian",
        "EEEE d MMMM y|d MMMM y|d MMM y|dd/MM/y|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1} {0}|{1} {0}|{1} {0}|{1} {0}|a h:mm:ss|HH:mm:ss|y G|d||MMM d y",
    ),
    (
        "am",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a||G y|||y MMM d",
    ),
    (
        "ar",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "ar",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "ar",
        "generic",
        "EEEE، d MMMM y G|d MMMM y G|dd‏/MM‏/y G|d‏/M‏/y GGGGG||||||||{1}, {0}",
    ),
    (
        "ar",
        "gregorian",
        "EEEE، d MMMM y|d MMMM y|dd‏/MM‏/y|d‏/M‏/y|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1}، {0}|{1}، {0}|{1}، {0}|{1}، {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM y",
    ),
    ("ar", "islamic", "||d MMM y G"),
    (
        "ar",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||G y|||y MMM d",
    ),
    (
        "bn",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "bn",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "bn",
        "generic",
        "EEEE, d MMMM, y G|d MMMM, y G|d MMM, y G|d/M/y GGGGG",
    ),
    (
        "bn",
        "gregorian",
        "EEEE, d MMMM, y|d MMMM, y|d MMM, y|d/M/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM, y",
    ),
    (
        "bn",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a||G y|||y MMM d",
    ),
    (
        "bo",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "bo",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "bo",
        "generic",
        "G y MMMM d, EEEE|G སྤྱི་ལོ་y MMMMའི་ཚེས་d|G y ལོའི་MMMཚེས་d|G y-MM-dd|||||||||||||G སྤྱི་ལོ་y MMMMའི་ཚེས་d|G y ལོའི་MMMཚེས་d",
    ),
    (
        "bo",
        "gregorian",
        "y MMMMའི་ཚེས་d, EEEE|སྤྱི་ལོ་y MMMMའི་ཚེས་d|y ལོའི་MMMཚེས་d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d|སྤྱི་ལོ་y MMMMའི་ཚེས་d|y ལོའི་MMMཚེས་d",
    ),
    (
        "bo",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d||||||||||h:mm:ss a|||||y MMM d",
    ),
    (
        "cs",
        "chinese",
        "EEEE, d. M. y|d. M. y|d. M. y|d. M. y|||||||||h:mm:ss a|HH:mm:ss|r U|d",
    ),
    (
        "cs",
        "dangi",
        "EEEE, d. M. y|d. M. y|d. M. y|d. M. y|||||||||h:mm:ss a|HH:mm:ss|r U|d",
    ),
    (
        "cs",
        "generic",
        "EEEE d. MMMM y G|d. MMMM y G|d. M. y G|dd.MM.yy GGGGG",
    ),
    (
        "cs",
        "gregorian",
        "EEEE d. MMMM y|d. MMMM y|d. M. y|dd.MM.yy|H:mm:ss, zzzz|H:mm:ss z|H:mm:ss|H:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|H:mm:ss|y G|d.|d. MMMM y|d. M. y",
    ),
    (
        "cs",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm||||||HH:mm:ss|G y|d||y MMM d",
    ),
    ("cs", "japanese", "EEEE, d. MMMM y G"),
    (
        "de",
        "chinese",
        "EEEE, d. MMMM U|d. MMMM U|dd.MM U|dd.MM.yy|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||U",
    ),
    (
        "de",
        "dangi",
        "EEEE, d. MMMM U|d. MMMM U|dd.MM U|dd.MM.yy|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||U",
    ),
    (
        "de",
        "generic",
        "EEEE, d. MMMM y G|d. MMMM y G|dd.MM.y G|dd.MM.yy GGGGG",
    ),
    (
        "de",
        "gregorian",
        "EEEE, d. MMMM y|d. MMMM y|dd.MM.y|dd.MM.yy|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y G|d||d. MMM y",
    ),
    (
        "de",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||G y|||y MMM d",
    ),
    (
        "en",
        "chinese",
        "EEEE, MMMM d, r(U)|MMMM d, r(U)|MMM d, r|M/d/r|||||||||||r(U)",
    ),
    (
        "en",
        "dangi",
        "EEEE, MMMM d, r(U)|MMMM d, r(U)|MMM d, r|M/d/r|||||||||||r(U)",
    ),
    (
        "en",
        "generic",
        "EEEE, MMMM d, y G|MMMM d, y G|MMM d, y G|M/d/y G",
    ),
    (
        "en",
        "gregorian",
        "EEEE, MMMM d, y|MMMM d, y|MMM d, y|M/d/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y G|d||MMM d, y",
    ),
    (
        "en",
        "hebrew",
        "EEEE, d MMMM y|d MMMM y|d MMM y|d MMM y||||||||||||||d MMM y",
    ),
    (
        "en",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|||G y|||y MMM d",
    ),
    (
        "en",
        "japanese",
        "|||M/d/y GGGGG|||||{1} 'at' {0}|{1} 'at' {0}",
    ),
    (
        "en-001",
        "buddhist",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-001",
        "chinese",
        "EEEE, d MMMM r(U)|d MMMM r(U)|d MMM r|dd/MM/r",
    ),
    (
        "en-001",
        "coptic",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-001",
        "dangi",
        "EEEE, d MMMM r(U)|d MMMM r(U)|d MMM r|dd/MM/r",
    ),
    (
        "en-001",
        "ethiopic",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-001",
        "generic",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-001",
        "gregorian",
        "EEEE, d MMMM y|d MMMM y|d MMM y|dd/MM/y||||||||||||||d MMM y",
    ),
    (
        "en-001",
        "indian",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-001",
        "islamic",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-001",
        "japanese",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-001",
        "persian",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-001",
        "roc",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "en-GB",
        "buddhist",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "chinese",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "coptic",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "dangi",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "ethiopic",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "generic",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "gregorian",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "hebrew",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "indian",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "islamic",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "japanese",
        "|||d/M/y GGGGG|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "persian",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "en-GB",
        "roc",
        "||||HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm",
    ),
    (
        "es",
        "chinese",
        "EEEE, d-M-r|d-M-r|d-M-r|d-M-r|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|hh:mm:ss a||r",
    ),
    (
        "es",
        "dangi",
        "EEEE, d-M-r|d-M-r|d-M-r|d-M-r|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|hh:mm:ss a||r",
    ),
    (
        "es",
        "generic",
        "EEEE, d 'de' MMMM 'de' y G|d 'de' MMMM 'de' y G|d 'de' MMM 'de' y G|d/M/y GGGGG||||||||||HH:mm:ss",
    ),
    (
        "es",
        "gregorian",
        "EEEE, d 'de' MMMM 'de' y|d 'de' MMMM 'de' y|d MMM y|d/M/yy|H:mm:ss (zzzz)|H:mm:ss z|H:mm:ss|H:mm|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|H:mm:ss|y G|d|d 'de' MMMM 'de' y|d MMM y",
    ),
    (
        "es",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}||HH:mm:ss|G y|||y MMM d",
    ),
    ("es", "japanese", "||dd/MM/y GGGGG|dd/MM/yy GGGGG"),
    (
        "es-419",
        "buddhist",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "chinese",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "coptic",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "dangi",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "ethiopic",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "generic",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "gregorian",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a||||||HH:mm:ss",
    ),
    (
        "es-419",
        "hebrew",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "indian",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "islamic",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "japanese",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "persian",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "es-419",
        "roc",
        "||||h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a",
    ),
    (
        "fa",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}||HH:mm:ss|r U",
    ),
    (
        "fa",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}||HH:mm:ss|r U",
    ),
    (
        "fa",
        "generic",
        "EEEE d MMMM y G|d MMMM y G|d MMM y G|y/M/d GGGGG",
    ),
    (
        "fa",
        "gregorian",
        "EEEE d MMMM y|d MMMM y|d MMM y|y/M/d|H:mm:ss (zzzz)|H:mm:ss (z)|H:mm:ss|H:mm|{1}، ساعت {0}|{1}، ساعت {0}|{1}،‏ {0}|{1}،‏ {0}|h:mm:ss a|H:mm:ss|y G|d||d MMM y",
    ),
    ("fa", "islamic", "|||y/M/d G"),
    (
        "fa",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|||y MMM d",
    ),
    ("fa", "persian", "y MMMM d, EEEE|d MMMM y|d MMM y|y/M/d"),
    (
        "fil",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||r U",
    ),
    (
        "fil",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||r U",
    ),
    (
        "fil",
        "generic",
        "EEEE, MMMM d, y G|MMMM d, y G|MMM d, y G|M/d/y GGGGG",
    ),
    (
        "fil",
        "gregorian",
        "EEEE, MMMM d, y|MMMM d, y|MMM d, y|M/d/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y G|d||MMM d, y",
    ),
    ("fil", "hebrew", "EEEE, MMMM d y|MMMM d y|MMM d y|MMM d y"),
    (
        "fil",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|||G y|||y MMM d",
    ),
    (
        "fr",
        "chinese",
        "EEEE d MMMM U|d MMMM U|d MMM U|d/M/y|||||||||h:mm:ss a||U",
    ),
    (
        "fr",
        "dangi",
        "EEEE d MMMM U|d MMMM U|d MMM U|d/M/y|||||||||h:mm:ss a||U",
    ),
    (
        "fr",
        "generic",
        "EEEE d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG|||||{1} {0}|{1} {0}|{1} {0}",
    ),
    (
        "fr",
        "gregorian",
        "EEEE d MMMM y|d MMMM y|d MMM y|dd/MM/y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1}, {0}|{1}, {0}|{1}, {0}|{1} {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM y",
    ),
    (
        "fr",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||{1} {0}|{1} {0}|{1} {0}||||G y|||y MMM d",
    ),
    (
        "ha",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "ha",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "ha",
        "generic",
        "EEEE, d MMMM, y G|d MMMM, y G|d MMM, y G|d/M/yy GGGGG|||||{0}, {1}|{0}, {1}|{0}, {1}|{0}, {1}|h:mm:ss a",
    ),
    (
        "ha",
        "gregorian",
        "EEEE d MMMM, y|d MMMM, y|d MMM, y|d/M/yy|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|G y|d||d MMM, y",
    ),
    ("ha", "islamic", "||||||||||||||y G"),
    (
        "ha",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd||||||{1} {0}|{1} {0}|{1} {0}||||||y MMM d",
    ),
    (
        "he",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}||HH:mm:ss|r U",
    ),
    (
        "he",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}||HH:mm:ss|r U",
    ),
    (
        "he",
        "generic",
        "EEEE, d בMMMM y G|d בMMMM y G|d בMMM y G|d.M.y GGGGG",
    ),
    (
        "he",
        "gregorian",
        "EEEE, d בMMMM y|d בMMMM y|d בMMM y|d.M.y|H:mm:ss zzzz|H:mm:ss z|H:mm:ss|H:mm|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|H:mm:ss|y G|d||d בMMM y",
    ),
    (
        "he",
        "hebrew",
        "EEEE, d בMMMM y|d בMMMM y|d בMMMM y|d בMMMM y",
    ),
    ("he", "islamic", "|||dd/MM/yy GGGGG"),
    (
        "he",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|||y MMM d",
    ),
    ("hi", "buddhist", "||||||||||||||y G"),
    (
        "hi",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "hi",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "hi",
        "generic",
        "G EEEE, d MMMM y|G d MMMM y|G d MMM y|G d/M/y|||||||||||G y",
    ),
    (
        "hi",
        "gregorian",
        "EEEE, d MMMM y|d MMMM y|d MMM y|d/M/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM y",
    ),
    (
        "hi",
        "indian",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|d/M/y GGGGG|||||||||||y G",
    ),
    (
        "hi",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||G y|||y MMM d",
    ),
    ("id", "buddhist", "|||d/M/y GGGGG"),
    (
        "id",
        "chinese",
        "EEEE, U MMMM dd|U MMMM d|U MMM d|y-M-d|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|r U",
    ),
    (
        "id",
        "dangi",
        "EEEE, U MMMM dd|U MMMM d|U MMM d|y-M-d|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|r U",
    ),
    (
        "id",
        "generic",
        "EEEE, dd MMMM y G|d MMMM y G|d MMM y G|dd/MM/yy GGGGG|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "id",
        "gregorian",
        "EEEE, dd MMMM y|d MMMM y|d MMM y|dd/MM/yy|HH.mm.ss zzzz|HH.mm.ss z|HH.mm.ss|HH.mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h.mm.ss a|HH.mm.ss|y G|d||d MMM y",
    ),
    ("id", "islamic", "|||d/M/y GGGGG"),
    (
        "id",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a|HH:mm:ss|G y|||y MMM d",
    ),
    ("id", "japanese", "|||d/M/y GGGGG"),
    ("id", "roc", "|||d/M/y GGGGG"),
    (
        "it",
        "chinese",
        "EEEE d MMMM U|dd MMMM U|dd MMM U|dd/MM/yy|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||r U",
    ),
    (
        "it",
        "dangi",
        "EEEE d MMMM U|dd MMMM U|dd MMM U|dd/MM/yy|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||r U",
    ),
    (
        "it",
        "generic",
        "EEEE d MMMM y G|dd MMMM y G|dd MMM y G|dd/MM/yy GGGGG|||||{1}, {0}|{1}, {0}|||hh:mm:ss a",
    ),
    (
        "it",
        "gregorian",
        "EEEE d MMMM y|d MMMM y|d MMM y|dd/MM/yy|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM y",
    ),
    (
        "it",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||||{1} {0}|{1} {0}|||G y|||y MMM d",
    ),
    (
        "ja",
        "buddhist",
        "GGGGy年M月d日EEEE|GGGGy年M月d日|Gy/MM/dd|Gy/MM/dd|||||||||||GGGGy年",
    ),
    (
        "ja",
        "chinese",
        "U年MMMd日EEEE|U年MMMd日|U年MMMd日|U-M-d|||||||||||U年",
    ),
    (
        "ja",
        "dangi",
        "U年MMMd日EEEE|U年MMMd日|U年MMMd日|U-M-d|||||||||||U年",
    ),
    (
        "ja",
        "generic",
        "Gy年M月d日(EEEE)|Gy年M月d日|GGGGGy/MM/dd|GGGGGy/M/d",
    ),
    (
        "ja",
        "gregorian",
        "y年M月d日EEEE|y年M月d日|y/MM/dd|y/MM/dd|H時mm分ss秒 zzzz|H:mm:ss z|H:mm:ss|H:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|aK:mm:ss|H:mm:ss|Gy年|d日||y年M月d日",
    ),
    ("ja", "hebrew", "Gy年M月d日EEEE||Gy/MM/dd|Gy/MM/dd"),
    ("ja", "islamic", "Gy年M月d日EEEE||Gy/MM/dd|Gy/MM/dd"),
    (
        "ja",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a|HH:mm:ss|G y|d||y MMM d",
    ),
    ("ja", "japanese", "Gy年M月d日EEEE||Gy年M月d日"),
    ("ja", "roc", "Gy年M月d日EEEE||Gy/MM/dd|Gy/MM/dd"),
    (
        "jv",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "jv",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "jv",
        "generic",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd-MM-y GGGGG|||||||{1} {0}|{1} {0}|||y G",
    ),
    (
        "jv",
        "gregorian",
        "EEEE, d MMMM y|d MMMM y|d MMM y|dd-MM-y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|G y|d||d MMM y",
    ),
    (
        "jv",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||||{1} {0}|{1} {0}||||||y MMM d",
    ),
    (
        "kab",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "kab",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "kab",
        "generic",
        "EEEE d MMMM y G|d MMMM y G|d MMM, y G|d/M/y GGGGG",
    ),
    (
        "kab",
        "gregorian",
        "EEEE d MMMM y|d MMMM y|d MMM, y|d/M/y|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d||d MMM y",
    ),
    (
        "kab",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a|||||y MMM d",
    ),
    (
        "ko",
        "chinese",
        "U년 MMM d일 EEEE|U년 MMM d일|y. M. d.|y/M/d|||||||||||r년(U년)",
    ),
    (
        "ko",
        "dangi",
        "U년 MMM d일 EEEE|U년 MMM d일|y. M. d.|y. M. d.|||||||||||r년(U년)",
    ),
    (
        "ko",
        "generic",
        "G y년 M월 d일 EEEE|G y년 M월 d일|G y. M. d.|G y. M. d.||||||||||HH:mm:ss",
    ),
    (
        "ko",
        "gregorian",
        "y년 MMMM d일 EEEE|y년 MMMM d일|y. M. d.|yy. M. d.|a h시 m분 s초 zzzz|a h시 m분 s초 z|a h:mm:ss|a h:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|a h:mm:ss|H시 m분 s초|G y년|d일||y년 MMM d일",
    ),
    (
        "ko",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a|HH:mm:ss|G y|d||y MMM d",
    ),
    (
        "ml",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "ml",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "ml",
        "generic",
        "G y MMMM d, EEEE|G y MMMM d|G y MMM d|G y-MM-dd",
    ),
    (
        "ml",
        "gregorian",
        "y MMMM d, EEEE|y MMMM d|y MMM d|d/M/yy|zzzz h:mm:ss a|z h:mm:ss a|h:mm:ss a|h:mm a|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|G y|d||y MMM d",
    ),
    (
        "ml",
        "iso8601",
        "|||y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a",
    ),
    (
        "mn",
        "chinese",
        "r(U) 'оны' MM 'сарын' d, EEEE|r(U) 'оны' MM-'р' 'сарын' d|r.MM.d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "mn",
        "dangi",
        "r(U) 'оны' MM 'сарын' d, EEEE|r(U) 'оны' MM-'р' 'сарын' d|r.MM.d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "mn",
        "generic",
        "G y 'оны' MMMM'ын' d. cccc 'гараг'|G y 'оны' MM 'сарын' dd|G y 'оны' MMM'ын' d|GGGGG y.MM.dd",
    ),
    (
        "mn",
        "gregorian",
        "y 'оны' MMMM'ын' d, EEEE 'гараг'|y 'оны' MMMM'ын' d|y 'оны' MMM'ын' d|y.MM.dd|HH:mm:ss (zzzz)|HH:mm:ss (z)|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d||y 'оны' MMM'ын' d",
    ),
    (
        "mn",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z||||||||||||y MMM d",
    ),
    (
        "mr",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}||HH:mm:ss|r U",
    ),
    (
        "mr",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}||HH:mm:ss|r U",
    ),
    (
        "mr",
        "generic",
        "EEEE, d MMMM, y G|d MMMM, y G|d MMM, y G|d/M/y GGGGG||||||||||H-mm-ss|y G",
    ),
    (
        "mr",
        "gregorian",
        "EEEE, d MMMM, y|d MMMM, y|d MMM, y|d/M/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|H:mm:ss|G y|d||d MMM, y",
    ),
    (
        "mr",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss||||y MMM d",
    ),
    (
        "my",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "my",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "my",
        "generic",
        "EEEE G dd MMMM y|G dd MMMM y|G d MMM y|GGGGG d/M/y",
    ),
    (
        "my",
        "gregorian",
        "y MMMM d EEEE|y MMMM d|y MMM d|d/M/yy|zzzz HH:mm:ss|z HH:mm:ss|H:mm:ss|H:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|a h:mm:ss|HH:mm:ss|G y|d||y MMM d",
    ),
    (
        "my",
        "iso8601",
        "y MMMM d, EEEE|||y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a",
    ),
    (
        "ne",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "ne",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "ne",
        "generic",
        "G y MMMM d, EEEE|G y MMMM d|G y MMM d|G y-MM-dd|||||{1}, {0}|{1}, {0}|{1},{0}|{1},{0}",
    ),
    (
        "ne",
        "gregorian",
        "y MMMM d, EEEE|y MMMM d|y MMM d|yy/M/d|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|G y|d||y MMM d",
    ),
    (
        "ne",
        "iso8601",
        "|||y-MM-dd|||||||{1} {0}|{1} {0}|h:mm:ss a",
    ),
    ("nl", "buddhist", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    (
        "nl",
        "chinese",
        "EEEE d MMMM U|d MMMM U|d MMM U|dd-MM-yy|||||{1} {0}|{1} {0}|{1} {0}||h:mm:ss a||U",
    ),
    ("nl", "coptic", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    (
        "nl",
        "dangi",
        "EEEE d MMMM r (U)|d MMMM r (U)|d MMM r|dd-MM-r|||||{1} {0}|{1} {0}|{1} {0}||h:mm:ss a||r (U)",
    ),
    ("nl", "ethiopic", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    (
        "nl",
        "generic",
        "EEEE d MMMM y G|d MMMM y G|d MMM y G|dd-MM-yy GGGGG||||||||{1} {0}",
    ),
    (
        "nl",
        "gregorian",
        "EEEE d MMMM y|d MMMM y|d MMM y|dd-MM-y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM y",
    ),
    ("nl", "hebrew", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    ("nl", "indian", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    ("nl", "islamic", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    (
        "nl",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||G y|||y MMM d",
    ),
    ("nl", "japanese", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    ("nl", "persian", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    ("nl", "roc", "||||||||{1} {0}|{1} {0}|{1} {0}"),
    (
        "pa-Arab",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "pa-Arab",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "pa-Arab",
        "generic",
        "EEEE, dd MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "pa-Arab",
        "gregorian",
        "EEEE, dd MMMM y|d MMMM y|d MMM y|dd/MM/y|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d||y MMM d",
    ),
    (
        "pa-Arab",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a",
    ),
    ("pa-Guru", "buddhist", "||||||||{1}, {0}|{1}, {0}"),
    (
        "pa-Guru",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||{1} {0}|{1} {0}|||r U",
    ),
    (
        "pa-Guru",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||{1} {0}|{1} {0}|||r U",
    ),
    (
        "pa-Guru",
        "generic",
        "EEEE, dd MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "pa-Guru",
        "gregorian",
        "EEEE, d MMMM y|d MMMM y|d MMM y|d/M/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1} {0}|{1} {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|G y|d||d MMM y",
    ),
    (
        "pa-Guru",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||{1} {0}|{1} {0}|h:mm:ss a|||||y MMM d",
    ),
    (
        "pcm",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||r U",
    ),
    (
        "pcm",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||r U",
    ),
    (
        "pcm",
        "generic",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "pcm",
        "gregorian",
        "EEEE, d MMMM y|d MMMM y|d MMM y|dd/MM/y|HH:mm:ss zzzz|H:mm:ss z|HH:mm:ss|HH:mm|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|G y|d||d MMM y",
    ),
    (
        "pcm",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd||HH:mm:ss z|||{1} {0}|{1} {0}|{1} {0}|{1} {0}||||||y MMM d",
    ),
    (
        "pl",
        "chinese",
        "EEEE, d MMMM U|d MMMM U|d MMM U|dd.MM.y|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "pl",
        "dangi",
        "EEEE, d MMMM U|d MMMM U|d MMM U|dd.MM.y|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "pl",
        "generic",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd.MM.y G|||||{1}, {0}|{1}, {0}|||h:mm:ss a",
    ),
    (
        "pl",
        "gregorian",
        "EEEE, d MMMM y|d MMMM y|d MMM y|d.MM.y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y G|d|d MMMM y|d MMM y",
    ),
    (
        "pl",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||||{1} {0}|{1} {0}|||G y|||y MMM d",
    ),
    (
        "ps",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd||||||||||HH:mm:ss|r U",
    ),
    (
        "ps",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd||||||||||HH:mm:ss|r U",
    ),
    (
        "ps",
        "generic",
        "G y MMMM d, EEEE|G y MMMM d|G y MMM d|GGGGG y/M/d||||||||||H:mm:ss",
    ),
    (
        "ps",
        "gregorian",
        "EEEE د y د MMMM d|y MMMM d|y MMM d|y/M/d|H:mm:ss (zzzz)|H:mm:ss (z)|H:mm:ss|H:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d||y MMM d",
    ),
    (
        "ps",
        "iso8601",
        "y MMMM d, EEEE|||y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a",
    ),
    (
        "pt",
        "chinese",
        "EEEE, d 'de' MMMM 'de' U|d 'de' MMMM 'de' U|dd/MM U|dd/MM/r|||||||||||r(U)",
    ),
    (
        "pt",
        "dangi",
        "EEEE, d 'de' MMMM 'de' U|d 'de' MMMM 'de' U|dd/MM U|dd/MM/r|||||||||||r(U)",
    ),
    (
        "pt",
        "generic",
        "EEEE, d 'de' MMMM 'de' y G|d 'de' MMMM 'de' y G|d 'de' MMM 'de' y G|dd/MM/y GGGGG",
    ),
    (
        "pt",
        "gregorian",
        "EEEE, d 'de' MMMM 'de' y|d 'de' MMMM 'de' y|d 'de' MMM 'de' y|dd/MM/y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|y G|d|d 'de' MMMM 'de' y|d 'de' MMM 'de' y",
    ),
    (
        "pt",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||||||||G y|||y MMM d",
    ),
    ("pt", "japanese", "||dd/MM/y G|dd/MM/yy GGGGG"),
    (
        "pt-PT",
        "buddhist",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "chinese",
        "||d 'de' MMM 'de' U|dd/MM/yy|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}",
    ),
    (
        "pt-PT",
        "coptic",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "dangi",
        "||d 'de' MMM 'de' U|dd/MM/yy|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}",
    ),
    (
        "pt-PT",
        "ethiopic",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "generic",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "gregorian",
        "||dd/MM/y|dd/MM/yy|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}||||||d/MM/y",
    ),
    (
        "pt-PT",
        "hebrew",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "indian",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "islamic",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "iso8601",
        "||||||||{1} {0}|{1} {0}|{1} {0}|{1} {0}",
    ),
    (
        "pt-PT",
        "japanese",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "persian",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "pt-PT",
        "roc",
        "|||d/M/y G|||||{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}",
    ),
    (
        "ru",
        "chinese",
        "EEEE, d MMMM U|d MMMM U|dd.MM U|dd.MM.y|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}||H:mm:ss|U",
    ),
    (
        "ru",
        "dangi",
        "EEEE, d MMMM U|d MMMM U|dd.MM U|dd.MM.y|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}||H:mm:ss|U",
    ),
    (
        "ru",
        "generic",
        "EEEE, d MMMM y 'г'. G|d MMMM y 'г'. G|d MMM y 'г'. G|dd.MM.y G",
    ),
    (
        "ru",
        "gregorian",
        "EEEE, d MMMM y 'г'.|d MMMM y 'г'.|d MMM y 'г'.|dd.MM.y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y 'г'. G|d||d MMM y 'г'.",
    ),
    (
        "ru",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||G y|||y MMM d",
    ),
    (
        "sa",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "sa",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "sa",
        "generic",
        "G EEEE, d MMMM y|G d MMMM y|G d MMM y|G d/M/y|||||||||||G y",
    ),
    (
        "sa",
        "gregorian",
        "EEEE, d MMMM y|d MMMM y|d MMM y|d/M/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM y",
    ),
    (
        "sa",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||G y|||y MMM d",
    ),
    (
        "sw",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "sw",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "sw",
        "generic",
        "EEEE, d MMMM y G|d MMMM y G|d MMM y G|dd/MM/y GGGGG",
    ),
    (
        "sw",
        "gregorian",
        "EEEE, d MMMM y|d MMMM y|d MMM y|dd/MM/y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM y",
    ),
    (
        "sw",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||||||||G y|||y MMM d",
    ),
    (
        "syr",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "syr",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||r U",
    ),
    (
        "syr",
        "generic",
        "EEEE، d ܒMMMM y G|d ܒMMMM y G|d ܒMMM y G|d/M/y GGGGG",
    ),
    (
        "syr",
        "gregorian",
        "EEEE، d ܒMMMM y|d ܒMMMM y|d ܒMMM y|d-MM-y|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1}، {0}|{1}، {0}|{1}، {0}|{1}، {0}|h:mm:ss a|HH:mm:ss|y G|d||d ܒMMM y",
    ),
    (
        "syr",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||G y|||y MMM d",
    ),
    (
        "ta",
        "chinese",
        "U MMMM d, EEEE|U, d MMMM|U, d MMM|dd-MM-y|||||||{1} {0}|{1} {0}|||r U",
    ),
    (
        "ta",
        "dangi",
        "U MMMM d, EEEE|U, d MMMM|U, d MMM|dd-MM-y|||||||{1} {0}|{1} {0}|||r U",
    ),
    (
        "ta",
        "generic",
        "EEEE, d MMMM, y G|d MMMM, y G|d MMM, y G|d/M/y GGGGG|||||{1} {0}|{1} {0}|||||y G",
    ),
    (
        "ta",
        "gregorian",
        "EEEE, d MMMM, y|d MMMM, y|d MMM, y|d/M/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1}, {0}|{1}, {0}|{1}, {0}|{1}, {0}|h:mm:ss a|HH:mm:ss|G y|d||d MMM, y",
    ),
    (
        "ta",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|||||y MMM d",
    ),
    (
        "te",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "te",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "te",
        "generic",
        "EEEE d MMMM y G|d MMMM y G|d MMM y G|dd-MM-y GGGGG|||||||||||y G",
    ),
    (
        "te",
        "gregorian",
        "d, MMMM y, EEEE|d MMMM, y|d MMM, y|dd-MM-yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d||d, MMM y",
    ),
    (
        "te",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a|||||y MMM d",
    ),
    ("th", "buddhist", "|d MMMM y|d MMM y|d/M/yy"),
    (
        "th",
        "chinese",
        "EEEE, U MMMM d|U MMMM d|U MMM d|y-M-d|||||||||||r U",
    ),
    (
        "th",
        "dangi",
        "EEEE, U MMMM d|U MMMM d|U MMM d|y-M-d|||||||||||r U",
    ),
    (
        "th",
        "generic",
        "EEEEที่ d MMMM G y|d MMMM G y|d MMM G y|d/M/y G",
    ),
    (
        "th",
        "gregorian",
        "EEEEที่ d MMMM G y|d MMMM G y|d MMM y|d/M/yy|H นาฬิกา mm นาที ss วินาที zzzz|H นาฬิกา mm นาที ss วินาที z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d|d MMMM y|d MMM y",
    ),
    (
        "th",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|||||||h:mm:ss a|||||y MMM d",
    ),
    ("th", "japanese", "EEEEที่ d MMMM ปีGที่ y|d MMMM ปีG y||d/M/yy G"),
    (
        "th",
        "roc",
        "EEEEที่ d MMMM ปีGที่ y|d MMMM ปีG y|||||||||||||ปีGที่ y",
    ),
    (
        "tr",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "tr",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||h:mm:ss a||r U",
    ),
    (
        "tr",
        "generic",
        "G d MMMM y EEEE|G d MMMM y|G d MMM y|GGGGG d.MM.y|||||||||h:mm:ss a",
    ),
    (
        "tr",
        "gregorian",
        "d MMMM y EEEE|d MMMM y|d MMM y|d.MM.y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|a h:mm:ss|HH:mm:ss|G y|d||d MMM y",
    ),
    (
        "tr",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||||||h:mm:ss a|||||y MMM d",
    ),
    (
        "tr",
        "japanese",
        "d MMMM y G EEEE|d MMMM y G|d MMM y G|d.MM.y G",
    ),
    (
        "und",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "und",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "und",
        "generic",
        "G y MMMM d, EEEE|G y MMMM d|G y MMM d|G y-MM-dd",
    ),
    (
        "und",
        "gregorian",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d||y MMM d",
    ),
    ("und", "iso8601", "||||||||||||h:mm:ss a"),
    (
        "ur",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "ur",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "ur",
        "generic",
        "EEEE، d MMMM، y G|d MMMM، y G|d MMM، y G|d/M/y GGGGG",
    ),
    (
        "ur",
        "gregorian",
        "EEEE، d MMMM، y|d MMMM، y|d MMM، y|d/M/yy|h:mm:ss a zzzz|h:mm:ss a z|h:mm:ss a|h:mm a|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|y G|d||d MMM، y",
    ),
    (
        "ur",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a||G y|||y MMM d",
    ),
    (
        "vi",
        "buddhist",
        "EEEE, 'ngày' dd MMMM 'năm' y G|||||||||||||||'Ngày' dd",
    ),
    (
        "vi",
        "chinese",
        "EEEE, 'ngày' dd MMMM 'năm' U|'Ngày' dd 'tháng' M 'năm' U|dd-MM U|dd/MM/y|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||U r",
    ),
    (
        "vi",
        "dangi",
        "EEEE, 'ngày' dd MMMM 'năm' U|'Ngày' dd 'tháng' M 'năm' U|dd-MM U|dd/MM/y|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||U r",
    ),
    (
        "vi",
        "generic",
        "EEEE, 'ngày' d 'tháng' M 'năm' y G|'ngày' d 'tháng' M 'năm' y G|d MMM, y G|d/M/y GGGGG",
    ),
    (
        "vi",
        "gregorian",
        "EEEE, d MMMM, y|d MMMM, y|d MMM, y|d/M/yy|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{0} {1}|{0} {1}|{0} {1}|{0} {1}|h:mm:ss a|HH:mm:ss|y G|d||d MMM, y",
    ),
    (
        "vi",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||{1} {0}|{1} {0}|{1} {0}|{1} {0}|||G y|||y MMM d",
    ),
    (
        "vi",
        "japanese",
        "EEEE, 'ngày' dd MMMM 'năm' y G|'Ngày' dd 'tháng' M 'năm' y G|dd-MM-y G|dd/MM/y G",
    ),
    (
        "vi",
        "roc",
        "EEEE, 'ngày' dd MMMM 'năm' y G|||||||||||||||'Ngày' dd",
    ),
    (
        "yue-Hans",
        "buddhist",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "yue-Hans",
        "chinese",
        "U (r) 年MMMdEEEE|U (r) 年MMMd|r年MMMd|r/M/d|||||{1} {0}||||h:mm:ss a|HH:mm:ss|rU年",
    ),
    (
        "yue-Hans",
        "dangi",
        "U年MMMd日EEEE|U年MMMd日|U年MMMd日|U/M/d|||||{1} {0}||||h:mm:ss a|HH:mm:ss|rU年",
    ),
    (
        "yue-Hans",
        "generic",
        "G y年M月d日 EEEE|G y年M月d日|G y年M月d日|G y/M/d|||||{1}{0}|||||H:mm:ss|G y年",
    ),
    (
        "yue-Hans",
        "gregorian",
        "y年M月d日 EEEE|y年M月d日|y年M月d日|y/M/d|HH:mm:ss [zzzz]|HH:mm:ss [z]|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|ah:mm:ss|HH:mm:ss|Gy年|d日||y年M月d日",
    ),
    (
        "yue-Hans",
        "hebrew",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d",
    ),
    (
        "yue-Hans",
        "islamic",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "yue-Hans",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|||||||h:mm:ss a||G y|d||y MMM d",
    ),
    (
        "yue-Hans",
        "japanese",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||{1} {0}|||||HH:mm:ss|Gy年",
    ),
    (
        "yue-Hans",
        "roc",
        "Gy年M月d日 EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "yue-Hant",
        "buddhist",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "yue-Hant",
        "chinese",
        "U (r) 年MMMdEEEE|U (r) 年MMMd|r年MMMd|r/M/d|||||{1} {0}||||h:mm:ss a|HH:mm:ss|rU年",
    ),
    (
        "yue-Hant",
        "dangi",
        "U年MMMd日EEEE|U年MMMd日|U年MMMd日|U/M/d|||||{1} {0}||||h:mm:ss a|HH:mm:ss|rU年",
    ),
    (
        "yue-Hant",
        "generic",
        "G y年M月d日 EEEE|G y年M月d日|G y年M月d日|G y/M/d|||||{1}{0}|||||H:mm:ss|G y年",
    ),
    (
        "yue-Hant",
        "gregorian",
        "y年M月d日 EEEE|y年M月d日|y年M月d日|y/M/d|ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|ah:mm:ss|HH:mm:ss|Gy年|d日||y年M月d日",
    ),
    (
        "yue-Hant",
        "hebrew",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d",
    ),
    (
        "yue-Hant",
        "islamic",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "yue-Hant",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|||||h:mm:ss a||G y|d||y MMM d",
    ),
    (
        "yue-Hant",
        "japanese",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||{1} {0}|||||HH:mm:ss|Gy年",
    ),
    (
        "yue-Hant",
        "roc",
        "Gy年M月d日 EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "zgh",
        "chinese",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "zgh",
        "dangi",
        "r(U) MMMM d, EEEE|r(U) MMMM d|r MMM d|r-MM-dd|||||||||||r U",
    ),
    (
        "zgh",
        "generic",
        "EEEE d MMMM y G|d MMMM y G|d MMM, y G|d/M/y GGGGG",
    ),
    (
        "zgh",
        "gregorian",
        "EEEE d MMMM y|d MMMM y|d MMM, y|d/M/y|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a|HH:mm:ss|G y|d||y MMM d",
    ),
    (
        "zgh",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|||||||||h:mm:ss a",
    ),
    ("zh-Hans", "buddhist", "|||Gy-M-d"),
    (
        "zh-Hans",
        "chinese",
        "rU年MMMdEEEE|rU年MMMd|r年MMMd|r/M/d|||||||||||rU年",
    ),
    ("zh-Hans", "coptic", "Gy年MM月d日EEEE"),
    (
        "zh-Hans",
        "dangi",
        "rU年MMMdEEEE|rU年MMMd|r年MMMd|r/M/d|||||||||||rU年",
    ),
    ("zh-Hans", "ethiopic", "Gy年MM月d日EEEE"),
    (
        "zh-Hans",
        "generic",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d",
    ),
    (
        "zh-Hans",
        "gregorian",
        "y年M月d日EEEE|y年M月d日|y年M月d日|y/M/d|zzzz HH:mm:ss|z HH:mm:ss|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|ah:mm:ss|HH:mm:ss|Gy年|d日||y年M月d日",
    ),
    ("zh-Hans", "hebrew", "|||G y/M/d"),
    (
        "zh-Hans",
        "indian",
        "Gy年MM月d日EEEE|Gy年MM月d日|Gy年MM月d日|G y/M/d",
    ),
    (
        "zh-Hans",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|||||||h:mm:ss a||G y|d||y MMM d",
    ),
    ("zh-Hans", "japanese", "|||Gy-MM-dd"),
    (
        "zh-Hant",
        "buddhist",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "zh-Hant",
        "chinese",
        "rU年MMMd EEEE|rU年MMMd|r年MMMd|r/M/d|||||{1} {0}|||||HH:mm:ss|rU年",
    ),
    (
        "zh-Hant",
        "dangi",
        "U年MMMd日EEEE|U年MMMd日|U年MMMd日|U/M/d|||||{1} {0}|||||HH:mm:ss|rU年",
    ),
    (
        "zh-Hant",
        "generic",
        "G y年M月d日 EEEE|G y年M月d日|G y年M月d日|G y/M/d||||||{1} {0}|{1} {0}|{1} {0}||H:mm:ss|G y年",
    ),
    (
        "zh-Hant",
        "gregorian",
        "y年M月d日 EEEE|y年M月d日|y年M月d日|y/M/d|Bh:mm:ss [zzzz]|Bh:mm:ss [z]|Bh:mm:ss|Bh:mm|{1}{0}|{1}{0}|{1}{0}|{1}{0}|Bh:mm:ss|HH:mm:ss|Gy年|d日||y年M月d日",
    ),
    (
        "zh-Hant",
        "hebrew",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d",
    ),
    (
        "zh-Hant",
        "islamic",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "zh-Hant",
        "iso8601",
        "y MMMM d, EEEE|y MMMM d|y MMM d|y-MM-dd|HH:mm:ss zzzz|HH:mm:ss z|HH:mm:ss|HH:mm|{1} {0}|{1} {0}|{1} {0}|{1} {0}|h:mm:ss a||G y|d||y MMM d",
    ),
    (
        "zh-Hant",
        "japanese",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d||||||||||HH:mm:ss|Gy年",
    ),
    (
        "zh-Hant",
        "roc",
        "Gy年M月d日 EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|||||||||||Gy年",
    ),
    (
        "zh-Hant-HK",
        "buddhist",
        "||||ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}",
    ),
    (
        "zh-Hant-HK",
        "chinese",
        "U（r）年MMMdEEEE|U（r）年MMMd|U年MMMd|U/M/d|ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|||{1}, {0}|{1}, {0}|ah:mm:ss",
    ),
    (
        "zh-Hant-HK",
        "coptic",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}||||||Gy年",
    ),
    (
        "zh-Hant-HK",
        "dangi",
        "||||ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|||{1}, {0}|{1}, {0}|ah:mm:ss",
    ),
    (
        "zh-Hant-HK",
        "ethiopic",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}||||||Gy年",
    ),
    (
        "zh-Hant-HK",
        "generic",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}||||Bh:mm:ss||Gy年",
    ),
    (
        "zh-Hant-HK",
        "gregorian",
        "y年M月d日EEEE|||d/M/y|ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|||||ah:mm:ss",
    ),
    (
        "zh-Hant-HK",
        "hebrew",
        "||||ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}||||||Gy年",
    ),
    (
        "zh-Hant-HK",
        "indian",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}||||||Gy年",
    ),
    (
        "zh-Hant-HK",
        "islamic",
        "||||ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}",
    ),
    (
        "zh-Hant-HK",
        "japanese",
        "||||ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}||||ah:mm:ss",
    ),
    (
        "zh-Hant-HK",
        "persian",
        "Gy年M月d日EEEE|Gy年M月d日|Gy年M月d日|Gy/M/d|ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}||||||Gy年",
    ),
    (
        "zh-Hant-HK",
        "roc",
        "||||ah:mm:ss [zzzz]|ah:mm:ss [z]|ah:mm:ss|ah:mm|{1} {0}",
    ),
];
