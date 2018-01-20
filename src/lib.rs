//! Library for parsing todo files

#[macro_use]
extern crate nom;

pub mod types;
pub mod parser;

use types::DateTime;

#[cfg(test)]
mod tests {
    use nom::IResult::{Done, Error};
    use nom::ErrorKind::{Digit, Tag};
    use parser;

    #[test]
    fn get_num() {
        assert_eq!(parser::get_num(b"1923//"), Done(&b"//"[..], 1923))
    }

    #[test]
    fn get_num_err() {
        assert_eq!(parser::get_num(b"//231"), Error(Digit));
    }

    #[test]
    fn get_time() {
        use types::Time;
        assert_eq!(parser::get_time(b"23:41//123"), Done(&b"//123"[..], Time{hours: 23, minutes: 41}));
    }

    #[test]
    fn get_date() {
        use types::DateTime;
        assert_eq!(parser::get_datetime(b"2014/12-04//2"), Done(&b"//2"[..], DateTime{year: 2014, month: 12, day: 4, time: None}));
    }

    #[test]
    fn get_datetime() {
        use types::{DateTime, Time};
        assert_eq!(parser::get_datetime(b"2014/12-04T2312//2"), Done(&b"//2"[..], DateTime{year: 2014, month: 12, day: 4, time: Some(Time{hours: 23, minutes: 12})}));
    }

    #[test]
    fn todo_box_true() {
        assert_eq!(parser::todo_box(b"[ ]"), Done(&b""[..], false));
        assert_eq!(parser::todo_box(b"[]"), Done(&b""[..], false));
        assert_eq!(parser::todo_box(b"[x]"), Done(&b""[..], true));
        assert_eq!(parser::todo_box(b"[X]"), Done(&b""[..], true));
        assert_eq!(parser::todo_box(b"[a]"), Error(Tag));
    }

    #[test]
    fn parse_item_correctly() {
        use types::Item;
        use nom::rest;
        named!(test_p<u32>, alt_complete!(value!(2, tag!("")) | value!(3, rest)));
        println!("{:?}", test_p(b"3"));
        assert_eq!(parser::parse_item(b"Hello there ;;"), Done(&b""[..], 
                                                   Item{
                                                       todo: Some(true), 
                                                       text: String::new(),
                                                       time: None,
                                                       description: None,
                                                       children: vec!(),
                                                   }));
    }
}
