use core::panic;
use core::prelude::rust_2024::*;
use core::result::Result;

use super::interface::TimeTemperatureCurve;
use super::polyline_shared::polyline_temperature_at;

/// Polyline interpolation strategy for time-temperature curve, const-generic version for compile-time usage.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplPolylineConst<const N: usize> {
    pub points: [(f64, f64); N],
}

impl<const N: usize> ImplPolylineConst<N> {
    pub const fn from_array(points: [(f64, f64); N]) -> Self {
        let mut i = 0;
        while i < N {
            let (t, temp) = points[i];
            if !(t.is_finite() && temp.is_finite()) {
                panic!("InvalidValue: NaN or infinite value in polyline");
            }
            // Duplicate time check using iterators is not possible in const fn yet, so keep the loop
            if i > 0 && (points[i].0 - points[i - 1].0).abs() < f64::EPSILON {
                panic!("DuplicateTime: Duplicate time value in polyline");
            }
            i += 1;
        }
        Self { points }
    }
}

impl<const N: usize> TimeTemperatureCurve for ImplPolylineConst<N> {
    fn temperature_at(
        &self,
        time: f64,
    ) -> Result<f64, super::mod_error::TimeTemperatureCurveError> {
        polyline_temperature_at(&self.points, time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: ImplPolylineConst<3> =
        ImplPolylineConst::from_array([(0.0, 10.0), (5.0, 20.0), (10.0, 30.0)]);

    #[test]
    fn test_valid_const_polyline() {
        assert_eq!(VALID.temperature_at(-1.0), Ok(10.0));
        assert_eq!(VALID.temperature_at(0.0), Ok(10.0));
        assert_eq!(VALID.temperature_at(2.5), Ok(15.0));
        assert_eq!(VALID.temperature_at(5.0), Ok(20.0));
        assert_eq!(VALID.temperature_at(7.5), Ok(25.0));
        assert_eq!(VALID.temperature_at(10.0), Ok(30.0));
        assert_eq!(VALID.temperature_at(15.0), Ok(30.0));
    }

    // The following compile_fail tests are for documentation only; they will fail to compile if uncommented.
    // To actually test compile-time errors, use trybuild or UI tests.
    /*
    const INVALID_NAN: ImplPolylineConst<2> = ImplPolylineConst::from_array([
        (0.0, f64::NAN),
        (1.0, 2.0),
    ]);
    const INVALID_DUP: ImplPolylineConst<2> = ImplPolylineConst::from_array([
        (0.0, 1.0),
        (0.0, 2.0),
    ]);
    */
}
