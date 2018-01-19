//! Types for the todo file parsing

/// Struct that holds time information, hours and optional minutes
pub struct Time {
    hours: u8,
    minutes: Option<u8>,
}

/// Struct that holds date and time information
pub struct Datetime {
    year: u32,
    month: u8,
    day: u8,
    time: Option<Time>,
}

impl Time {
    /// Find the difference between two times
    pub fn diff(&self, ref other: &Time) {

    }
}
