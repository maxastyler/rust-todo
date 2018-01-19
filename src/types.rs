//! Types for the todo file parsing

/// Struct that holds time information, hours and optional minutes
#[derive(Debug, PartialEq)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
}

/// Struct that holds date and time information
#[derive(Debug, PartialEq)]
pub struct DateTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub time: Option<Time>,
}

impl Time {
    /// Find the difference between two times
    pub fn diff(&self, ref other: &Time) {

    }
}
