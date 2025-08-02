use super::mod_error::TimeTemperatureCurveError;
use core::result::Result;

/// Trait for time-temperature curve interpolation strategies.
pub trait TimeTemperatureCurve {
    /// Get the temperature at a given time.
    ///
    /// # Arguments
    ///
    /// * `time` - The time at which to get the temperature.
    ///
    /// # Returns
    ///
    /// The interpolated temperature at the specified time.
    fn temperature_at(&self, time: f64) -> Result<f64, TimeTemperatureCurveError>;
}
