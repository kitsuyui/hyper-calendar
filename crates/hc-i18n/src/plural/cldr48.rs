//! The plural rules of every language CLDR 48 gives one, generated.
//!
//! **Do not edit.** `scripts/plurals-cldr.py` writes this file from Unicode
//! CLDR 48 (`release-48`, read 2026-10-04, `cldr48-supplemental`):
//! `supplemental/plurals.xml`, the cardinal rules of 226 locales in
//! 40 blocks, and `supplemental/ordinals.xml`, the
//! ordinal rules of 109 locales in 25 blocks,
//! each block's conditions parsed to the relations of UTS #35 Part 3 and
//! written as data for `super`'s one evaluator, as the script's
//! documentation says.

#[cfg(test)]
use super::Samples;
use super::{Operand, PluralCategory, Relation, Rule};

/// The cardinal rules of `bm bo dz hnj id ig ii in ja jbo jv jw kde kea km ko lkt lo ms my nqo osa root sah ses sg su th to tpi vi wo yo yue zh`.
static CARDINAL_0: &[Rule] = &[];

/// The cardinal rules of `am as bn doi fa gu hi kn kok kok_Latn pcm zu`.
static CARDINAL_1: &[Rule] = &[
    // one: i = 0 or n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(1, 1)],
            }],
        ],
    },
];

/// The cardinal rules of `ff hy kab`.
static CARDINAL_2: &[Rule] = &[
    // one: i = 0,1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::I,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0), (1, 1)],
        }]],
    },
];

/// The cardinal rules of `ast de en et fi fy gl ia ie io ji lij nl sc sv sw ur yi`.
static CARDINAL_3: &[Rule] = &[
    // one: i = 1 and v = 0
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
        ]],
    },
];

/// The cardinal rules of `si`.
static CARDINAL_4: &[Rule] = &[
    // one: n = 0,1 or i = 0 and f = 1
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0), (1, 1)],
            }],
            &[
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 0,
                    negated: false,
                    ranges: &[(1, 1)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `ak bho csw guw ln mg nso pa ti wa`.
static CARDINAL_5: &[Rule] = &[
    // one: n = 0..1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(0, 1)],
        }]],
    },
];

/// The cardinal rules of `tzm`.
static CARDINAL_6: &[Rule] = &[
    // one: n = 0..1 or n = 11..99
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(0, 1)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(11, 99)],
            }],
        ],
    },
];

/// The cardinal rules of `af an asa az bal bem bez bg brx ce cgg chr ckb dv ee el eo eu fo fur gsw ha haw hu jgo jmc ka kaj kcg kk kkj kl ks ksb ku ky lb lg mas mgo ml mn mr nah nb nd ne nn nnh no nr ny nyn om or os pap ps rm rof rwk saq sd sdh seh sn so sq ss ssy st syr ta te teo tig tk tn tr ts ug uz ve vo vun wae xh xog`.
static CARDINAL_7: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
];

/// The cardinal rules of `da`.
static CARDINAL_8: &[Rule] = &[
    // one: n = 1 or t != 0 and i = 0,1
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(1, 1)],
            }],
            &[
                Relation {
                    operand: Operand::T,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0), (1, 1)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `is`.
static CARDINAL_9: &[Rule] = &[
    // one: t = 0 and i % 10 = 1 and i % 100 != 11 or t % 10 = 1 and t % 100 != 11
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[
                Relation {
                    operand: Operand::T,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: true,
                    ranges: &[(11, 11)],
                },
            ],
            &[
                Relation {
                    operand: Operand::T,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::T,
                    modulus: 100,
                    negated: true,
                    ranges: &[(11, 11)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `mk`.
static CARDINAL_10: &[Rule] = &[
    // one: v = 0 and i % 10 = 1 and i % 100 != 11 or f % 10 = 1 and f % 100 != 11
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: true,
                    ranges: &[(11, 11)],
                },
            ],
            &[
                Relation {
                    operand: Operand::F,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 100,
                    negated: true,
                    ranges: &[(11, 11)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `ceb fil tl`.
static CARDINAL_11: &[Rule] = &[
    // one: v = 0 and i = 1,2,3 or v = 0 and i % 10 != 4,6,9 or v != 0 and f % 10 != 4,6,9
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: false,
                    ranges: &[(1, 1), (2, 2), (3, 3)],
                },
            ],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: true,
                    ranges: &[(4, 4), (6, 6), (9, 9)],
                },
            ],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 10,
                    negated: true,
                    ranges: &[(4, 4), (6, 6), (9, 9)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `lv prg`.
static CARDINAL_12: &[Rule] = &[
    // zero: n % 10 = 0 or n % 100 = 11..19 or v = 2 and f % 100 = 11..19
    Rule {
        category: PluralCategory::Zero,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(0, 0)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 100,
                negated: false,
                ranges: &[(11, 19)],
            }],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(2, 2)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 100,
                    negated: false,
                    ranges: &[(11, 19)],
                },
            ],
        ],
    },
    // one: n % 10 = 1 and n % 100 != 11 or v = 2 and f % 10 = 1 and f % 100 != 11 or v != 2 and f % 10 = 1
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[
                Relation {
                    operand: Operand::N,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::N,
                    modulus: 100,
                    negated: true,
                    ranges: &[(11, 11)],
                },
            ],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(2, 2)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 100,
                    negated: true,
                    ranges: &[(11, 11)],
                },
            ],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: true,
                    ranges: &[(2, 2)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `lag`.
static CARDINAL_13: &[Rule] = &[
    // zero: n = 0
    Rule {
        category: PluralCategory::Zero,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0)],
        }]],
    },
    // one: i = 0,1 and n != 0
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0), (1, 1)],
            },
            Relation {
                operand: Operand::N,
                modulus: 0,
                negated: true,
                ranges: &[(0, 0)],
            },
        ]],
    },
];

/// The cardinal rules of `blo cv ksh`.
static CARDINAL_14: &[Rule] = &[
    // zero: n = 0
    Rule {
        category: PluralCategory::Zero,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0)],
        }]],
    },
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
];

/// The cardinal rules of `he iw`.
static CARDINAL_15: &[Rule] = &[
    // one: i = 1 and v = 0 or i = 0 and v != 0
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
            ],
            &[
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
            ],
        ],
    },
    // two: i = 2 and v = 0
    Rule {
        category: PluralCategory::Two,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(2, 2)],
            },
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
        ]],
    },
];

/// The cardinal rules of `iu naq sat se sma smi smj smn sms`.
static CARDINAL_16: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2)],
        }]],
    },
];

/// The cardinal rules of `shi`.
static CARDINAL_17: &[Rule] = &[
    // one: i = 0 or n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(1, 1)],
            }],
        ],
    },
    // few: n = 2..10
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 10)],
        }]],
    },
];

/// The cardinal rules of `mo ro`.
static CARDINAL_18: &[Rule] = &[
    // one: i = 1 and v = 0
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
        ]],
    },
    // few: v != 0 or n = 0 or n != 1 and n % 100 = 1..19
    Rule {
        category: PluralCategory::Few,
        condition: &[
            &[Relation {
                operand: Operand::V,
                modulus: 0,
                negated: true,
                ranges: &[(0, 0)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            }],
            &[
                Relation {
                    operand: Operand::N,
                    modulus: 0,
                    negated: true,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::N,
                    modulus: 100,
                    negated: false,
                    ranges: &[(1, 19)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `bs hr sh sr`.
static CARDINAL_19: &[Rule] = &[
    // one: v = 0 and i % 10 = 1 and i % 100 != 11 or f % 10 = 1 and f % 100 != 11
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: true,
                    ranges: &[(11, 11)],
                },
            ],
            &[
                Relation {
                    operand: Operand::F,
                    modulus: 10,
                    negated: false,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 100,
                    negated: true,
                    ranges: &[(11, 11)],
                },
            ],
        ],
    },
    // few: v = 0 and i % 10 = 2..4 and i % 100 != 12..14 or f % 10 = 2..4 and f % 100 != 12..14
    Rule {
        category: PluralCategory::Few,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: false,
                    ranges: &[(2, 4)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: true,
                    ranges: &[(12, 14)],
                },
            ],
            &[
                Relation {
                    operand: Operand::F,
                    modulus: 10,
                    negated: false,
                    ranges: &[(2, 4)],
                },
                Relation {
                    operand: Operand::F,
                    modulus: 100,
                    negated: true,
                    ranges: &[(12, 14)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `fr`.
static CARDINAL_20: &[Rule] = &[
    // one: i = 0,1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::I,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0), (1, 1)],
        }]],
    },
    // many: e = 0 and i != 0 and i % 1000000 = 0 and v = 0 or e != 0..5
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[
                Relation {
                    operand: Operand::C,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 1000000,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
            ],
            &[Relation {
                operand: Operand::C,
                modulus: 0,
                negated: true,
                ranges: &[(0, 5)],
            }],
        ],
    },
];

/// The cardinal rules of `pt`.
static CARDINAL_21: &[Rule] = &[
    // one: i = 0..1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::I,
            modulus: 0,
            negated: false,
            ranges: &[(0, 1)],
        }]],
    },
    // many: e = 0 and i != 0 and i % 1000000 = 0 and v = 0 or e != 0..5
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[
                Relation {
                    operand: Operand::C,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 1000000,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
            ],
            &[Relation {
                operand: Operand::C,
                modulus: 0,
                negated: true,
                ranges: &[(0, 5)],
            }],
        ],
    },
];

/// The cardinal rules of `ca it lld pt_PT scn vec`.
static CARDINAL_22: &[Rule] = &[
    // one: i = 1 and v = 0
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
        ]],
    },
    // many: e = 0 and i != 0 and i % 1000000 = 0 and v = 0 or e != 0..5
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[
                Relation {
                    operand: Operand::C,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 1000000,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
            ],
            &[Relation {
                operand: Operand::C,
                modulus: 0,
                negated: true,
                ranges: &[(0, 5)],
            }],
        ],
    },
];

/// The cardinal rules of `es`.
static CARDINAL_23: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // many: e = 0 and i != 0 and i % 1000000 = 0 and v = 0 or e != 0..5
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[
                Relation {
                    operand: Operand::C,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 1000000,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
            ],
            &[Relation {
                operand: Operand::C,
                modulus: 0,
                negated: true,
                ranges: &[(0, 5)],
            }],
        ],
    },
];

/// The cardinal rules of `gd`.
static CARDINAL_24: &[Rule] = &[
    // one: n = 1,11
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1), (11, 11)],
        }]],
    },
    // two: n = 2,12
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2), (12, 12)],
        }]],
    },
    // few: n = 3..10,13..19
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(3, 10), (13, 19)],
        }]],
    },
];

/// The cardinal rules of `sl`.
static CARDINAL_25: &[Rule] = &[
    // one: v = 0 and i % 100 = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: false,
                ranges: &[(1, 1)],
            },
        ]],
    },
    // two: v = 0 and i % 100 = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: false,
                ranges: &[(2, 2)],
            },
        ]],
    },
    // few: v = 0 and i % 100 = 3..4 or v != 0
    Rule {
        category: PluralCategory::Few,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: false,
                    ranges: &[(3, 4)],
                },
            ],
            &[Relation {
                operand: Operand::V,
                modulus: 0,
                negated: true,
                ranges: &[(0, 0)],
            }],
        ],
    },
];

/// The cardinal rules of `dsb hsb`.
static CARDINAL_26: &[Rule] = &[
    // one: v = 0 and i % 100 = 1 or f % 100 = 1
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: false,
                    ranges: &[(1, 1)],
                },
            ],
            &[Relation {
                operand: Operand::F,
                modulus: 100,
                negated: false,
                ranges: &[(1, 1)],
            }],
        ],
    },
    // two: v = 0 and i % 100 = 2 or f % 100 = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: false,
                    ranges: &[(2, 2)],
                },
            ],
            &[Relation {
                operand: Operand::F,
                modulus: 100,
                negated: false,
                ranges: &[(2, 2)],
            }],
        ],
    },
    // few: v = 0 and i % 100 = 3..4 or f % 100 = 3..4
    Rule {
        category: PluralCategory::Few,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: false,
                    ranges: &[(3, 4)],
                },
            ],
            &[Relation {
                operand: Operand::F,
                modulus: 100,
                negated: false,
                ranges: &[(3, 4)],
            }],
        ],
    },
];

/// The cardinal rules of `cs sk`.
static CARDINAL_27: &[Rule] = &[
    // one: i = 1 and v = 0
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
        ]],
    },
    // few: i = 2..4 and v = 0
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(2, 4)],
            },
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
        ]],
    },
    // many: v != 0
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::V,
            modulus: 0,
            negated: true,
            ranges: &[(0, 0)],
        }]],
    },
];

/// The cardinal rules of `pl`.
static CARDINAL_28: &[Rule] = &[
    // one: i = 1 and v = 0
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
        ]],
    },
    // few: v = 0 and i % 10 = 2..4 and i % 100 != 12..14
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(2, 4)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: true,
                ranges: &[(12, 14)],
            },
        ]],
    },
    // many: v = 0 and i != 1 and i % 10 = 0..1 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 12..14
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 0,
                    negated: true,
                    ranges: &[(1, 1)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: false,
                    ranges: &[(0, 1)],
                },
            ],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: false,
                    ranges: &[(5, 9)],
                },
            ],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: false,
                    ranges: &[(12, 14)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `be`.
static CARDINAL_29: &[Rule] = &[
    // one: n % 10 = 1 and n % 100 != 11
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(11, 11)],
            },
        ]],
    },
    // few: n % 10 = 2..4 and n % 100 != 12..14
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(2, 4)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(12, 14)],
            },
        ]],
    },
    // many: n % 10 = 0 or n % 10 = 5..9 or n % 100 = 11..14
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(0, 0)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(5, 9)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 100,
                negated: false,
                ranges: &[(11, 14)],
            }],
        ],
    },
];

/// The cardinal rules of `lt`.
static CARDINAL_30: &[Rule] = &[
    // one: n % 10 = 1 and n % 100 != 11..19
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(11, 19)],
            },
        ]],
    },
    // few: n % 10 = 2..9 and n % 100 != 11..19
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(2, 9)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(11, 19)],
            },
        ]],
    },
    // many: f != 0
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::F,
            modulus: 0,
            negated: true,
            ranges: &[(0, 0)],
        }]],
    },
];

/// The cardinal rules of `ru uk`.
static CARDINAL_31: &[Rule] = &[
    // one: v = 0 and i % 10 = 1 and i % 100 != 11
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: true,
                ranges: &[(11, 11)],
            },
        ]],
    },
    // few: v = 0 and i % 10 = 2..4 and i % 100 != 12..14
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(2, 4)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: true,
                ranges: &[(12, 14)],
            },
        ]],
    },
    // many: v = 0 and i % 10 = 0 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 11..14
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: false,
                    ranges: &[(0, 0)],
                },
            ],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 10,
                    negated: false,
                    ranges: &[(5, 9)],
                },
            ],
            &[
                Relation {
                    operand: Operand::V,
                    modulus: 0,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::I,
                    modulus: 100,
                    negated: false,
                    ranges: &[(11, 14)],
                },
            ],
        ],
    },
];

/// The cardinal rules of `sgs`.
static CARDINAL_32: &[Rule] = &[
    // one: n % 10 = 1 and n % 100 != 11
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(11, 11)],
            },
        ]],
    },
    // two: n = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2)],
        }]],
    },
    // few: n != 2 and n % 10 = 2..9 and n % 100 != 11..19
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 0,
                negated: true,
                ranges: &[(2, 2)],
            },
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(2, 9)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(11, 19)],
            },
        ]],
    },
    // many: f != 0
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::F,
            modulus: 0,
            negated: true,
            ranges: &[(0, 0)],
        }]],
    },
];

/// The cardinal rules of `br`.
static CARDINAL_33: &[Rule] = &[
    // one: n % 10 = 1 and n % 100 != 11,71,91
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(11, 11), (71, 71), (91, 91)],
            },
        ]],
    },
    // two: n % 10 = 2 and n % 100 != 12,72,92
    Rule {
        category: PluralCategory::Two,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(2, 2)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(12, 12), (72, 72), (92, 92)],
            },
        ]],
    },
    // few: n % 10 = 3..4,9 and n % 100 != 10..19,70..79,90..99
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(3, 4), (9, 9)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(10, 19), (70, 79), (90, 99)],
            },
        ]],
    },
    // many: n != 0 and n % 1000000 = 0
    Rule {
        category: PluralCategory::Many,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 0,
                negated: true,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::N,
                modulus: 1000000,
                negated: false,
                ranges: &[(0, 0)],
            },
        ]],
    },
];

/// The cardinal rules of `mt`.
static CARDINAL_34: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2)],
        }]],
    },
    // few: n = 0 or n % 100 = 3..10
    Rule {
        category: PluralCategory::Few,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 100,
                negated: false,
                ranges: &[(3, 10)],
            }],
        ],
    },
    // many: n % 100 = 11..19
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 100,
            negated: false,
            ranges: &[(11, 19)],
        }]],
    },
];

/// The cardinal rules of `ga`.
static CARDINAL_35: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2)],
        }]],
    },
    // few: n = 3..6
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(3, 6)],
        }]],
    },
    // many: n = 7..10
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(7, 10)],
        }]],
    },
];

/// The cardinal rules of `gv`.
static CARDINAL_36: &[Rule] = &[
    // one: v = 0 and i % 10 = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1)],
            },
        ]],
    },
    // two: v = 0 and i % 10 = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(2, 2)],
            },
        ]],
    },
    // few: v = 0 and i % 100 = 0,20,40,60,80
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::V,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: false,
                ranges: &[(0, 0), (20, 20), (40, 40), (60, 60), (80, 80)],
            },
        ]],
    },
    // many: v != 0
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::V,
            modulus: 0,
            negated: true,
            ranges: &[(0, 0)],
        }]],
    },
];

/// The cardinal rules of `kw`.
static CARDINAL_37: &[Rule] = &[
    // zero: n = 0
    Rule {
        category: PluralCategory::Zero,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0)],
        }]],
    },
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n % 100 = 2,22,42,62,82 or n % 1000 = 0 and n % 100000 = 1000..20000,40000,60000,80000 or n != 0 and n % 1000000 = 100000
    Rule {
        category: PluralCategory::Two,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 100,
                negated: false,
                ranges: &[(2, 2), (22, 22), (42, 42), (62, 62), (82, 82)],
            }],
            &[
                Relation {
                    operand: Operand::N,
                    modulus: 1000,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::N,
                    modulus: 100000,
                    negated: false,
                    ranges: &[
                        (1000, 20000),
                        (40000, 40000),
                        (60000, 60000),
                        (80000, 80000),
                    ],
                },
            ],
            &[
                Relation {
                    operand: Operand::N,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::N,
                    modulus: 1000000,
                    negated: false,
                    ranges: &[(100000, 100000)],
                },
            ],
        ],
    },
    // few: n % 100 = 3,23,43,63,83
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 100,
            negated: false,
            ranges: &[(3, 3), (23, 23), (43, 43), (63, 63), (83, 83)],
        }]],
    },
    // many: n != 1 and n % 100 = 1,21,41,61,81
    Rule {
        category: PluralCategory::Many,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 0,
                negated: true,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: false,
                ranges: &[(1, 1), (21, 21), (41, 41), (61, 61), (81, 81)],
            },
        ]],
    },
];

/// The cardinal rules of `ar ars`.
static CARDINAL_38: &[Rule] = &[
    // zero: n = 0
    Rule {
        category: PluralCategory::Zero,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0)],
        }]],
    },
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2)],
        }]],
    },
    // few: n % 100 = 3..10
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 100,
            negated: false,
            ranges: &[(3, 10)],
        }]],
    },
    // many: n % 100 = 11..99
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 100,
            negated: false,
            ranges: &[(11, 99)],
        }]],
    },
];

/// The cardinal rules of `cy`.
static CARDINAL_39: &[Rule] = &[
    // zero: n = 0
    Rule {
        category: PluralCategory::Zero,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0)],
        }]],
    },
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2)],
        }]],
    },
    // few: n = 3
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(3, 3)],
        }]],
    },
    // many: n = 6
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(6, 6)],
        }]],
    },
];

/// Every locale of `plurals.xml` but `root`, sorted, with its cardinal rules.
pub(super) static CARDINAL: &[(&str, &[Rule])] = &[
    ("af", CARDINAL_7),
    ("ak", CARDINAL_5),
    ("am", CARDINAL_1),
    ("an", CARDINAL_7),
    ("ar", CARDINAL_38),
    ("ars", CARDINAL_38),
    ("as", CARDINAL_1),
    ("asa", CARDINAL_7),
    ("ast", CARDINAL_3),
    ("az", CARDINAL_7),
    ("bal", CARDINAL_7),
    ("be", CARDINAL_29),
    ("bem", CARDINAL_7),
    ("bez", CARDINAL_7),
    ("bg", CARDINAL_7),
    ("bho", CARDINAL_5),
    ("blo", CARDINAL_14),
    ("bm", CARDINAL_0),
    ("bn", CARDINAL_1),
    ("bo", CARDINAL_0),
    ("br", CARDINAL_33),
    ("brx", CARDINAL_7),
    ("bs", CARDINAL_19),
    ("ca", CARDINAL_22),
    ("ce", CARDINAL_7),
    ("ceb", CARDINAL_11),
    ("cgg", CARDINAL_7),
    ("chr", CARDINAL_7),
    ("ckb", CARDINAL_7),
    ("cs", CARDINAL_27),
    ("csw", CARDINAL_5),
    ("cv", CARDINAL_14),
    ("cy", CARDINAL_39),
    ("da", CARDINAL_8),
    ("de", CARDINAL_3),
    ("doi", CARDINAL_1),
    ("dsb", CARDINAL_26),
    ("dv", CARDINAL_7),
    ("dz", CARDINAL_0),
    ("ee", CARDINAL_7),
    ("el", CARDINAL_7),
    ("en", CARDINAL_3),
    ("eo", CARDINAL_7),
    ("es", CARDINAL_23),
    ("et", CARDINAL_3),
    ("eu", CARDINAL_7),
    ("fa", CARDINAL_1),
    ("ff", CARDINAL_2),
    ("fi", CARDINAL_3),
    ("fil", CARDINAL_11),
    ("fo", CARDINAL_7),
    ("fr", CARDINAL_20),
    ("fur", CARDINAL_7),
    ("fy", CARDINAL_3),
    ("ga", CARDINAL_35),
    ("gd", CARDINAL_24),
    ("gl", CARDINAL_3),
    ("gsw", CARDINAL_7),
    ("gu", CARDINAL_1),
    ("guw", CARDINAL_5),
    ("gv", CARDINAL_36),
    ("ha", CARDINAL_7),
    ("haw", CARDINAL_7),
    ("he", CARDINAL_15),
    ("hi", CARDINAL_1),
    ("hnj", CARDINAL_0),
    ("hr", CARDINAL_19),
    ("hsb", CARDINAL_26),
    ("hu", CARDINAL_7),
    ("hy", CARDINAL_2),
    ("ia", CARDINAL_3),
    ("id", CARDINAL_0),
    ("ie", CARDINAL_3),
    ("ig", CARDINAL_0),
    ("ii", CARDINAL_0),
    ("in", CARDINAL_0),
    ("io", CARDINAL_3),
    ("is", CARDINAL_9),
    ("it", CARDINAL_22),
    ("iu", CARDINAL_16),
    ("iw", CARDINAL_15),
    ("ja", CARDINAL_0),
    ("jbo", CARDINAL_0),
    ("jgo", CARDINAL_7),
    ("ji", CARDINAL_3),
    ("jmc", CARDINAL_7),
    ("jv", CARDINAL_0),
    ("jw", CARDINAL_0),
    ("ka", CARDINAL_7),
    ("kab", CARDINAL_2),
    ("kaj", CARDINAL_7),
    ("kcg", CARDINAL_7),
    ("kde", CARDINAL_0),
    ("kea", CARDINAL_0),
    ("kk", CARDINAL_7),
    ("kkj", CARDINAL_7),
    ("kl", CARDINAL_7),
    ("km", CARDINAL_0),
    ("kn", CARDINAL_1),
    ("ko", CARDINAL_0),
    ("kok", CARDINAL_1),
    ("kok-Latn", CARDINAL_1),
    ("ks", CARDINAL_7),
    ("ksb", CARDINAL_7),
    ("ksh", CARDINAL_14),
    ("ku", CARDINAL_7),
    ("kw", CARDINAL_37),
    ("ky", CARDINAL_7),
    ("lag", CARDINAL_13),
    ("lb", CARDINAL_7),
    ("lg", CARDINAL_7),
    ("lij", CARDINAL_3),
    ("lkt", CARDINAL_0),
    ("lld", CARDINAL_22),
    ("ln", CARDINAL_5),
    ("lo", CARDINAL_0),
    ("lt", CARDINAL_30),
    ("lv", CARDINAL_12),
    ("mas", CARDINAL_7),
    ("mg", CARDINAL_5),
    ("mgo", CARDINAL_7),
    ("mk", CARDINAL_10),
    ("ml", CARDINAL_7),
    ("mn", CARDINAL_7),
    ("mo", CARDINAL_18),
    ("mr", CARDINAL_7),
    ("ms", CARDINAL_0),
    ("mt", CARDINAL_34),
    ("my", CARDINAL_0),
    ("nah", CARDINAL_7),
    ("naq", CARDINAL_16),
    ("nb", CARDINAL_7),
    ("nd", CARDINAL_7),
    ("ne", CARDINAL_7),
    ("nl", CARDINAL_3),
    ("nn", CARDINAL_7),
    ("nnh", CARDINAL_7),
    ("no", CARDINAL_7),
    ("nqo", CARDINAL_0),
    ("nr", CARDINAL_7),
    ("nso", CARDINAL_5),
    ("ny", CARDINAL_7),
    ("nyn", CARDINAL_7),
    ("om", CARDINAL_7),
    ("or", CARDINAL_7),
    ("os", CARDINAL_7),
    ("osa", CARDINAL_0),
    ("pa", CARDINAL_5),
    ("pap", CARDINAL_7),
    ("pcm", CARDINAL_1),
    ("pl", CARDINAL_28),
    ("prg", CARDINAL_12),
    ("ps", CARDINAL_7),
    ("pt", CARDINAL_21),
    ("pt-PT", CARDINAL_22),
    ("rm", CARDINAL_7),
    ("ro", CARDINAL_18),
    ("rof", CARDINAL_7),
    ("ru", CARDINAL_31),
    ("rwk", CARDINAL_7),
    ("sah", CARDINAL_0),
    ("saq", CARDINAL_7),
    ("sat", CARDINAL_16),
    ("sc", CARDINAL_3),
    ("scn", CARDINAL_22),
    ("sd", CARDINAL_7),
    ("sdh", CARDINAL_7),
    ("se", CARDINAL_16),
    ("seh", CARDINAL_7),
    ("ses", CARDINAL_0),
    ("sg", CARDINAL_0),
    ("sgs", CARDINAL_32),
    ("sh", CARDINAL_19),
    ("shi", CARDINAL_17),
    ("si", CARDINAL_4),
    ("sk", CARDINAL_27),
    ("sl", CARDINAL_25),
    ("sma", CARDINAL_16),
    ("smi", CARDINAL_16),
    ("smj", CARDINAL_16),
    ("smn", CARDINAL_16),
    ("sms", CARDINAL_16),
    ("sn", CARDINAL_7),
    ("so", CARDINAL_7),
    ("sq", CARDINAL_7),
    ("sr", CARDINAL_19),
    ("ss", CARDINAL_7),
    ("ssy", CARDINAL_7),
    ("st", CARDINAL_7),
    ("su", CARDINAL_0),
    ("sv", CARDINAL_3),
    ("sw", CARDINAL_3),
    ("syr", CARDINAL_7),
    ("ta", CARDINAL_7),
    ("te", CARDINAL_7),
    ("teo", CARDINAL_7),
    ("th", CARDINAL_0),
    ("ti", CARDINAL_5),
    ("tig", CARDINAL_7),
    ("tk", CARDINAL_7),
    ("tl", CARDINAL_11),
    ("tn", CARDINAL_7),
    ("to", CARDINAL_0),
    ("tpi", CARDINAL_0),
    ("tr", CARDINAL_7),
    ("ts", CARDINAL_7),
    ("tzm", CARDINAL_6),
    ("ug", CARDINAL_7),
    ("uk", CARDINAL_31),
    ("ur", CARDINAL_3),
    ("uz", CARDINAL_7),
    ("ve", CARDINAL_7),
    ("vec", CARDINAL_22),
    ("vi", CARDINAL_0),
    ("vo", CARDINAL_7),
    ("vun", CARDINAL_7),
    ("wa", CARDINAL_5),
    ("wae", CARDINAL_7),
    ("wo", CARDINAL_0),
    ("xh", CARDINAL_7),
    ("xog", CARDINAL_7),
    ("yi", CARDINAL_3),
    ("yo", CARDINAL_0),
    ("yue", CARDINAL_0),
    ("zh", CARDINAL_0),
    ("zu", CARDINAL_1),
];

/// The samples of each cardinal block, under the first locale of the block:
/// (category, `@integer` samples, `@decimal` samples) as the file writes them.
#[cfg(test)]
pub(super) static CARDINAL_SAMPLES: &[(&str, Samples)] = &[
    (
        "bm",
        &[(
            PluralCategory::Other,
            "0~15, 100, 1000, 10000, 100000, 1000000, …",
            "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
        )],
    ),
    (
        "am",
        &[
            (PluralCategory::One, "0, 1", "0.0~1.0, 0.00~0.04"),
            (
                PluralCategory::Other,
                "2~17, 100, 1000, 10000, 100000, 1000000, …",
                "1.1~2.6, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "ff",
        &[
            (PluralCategory::One, "0, 1", "0.0~1.5"),
            (
                PluralCategory::Other,
                "2~17, 100, 1000, 10000, 100000, 1000000, …",
                "2.0~3.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "ast",
        &[
            (PluralCategory::One, "1", ""),
            (
                PluralCategory::Other,
                "0, 2~16, 100, 1000, 10000, 100000, 1000000, …",
                "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "si",
        &[
            (
                PluralCategory::One,
                "0, 1",
                "0.0, 0.1, 1.0, 0.00, 0.01, 1.00, 0.000, 0.001, 1.000, 0.0000, 0.0001, 1.0000",
            ),
            (
                PluralCategory::Other,
                "2~17, 100, 1000, 10000, 100000, 1000000, …",
                "0.2~0.9, 1.1~1.8, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "ak",
        &[
            (
                PluralCategory::One,
                "0, 1",
                "0.0, 1.0, 0.00, 1.00, 0.000, 1.000, 0.0000, 1.0000",
            ),
            (
                PluralCategory::Other,
                "2~17, 100, 1000, 10000, 100000, 1000000, …",
                "0.1~0.9, 1.1~1.7, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "tzm",
        &[
            (
                PluralCategory::One,
                "0, 1, 11~24",
                "0.0, 1.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0",
            ),
            (
                PluralCategory::Other,
                "2~10, 100~106, 1000, 10000, 100000, 1000000, …",
                "0.1~0.9, 1.1~1.7, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "af",
        &[
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (
                PluralCategory::Other,
                "0, 2~16, 100, 1000, 10000, 100000, 1000000, …",
                "0.0~0.9, 1.1~1.6, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "da",
        &[
            (PluralCategory::One, "1", "0.1~1.6"),
            (
                PluralCategory::Other,
                "0, 2~16, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 2.0~3.4, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "is",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "0.1, 1.0, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 10.1, 100.1, 1000.1, …",
            ),
            (
                PluralCategory::Other,
                "0, 2~16, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 0.2~0.9, 1.2~1.8, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "mk",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "0.1, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 10.1, 100.1, 1000.1, …",
            ),
            (
                PluralCategory::Other,
                "0, 2~16, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 0.2~1.0, 1.2~1.7, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "ceb",
        &[
            (
                PluralCategory::One,
                "0~3, 5, 7, 8, 10~13, 15, 17, 18, 20, 21, 100, 1000, 10000, 100000, 1000000, …",
                "0.0~0.3, 0.5, 0.7, 0.8, 1.0~1.3, 1.5, 1.7, 1.8, 2.0, 2.1, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
            (
                PluralCategory::Other,
                "4, 6, 9, 14, 16, 19, 24, 26, 104, 1004, …",
                "0.4, 0.6, 0.9, 1.4, 1.6, 1.9, 2.4, 2.6, 10.4, 100.4, 1000.4, …",
            ),
        ],
    ),
    (
        "lv",
        &[
            (
                PluralCategory::Zero,
                "0, 10~20, 30, 40, 50, 60, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "0.1, 1.0, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 10.1, 100.1, 1000.1, …",
            ),
            (
                PluralCategory::Other,
                "2~9, 22~29, 102, 1002, …",
                "0.2~0.9, 1.2~1.9, 10.2, 100.2, 1000.2, …",
            ),
        ],
    ),
    (
        "lag",
        &[
            (PluralCategory::Zero, "0", "0.0, 0.00, 0.000, 0.0000"),
            (PluralCategory::One, "1", "0.1~1.6"),
            (
                PluralCategory::Other,
                "2~17, 100, 1000, 10000, 100000, 1000000, …",
                "2.0~3.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "blo",
        &[
            (PluralCategory::Zero, "0", "0.0, 0.00, 0.000, 0.0000"),
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (
                PluralCategory::Other,
                "2~17, 100, 1000, 10000, 100000, 1000000, …",
                "0.1~0.9, 1.1~1.7, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "he",
        &[
            (PluralCategory::One, "1", "0.0~0.9, 0.00~0.05"),
            (PluralCategory::Two, "2", ""),
            (
                PluralCategory::Other,
                "0, 3~17, 100, 1000, 10000, 100000, 1000000, …",
                "1.0~2.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "iu",
        &[
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (PluralCategory::Two, "2", "2.0, 2.00, 2.000, 2.0000"),
            (
                PluralCategory::Other,
                "0, 3~17, 100, 1000, 10000, 100000, 1000000, …",
                "0.0~0.9, 1.1~1.6, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "shi",
        &[
            (PluralCategory::One, "0, 1", "0.0~1.0, 0.00~0.04"),
            (
                PluralCategory::Few,
                "2~10",
                "2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 2.00, 3.00, 4.00, 5.00, 6.00, 7.00, 8.00",
            ),
            (
                PluralCategory::Other,
                "11~26, 100, 1000, 10000, 100000, 1000000, …",
                "1.1~1.9, 2.1~2.7, 10.1, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "mo",
        &[
            (PluralCategory::One, "1", ""),
            (
                PluralCategory::Few,
                "0, 2~16, 101, 1001, …",
                "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
            (
                PluralCategory::Other,
                "20~35, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "bs",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "0.1, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 10.1, 100.1, 1000.1, …",
            ),
            (
                PluralCategory::Few,
                "2~4, 22~24, 32~34, 42~44, 52~54, 62, 102, 1002, …",
                "0.2~0.4, 1.2~1.4, 2.2~2.4, 3.2~3.4, 4.2~4.4, 5.2, 10.2, 100.2, 1000.2, …",
            ),
            (
                PluralCategory::Other,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 0.5~1.0, 1.5~2.0, 2.5~2.7, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "fr",
        &[
            (PluralCategory::One, "0, 1", "0.0~1.5"),
            (
                PluralCategory::Many,
                "1000000, 1c6, 2c6, 3c6, 4c6, 5c6, 6c6, …",
                "1.0000001c6, 1.1c6, 2.0000001c6, 2.1c6, 3.0000001c6, 3.1c6, …",
            ),
            (
                PluralCategory::Other,
                "2~17, 100, 1000, 10000, 100000, 1c3, 2c3, 3c3, 4c3, 5c3, 6c3, …",
                "2.0~3.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, 1.0001c3, 1.1c3, 2.0001c3, 2.1c3, 3.0001c3, 3.1c3, …",
            ),
        ],
    ),
    (
        "pt",
        &[
            (PluralCategory::One, "0, 1", "0.0~1.5"),
            (
                PluralCategory::Many,
                "1000000, 1c6, 2c6, 3c6, 4c6, 5c6, 6c6, …",
                "1.0000001c6, 1.1c6, 2.0000001c6, 2.1c6, 3.0000001c6, 3.1c6, …",
            ),
            (
                PluralCategory::Other,
                "2~17, 100, 1000, 10000, 100000, 1c3, 2c3, 3c3, 4c3, 5c3, 6c3, …",
                "2.0~3.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, 1.0001c3, 1.1c3, 2.0001c3, 2.1c3, 3.0001c3, 3.1c3, …",
            ),
        ],
    ),
    (
        "ca",
        &[
            (PluralCategory::One, "1", ""),
            (
                PluralCategory::Many,
                "1000000, 1c6, 2c6, 3c6, 4c6, 5c6, 6c6, …",
                "1.0000001c6, 1.1c6, 2.0000001c6, 2.1c6, 3.0000001c6, 3.1c6, …",
            ),
            (
                PluralCategory::Other,
                "0, 2~16, 100, 1000, 10000, 100000, 1c3, 2c3, 3c3, 4c3, 5c3, 6c3, …",
                "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, 1.0001c3, 1.1c3, 2.0001c3, 2.1c3, 3.0001c3, 3.1c3, …",
            ),
        ],
    ),
    (
        "es",
        &[
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (
                PluralCategory::Many,
                "1000000, 1c6, 2c6, 3c6, 4c6, 5c6, 6c6, …",
                "1.0000001c6, 1.1c6, 2.0000001c6, 2.1c6, 3.0000001c6, 3.1c6, …",
            ),
            (
                PluralCategory::Other,
                "0, 2~16, 100, 1000, 10000, 100000, 1c3, 2c3, 3c3, 4c3, 5c3, 6c3, …",
                "0.0~0.9, 1.1~1.6, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, 1.0001c3, 1.1c3, 2.0001c3, 2.1c3, 3.0001c3, 3.1c3, …",
            ),
        ],
    ),
    (
        "gd",
        &[
            (
                PluralCategory::One,
                "1, 11",
                "1.0, 11.0, 1.00, 11.00, 1.000, 11.000, 1.0000",
            ),
            (
                PluralCategory::Two,
                "2, 12",
                "2.0, 12.0, 2.00, 12.00, 2.000, 12.000, 2.0000",
            ),
            (
                PluralCategory::Few,
                "3~10, 13~19",
                "3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 3.00",
            ),
            (
                PluralCategory::Other,
                "0, 20~34, 100, 1000, 10000, 100000, 1000000, …",
                "0.0~0.9, 1.1~1.6, 10.1, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "sl",
        &[
            (
                PluralCategory::One,
                "1, 101, 201, 301, 401, 501, 601, 701, 1001, …",
                "",
            ),
            (
                PluralCategory::Two,
                "2, 102, 202, 302, 402, 502, 602, 702, 1002, …",
                "",
            ),
            (
                PluralCategory::Few,
                "3, 4, 103, 104, 203, 204, 303, 304, 403, 404, 503, 504, 603, 604, 703, 704, 1003, …",
                "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
            (
                PluralCategory::Other,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "dsb",
        &[
            (
                PluralCategory::One,
                "1, 101, 201, 301, 401, 501, 601, 701, 1001, …",
                "0.1, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 10.1, 100.1, 1000.1, …",
            ),
            (
                PluralCategory::Two,
                "2, 102, 202, 302, 402, 502, 602, 702, 1002, …",
                "0.2, 1.2, 2.2, 3.2, 4.2, 5.2, 6.2, 7.2, 10.2, 100.2, 1000.2, …",
            ),
            (
                PluralCategory::Few,
                "3, 4, 103, 104, 203, 204, 303, 304, 403, 404, 503, 504, 603, 604, 703, 704, 1003, …",
                "0.3, 0.4, 1.3, 1.4, 2.3, 2.4, 3.3, 3.4, 4.3, 4.4, 5.3, 5.4, 6.3, 6.4, 7.3, 7.4, 10.3, 100.3, 1000.3, …",
            ),
            (
                PluralCategory::Other,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 0.5~1.0, 1.5~2.0, 2.5~2.7, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "cs",
        &[
            (PluralCategory::One, "1", ""),
            (PluralCategory::Few, "2~4", ""),
            (
                PluralCategory::Many,
                "",
                "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
            (
                PluralCategory::Other,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "pl",
        &[
            (PluralCategory::One, "1", ""),
            (
                PluralCategory::Few,
                "2~4, 22~24, 32~34, 42~44, 52~54, 62, 102, 1002, …",
                "",
            ),
            (
                PluralCategory::Many,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
            (
                PluralCategory::Other,
                "",
                "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "be",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "1.0, 21.0, 31.0, 41.0, 51.0, 61.0, 71.0, 81.0, 101.0, 1001.0, …",
            ),
            (
                PluralCategory::Few,
                "2~4, 22~24, 32~34, 42~44, 52~54, 62, 102, 1002, …",
                "2.0, 3.0, 4.0, 22.0, 23.0, 24.0, 32.0, 33.0, 102.0, 1002.0, …",
            ),
            (
                PluralCategory::Many,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
            (
                PluralCategory::Other,
                "",
                "0.1~0.9, 1.1~1.7, 10.1, 100.1, 1000.1, …",
            ),
        ],
    ),
    (
        "lt",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "1.0, 21.0, 31.0, 41.0, 51.0, 61.0, 71.0, 81.0, 101.0, 1001.0, …",
            ),
            (
                PluralCategory::Few,
                "2~9, 22~29, 102, 1002, …",
                "2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 22.0, 102.0, 1002.0, …",
            ),
            (
                PluralCategory::Many,
                "",
                "0.1~0.9, 1.1~1.7, 10.1, 100.1, 1000.1, …",
            ),
            (
                PluralCategory::Other,
                "0, 10~20, 30, 40, 50, 60, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "ru",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "",
            ),
            (
                PluralCategory::Few,
                "2~4, 22~24, 32~34, 42~44, 52~54, 62, 102, 1002, …",
                "",
            ),
            (
                PluralCategory::Many,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
            (
                PluralCategory::Other,
                "",
                "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "sgs",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "1.0, 21.0, 31.0, 41.0, 51.0, 61.0, 71.0, 81.0, 101.0, 1001.0, …",
            ),
            (PluralCategory::Two, "2", "2.0, 2.00, 2.000, 2.0000"),
            (
                PluralCategory::Few,
                "3~9, 22~29, 32, 102, 1002, …",
                "3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 22.0, 102.0, 1002.0, …",
            ),
            (
                PluralCategory::Many,
                "",
                "0.1~0.9, 1.1~1.7, 10.1, 100.1, 1000.1, …",
            ),
            (
                PluralCategory::Other,
                "0, 10~20, 30, 40, 50, 60, 100, 1000, 10000, 100000, 1000000, …",
                "0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "br",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 81, 101, 1001, …",
                "1.0, 21.0, 31.0, 41.0, 51.0, 61.0, 81.0, 101.0, 1001.0, …",
            ),
            (
                PluralCategory::Two,
                "2, 22, 32, 42, 52, 62, 82, 102, 1002, …",
                "2.0, 22.0, 32.0, 42.0, 52.0, 62.0, 82.0, 102.0, 1002.0, …",
            ),
            (
                PluralCategory::Few,
                "3, 4, 9, 23, 24, 29, 33, 34, 39, 43, 44, 49, 103, 1003, …",
                "3.0, 4.0, 9.0, 23.0, 24.0, 29.0, 33.0, 34.0, 103.0, 1003.0, …",
            ),
            (
                PluralCategory::Many,
                "1000000, …",
                "1000000.0, 1000000.00, 1000000.000, 1000000.0000, …",
            ),
            (
                PluralCategory::Other,
                "0, 5~8, 10~20, 100, 1000, 10000, 100000, …",
                "0.0~0.9, 1.1~1.6, 10.0, 100.0, 1000.0, 10000.0, 100000.0, …",
            ),
        ],
    ),
    (
        "mt",
        &[
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (PluralCategory::Two, "2", "2.0, 2.00, 2.000, 2.0000"),
            (
                PluralCategory::Few,
                "0, 3~10, 103~109, 1003, …",
                "0.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 103.0, 1003.0, …",
            ),
            (
                PluralCategory::Many,
                "11~19, 111~117, 1011, …",
                "11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 111.0, 1011.0, …",
            ),
            (
                PluralCategory::Other,
                "20~35, 100, 1000, 10000, 100000, 1000000, …",
                "0.1~0.9, 1.1~1.7, 10.1, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "ga",
        &[
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (PluralCategory::Two, "2", "2.0, 2.00, 2.000, 2.0000"),
            (
                PluralCategory::Few,
                "3~6",
                "3.0, 4.0, 5.0, 6.0, 3.00, 4.00, 5.00, 6.00, 3.000, 4.000, 5.000, 6.000, 3.0000, 4.0000, 5.0000, 6.0000",
            ),
            (
                PluralCategory::Many,
                "7~10",
                "7.0, 8.0, 9.0, 10.0, 7.00, 8.00, 9.00, 10.00, 7.000, 8.000, 9.000, 10.000, 7.0000, 8.0000, 9.0000, 10.0000",
            ),
            (
                PluralCategory::Other,
                "0, 11~25, 100, 1000, 10000, 100000, 1000000, …",
                "0.0~0.9, 1.1~1.6, 10.1, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "gv",
        &[
            (
                PluralCategory::One,
                "1, 11, 21, 31, 41, 51, 61, 71, 101, 1001, …",
                "",
            ),
            (
                PluralCategory::Two,
                "2, 12, 22, 32, 42, 52, 62, 72, 102, 1002, …",
                "",
            ),
            (
                PluralCategory::Few,
                "0, 20, 40, 60, 80, 100, 120, 140, 1000, 10000, 100000, 1000000, …",
                "",
            ),
            (
                PluralCategory::Many,
                "",
                "0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
            (PluralCategory::Other, "3~10, 13~19, 23, 103, 1003, …", ""),
        ],
    ),
    (
        "kw",
        &[
            (PluralCategory::Zero, "0", "0.0, 0.00, 0.000, 0.0000"),
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (
                PluralCategory::Two,
                "2, 22, 42, 62, 82, 102, 122, 142, 1000, 10000, 100000, …",
                "2.0, 22.0, 42.0, 62.0, 82.0, 102.0, 122.0, 142.0, 1000.0, 10000.0, 100000.0, …",
            ),
            (
                PluralCategory::Few,
                "3, 23, 43, 63, 83, 103, 123, 143, 1003, …",
                "3.0, 23.0, 43.0, 63.0, 83.0, 103.0, 123.0, 143.0, 1003.0, …",
            ),
            (
                PluralCategory::Many,
                "21, 41, 61, 81, 101, 121, 141, 161, 1001, …",
                "21.0, 41.0, 61.0, 81.0, 101.0, 121.0, 141.0, 161.0, 1001.0, …",
            ),
            (
                PluralCategory::Other,
                "4~19, 100, 1004, 1000000, …",
                "0.1~0.9, 1.1~1.7, 10.0, 100.0, 1000.1, 1000000.0, …",
            ),
        ],
    ),
    (
        "ar",
        &[
            (PluralCategory::Zero, "0", "0.0, 0.00, 0.000, 0.0000"),
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (PluralCategory::Two, "2", "2.0, 2.00, 2.000, 2.0000"),
            (
                PluralCategory::Few,
                "3~10, 103~110, 1003, …",
                "3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 103.0, 1003.0, …",
            ),
            (
                PluralCategory::Many,
                "11~26, 111, 1011, …",
                "11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 111.0, 1011.0, …",
            ),
            (
                PluralCategory::Other,
                "100~102, 200~202, 300~302, 400~402, 500~502, 600, 1000, 10000, 100000, 1000000, …",
                "0.1~0.9, 1.1~1.7, 10.1, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
    (
        "cy",
        &[
            (PluralCategory::Zero, "0", "0.0, 0.00, 0.000, 0.0000"),
            (PluralCategory::One, "1", "1.0, 1.00, 1.000, 1.0000"),
            (PluralCategory::Two, "2", "2.0, 2.00, 2.000, 2.0000"),
            (PluralCategory::Few, "3", "3.0, 3.00, 3.000, 3.0000"),
            (PluralCategory::Many, "6", "6.0, 6.00, 6.000, 6.0000"),
            (
                PluralCategory::Other,
                "4, 5, 7~20, 100, 1000, 10000, 100000, 1000000, …",
                "0.1~0.9, 1.1~1.7, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
            ),
        ],
    ),
];

/// The ordinal rules of `af am an ar ast bg bs ce cs cv da de dsb el es et eu fa fi fy gl gsw he hr hsb ia id ie in is iw ja km kn ko ky lt lv ml mn my nb nl no pa pl prg ps pt root ru sd sh si sk sl sr sw ta te th tpi tr ur uz yue zh zu`.
static ORDINAL_0: &[Rule] = &[];

/// The ordinal rules of `sv`.
static ORDINAL_1: &[Rule] = &[
    // one: n % 10 = 1,2 and n % 100 != 11,12
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1), (2, 2)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(11, 11), (12, 12)],
            },
        ]],
    },
];

/// The ordinal rules of `bal fil fr ga hy lo mo ms ro tl vi`.
static ORDINAL_2: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
];

/// The ordinal rules of `hu`.
static ORDINAL_3: &[Rule] = &[
    // one: n = 1,5
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1), (5, 5)],
        }]],
    },
];

/// The ordinal rules of `ne`.
static ORDINAL_4: &[Rule] = &[
    // one: n = 1..4
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 4)],
        }]],
    },
];

/// The ordinal rules of `be`.
static ORDINAL_5: &[Rule] = &[
    // few: n % 10 = 2,3 and n % 100 != 12,13
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(2, 2), (3, 3)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(12, 12), (13, 13)],
            },
        ]],
    },
];

/// The ordinal rules of `uk`.
static ORDINAL_6: &[Rule] = &[
    // few: n % 10 = 3 and n % 100 != 13
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(3, 3)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(13, 13)],
            },
        ]],
    },
];

/// The ordinal rules of `tk`.
static ORDINAL_7: &[Rule] = &[
    // few: n % 10 = 6,9 or n = 10
    Rule {
        category: PluralCategory::Few,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(6, 6), (9, 9)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(10, 10)],
            }],
        ],
    },
];

/// The ordinal rules of `kk`.
static ORDINAL_8: &[Rule] = &[
    // many: n % 10 = 6 or n % 10 = 9 or n % 10 = 0 and n != 0
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(6, 6)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(9, 9)],
            }],
            &[
                Relation {
                    operand: Operand::N,
                    modulus: 10,
                    negated: false,
                    ranges: &[(0, 0)],
                },
                Relation {
                    operand: Operand::N,
                    modulus: 0,
                    negated: true,
                    ranges: &[(0, 0)],
                },
            ],
        ],
    },
];

/// The ordinal rules of `it lld sc vec`.
static ORDINAL_9: &[Rule] = &[
    // many: n = 11,8,80,800
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(11, 11), (8, 8), (80, 80), (800, 800)],
        }]],
    },
];

/// The ordinal rules of `lij scn`.
static ORDINAL_10: &[Rule] = &[
    // many: n = 11,8,80..89,800..899
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(11, 11), (8, 8), (80, 89), (800, 899)],
        }]],
    },
];

/// The ordinal rules of `ka`.
static ORDINAL_11: &[Rule] = &[
    // one: i = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::I,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // many: i = 0 or i % 100 = 2..20,40,60,80
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            }],
            &[Relation {
                operand: Operand::I,
                modulus: 100,
                negated: false,
                ranges: &[(2, 20), (40, 40), (60, 60), (80, 80)],
            }],
        ],
    },
];

/// The ordinal rules of `sq`.
static ORDINAL_12: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // many: n % 10 = 4 and n % 100 != 14
    Rule {
        category: PluralCategory::Many,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(4, 4)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(14, 14)],
            },
        ]],
    },
];

/// The ordinal rules of `kw`.
static ORDINAL_13: &[Rule] = &[
    // one: n = 1..4 or n % 100 = 1..4,21..24,41..44,61..64,81..84
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(1, 4)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 100,
                negated: false,
                ranges: &[(1, 4), (21, 24), (41, 44), (61, 64), (81, 84)],
            }],
        ],
    },
    // many: n = 5 or n % 100 = 5
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[Relation {
                operand: Operand::N,
                modulus: 0,
                negated: false,
                ranges: &[(5, 5)],
            }],
            &[Relation {
                operand: Operand::N,
                modulus: 100,
                negated: false,
                ranges: &[(5, 5)],
            }],
        ],
    },
];

/// The ordinal rules of `blo`.
static ORDINAL_14: &[Rule] = &[
    // zero: i = 0
    Rule {
        category: PluralCategory::Zero,
        condition: &[&[Relation {
            operand: Operand::I,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0)],
        }]],
    },
    // one: i = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::I,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // few: i = 2,3,4,5,6
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::I,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2), (3, 3), (4, 4), (5, 5), (6, 6)],
        }]],
    },
];

/// The ordinal rules of `en`.
static ORDINAL_15: &[Rule] = &[
    // one: n % 10 = 1 and n % 100 != 11
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(11, 11)],
            },
        ]],
    },
    // two: n % 10 = 2 and n % 100 != 12
    Rule {
        category: PluralCategory::Two,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(2, 2)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(12, 12)],
            },
        ]],
    },
    // few: n % 10 = 3 and n % 100 != 13
    Rule {
        category: PluralCategory::Few,
        condition: &[&[
            Relation {
                operand: Operand::N,
                modulus: 10,
                negated: false,
                ranges: &[(3, 3)],
            },
            Relation {
                operand: Operand::N,
                modulus: 100,
                negated: true,
                ranges: &[(13, 13)],
            },
        ]],
    },
];

/// The ordinal rules of `kok kok_Latn mr`.
static ORDINAL_16: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n = 2,3
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2), (3, 3)],
        }]],
    },
    // few: n = 4
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(4, 4)],
        }]],
    },
];

/// The ordinal rules of `gd`.
static ORDINAL_17: &[Rule] = &[
    // one: n = 1,11
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1), (11, 11)],
        }]],
    },
    // two: n = 2,12
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2), (12, 12)],
        }]],
    },
    // few: n = 3,13
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(3, 3), (13, 13)],
        }]],
    },
];

/// The ordinal rules of `ca`.
static ORDINAL_18: &[Rule] = &[
    // one: n = 1,3
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1), (3, 3)],
        }]],
    },
    // two: n = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2)],
        }]],
    },
    // few: n = 4
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(4, 4)],
        }]],
    },
];

/// The ordinal rules of `mk`.
static ORDINAL_19: &[Rule] = &[
    // one: i % 10 = 1 and i % 100 != 11
    Rule {
        category: PluralCategory::One,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: true,
                ranges: &[(11, 11)],
            },
        ]],
    },
    // two: i % 10 = 2 and i % 100 != 12
    Rule {
        category: PluralCategory::Two,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(2, 2)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: true,
                ranges: &[(12, 12)],
            },
        ]],
    },
    // many: i % 10 = 7,8 and i % 100 != 17,18
    Rule {
        category: PluralCategory::Many,
        condition: &[&[
            Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(7, 7), (8, 8)],
            },
            Relation {
                operand: Operand::I,
                modulus: 100,
                negated: true,
                ranges: &[(17, 17), (18, 18)],
            },
        ]],
    },
];

/// The ordinal rules of `az`.
static ORDINAL_20: &[Rule] = &[
    // one: i % 10 = 1,2,5,7,8 or i % 100 = 20,50,70,80
    Rule {
        category: PluralCategory::One,
        condition: &[
            &[Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(1, 1), (2, 2), (5, 5), (7, 7), (8, 8)],
            }],
            &[Relation {
                operand: Operand::I,
                modulus: 100,
                negated: false,
                ranges: &[(20, 20), (50, 50), (70, 70), (80, 80)],
            }],
        ],
    },
    // few: i % 10 = 3,4 or i % 1000 = 100,200,300,400,500,600,700,800,900
    Rule {
        category: PluralCategory::Few,
        condition: &[
            &[Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(3, 3), (4, 4)],
            }],
            &[Relation {
                operand: Operand::I,
                modulus: 1000,
                negated: false,
                ranges: &[
                    (100, 100),
                    (200, 200),
                    (300, 300),
                    (400, 400),
                    (500, 500),
                    (600, 600),
                    (700, 700),
                    (800, 800),
                    (900, 900),
                ],
            }],
        ],
    },
    // many: i = 0 or i % 10 = 6 or i % 100 = 40,60,90
    Rule {
        category: PluralCategory::Many,
        condition: &[
            &[Relation {
                operand: Operand::I,
                modulus: 0,
                negated: false,
                ranges: &[(0, 0)],
            }],
            &[Relation {
                operand: Operand::I,
                modulus: 10,
                negated: false,
                ranges: &[(6, 6)],
            }],
            &[Relation {
                operand: Operand::I,
                modulus: 100,
                negated: false,
                ranges: &[(40, 40), (60, 60), (90, 90)],
            }],
        ],
    },
];

/// The ordinal rules of `gu hi`.
static ORDINAL_21: &[Rule] = &[
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n = 2,3
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2), (3, 3)],
        }]],
    },
    // few: n = 4
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(4, 4)],
        }]],
    },
    // many: n = 6
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(6, 6)],
        }]],
    },
];

/// The ordinal rules of `as bn`.
static ORDINAL_22: &[Rule] = &[
    // one: n = 1,5,7,8,9,10
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1), (5, 5), (7, 7), (8, 8), (9, 9), (10, 10)],
        }]],
    },
    // two: n = 2,3
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2), (3, 3)],
        }]],
    },
    // few: n = 4
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(4, 4)],
        }]],
    },
    // many: n = 6
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(6, 6)],
        }]],
    },
];

/// The ordinal rules of `or`.
static ORDINAL_23: &[Rule] = &[
    // one: n = 1,5,7..9
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1), (5, 5), (7, 9)],
        }]],
    },
    // two: n = 2,3
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2), (3, 3)],
        }]],
    },
    // few: n = 4
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(4, 4)],
        }]],
    },
    // many: n = 6
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(6, 6)],
        }]],
    },
];

/// The ordinal rules of `cy`.
static ORDINAL_24: &[Rule] = &[
    // zero: n = 0,7,8,9
    Rule {
        category: PluralCategory::Zero,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(0, 0), (7, 7), (8, 8), (9, 9)],
        }]],
    },
    // one: n = 1
    Rule {
        category: PluralCategory::One,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(1, 1)],
        }]],
    },
    // two: n = 2
    Rule {
        category: PluralCategory::Two,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(2, 2)],
        }]],
    },
    // few: n = 3,4
    Rule {
        category: PluralCategory::Few,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(3, 3), (4, 4)],
        }]],
    },
    // many: n = 5,6
    Rule {
        category: PluralCategory::Many,
        condition: &[&[Relation {
            operand: Operand::N,
            modulus: 0,
            negated: false,
            ranges: &[(5, 5), (6, 6)],
        }]],
    },
];

/// Every locale of `ordinals.xml` but `root`, sorted, with its ordinal rules.
pub(super) static ORDINAL: &[(&str, &[Rule])] = &[
    ("af", ORDINAL_0),
    ("am", ORDINAL_0),
    ("an", ORDINAL_0),
    ("ar", ORDINAL_0),
    ("as", ORDINAL_22),
    ("ast", ORDINAL_0),
    ("az", ORDINAL_20),
    ("bal", ORDINAL_2),
    ("be", ORDINAL_5),
    ("bg", ORDINAL_0),
    ("blo", ORDINAL_14),
    ("bn", ORDINAL_22),
    ("bs", ORDINAL_0),
    ("ca", ORDINAL_18),
    ("ce", ORDINAL_0),
    ("cs", ORDINAL_0),
    ("cv", ORDINAL_0),
    ("cy", ORDINAL_24),
    ("da", ORDINAL_0),
    ("de", ORDINAL_0),
    ("dsb", ORDINAL_0),
    ("el", ORDINAL_0),
    ("en", ORDINAL_15),
    ("es", ORDINAL_0),
    ("et", ORDINAL_0),
    ("eu", ORDINAL_0),
    ("fa", ORDINAL_0),
    ("fi", ORDINAL_0),
    ("fil", ORDINAL_2),
    ("fr", ORDINAL_2),
    ("fy", ORDINAL_0),
    ("ga", ORDINAL_2),
    ("gd", ORDINAL_17),
    ("gl", ORDINAL_0),
    ("gsw", ORDINAL_0),
    ("gu", ORDINAL_21),
    ("he", ORDINAL_0),
    ("hi", ORDINAL_21),
    ("hr", ORDINAL_0),
    ("hsb", ORDINAL_0),
    ("hu", ORDINAL_3),
    ("hy", ORDINAL_2),
    ("ia", ORDINAL_0),
    ("id", ORDINAL_0),
    ("ie", ORDINAL_0),
    ("in", ORDINAL_0),
    ("is", ORDINAL_0),
    ("it", ORDINAL_9),
    ("iw", ORDINAL_0),
    ("ja", ORDINAL_0),
    ("ka", ORDINAL_11),
    ("kk", ORDINAL_8),
    ("km", ORDINAL_0),
    ("kn", ORDINAL_0),
    ("ko", ORDINAL_0),
    ("kok", ORDINAL_16),
    ("kok-Latn", ORDINAL_16),
    ("kw", ORDINAL_13),
    ("ky", ORDINAL_0),
    ("lij", ORDINAL_10),
    ("lld", ORDINAL_9),
    ("lo", ORDINAL_2),
    ("lt", ORDINAL_0),
    ("lv", ORDINAL_0),
    ("mk", ORDINAL_19),
    ("ml", ORDINAL_0),
    ("mn", ORDINAL_0),
    ("mo", ORDINAL_2),
    ("mr", ORDINAL_16),
    ("ms", ORDINAL_2),
    ("my", ORDINAL_0),
    ("nb", ORDINAL_0),
    ("ne", ORDINAL_4),
    ("nl", ORDINAL_0),
    ("no", ORDINAL_0),
    ("or", ORDINAL_23),
    ("pa", ORDINAL_0),
    ("pl", ORDINAL_0),
    ("prg", ORDINAL_0),
    ("ps", ORDINAL_0),
    ("pt", ORDINAL_0),
    ("ro", ORDINAL_2),
    ("ru", ORDINAL_0),
    ("sc", ORDINAL_9),
    ("scn", ORDINAL_10),
    ("sd", ORDINAL_0),
    ("sh", ORDINAL_0),
    ("si", ORDINAL_0),
    ("sk", ORDINAL_0),
    ("sl", ORDINAL_0),
    ("sq", ORDINAL_12),
    ("sr", ORDINAL_0),
    ("sv", ORDINAL_1),
    ("sw", ORDINAL_0),
    ("ta", ORDINAL_0),
    ("te", ORDINAL_0),
    ("th", ORDINAL_0),
    ("tk", ORDINAL_7),
    ("tl", ORDINAL_2),
    ("tpi", ORDINAL_0),
    ("tr", ORDINAL_0),
    ("uk", ORDINAL_6),
    ("ur", ORDINAL_0),
    ("uz", ORDINAL_0),
    ("vec", ORDINAL_9),
    ("vi", ORDINAL_2),
    ("yue", ORDINAL_0),
    ("zh", ORDINAL_0),
    ("zu", ORDINAL_0),
];

/// The samples of each ordinal block, under the first locale of the block:
/// (category, `@integer` samples, `@decimal` samples) as the file writes them.
#[cfg(test)]
pub(super) static ORDINAL_SAMPLES: &[(&str, Samples)] = &[
    (
        "af",
        &[(
            PluralCategory::Other,
            "0~15, 100, 1000, 10000, 100000, 1000000, …",
            "",
        )],
    ),
    (
        "sv",
        &[
            (
                PluralCategory::One,
                "1, 2, 21, 22, 31, 32, 41, 42, 51, 52, 61, 62, 71, 72, 81, 82, 101, 1001, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0, 3~17, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "bal",
        &[
            (PluralCategory::One, "1", ""),
            (
                PluralCategory::Other,
                "0, 2~16, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "hu",
        &[
            (PluralCategory::One, "1, 5", ""),
            (
                PluralCategory::Other,
                "0, 2~4, 6~17, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "ne",
        &[
            (PluralCategory::One, "1~4", ""),
            (
                PluralCategory::Other,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "be",
        &[
            (
                PluralCategory::Few,
                "2, 3, 22, 23, 32, 33, 42, 43, 52, 53, 62, 63, 72, 73, 82, 83, 102, 1002, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0, 1, 4~17, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "uk",
        &[
            (
                PluralCategory::Few,
                "3, 23, 33, 43, 53, 63, 73, 83, 103, 1003, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0~2, 4~16, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "tk",
        &[
            (
                PluralCategory::Few,
                "6, 9, 10, 16, 19, 26, 29, 36, 39, 106, 1006, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0~5, 7, 8, 11~15, 17, 18, 20, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "kk",
        &[
            (
                PluralCategory::Many,
                "6, 9, 10, 16, 19, 20, 26, 29, 30, 36, 39, 40, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0~5, 7, 8, 11~15, 17, 18, 21, 101, 1001, …",
                "",
            ),
        ],
    ),
    (
        "it",
        &[
            (PluralCategory::Many, "8, 11, 80, 800", ""),
            (
                PluralCategory::Other,
                "0~7, 9, 10, 12~17, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "lij",
        &[
            (PluralCategory::Many, "8, 11, 80~89, 800~803", ""),
            (
                PluralCategory::Other,
                "0~7, 9, 10, 12~17, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "ka",
        &[
            (PluralCategory::One, "1", ""),
            (PluralCategory::Many, "0, 2~16, 102, 1002, …", ""),
            (
                PluralCategory::Other,
                "21~36, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "sq",
        &[
            (PluralCategory::One, "1", ""),
            (
                PluralCategory::Many,
                "4, 24, 34, 44, 54, 64, 74, 84, 104, 1004, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0, 2, 3, 5~17, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "kw",
        &[
            (
                PluralCategory::One,
                "1~4, 21~24, 41~44, 61~64, 101, 1001, …",
                "",
            ),
            (
                PluralCategory::Many,
                "5, 105, 205, 305, 405, 505, 605, 705, 1005, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0, 6~20, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "blo",
        &[
            (PluralCategory::Zero, "0", ""),
            (PluralCategory::One, "1", ""),
            (PluralCategory::Few, "2~6", ""),
            (
                PluralCategory::Other,
                "7~22, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "en",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "",
            ),
            (
                PluralCategory::Two,
                "2, 22, 32, 42, 52, 62, 72, 82, 102, 1002, …",
                "",
            ),
            (
                PluralCategory::Few,
                "3, 23, 33, 43, 53, 63, 73, 83, 103, 1003, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0, 4~18, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "kok",
        &[
            (PluralCategory::One, "1", ""),
            (PluralCategory::Two, "2, 3", ""),
            (PluralCategory::Few, "4", ""),
            (
                PluralCategory::Other,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "gd",
        &[
            (PluralCategory::One, "1, 11", ""),
            (PluralCategory::Two, "2, 12", ""),
            (PluralCategory::Few, "3, 13", ""),
            (
                PluralCategory::Other,
                "0, 4~10, 14~21, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "ca",
        &[
            (PluralCategory::One, "1, 3", ""),
            (PluralCategory::Two, "2", ""),
            (PluralCategory::Few, "4", ""),
            (
                PluralCategory::Other,
                "0, 5~19, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "mk",
        &[
            (
                PluralCategory::One,
                "1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, …",
                "",
            ),
            (
                PluralCategory::Two,
                "2, 22, 32, 42, 52, 62, 72, 82, 102, 1002, …",
                "",
            ),
            (
                PluralCategory::Many,
                "7, 8, 27, 28, 37, 38, 47, 48, 57, 58, 67, 68, 77, 78, 87, 88, 107, 1007, …",
                "",
            ),
            (
                PluralCategory::Other,
                "0, 3~6, 9~19, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "az",
        &[
            (
                PluralCategory::One,
                "1, 2, 5, 7, 8, 11, 12, 15, 17, 18, 20~22, 25, 101, 1001, …",
                "",
            ),
            (
                PluralCategory::Few,
                "3, 4, 13, 14, 23, 24, 33, 34, 43, 44, 53, 54, 63, 64, 73, 74, 100, 1003, …",
                "",
            ),
            (
                PluralCategory::Many,
                "0, 6, 16, 26, 36, 40, 46, 56, 106, 1006, …",
                "",
            ),
            (
                PluralCategory::Other,
                "9, 10, 19, 29, 30, 39, 49, 59, 69, 79, 109, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "gu",
        &[
            (PluralCategory::One, "1", ""),
            (PluralCategory::Two, "2, 3", ""),
            (PluralCategory::Few, "4", ""),
            (PluralCategory::Many, "6", ""),
            (
                PluralCategory::Other,
                "0, 5, 7~20, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "as",
        &[
            (PluralCategory::One, "1, 5, 7~10", ""),
            (PluralCategory::Two, "2, 3", ""),
            (PluralCategory::Few, "4", ""),
            (PluralCategory::Many, "6", ""),
            (
                PluralCategory::Other,
                "0, 11~25, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "or",
        &[
            (PluralCategory::One, "1, 5, 7~9", ""),
            (PluralCategory::Two, "2, 3", ""),
            (PluralCategory::Few, "4", ""),
            (PluralCategory::Many, "6", ""),
            (
                PluralCategory::Other,
                "0, 10~24, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
    (
        "cy",
        &[
            (PluralCategory::Zero, "0, 7~9", ""),
            (PluralCategory::One, "1", ""),
            (PluralCategory::Two, "2", ""),
            (PluralCategory::Few, "3, 4", ""),
            (PluralCategory::Many, "5, 6", ""),
            (
                PluralCategory::Other,
                "10~25, 100, 1000, 10000, 100000, 1000000, …",
                "",
            ),
        ],
    ),
];
