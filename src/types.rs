//! Types for the todo file parsing

use std::rc::Rc;
use std::cell::RefCell;

/// Struct that holds time information, hours and optional minutes
#[derive(Debug, PartialEq, Clone)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
}

/// Struct that holds date and time information
#[derive(Debug, PartialEq, Clone)]
pub struct DateTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub time: Option<Time>,
}

/// Struct containing a todo item
#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub todo: Option<bool>,
    pub text: String,
    pub time: Option<DateTime>,
    pub description: Option<String>,
    pub children: Vec<Rc<RefCell<Item>>>,
}

impl Time {
    /// Find the difference between two times
    pub fn diff(&self, ref other: &Time) {

    }
}

impl DateTime {
    pub fn new(y: u32, mo: u8, d: u8, h: u8, m: u8) -> DateTime {
        DateTime {
            year: y,
            month: mo,
            day: d,
            time: Some(Time{ hours: h, minutes: m }),
        }
    }
}

impl Item {
    pub fn new_default() -> Item {
        Item {
            todo: None,
            text: String::new(),
            time: None,
            description: None,
            children: vec!(),
        }
    }

}
