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

impl std::fmt::Display for TimeTemperatureCurveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeTemperatureCurveError::EmptyPoints => write!(f, "TimeTemperatureCurve cannot be created with empty points"),
            TimeTemperatureCurveError::DuplicateTime => write!(f, "TimeTemperatureCurve cannot have duplicate time values"),
            TimeTemperatureCurveError::InvalidValue => write!(f, "TimeTemperatureCurve cannot have NaN or infinite values"),
        }
    }
}

impl std::error::Error for TimeTemperatureCurveError {}
