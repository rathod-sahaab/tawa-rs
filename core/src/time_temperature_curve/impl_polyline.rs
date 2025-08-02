use super::mod_error::TimeTemperatureCurveError;
// use super::interface::TimeTemperatureCurve;
use super::interface::TimeTemperatureCurve;

/// Polyline interpolation strategy for time-temperature curve.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplPolyline {
    pub points: Vec<(f64, f64)>,
}

impl ImplPolyline {
    pub fn new(mut points: Vec<(f64, f64)>) -> Result<Self, TimeTemperatureCurveError> {
        if points.is_empty() {
            return Err(TimeTemperatureCurveError::EmptyPoints);
        }
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Ok(ImplPolyline { points })
    }
}

impl TimeTemperatureCurve for ImplPolyline {
    fn temperature_at(&self, time: f64) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        if time <= self.points[0].0 {
            return self.points[0].1;
        }
        if time >= self.points[self.points.len() - 1].0 {
            return self.points[self.points.len() - 1].1;
        }
        for i in 0..self.points.len() - 1 {
            let (t0, temp0) = self.points[i];
            let (t1, temp1) = self.points[i + 1];
            if time >= t0 && time <= t1 {
                let ratio = (time - t0) / (t1 - t0);
                return temp0 + ratio * (temp1 - temp0);
            }
        }
        self.points[self.points.len() - 1].1
    }
}
