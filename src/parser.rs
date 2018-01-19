//! Parser contains all the parsing logic for the file

use types::{DateTime, Time};
use nom::{be_u8, be_u32, digit, IResult};
use std::str::{from_utf8, FromStr};

/// Get a number from an array of u8
named!(pub get_num<u32>, 
    map_res!(
        map_res!(digit,
            from_utf8
        ),
        FromStr::from_str
    ) 
);

/// Get a time, from a string of hh:mm, colon-optional!
named!(pub get_time<Time>, do_parse!(
        h: flat_map!(take!(2), get_num) >>
        opt!(tag!(":")) >>
        m: flat_map!(take!(2), get_num) >>
        (Time{hours: h as u8, minutes: m as u8})
        ));

/// Get a datetime struct from a string, dates separated by / or -, and optional time which can be
/// separated by t/T
named!(pub get_datetime<DateTime>, do_parse!(
        y: flat_map!(take!(4), get_num) >>
        one_of!("-/") >>
        m: flat_map!(take!(2), get_num) >>
        one_of!("-/") >>
        d: flat_map!(take!(2), get_num) >>
        opt!(one_of!("tT")) >>
        t: opt!(get_time) >>
        (DateTime{year: y, month: m as u8, day: d as u8, time: t})
        ));
