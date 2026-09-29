//! The day periods of every carried language, generated.
//!
//! **Do not edit.** `scripts/day-periods-cldr.py` writes this file from
//! Unicode CLDR 48 (`release-48`, read 2026-09-29, `cldr48-day-periods`):
//! `supplemental/dayPeriods.xml`'s format rule set and each carried
//! locale's `common/main/<file>.xml` Gregorian format day periods, as the
//! script's documentation says.

use super::{FlexibleDayPeriod, Rules};

/// Each carried language's format rule set: (period, from, before), in
/// minutes after midnight, `from == before` for a fixed period's `at`.
pub(super) static RULES: &[(&str, Rules)] = &[
    (
        "am",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1440),
            (FlexibleDayPeriod::Night1, 0, 360),
        ],
    ),
    (
        "ar",
        &[
            (FlexibleDayPeriod::Morning1, 180, 360),
            (FlexibleDayPeriod::Morning2, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 780),
            (FlexibleDayPeriod::Afternoon2, 780, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1440),
            (FlexibleDayPeriod::Night1, 0, 60),
            (FlexibleDayPeriod::Night2, 60, 180),
        ],
    ),
    (
        "bn",
        &[
            (FlexibleDayPeriod::Morning1, 240, 360),
            (FlexibleDayPeriod::Morning2, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Afternoon2, 960, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1200),
            (FlexibleDayPeriod::Night1, 1200, 240),
        ],
    ),
    (
        "cs",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 240, 540),
            (FlexibleDayPeriod::Morning2, 540, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1320),
            (FlexibleDayPeriod::Night1, 1320, 240),
        ],
    ),
    (
        "de",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 300, 600),
            (FlexibleDayPeriod::Morning2, 600, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 780),
            (FlexibleDayPeriod::Afternoon2, 780, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1440),
            (FlexibleDayPeriod::Night1, 0, 300),
        ],
    ),
    (
        "en",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 0, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1260),
            (FlexibleDayPeriod::Night1, 1260, 1440),
        ],
    ),
    (
        "es",
        &[
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 0, 360),
            (FlexibleDayPeriod::Morning2, 360, 720),
            (FlexibleDayPeriod::Evening1, 720, 1200),
            (FlexibleDayPeriod::Night1, 1200, 1440),
        ],
    ),
    (
        "fa",
        &[
            (FlexibleDayPeriod::Morning1, 60, 240),
            (FlexibleDayPeriod::Morning2, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 780),
            (FlexibleDayPeriod::Afternoon2, 780, 1140),
            (FlexibleDayPeriod::Night1, 1140, 1440),
            (FlexibleDayPeriod::Night2, 0, 60),
        ],
    ),
    (
        "fil",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 0, 360),
            (FlexibleDayPeriod::Morning2, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Evening1, 960, 1080),
            (FlexibleDayPeriod::Night1, 1080, 1440),
        ],
    ),
    (
        "fr",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1440),
            (FlexibleDayPeriod::Night1, 0, 240),
        ],
    ),
    (
        "he",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Afternoon2, 960, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1320),
            (FlexibleDayPeriod::Night1, 1320, 180),
            (FlexibleDayPeriod::Night2, 180, 360),
        ],
    ),
    (
        "hi",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Evening1, 960, 1200),
            (FlexibleDayPeriod::Night1, 1200, 240),
        ],
    ),
    (
        "id",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 0, 600),
            (FlexibleDayPeriod::Afternoon1, 600, 900),
            (FlexibleDayPeriod::Evening1, 900, 1080),
            (FlexibleDayPeriod::Night1, 1080, 1440),
        ],
    ),
    (
        "it",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1440),
            (FlexibleDayPeriod::Night1, 0, 360),
        ],
    ),
    (
        "ja",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Evening1, 960, 1140),
            (FlexibleDayPeriod::Night1, 1140, 1380),
            (FlexibleDayPeriod::Night2, 1380, 240),
        ],
    ),
    (
        "ko",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 180, 360),
            (FlexibleDayPeriod::Morning2, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1260),
            (FlexibleDayPeriod::Night1, 1260, 180),
        ],
    ),
    (
        "ml",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 180, 360),
            (FlexibleDayPeriod::Morning2, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 840),
            (FlexibleDayPeriod::Afternoon2, 840, 900),
            (FlexibleDayPeriod::Evening1, 900, 1080),
            (FlexibleDayPeriod::Evening2, 1080, 1140),
            (FlexibleDayPeriod::Night1, 1140, 180),
        ],
    ),
    (
        "mn",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1260),
            (FlexibleDayPeriod::Night1, 1260, 360),
        ],
    ),
    (
        "mr",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 240, 360),
            (FlexibleDayPeriod::Morning2, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Evening1, 960, 1260),
            (FlexibleDayPeriod::Night1, 1260, 240),
        ],
    ),
    (
        "my",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 0, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Evening1, 960, 1140),
            (FlexibleDayPeriod::Night1, 1140, 1440),
        ],
    ),
    (
        "ne",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Afternoon2, 960, 1140),
            (FlexibleDayPeriod::Evening1, 1140, 1320),
            (FlexibleDayPeriod::Night1, 1320, 240),
        ],
    ),
    (
        "nl",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1440),
            (FlexibleDayPeriod::Night1, 0, 360),
        ],
    ),
    (
        "pa",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Evening1, 960, 1260),
            (FlexibleDayPeriod::Night1, 1260, 240),
        ],
    ),
    (
        "pl",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 360, 600),
            (FlexibleDayPeriod::Morning2, 600, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1260),
            (FlexibleDayPeriod::Night1, 1260, 360),
        ],
    ),
    (
        "pt",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1140),
            (FlexibleDayPeriod::Evening1, 1140, 1440),
            (FlexibleDayPeriod::Night1, 0, 360),
        ],
    ),
    (
        "ru",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1320),
            (FlexibleDayPeriod::Night1, 1320, 240),
        ],
    ),
    (
        "sw",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 240, 420),
            (FlexibleDayPeriod::Morning2, 420, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Evening1, 960, 1140),
            (FlexibleDayPeriod::Night1, 1140, 240),
        ],
    ),
    (
        "ta",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 180, 300),
            (FlexibleDayPeriod::Morning2, 300, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 840),
            (FlexibleDayPeriod::Afternoon2, 840, 960),
            (FlexibleDayPeriod::Evening1, 960, 1080),
            (FlexibleDayPeriod::Evening2, 1080, 1260),
            (FlexibleDayPeriod::Night1, 1260, 180),
        ],
    ),
    (
        "te",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1260),
            (FlexibleDayPeriod::Night1, 1260, 360),
        ],
    ),
    (
        "th",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 360, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 780),
            (FlexibleDayPeriod::Afternoon2, 780, 960),
            (FlexibleDayPeriod::Evening1, 960, 1080),
            (FlexibleDayPeriod::Evening2, 1080, 1260),
            (FlexibleDayPeriod::Night1, 1260, 360),
        ],
    ),
    (
        "tr",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 360, 660),
            (FlexibleDayPeriod::Morning2, 660, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Afternoon2, 1080, 1140),
            (FlexibleDayPeriod::Evening1, 1140, 1260),
            (FlexibleDayPeriod::Night1, 1260, 360),
        ],
    ),
    (
        "ur",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 960),
            (FlexibleDayPeriod::Afternoon2, 960, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1200),
            (FlexibleDayPeriod::Night1, 1200, 240),
        ],
    ),
    (
        "vi",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Noon, 720, 720),
            (FlexibleDayPeriod::Morning1, 240, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 1080),
            (FlexibleDayPeriod::Evening1, 1080, 1260),
            (FlexibleDayPeriod::Night1, 1260, 240),
        ],
    ),
    (
        "yue",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 300, 480),
            (FlexibleDayPeriod::Morning2, 480, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 780),
            (FlexibleDayPeriod::Afternoon2, 780, 1140),
            (FlexibleDayPeriod::Evening1, 1140, 1440),
            (FlexibleDayPeriod::Night1, 0, 300),
        ],
    ),
    (
        "zh",
        &[
            (FlexibleDayPeriod::Midnight, 0, 0),
            (FlexibleDayPeriod::Morning1, 300, 480),
            (FlexibleDayPeriod::Morning2, 480, 720),
            (FlexibleDayPeriod::Afternoon1, 720, 780),
            (FlexibleDayPeriod::Afternoon2, 780, 1140),
            (FlexibleDayPeriod::Evening1, 1140, 1440),
            (FlexibleDayPeriod::Night1, 0, 300),
        ],
    ),
];

/// Each carried locale's names: a line per period, in the order of
/// `FlexibleDayPeriod`, its abbreviated, wide and narrow names separated
/// by `|`, empty where the locale states none.
pub(super) static NAMES: &[(&str, &str)] = &[
    (
        "am",
        "እኩለ ሌሊት|እኩለ ሌሊት|እኩለ ሌሊት\nቀትር|ቀትር|ቀ\nጥዋት|ጥዋት|ጥዋት\n\nከሰዓት|ከሰዓት|ከሰዓት\n\nበምሽት|በምሽት|በምሽት\n\nበሌሊት|በሌሊት|በሌሊት\n\n",
    ),
    (
        "ar",
        "\n\nفجرًا|في الصباح|فجرًا\nص|صباحًا|صباحًا\nظهرًا|ظهرًا|ظهرًا\nبعد الظهر|بعد الظهر|بعد الظهر\nمساءً|مساءً|مساءً\n\nفي المساء|في المساء|منتصف الليل\nليلاً|ليلاً|ليلاً\n",
    ),
    (
        "bn",
        "\n\nভোর|ভোরবেলায়|ভোর\nসকাল|সকালবেলায়|সকাল\nদুপুর|দুপুরবেলায়|দুপুর\nবিকাল|বিকাল|বিকাল\nসন্ধ্যা|সন্ধ্যাবেলায়|সন্ধ্যা\n\nরাত্রি|রাত্রিবেলায়|রাত্রি\n\n",
    ),
    (
        "cs",
        "půln.|půlnoc|půl.\npol.|poledne|pol.\nr.|ráno|r.\ndop.|dopoledne|d.\nodp.|odpoledne|o.\n\nveč.|večer|v.\n\nv n.|v noci|n.\n\n",
    ),
    (
        "de",
        "Mitternacht|Mitternacht|Mitternacht\n\nmorgens|morgens|morgens\nvorm.|vormittags|vorm.\nmittags|mittags|mittags\nnachm.|nachmittags|nachm.\nabends|abends|abends\n\nnachts|nachts|nachts\n\n",
    ),
    (
        "en",
        "midnight|midnight|mi\nnoon|noon|n\nin the morning|in the morning|in the morning\n\nin the afternoon|in the afternoon|in the afternoon\n\nin the evening|in the evening|in the evening\n\nat night|at night|at night\n\n",
    ),
    (
        "es",
        "\ndel mediodía|del mediodía|del mediodía\nde la madrugada|de la madrugada|de la madrugada\nde la mañana|de la mañana|de la mañana\n\n\nde la tarde|de la tarde|de la tarde\n\nde la noche|de la noche|de la noche\n\n",
    ),
    (
        "fa",
        "\n\nبامداد|بامداد|بامداد\nصبح|صبح|صبح\nظهر|بعدازظهر|ظهر\nعصر|عصر|عصر\n\n\nشب|شب|شب\nنیمه‌شب|نیمه‌شب|نیمه‌شب\n",
    ),
    (
        "fil",
        "hatinggabi|hatinggabi|hatinggabi\ntanghaling-tapat|tanghaling-tapat|tanghaling-tapat\nng umaga|ng umaga|ng umaga\nmadaling-araw|madaling-araw|madaling-araw\nng hapon|ng hapon|ng hapon\n\nng gabi|ng gabi|ng gabi\n\nng gabi|ng gabi|ng gabi\n\n",
    ),
    (
        "fr",
        "minuit|minuit|minuit\nmidi|midi|midi\nmatin|du matin|mat.\n\naprès-midi|de l’après-midi|ap.m.\n\nsoir|du soir|soir\n\nmatin|du matin|matin\n\n",
    ),
    (
        "he",
        "חצות|חצות|חצות\n\nבוקר|בבוקר|בבוקר\n\nצהריים|בצהריים|בצהריים\nאחר הצהריים|אחר הצהריים|אחה״צ\nערב|בערב|בערב\n\nלילה|בלילה|בלילה\nלפנות בוקר|לפנות בוקר|לפנות בוקר\n",
    ),
    (
        "hi",
        "मध्यरात्रि|मध्यरात्रि|मध्यरात्रि\n\nसुबह|सुबह|सुबह\n\nदोपहर|दोपहर|दोपहर\n\nशाम|शाम|शाम\n\nरात|रात|रात\n\n",
    ),
    (
        "id",
        "tengah malam|tengah malam|tengah malam\ntengah hari|tengah hari|tengah hari\npagi|pagi|pagi\n\nsiang|siang|siang\n\nsore|sore|sore\n\nmalam|malam|malam\n\n",
    ),
    (
        "it",
        "mezzanotte|mezzanotte|mezzanotte\nmezzogiorno|mezzogiorno|mezzogiorno\ndi mattina|di mattina|di mattina\n\ndi pomeriggio|del pomeriggio|di pomeriggio\n\ndi sera|di sera|di sera\n\ndi notte|di notte|di notte\n\n",
    ),
    (
        "ja",
        "真夜中|真夜中|真夜中\n正午|正午|正午\n朝|朝|朝\n\n昼|昼|昼\n\n夕方|夕方|夕方\n\n夜|夜|夜\n夜中|夜中|夜中\n",
    ),
    (
        "ko",
        "자정|자정|자정\n정오|정오|정오\n아침|아침|아침\n오전|오전|오전\n오후|오후|오후\n\n저녁|저녁|저녁\n\n밤|밤|밤\n\n",
    ),
    (
        "ml",
        "അർദ്ധരാത്രി|അർദ്ധരാത്രി|അ\nഉച്ച|ഉച്ച|ഉച്ച\nപുലർച്ചെ|പുലർച്ചെ|പുലർച്ചെ\nരാവിലെ|രാവിലെ|രാവിലെ\nഉച്ചയ്ക്ക്|ഉച്ചയ്ക്ക്|ഉച്ചയ്ക്ക്\nഉച്ചതിരിഞ്ഞ്|ഉച്ചതിരിഞ്ഞ്|ഉച്ചതിരിഞ്ഞ്\nവൈകുന്നേരം|വൈകുന്നേരം|വൈകുന്നേരം\nസന്ധ്യ|സന്ധ്യ|സന്ധ്യ\nരാത്രി|രാത്രി|രാത്രി\n\n",
    ),
    (
        "mn",
        "шөнө дунд|шөнө дунд|шөнө дунд\nүд дунд|үд дунд|үд дунд\nөглөө|өглөө|өглөө\n\nөдөр|өдөр|өдөр\n\nорой|орой|орой\n\nшөнө|шөнө|шөнө\n\n",
    ),
    (
        "mr",
        "मध्यरात्र|मध्यरात्र|म.रा.\nमध्यान्ह|मध्यान्ह|दु\nपहाट|पहाट|प\nसकाळ|सकाळ|स\nदुपार|दुपार|दु\n\nसंध्याकाळ|संध्याकाळ|सं\n\nरात्र|रात्र|रा\n\n",
    ),
    (
        "my",
        "သန်းခေါင်ယံ|သန်းခေါင်ယံ|သန်းခေါင်ယံ\nမွန်းတည့်|မွန်းတည့်|မွန်းတည့်\nနံနက်|နံနက်|နံနက်\n\nနေ့လယ်|နေ့လယ်|နေ့လယ်\n\nညနေ|ညနေ|ညနေ\n\nည|ည|ည\n\n",
    ),
    (
        "ne",
        "मध्यरात|मध्यरात|मध्यरात\nमध्यान्ह|मध्यान्ह|मध्यान्ह\nबिहान|बिहान|बिहान\n\nअपरान्ह|अपरान्ह|अपरान्ह\nसाँझ|साँझ|साँझ\nबेलुकी|बेलुकी|बेलुकी\n\nरात|रात|रात\n\n",
    ),
    (
        "nl",
        "middernacht|middernacht|middernacht\n\n’s ochtends|’s ochtends|’s ochtends\n\n’s middags|’s middags|’s middags\n\n’s avonds|’s avonds|’s avonds\n\n’s nachts|’s nachts|’s nachts\n\n",
    ),
    (
        "pa-Guru",
        "ਅੱਧੀ ਰਾਤ|ਅੱਧੀ ਰਾਤ|ਅੱਧੀ ਰਾਤ\n\nਸਵੇਰੇ|ਸਵੇਰੇ|ਸਵੇਰੇ\n\nਦੁਪਹਿਰੇ|ਦੁਪਹਿਰੇ|ਦੁਪਹਿਰੇ\n\nਸ਼ਾਮੀਂ|ਸ਼ਾਮੀਂ|ਸ਼ਾਮੀਂ\n\nਰਾਤੀਂ|ਰਾਤੀਂ|ਰਾਤੀਂ\n\n",
    ),
    (
        "pl",
        "o północy|o północy|o półn.\nw południe|w południe|w poł.\nrano|rano|rano\nprzed południem|przed południem|przed poł.\npo południu|po południu|po poł.\n\nwieczorem|wieczorem|wiecz.\n\nw nocy|w nocy|w nocy\n\n",
    ),
    (
        "pt",
        "meia-noite|meia-noite|meia-noite\nmeio-dia|meio-dia|meio-dia\nda manhã|da manhã|da manhã\n\nda tarde|da tarde|da tarde\n\nda noite|da noite|da noite\n\nda madrugada|da madrugada|da madrugada\n\n",
    ),
    (
        "pt-PT",
        "\n\n||manhã\n\n||tarde\n\n||noite\n\n||madrugada\n\n",
    ),
    (
        "ru",
        "полн.|полночь|полн.\nполд.|полдень|полд.\nутра|утра|утра\n\nдня|дня|дня\n\nвечера|вечера|веч.\n\nночи|ночи|ночи\n\n",
    ),
    (
        "sw",
        "saa sita za usiku|saa sita za usiku|usiku\nadhuhuri|saa sita za mchana|mchana\nalfajiri|alfajiri|alfajiri\nasubuhi|asubuhi|asubuhi\nmchana|mchana|mchana\n\njioni|jioni|jioni\n\nusiku|usiku|usiku\n\n",
    ),
    (
        "ta",
        "நள்ளிரவு|நள்ளிரவு|நள்.\nநண்பகல்|நண்பகல்|நண்.\nஅதிகாலை|அதிகாலை|காலை\nகாலை|காலை|கா.\nமதியம்|மதியம்|மதி.\nபிற்பகல்|பிற்பகல்|பிற்.\nமாலை|மாலை|மா.\nஅந்தி மாலை|அந்தி மாலை|அந்தி மா.\nஇரவு|இரவு|இர.\n\n",
    ),
    (
        "te",
        "అర్ధరాత్రి|అర్ధరాత్రి|అర్ధరాత్రి\n\nఉదయం|ఉదయం|ఉదయం\n\nమధ్యాహ్నం|మధ్యాహ్నం|మధ్యాహ్నం\n\nసాయంత్రం|సాయంత్రం|సాయంత్రం\n\nరాత్రి|రాత్రి|రాత్రి\n\n",
    ),
    (
        "th",
        "เที่ยงคืน|เที่ยงคืน|เที่ยงคืน\nเที่ยง|เที่ยง|เที่ยง\nในตอนเช้า|ในตอนเช้า|เช้า\n\nในตอนบ่าย|ในตอนบ่าย|เที่ยง\nบ่าย|บ่าย|บ่าย\nในตอนเย็น|ในตอนเย็น|เย็น\nค่ำ|ค่ำ|ค่ำ\nกลางคืน|กลางคืน|กลางคืน\n\n",
    ),
    (
        "tr",
        "gece yarısı|gece yarısı|gece\nöğle|öğle|ö\nsabah|sabah|sabah\nöğleden önce|öğleden önce|öğleden önce\nöğleden sonra|öğleden sonra|öğleden sonra\nakşamüstü|akşamüstü|akşamüstü\nakşam|akşam|akşam\n\ngece|gece|gece\n\n",
    ),
    (
        "ur",
        "آدھی رات|آدھی رات|نصف شب\n\nصبح|صبح میں|صبح\n\nدوپہر|دوپہر میں|دوپہر\nسہ پہر|سہ پہر|سہ پہر\nشام|شام میں|شام\n\nرات|رات میں|رات\n\n",
    ),
    (
        "vi",
        "nửa đêm|nửa đêm|nửa đêm\nTR|trưa|tr\nsáng|sáng|sáng\n\nchiều|chiều|chiều\n\ntối|tối|tối\n\nđêm|đêm|đêm\n\n",
    ),
    (
        "yue-Hans",
        "午夜|午夜|午夜\n\n清晨|清晨|清晨\n朝早|朝早|朝早\n中午|中午|中午\n下昼|下昼|下昼\n夜晚|夜晚|夜晚\n\n凌晨|凌晨|凌晨\n\n",
    ),
    (
        "yue-Hant",
        "午夜|午夜|午夜\n\n清晨|清晨|清晨\n朝早|朝早|朝早\n中午|中午|中午\n下晝|下晝|下晝\n夜晚|夜晚|夜晚\n\n凌晨|凌晨|凌晨\n\n",
    ),
    (
        "zh-Hans",
        "午夜|午夜|午夜\n\n早上|清晨|早上\n上午|上午|上午\n中午|中午|中午\n下午|下午|下午\n晚上|晚上|晚上\n\n凌晨|凌晨|凌晨\n\n",
    ),
    (
        "zh-Hant",
        "午夜|午夜|午夜\n\n清晨|清晨|清晨\n上午|上午|上午\n中午|中午|中午\n下午|下午|下午\n晚上|晚上|晚上\n\n凌晨|凌晨|凌晨\n\n",
    ),
    ("zh-Hant-HK", "\n\n早上|早上|早上\n\n\n\n\n\n\n\n"),
];
