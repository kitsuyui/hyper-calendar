//! Berger's 1978 trigonometric series, term by term.
//!
//! The three tables below are the coefficient tables of Berger (1978), *J.
//! Atmos. Sci.* 35, 2362–2367, transcribed from the file `INSOL.IN` of the
//! author's own deposit of the solution — "1978 BERGER SOLUTION", the input
//! to the program `insol14.f`, "BERGER 78 version 2014" — published under
//! CC BY 4.0 at <https://doi.org/10.5281/zenodo.7198109> and read there on
//! 2026-09-25. The transcription was generated from the file, not typed,
//! and [`super::elements`]'s tests check it against the author's own
//! tabulated values of the elements and against two independent
//! transcriptions (NASA GISS's `ORBPAR` and NCAR's `shr_orb_mod`).
//!
//! Every term is `amplitude × trig(rate × t + phase)` with `t` in years
//! from 1950, negative in the past; how the three sums combine is in
//! [`super::elements`] and in `docs/systems/orbital-elements.md`.

/// One term of a trigonometric series: `amplitude × trig(rate × t + phase)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Term {
    /// The term's number in the paper's table.
    pub n: u8,
    /// The amplitude, in the unit the table states.
    pub amplitude: f64,
    /// The mean rate, in arcseconds per year.
    pub rate: f64,
    /// The phase at 1950, in degrees.
    pub phase: f64,
}

/// The obliquity's constant term ε*, in degrees.
pub const OBLIQUITY_CONSTANT_DEGREES: f64 = 23.320556;

/// The general precession's constant term ζ, in degrees.
pub const PRECESSION_CONSTANT_DEGREES: f64 = 3.392506;

/// The general precession's mean rate ψ̄, in arcseconds per year.
pub const PRECESSION_RATE_ARCSEC_PER_YEAR: f64 = 50.439273;

/// The 19 terms of *e* sin π and *e* cos π: Berger's Table 1 (the paper's
/// Table 4), as `INSOL.IN` lists them. `n` is the term's number in the
/// paper, which orders the terms differently from the file.
///
/// `amplitude` is dimensionless; `rate` is in arcseconds per year; `phase`
/// in degrees.
pub const ECCENTRICITY: [Term; 19] = [
    Term {
        n: 5,
        amplitude: 0.01860798,
        rate: 4.2072050,
        phase: 28.620089,
    },
    Term {
        n: 2,
        amplitude: 0.01627522,
        rate: 7.3460910,
        phase: 193.788772,
    },
    Term {
        n: 4,
        amplitude: -0.01300660,
        rate: 17.8572630,
        phase: 308.307024,
    },
    Term {
        n: 3,
        amplitude: 0.00988829,
        rate: 17.2205460,
        phase: 320.199637,
    },
    Term {
        n: 19,
        amplitude: -0.00336700,
        rate: 16.8467330,
        phase: 279.376984,
    },
    Term {
        n: 1,
        amplitude: 0.00333077,
        rate: 5.1990790,
        phase: 87.195000,
    },
    Term {
        n: 16,
        amplitude: -0.00235400,
        rate: 18.2310760,
        phase: 349.129677,
    },
    Term {
        n: 6,
        amplitude: 0.00140015,
        rate: 26.2167580,
        phase: 128.443387,
    },
    Term {
        n: 13,
        amplitude: 0.00100700,
        rate: 6.3591690,
        phase: 154.143880,
    },
    Term {
        n: 18,
        amplitude: 0.00085700,
        rate: 16.2100160,
        phase: 291.269597,
    },
    Term {
        n: 7,
        amplitude: 0.00064990,
        rate: 3.0651810,
        phase: 114.860583,
    },
    Term {
        n: 10,
        amplitude: 0.00059900,
        rate: 16.5838290,
        phase: 332.092251,
    },
    Term {
        n: 9,
        amplitude: 0.00037800,
        rate: 18.4939800,
        phase: 296.414411,
    },
    Term {
        n: 11,
        amplitude: -0.00033700,
        rate: 6.1909530,
        phase: 145.769910,
    },
    Term {
        n: 17,
        amplitude: 0.00027600,
        rate: 18.8677930,
        phase: 337.237063,
    },
    Term {
        n: 15,
        amplitude: 0.00018200,
        rate: 17.4255670,
        phase: 152.092288,
    },
    Term {
        n: 12,
        amplitude: -0.00017400,
        rate: 6.1860010,
        phase: 126.839891,
    },
    Term {
        n: 14,
        amplitude: -0.00012400,
        rate: 18.4174410,
        phase: 210.667199,
    },
    Term {
        n: 8,
        amplitude: 0.00001250,
        rate: 0.6678630,
        phase: 72.108838,
    },
];
/// The 47 terms of the obliquity: Berger's Table 2. `amplitude` is in
/// arcseconds, `rate` in arcseconds per year, `phase` in degrees. The file
/// also prints each term's period in years, which is 1 296 000 / `rate` and
/// is not carried.
pub const OBLIQUITY: [Term; 47] = [
    Term {
        n: 1,
        amplitude: -2462.2214466,
        rate: 31.609974,
        phase: 251.9025,
    },
    Term {
        n: 2,
        amplitude: -857.3232075,
        rate: 32.620504,
        phase: 280.8325,
    },
    Term {
        n: 3,
        amplitude: -629.3231835,
        rate: 24.172203,
        phase: 128.3057,
    },
    Term {
        n: 4,
        amplitude: -414.2804924,
        rate: 31.983787,
        phase: 292.7252,
    },
    Term {
        n: 5,
        amplitude: -311.7632587,
        rate: 44.828336,
        phase: 15.3747,
    },
    Term {
        n: 6,
        amplitude: 308.9408604,
        rate: 30.973257,
        phase: 263.7951,
    },
    Term {
        n: 7,
        amplitude: -162.5533601,
        rate: 43.668246,
        phase: 308.4258,
    },
    Term {
        n: 8,
        amplitude: -116.1077911,
        rate: 32.246691,
        phase: 240.0099,
    },
    Term {
        n: 9,
        amplitude: 101.1189923,
        rate: 30.599444,
        phase: 222.9725,
    },
    Term {
        n: 10,
        amplitude: -67.6856209,
        rate: 42.681324,
        phase: 268.7809,
    },
    Term {
        n: 11,
        amplitude: 24.9079067,
        rate: 43.836462,
        phase: 316.7998,
    },
    Term {
        n: 12,
        amplitude: 22.5811241,
        rate: 47.439436,
        phase: 319.6024,
    },
    Term {
        n: 13,
        amplitude: -21.1648355,
        rate: 63.219948,
        phase: 143.8050,
    },
    Term {
        n: 14,
        amplitude: -15.6549876,
        rate: 64.230478,
        phase: 172.7351,
    },
    Term {
        n: 15,
        amplitude: 15.3936813,
        rate: 1.010530,
        phase: 28.9300,
    },
    Term {
        n: 16,
        amplitude: 14.6660938,
        rate: 7.437771,
        phase: 123.5968,
    },
    Term {
        n: 17,
        amplitude: -11.7273029,
        rate: 55.782177,
        phase: 20.2082,
    },
    Term {
        n: 18,
        amplitude: 10.2742696,
        rate: 0.373813,
        phase: 40.8226,
    },
    Term {
        n: 19,
        amplitude: 6.4914588,
        rate: 13.218362,
        phase: 123.4722,
    },
    Term {
        n: 20,
        amplitude: 5.8539148,
        rate: 62.583231,
        phase: 155.6977,
    },
    Term {
        n: 21,
        amplitude: -5.4872205,
        rate: 63.593761,
        phase: 184.6277,
    },
    Term {
        n: 22,
        amplitude: -5.4290191,
        rate: 76.438310,
        phase: 267.2772,
    },
    Term {
        n: 23,
        amplitude: 5.1609570,
        rate: 45.815258,
        phase: 55.0196,
    },
    Term {
        n: 24,
        amplitude: 5.0786314,
        rate: 8.448301,
        phase: 152.5268,
    },
    Term {
        n: 25,
        amplitude: -4.0735782,
        rate: 56.792707,
        phase: 49.1382,
    },
    Term {
        n: 26,
        amplitude: 3.7227167,
        rate: 49.747842,
        phase: 204.6609,
    },
    Term {
        n: 27,
        amplitude: 3.3971932,
        rate: 12.058272,
        phase: 56.5233,
    },
    Term {
        n: 28,
        amplitude: -2.8347004,
        rate: 75.278220,
        phase: 200.3284,
    },
    Term {
        n: 29,
        amplitude: -2.6550721,
        rate: 65.241008,
        phase: 201.6651,
    },
    Term {
        n: 30,
        amplitude: -2.5717867,
        rate: 64.604291,
        phase: 213.5577,
    },
    Term {
        n: 31,
        amplitude: -2.4712188,
        rate: 1.647247,
        phase: 17.0374,
    },
    Term {
        n: 32,
        amplitude: 2.4625410,
        rate: 7.811584,
        phase: 164.4194,
    },
    Term {
        n: 33,
        amplitude: 2.2464112,
        rate: 12.207832,
        phase: 94.5422,
    },
    Term {
        n: 34,
        amplitude: -2.0755511,
        rate: 63.856665,
        phase: 131.9124,
    },
    Term {
        n: 35,
        amplitude: -1.9713669,
        rate: 56.155990,
        phase: 61.0309,
    },
    Term {
        n: 36,
        amplitude: -1.8813061,
        rate: 77.448840,
        phase: 296.2073,
    },
    Term {
        n: 37,
        amplitude: -1.8468785,
        rate: 6.801054,
        phase: 135.4894,
    },
    Term {
        n: 38,
        amplitude: 1.8186742,
        rate: 62.209418,
        phase: 114.8750,
    },
    Term {
        n: 39,
        amplitude: 1.7601888,
        rate: 20.656133,
        phase: 247.0691,
    },
    Term {
        n: 40,
        amplitude: -1.5428851,
        rate: 48.344406,
        phase: 256.6114,
    },
    Term {
        n: 41,
        amplitude: 1.4738838,
        rate: 55.145460,
        phase: 32.1008,
    },
    Term {
        n: 42,
        amplitude: -1.4593669,
        rate: 69.000539,
        phase: 143.6804,
    },
    Term {
        n: 43,
        amplitude: 1.4192259,
        rate: 11.071350,
        phase: 16.8784,
    },
    Term {
        n: 44,
        amplitude: -1.1818980,
        rate: 74.291298,
        phase: 160.6835,
    },
    Term {
        n: 45,
        amplitude: 1.1756474,
        rate: 11.047742,
        phase: 27.5932,
    },
    Term {
        n: 46,
        amplitude: -1.1316126,
        rate: 0.636717,
        phase: 348.1074,
    },
    Term {
        n: 47,
        amplitude: 1.0896928,
        rate: 12.844549,
        phase: 82.6496,
    },
];
/// The 78 terms of the general precession in longitude: Berger's Table 3.
/// Units as [`OBLIQUITY`].
pub const PRECESSION: [Term; 78] = [
    Term {
        n: 1,
        amplitude: 7391.0225890,
        rate: 31.609974,
        phase: 251.9025,
    },
    Term {
        n: 2,
        amplitude: 2555.1526947,
        rate: 32.620504,
        phase: 280.8325,
    },
    Term {
        n: 3,
        amplitude: 2022.7629188,
        rate: 24.172203,
        phase: 128.3057,
    },
    Term {
        n: 4,
        amplitude: -1973.6517951,
        rate: 0.636717,
        phase: 348.1074,
    },
    Term {
        n: 5,
        amplitude: 1240.2321818,
        rate: 31.983787,
        phase: 292.7252,
    },
    Term {
        n: 6,
        amplitude: 953.8679112,
        rate: 3.138886,
        phase: 165.1686,
    },
    Term {
        n: 7,
        amplitude: -931.7537108,
        rate: 30.973257,
        phase: 263.7951,
    },
    Term {
        n: 8,
        amplitude: 872.3795383,
        rate: 44.828336,
        phase: 15.3747,
    },
    Term {
        n: 9,
        amplitude: 606.3544732,
        rate: 0.991874,
        phase: 58.5749,
    },
    Term {
        n: 10,
        amplitude: -496.0274038,
        rate: 0.373813,
        phase: 40.8226,
    },
    Term {
        n: 11,
        amplitude: 456.9608039,
        rate: 43.668246,
        phase: 308.4258,
    },
    Term {
        n: 12,
        amplitude: 346.9462320,
        rate: 32.246691,
        phase: 240.0099,
    },
    Term {
        n: 13,
        amplitude: -305.8412902,
        rate: 30.599444,
        phase: 222.9725,
    },
    Term {
        n: 14,
        amplitude: 249.6173246,
        rate: 2.147012,
        phase: 106.5937,
    },
    Term {
        n: 15,
        amplitude: -199.1027200,
        rate: 10.511172,
        phase: 114.5182,
    },
    Term {
        n: 16,
        amplitude: 191.0560889,
        rate: 42.681324,
        phase: 268.7809,
    },
    Term {
        n: 17,
        amplitude: -175.2936572,
        rate: 13.650058,
        phase: 279.6869,
    },
    Term {
        n: 18,
        amplitude: 165.9068833,
        rate: 0.986922,
        phase: 39.6448,
    },
    Term {
        n: 19,
        amplitude: 161.1285917,
        rate: 9.874455,
        phase: 126.4108,
    },
    Term {
        n: 20,
        amplitude: 139.7878093,
        rate: 13.013341,
        phase: 291.5795,
    },
    Term {
        n: 21,
        amplitude: -133.5228399,
        rate: 0.262904,
        phase: 307.2848,
    },
    Term {
        n: 22,
        amplitude: 117.0673811,
        rate: 0.004952,
        phase: 18.9300,
    },
    Term {
        n: 23,
        amplitude: 104.6907281,
        rate: 1.142024,
        phase: 273.7596,
    },
    Term {
        n: 24,
        amplitude: 95.3227476,
        rate: 63.219948,
        phase: 143.8050,
    },
    Term {
        n: 25,
        amplitude: 86.7824524,
        rate: 0.205021,
        phase: 191.8927,
    },
    Term {
        n: 26,
        amplitude: 86.0857729,
        rate: 2.151964,
        phase: 125.5237,
    },
    Term {
        n: 27,
        amplitude: 70.5893698,
        rate: 64.230478,
        phase: 172.7351,
    },
    Term {
        n: 28,
        amplitude: -69.9719343,
        rate: 43.836462,
        phase: 316.7998,
    },
    Term {
        n: 29,
        amplitude: -62.5817473,
        rate: 47.439436,
        phase: 319.6024,
    },
    Term {
        n: 30,
        amplitude: 61.5450059,
        rate: 1.384343,
        phase: 69.7526,
    },
    Term {
        n: 31,
        amplitude: -57.9364011,
        rate: 7.437771,
        phase: 123.5968,
    },
    Term {
        n: 32,
        amplitude: 57.1899832,
        rate: 18.829299,
        phase: 217.6432,
    },
    Term {
        n: 33,
        amplitude: -57.0236109,
        rate: 9.500642,
        phase: 85.5882,
    },
    Term {
        n: 34,
        amplitude: -54.2119253,
        rate: 0.431696,
        phase: 156.2147,
    },
    Term {
        n: 35,
        amplitude: 53.2834147,
        rate: 1.160090,
        phase: 66.9489,
    },
    Term {
        n: 36,
        amplitude: 52.1223575,
        rate: 55.782177,
        phase: 20.2082,
    },
    Term {
        n: 37,
        amplitude: -49.0059908,
        rate: 12.639528,
        phase: 250.7568,
    },
    Term {
        n: 38,
        amplitude: -48.3118757,
        rate: 1.155138,
        phase: 48.0188,
    },
    Term {
        n: 39,
        amplitude: -45.4191685,
        rate: 0.168216,
        phase: 8.3739,
    },
    Term {
        n: 40,
        amplitude: -42.2357920,
        rate: 1.647247,
        phase: 17.0374,
    },
    Term {
        n: 41,
        amplitude: -34.7971099,
        rate: 10.884985,
        phase: 155.3409,
    },
    Term {
        n: 42,
        amplitude: 34.4623613,
        rate: 5.610937,
        phase: 94.1709,
    },
    Term {
        n: 43,
        amplitude: -33.8356643,
        rate: 12.658184,
        phase: 221.1120,
    },
    Term {
        n: 44,
        amplitude: 33.6689362,
        rate: 1.010530,
        phase: 28.9300,
    },
    Term {
        n: 45,
        amplitude: -31.2521586,
        rate: 1.983748,
        phase: 117.1498,
    },
    Term {
        n: 46,
        amplitude: -30.8798701,
        rate: 14.023871,
        phase: 320.5095,
    },
    Term {
        n: 47,
        amplitude: 28.4640769,
        rate: 0.560178,
        phase: 262.3602,
    },
    Term {
        n: 48,
        amplitude: -27.1960802,
        rate: 1.273434,
        phase: 336.2148,
    },
    Term {
        n: 49,
        amplitude: 27.0860736,
        rate: 12.021467,
        phase: 233.0046,
    },
    Term {
        n: 50,
        amplitude: -26.3437456,
        rate: 62.583231,
        phase: 155.6977,
    },
    Term {
        n: 51,
        amplitude: 24.7253740,
        rate: 63.593761,
        phase: 184.6277,
    },
    Term {
        n: 52,
        amplitude: 24.6732126,
        rate: 76.438310,
        phase: 267.2772,
    },
    Term {
        n: 53,
        amplitude: 24.4272733,
        rate: 4.280910,
        phase: 78.9281,
    },
    Term {
        n: 54,
        amplitude: 24.0127327,
        rate: 13.218362,
        phase: 123.4722,
    },
    Term {
        n: 55,
        amplitude: 21.7150294,
        rate: 17.818769,
        phase: 188.7132,
    },
    Term {
        n: 56,
        amplitude: -21.5375347,
        rate: 8.359495,
        phase: 180.1364,
    },
    Term {
        n: 57,
        amplitude: 18.1148363,
        rate: 56.792707,
        phase: 49.1382,
    },
    Term {
        n: 58,
        amplitude: -16.9603104,
        rate: 8.448301,
        phase: 152.5268,
    },
    Term {
        n: 59,
        amplitude: -16.1765215,
        rate: 1.978796,
        phase: 98.2198,
    },
    Term {
        n: 60,
        amplitude: 15.5567653,
        rate: 8.863925,
        phase: 97.4808,
    },
    Term {
        n: 61,
        amplitude: 15.4846529,
        rate: 0.186365,
        phase: 221.5376,
    },
    Term {
        n: 62,
        amplitude: 15.2150632,
        rate: 8.996212,
        phase: 168.2438,
    },
    Term {
        n: 63,
        amplitude: 14.5047426,
        rate: 6.771027,
        phase: 161.1199,
    },
    Term {
        n: 64,
        amplitude: -14.3873316,
        rate: 45.815258,
        phase: 55.0196,
    },
    Term {
        n: 65,
        amplitude: 13.1351419,
        rate: 12.002811,
        phase: 262.6495,
    },
    Term {
        n: 66,
        amplitude: 12.8776311,
        rate: 75.278220,
        phase: 200.3284,
    },
    Term {
        n: 67,
        amplitude: 11.9867234,
        rate: 65.241008,
        phase: 201.6651,
    },
    Term {
        n: 68,
        amplitude: 11.9385578,
        rate: 18.870667,
        phase: 294.6547,
    },
    Term {
        n: 69,
        amplitude: 11.7030822,
        rate: 22.009553,
        phase: 99.8233,
    },
    Term {
        n: 70,
        amplitude: 11.6018181,
        rate: 64.604291,
        phase: 213.5577,
    },
    Term {
        n: 71,
        amplitude: -11.2617293,
        rate: 11.498094,
        phase: 154.1631,
    },
    Term {
        n: 72,
        amplitude: -10.4664199,
        rate: 0.578834,
        phase: 232.7153,
    },
    Term {
        n: 73,
        amplitude: 10.4333970,
        rate: 9.237738,
        phase: 138.3034,
    },
    Term {
        n: 74,
        amplitude: -10.2377466,
        rate: 49.747842,
        phase: 204.6609,
    },
    Term {
        n: 75,
        amplitude: 10.1934446,
        rate: 2.147012,
        phase: 106.5938,
    },
    Term {
        n: 76,
        amplitude: -10.1280191,
        rate: 1.196895,
        phase: 250.4676,
    },
    Term {
        n: 77,
        amplitude: 10.0289441,
        rate: 2.133898,
        phase: 332.3345,
    },
    Term {
        n: 78,
        amplitude: -10.0034259,
        rate: 0.173168,
        phase: 27.3039,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// The paper numbers its terms; the file lists them by size. Every
    /// number from 1 to the table's length appears exactly once, which is
    /// what a complete transcription looks like.
    #[test]
    fn every_table_carries_each_of_its_term_numbers_once() {
        fn check(table: &[Term]) {
            let mut seen = [false; 79];
            for term in table {
                let index = usize::from(term.n);
                assert!(
                    index >= 1 && index <= table.len(),
                    "term {} out of range",
                    term.n
                );
                assert!(!seen[index], "term {} listed twice", term.n);
                seen[index] = true;
            }
        }
        check(&ECCENTRICITY);
        check(&OBLIQUITY);
        check(&PRECESSION);
    }

    /// The tables are listed largest amplitude first, which is how the
    /// paper prints them and the file keeps them.
    #[test]
    fn every_table_is_sorted_by_falling_amplitude() {
        for table in [&ECCENTRICITY[..], &OBLIQUITY[..], &PRECESSION[..]] {
            for pair in table.windows(2) {
                assert!(
                    hc_core::math::abs(pair[0].amplitude) >= hc_core::math::abs(pair[1].amplitude),
                    "terms {} and {} are out of order",
                    pair[0].n,
                    pair[1].n
                );
            }
        }
    }

    #[test]
    fn every_phase_is_a_bearing_and_every_rate_positive() {
        for term in ECCENTRICITY.iter().chain(&OBLIQUITY).chain(&PRECESSION) {
            assert!((0.0..360.0).contains(&term.phase), "term {}", term.n);
            assert!(term.rate > 0.0, "term {}", term.n);
        }
    }
}
