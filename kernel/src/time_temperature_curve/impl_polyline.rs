use super::interface::TimeTemperatureCurve;
use super::mod_error::TimeTemperatureCurveError;
use super::polyline_shared::polyline_temperature_at;
extern crate alloc;
use alloc::vec::Vec;
use core::result::Result;

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
        if points
            .iter()
            .any(|(t, temp)| t.is_nan() || temp.is_nan() || t.is_infinite() || temp.is_infinite())
        {
            return Err(TimeTemperatureCurveError::InvalidValue);
        }
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        if points
            .windows(2)
            .any(|w| (w[0].0 - w[1].0).abs() < f64::EPSILON)
        {
            return Err(TimeTemperatureCurveError::DuplicateTime);
        }
        Ok(ImplPolyline { points })
    }
}

impl TimeTemperatureCurve for ImplPolyline {
    fn temperature_at(&self, time: f64) -> Result<f64, TimeTemperatureCurveError> {
        polyline_temperature_at(&self.points, time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use alloc::vec;

    #[test]
    fn test_empty_points() {
        let result = ImplPolyline::new(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_single_point() {
        let result = ImplPolyline::new(vec![(0.0, 42.0)]).unwrap();
        assert_eq!(result.temperature_at(-10.0), Ok(42.0));
        assert_eq!(result.temperature_at(0.0), Ok(42.0));
        assert_eq!(result.temperature_at(10.0), Ok(42.0));
    }

    #[test]
    fn test_interpolation() {
        let curve = ImplPolyline::new(vec![(0.0, 20.0), (10.0, 100.0), (20.0, 50.0)]).unwrap();
        assert_eq!(curve.temperature_at(-5.0), Ok(20.0));
        assert_eq!(curve.temperature_at(0.0), Ok(20.0));
        assert_eq!(curve.temperature_at(5.0), Ok(60.0));
        assert_eq!(curve.temperature_at(10.0), Ok(100.0));
        assert_eq!(curve.temperature_at(15.0), Ok(75.0));
        assert_eq!(curve.temperature_at(20.0), Ok(50.0));
        assert_eq!(curve.temperature_at(25.0), Ok(50.0));
    }

    #[test]
    fn test_duplicate_time_error() {
        let result = ImplPolyline::new(vec![(0.0, 1.0), (0.0, 2.0)]);
        assert!(matches!(
            result,
            Err(TimeTemperatureCurveError::DuplicateTime)
        ));
    }

    #[test]
    fn test_invalid_value_error() {
        let result = ImplPolyline::new(vec![(0.0, 1.0), (f64::NAN, 2.0)]);
        assert!(matches!(
            result,
            Err(TimeTemperatureCurveError::InvalidValue)
        ));
        let result = ImplPolyline::new(vec![(0.0, 1.0), (f64::INFINITY, 2.0)]);
        assert!(matches!(
            result,
            Err(TimeTemperatureCurveError::InvalidValue)
        ));
    }

    #[test]
    fn test_invalid_time_error() {
        let curve = ImplPolyline::new(vec![(0.0, 1.0), (1.0, 2.0)]).unwrap();
        assert!(matches!(
            curve.temperature_at(f64::NAN),
            Err(TimeTemperatureCurveError::InvalidValue)
        ));
        assert!(matches!(
            curve.temperature_at(f64::INFINITY),
            Err(TimeTemperatureCurveError::InvalidValue)
        ));
        assert!(matches!(
            curve.temperature_at(f64::NEG_INFINITY),
            Err(TimeTemperatureCurveError::InvalidValue)
        ));
    }
}
