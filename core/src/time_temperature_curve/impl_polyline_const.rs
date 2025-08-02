use super::mod_error::TimeTemperatureCurveError;
use super::interface::TimeTemperatureCurve;

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
            if i > 0 && (points[i].0 - points[i - 1].0).abs() < f64::EPSILON {
                panic!("DuplicateTime: Duplicate time value in polyline");
            }
            i += 1;
        }
        Self { points }
    }
}

impl<const N: usize> TimeTemperatureCurve for ImplPolylineConst<N> {
    fn temperature_at(&self, time: f64) -> Result<f64, TimeTemperatureCurveError> {
        if time.is_nan() || time.is_infinite() {
            return Err(TimeTemperatureCurveError::InvalidValue);
        }
        if N == 0 {
            return Ok(0.0);
        }
        if time <= self.points[0].0 {
            return Ok(self.points[0].1);
        }
        if time >= self.points[N - 1].0 {
            return Ok(self.points[N - 1].1);
        }
        // Binary search for const arrays
        let mut left = 0;
        let mut right = N;
        while left < right {
            let mid = (left + right) / 2;
            let t = self.points[mid].0;
            if t < time {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let idx = left;
        if idx < N && (self.points[idx].0 - time).abs() < f64::EPSILON {
            return Ok(self.points[idx].1);
        }
        let (t0, temp0) = self.points[idx - 1];
        let (t1, temp1) = self.points[idx];
        let ratio = (time - t0) / (t1 - t0);
        Ok(temp0 + ratio * (temp1 - temp0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::mod_error::TimeTemperatureCurveError;

    const VALID: ImplPolylineConst<3> = ImplPolylineConst::from_array([
        (0.0, 10.0),
        (5.0, 20.0),
        (10.0, 30.0),
    ]);

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
