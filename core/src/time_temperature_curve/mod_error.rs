/// Error types for TimeTemperatureCurve operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeTemperatureCurveError {
    /// Attempted to create a curve with no points.
    EmptyPoints,
    /// Duplicate time values found in input.
    DuplicateTime,
    /// Invalid (NaN or infinite) time or temperature value found in input.
    InvalidValue,
}

// Removed std::fmt::Display and std::error::Error for no_std compatibility.
