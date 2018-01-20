//! Parser contains all the parsing logic for the file

use types::{DateTime, Time, Item};
use nom::{be_u8, be_u32, digit, IResult, non_empty, rest};
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
        t: opt!(preceded!(one_of!("tT"), get_time)) >>
        (DateTime{year: y, month: m as u8, day: d as u8, time: t})
        ));

/// Parse todo, either ([], [ ], [x])
named!(pub todo_box<bool>, 
       map!(delimited!(tag!("["), opt!(one_of!(" xX")), tag!("]")),
       |c| { match c {
            Some('x') | Some('X') => true,
            _ => false,
       }})
       );

/// Parse an item's header text
named!(pub item_head<String>, map_res!(map_res!(take_until_and_consume!(";;"), from_utf8), FromStr::from_str));

/// Parse an item's body text
named!(pub item_body<String>, map_res!(map_res!(rest, from_utf8), FromStr::from_str));

/// Parses a todo list item, fully consisting of:
/// [x] Line awlkdjhlkjhvr v ;; :2016/12/13T13:00:
///  alkwjdhalkfjhoisfh poishpogposugmpoeirug pwoeiug pwoireugpoiusf
named!(pub parse_item<Item>, do_parse!(
        todo: opt!(ws!(todo_box)) >>
        text: item_head >>
        (Item{
            todo: todo, 
            text: text,
            time: None,
            description: None,
            children: vec!(),
        })
        ));
