use super::mod_error::TimeTemperatureCurveError;

/// Shared polyline temperature interpolation for slices.
pub fn polyline_temperature_at(points: &[(f64, f64)], time: f64) -> Result<f64, TimeTemperatureCurveError> {
    if time.is_nan() || time.is_infinite() {
        return Err(TimeTemperatureCurveError::InvalidValue);
    }
    if points.is_empty() {
        return Ok(0.0);
    }
    if time <= points[0].0 {
        return Ok(points[0].1);
    }
    if time >= points[points.len() - 1].0 {
        return Ok(points[points.len() - 1].1);
    }
    match points.binary_search_by(|(t, _)| t.partial_cmp(&time).unwrap()) {
        Ok(idx) => Ok(points[idx].1),
        Err(idx) => {
            let (t0, temp0) = points[idx - 1];
            let (t1, temp1) = points[idx];
            let ratio = (time - t0) / (t1 - t0);
            Ok(temp0 + ratio * (temp1 - temp0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_points() {
        assert_eq!(polyline_temperature_at(&[], 1.0), Ok(0.0));
    }

    #[test]
    fn test_single_point() {
        let pts = [(0.0, 42.0)];
        assert_eq!(polyline_temperature_at(&pts, -10.0), Ok(42.0));
        assert_eq!(polyline_temperature_at(&pts, 0.0), Ok(42.0));
        assert_eq!(polyline_temperature_at(&pts, 10.0), Ok(42.0));
    }

    #[test]
    fn test_interpolation() {
        let pts = [(0.0, 20.0), (10.0, 100.0), (20.0, 50.0)];
        assert_eq!(polyline_temperature_at(&pts, -5.0), Ok(20.0));
        assert_eq!(polyline_temperature_at(&pts, 0.0), Ok(20.0));
        assert_eq!(polyline_temperature_at(&pts, 5.0), Ok(60.0));
        assert_eq!(polyline_temperature_at(&pts, 10.0), Ok(100.0));
        assert_eq!(polyline_temperature_at(&pts, 15.0), Ok(75.0));
        assert_eq!(polyline_temperature_at(&pts, 20.0), Ok(50.0));
        assert_eq!(polyline_temperature_at(&pts, 25.0), Ok(50.0));
    }

    #[test]
    fn test_invalid_time() {
        let pts = [(0.0, 1.0), (1.0, 2.0)];
        assert!(matches!(polyline_temperature_at(&pts, f64::NAN), Err(TimeTemperatureCurveError::InvalidValue)));
        assert!(matches!(polyline_temperature_at(&pts, f64::INFINITY), Err(TimeTemperatureCurveError::InvalidValue)));
        assert!(matches!(polyline_temperature_at(&pts, f64::NEG_INFINITY), Err(TimeTemperatureCurveError::InvalidValue)));
    }
}
