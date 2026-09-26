//! The named models of ΔT outside the USNO's tables.
//!
//! Where the USNO's observations and predictions stop, ΔT is whatever a fit
//! to the historical record says, and the fits disagree. Two are carried
//! here, each under its own name, and [`crate::time::delta_t_with`] takes
//! one explicitly:
//!
//! | Model | Span | Source |
//! |---|---|---|
//! | [`ESPENAK_MEEUS_2006`] | any year: 15 polynomials over −1999 to +3000, fitted over −500 to +2150, and the parabola `−20 + 32u²`, `u = (year − 1820)/100`, beyond | Espenak and Meeus, *Five Millennium Canon of Solar Eclipses*, NASA/TP-2006-214141 (`espenak-meeus-2006`) |
//! | [`MORRISON_STEPHENSON_2021`] | −720.0 to 2019.0, and nothing outside | Morrison, Stephenson, Hohenkerk and Zawilski, "Addendum 2020 to 'Measurement of the Earth's rotation: 720 BC to AD 2015'", *Proc. R. Soc. A* 477 (2021) 20200776, Table S15 v. 2020 (`morrison2021`), which revises Stephenson, Morrison and Hohenkerk, *Proc. R. Soc. A* 472 (2016) 20160404 (`stephenson2016`) |
//!
//! # Which is the default, and why
//!
//! [`crate::time::delta_t`], and with it [`crate::time::dynamical_time`],
//! [`crate::time::universal_time`] and every series in this crate that
//! converts between them, uses [`ESPENAK_MEEUS_2006`]. It is the one model
//! here that answers for every year: a solar term or a new moon in 3000
//! BCE or 3000 CE has to get *some* ΔT, and the Morrison–Stephenson spline
//! refuses outside −720…2019 by construction. It is a default for that
//! reason and not because it is the better fit: over the span both cover,
//! a caller who wants the later analysis of the eclipse and occultation
//! record asks for [`MORRISON_STEPHENSON_2021`] by name.
//!
//! The list is data, not an enum (ADR 0007): a later fit is a new entry
//! under its own identifier, and the earlier ones stay.

/// A named ΔT model: a fit to the historical record, valid over a stated
/// span, with its source.
#[derive(Debug, Clone, Copy)]
pub struct DeltaTModel {
    /// A stable identifier, lowercase and hyphenated.
    pub id: &'static str,
    /// The English name, naming the authors and the version.
    pub english_name: &'static str,
    /// The first decimal year the model answers for.
    pub first_year: f64,
    /// The last decimal year the model answers for.
    pub last_year: f64,
    /// Where the model came from.
    pub source: &'static str,
    evaluate: fn(f64) -> f64,
}

impl DeltaTModel {
    /// ΔT = TT − UT1 in seconds for a decimal Gregorian year, or `None`
    /// outside [`Self::first_year`]`..=`[`Self::last_year`].
    #[must_use]
    pub fn seconds(&self, year: f64) -> Option<f64> {
        (self.first_year..=self.last_year)
            .contains(&year)
            .then(|| (self.evaluate)(year))
    }
}

impl PartialEq for DeltaTModel {
    /// Two models are the same when they carry the same identifier.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DeltaTModel {}

hc_core::catalogue! {
    type: DeltaTModel,
    id: |model| model.id,
    provenance: |model| model.source,
    tests: delta_t_model_catalogue,

    /// Every ΔT model this crate carries.
    pub const DELTA_T_MODELS;

    /// The model with this identifier.
    pub fn by_id;

    entries: {
        /// Espenak and Meeus 2006, the polynomials of the NASA *Five
        /// Millennium Canon* with the Morrison–Stephenson 2004 parabola
        /// beyond them; see [`crate::time::delta_t_polynomial`].
        pub const ESPENAK_MEEUS_2006 = DeltaTModel {
            id: "espenak-meeus-2006",
            english_name: "Espenak and Meeus 2006",
            first_year: f64::NEG_INFINITY,
            last_year: f64::INFINITY,
            source: "F. Espenak and J. Meeus, Five Millennium Canon of Solar Eclipses: -1999 to \
                     +3000, NASA/TP-2006-214141 (2006), \"Polynomial Expressions for Delta T\"",
            evaluate: crate::time::delta_t_polynomial,
        };

        /// Morrison, Stephenson, Hohenkerk and Zawilski 2021: the cubic
        /// spline of their Table S15, version 2020.
        pub const MORRISON_STEPHENSON_2021 = DeltaTModel {
            id: "morrison-stephenson-2021",
            english_name: "Morrison, Stephenson, Hohenkerk and Zawilski 2021 (Table S15, v. 2020)",
            first_year: -720.0,
            last_year: 2019.0,
            source: "L. V. Morrison, F. R. Stephenson, C. Y. Hohenkerk and M. Zawilski, Addendum \
                     2020 to 'Measurement of the Earth's rotation: 720 BC to AD 2015', Proc. R. \
                     Soc. A 477 (2021) 20200776, Table S15 v. 2020",
            evaluate: morrison_stephenson_2021,
        };
    }
}

/// Table S15, v. 2020: for each row `K_i`, `K_{i+1}`, `a_0`…`a_3`.
///
/// Transcribed from the file `Table-S15.2020.txt` that HM Nautical Almanac
/// Office publishes at <http://astro.ukho.gov.uk/nao/lvm/>; that server
/// answered 503 on 2026-09-26, and the file was read that day as the copy
/// in the Skyfield repository,
/// <https://github.com/skyfielders/python-skyfield/blob/master/Table-S15.2020.txt>.
/// For `K_i ≤ Y ≤ K_{i+1}`, `t = (Y − K_i)/(K_{i+1} − K_i)` and
/// `ΔT = a_0 + a_1 t + a_2 t² + a_3 t³` seconds, as the file states.
const TABLE_S15_2020: [(f64, f64, f64, f64, f64, f64); 58] = [
    (-720.0, -100.0, 20371.848, -9999.586, 776.247, 409.160),
    (-100.0, 400.0, 11557.668, -5822.270, 1303.151, -503.433),
    (400.0, 1000.0, 6535.116, -5671.519, -298.291, 1085.087),
    (1000.0, 1150.0, 1650.393, -753.210, 184.811, -25.346),
    (1150.0, 1300.0, 1056.647, -459.628, 108.771, -24.641),
    (1300.0, 1500.0, 681.149, -421.345, 61.953, -29.414),
    (1500.0, 1600.0, 292.343, -192.841, -6.572, 16.197),
    (1600.0, 1650.0, 109.127, -78.697, 10.505, 3.018),
    (1650.0, 1720.0, 43.952, -68.089, 38.333, -2.127),
    (1720.0, 1800.0, 12.068, 2.507, 41.731, -37.939),
    (1800.0, 1810.0, 18.367, -3.481, -1.126, 1.918),
    (1810.0, 1820.0, 15.678, 0.021, 4.629, -3.812),
    (1820.0, 1830.0, 16.516, -2.157, -6.806, 3.250),
    (1830.0, 1840.0, 10.804, -6.018, 2.944, -0.096),
    (1840.0, 1850.0, 7.634, -0.416, 2.658, -0.539),
    (1850.0, 1855.0, 9.338, 1.642, 0.261, -0.883),
    (1855.0, 1860.0, 10.357, -0.486, -2.389, 1.558),
    (1860.0, 1865.0, 9.040, -0.591, 2.284, -2.477),
    (1865.0, 1870.0, 8.255, -3.456, -5.148, 2.720),
    (1870.0, 1875.0, 2.371, -5.593, 3.011, -0.914),
    (1875.0, 1880.0, -1.126, -2.314, 0.269, -0.039),
    (1880.0, 1885.0, -3.210, -1.893, 0.152, 0.563),
    (1885.0, 1890.0, -4.388, 0.101, 1.842, -1.438),
    (1890.0, 1895.0, -3.884, -0.531, -2.474, 1.871),
    (1895.0, 1900.0, -5.017, 0.134, 3.138, -0.232),
    (1900.0, 1905.0, -1.977, 5.715, 2.443, -1.257),
    (1905.0, 1910.0, 4.923, 6.828, -1.329, 0.720),
    (1910.0, 1915.0, 11.142, 6.330, 0.831, -0.825),
    (1915.0, 1920.0, 17.479, 5.518, -1.643, 0.262),
    (1920.0, 1925.0, 21.617, 3.020, -0.856, 0.008),
    (1925.0, 1930.0, 23.789, 1.333, -0.831, 0.127),
    (1930.0, 1935.0, 24.418, 0.052, -0.449, 0.142),
    (1935.0, 1940.0, 24.164, -0.419, -0.022, 0.702),
    (1940.0, 1945.0, 24.426, 1.645, 2.086, -1.106),
    (1945.0, 1950.0, 27.050, 2.499, -1.232, 0.614),
    (1950.0, 1953.0, 28.932, 1.127, 0.220, -0.277),
    (1953.0, 1956.0, 30.002, 0.737, -0.610, 0.631),
    (1956.0, 1959.0, 30.760, 1.409, 1.282, -0.799),
    (1959.0, 1962.0, 32.652, 1.577, -1.115, 0.507),
    (1962.0, 1965.0, 33.621, 0.868, 0.406, 0.199),
    (1965.0, 1968.0, 35.093, 2.275, 1.002, -0.414),
    (1968.0, 1971.0, 37.956, 3.035, -0.242, 0.202),
    (1971.0, 1974.0, 40.951, 3.157, 0.364, -0.229),
    (1974.0, 1977.0, 44.244, 3.199, -0.323, 0.172),
    (1977.0, 1980.0, 47.291, 3.069, 0.193, -0.192),
    (1980.0, 1983.0, 50.361, 2.878, -0.384, 0.081),
    (1983.0, 1986.0, 52.936, 2.354, -0.140, -0.165),
    (1986.0, 1989.0, 54.984, 1.577, -0.637, 0.448),
    (1989.0, 1992.0, 56.373, 1.648, 0.708, -0.276),
    (1992.0, 1995.0, 58.453, 2.235, -0.121, 0.110),
    (1995.0, 1998.0, 60.678, 2.324, 0.210, -0.313),
    (1998.0, 2001.0, 62.898, 1.804, -0.729, 0.109),
    (2001.0, 2004.0, 64.083, 0.674, -0.402, 0.199),
    (2004.0, 2007.0, 64.553, 0.466, 0.194, -0.017),
    (2007.0, 2010.0, 65.197, 0.804, 0.144, -0.084),
    (2010.0, 2013.0, 66.061, 0.839, -0.109, 0.128),
    (2013.0, 2016.0, 66.920, 1.007, 0.277, -0.095),
    (2016.0, 2019.0, 68.109, 1.277, -0.007, -0.139),
];

/// The spline of [`TABLE_S15_2020`], for a year inside its span.
fn morrison_stephenson_2021(year: f64) -> f64 {
    let index = TABLE_S15_2020
        .partition_point(|row| row.1 < year)
        .min(TABLE_S15_2020.len() - 1);
    let (start, end, a0, a1, a2, a3) = TABLE_S15_2020[index];
    let t = (year - start) / (end - start);
    a0 + t * (a1 + t * (a2 + t * a3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_spline_returns_each_rows_constant_at_its_knot() {
        // At `K_i`, `t = 0` and ΔT is `a_0`: 1900.0 is row 26, 2016.0 row 58.
        let model = by_id("morrison-stephenson-2021").unwrap();
        assert!((model.seconds(1900.0).unwrap() - -1.977).abs() < 1e-9);
        assert!((model.seconds(2016.0).unwrap() - 68.109).abs() < 1e-9);
        assert!((model.seconds(-720.0).unwrap() - 20_371.848).abs() < 1e-9);
    }

    #[test]
    fn the_spline_is_continuous_at_its_knots() {
        // Each row at `t = 1` meets the next row's `a_0`, to the rounding of
        // the three printed decimals.
        for pair in TABLE_S15_2020.windows(2) {
            let (_, end, a0, a1, a2, a3) = pair[0];
            let next = pair[1];
            assert!((end - next.0).abs() < 1e-9, "rows do not abut at {end}");
            let at_end = a0 + a1 + a2 + a3;
            assert!(
                (at_end - next.2).abs() < 0.01,
                "{end}: {at_end} against {}",
                next.2
            );
        }
    }

    #[test]
    fn the_default_is_espenak_meeus_by_name() {
        for year in [
            -3000.0, -500.0, 1000.0, 1900.0, 2024.0, 2030.0, 2100.0, 4000.0,
        ] {
            assert_eq!(
                crate::time::delta_t_for_year_with(year, &ESPENAK_MEEUS_2006),
                Some(crate::time::delta_t_for_year(year)),
                "{year}"
            );
        }
        // Inside the USNO's tables the model is not consulted.
        assert_eq!(
            crate::time::delta_t_for_year_with(2024.0, &MORRISON_STEPHENSON_2021),
            crate::time::delta_t_tabulated(2024.0)
        );
        assert_eq!(
            crate::time::delta_t_for_year_with(2100.0, &MORRISON_STEPHENSON_2021),
            None
        );
    }

    #[test]
    fn the_spline_refuses_outside_its_span() {
        assert_eq!(MORRISON_STEPHENSON_2021.seconds(-720.1), None);
        assert_eq!(MORRISON_STEPHENSON_2021.seconds(2019.1), None);
        assert!(ESPENAK_MEEUS_2006.seconds(-10_000.0).is_some());
        assert!(ESPENAK_MEEUS_2006.seconds(10_000.0).is_some());
    }

    #[test]
    fn the_spline_meets_the_usno_observations_where_both_exist() {
        // Over the years both cover, the spline, a smooth fit, and the
        // USNO's observed table stay within a quarter of a second; the worst
        // quarter-year step is 1974.0, at 0.24 s.
        let mut worst: f64 = 0.0;
        let mut year = 1974.0;
        while year <= 2019.0 {
            let observed = crate::time::delta_t_tabulated(year).unwrap();
            let spline = MORRISON_STEPHENSON_2021.seconds(year).unwrap();
            worst = worst.max((observed - spline).abs());
            year += 0.25;
        }
        assert!(worst < 0.25, "worst difference {worst}");
    }

    #[test]
    fn the_two_models_disagree_in_the_ancient_past() {
        // The reason there are two names: at −500 (501 BCE) they differ by
        // 264 s, 17 204 s against 16 940 s.
        let em = ESPENAK_MEEUS_2006.seconds(-500.0).unwrap();
        let ms = MORRISON_STEPHENSON_2021.seconds(-500.0).unwrap();
        assert!((em - ms - 264.0).abs() < 1.0, "EM {em}, MS {ms}");
    }
}
