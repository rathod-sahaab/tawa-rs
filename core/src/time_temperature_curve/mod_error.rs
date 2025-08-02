/// Error types for TimeTemperatureCurve operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeTemperatureCurveError {
    /// Attempted to create a curve with no points.
    EmptyPoints,
}

impl std::fmt::Display for TimeTemperatureCurveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeTemperatureCurveError::EmptyPoints => write!(f, "TimeTemperatureCurve cannot be created with empty points"),
        }
    }
}

impl std::error::Error for TimeTemperatureCurveError {}
