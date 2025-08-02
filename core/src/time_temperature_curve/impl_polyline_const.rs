use super::mod_error::TimeTemperatureCurveError;
use super::interface::TimeTemperatureCurve;

/// Polyline interpolation strategy for time-temperature curve, const-generic version for compile-time usage.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplPolylineConst<const N: usize> {
    pub points: [(f64, f64); N],
}

impl<const N: usize> ImplPolylineConst<N> {
    pub const fn from_array(points: [(f64, f64); N]) -> Result<Self, TimeTemperatureCurveError> {
        let mut i = 0;
        while i < N {
            let (t, temp) = points[i];
            if t.is_nan() || temp.is_nan() || t.is_infinite() || temp.is_infinite() {
                return Err(TimeTemperatureCurveError::InvalidValue);
            }
            if i > 0 && (points[i].0 - points[i - 1].0).abs() < f64::EPSILON {
                return Err(TimeTemperatureCurveError::DuplicateTime);
            }
            i += 1;
        }
        Ok(ImplPolylineConst { points })
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
