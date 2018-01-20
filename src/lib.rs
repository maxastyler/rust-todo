//! Library for parsing todo files

#[macro_use]
extern crate nom;

pub mod types;
pub mod parser;

use types::{DateTime, Item};
use parser::{read_lines_and_parse};
use std::rc::Rc;
use std::cell::RefCell;

pub fn structure_todo(mut v: Vec<(usize, Item)>) -> Vec<Rc<RefCell<Item>>> { 
    vec!(Rc::from(RefCell::new(v[0].1.clone()))) 
}

#[cfg(test)]
mod tests {
    use nom::IResult::{Done, Error};
    use nom::ErrorKind::{Digit, Tag, Many1, Alt};
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
    fn parse_minimal_correctly() {
        use types::Item;
        assert_eq!(parser::parse_item(b";;"), Done(&b""[..], 
                                                   Item{
                                                       todo: None, 
                                                       text: String::new(),
                                                       time: None,
                                                       description: None,
                                                       children: vec!(),
                                                   }));
    }
    #[test]
    fn parse_maximal_correctly() {
        use types::{DateTime, Time, Item};
        assert_eq!(parser::parse_item(b"[x] Do washing up;;:2019/12/13T1230:\nPunydonky"), Done(&b""[..], 
                                                   Item{
                                                       todo: Some(true), 
                                                       text: String::from("Do washing up"),
                                                       time: Some(DateTime::new(
                                                               2019, 12, 13, 12, 30)),
                                                       description: Some(String::from("Punydonky")),
                                                       children: vec!(),
                                                   }));
    }

    #[test]
    fn parse_dashes() {
        assert_eq!(parser::count_dash(b"-------a--"), Done(&b"-a--"[..], 3));
        assert_eq!(parser::count_dash(b"a--"), Error(Many1));
    }

    #[test]
    fn parse_dashes_and_line() {
        assert_eq!(parser::match_line(b"-- testing this line"), Done(&b""[..], (1, &b" testing this line"[..])));
        assert_eq!(parser::match_line(b"-- testing this line\n--"), Done(&b"--"[..], (1, &b" testing this line"[..])));
    }

    #[test]
    fn parse_item_tup() {
        use types::Item;
        assert_eq!(parser::convert_item_tup((2, b";;")), Some((2, Item::new_default())));
        assert_eq!(parser::convert_item_tup((2, b"")), None);
    }

    #[test]
    fn parse_lines() {
        assert_eq!(parser::match_lines(b"-- testing this line\n---test line 2"), Done(&b""[..], vec!((1, &b" testing this line"[..]), (1, &b"-test line 2"[..]))));
        assert_eq!(parser::match_lines(b"testing this line\n---test line 2"), Error(Alt));
        assert_eq!(parser::match_lines(b"    "), Done(&b""[..], vec!()));
    }

    #[test]
    fn convert_vector_items() {
        use types::Item;
        let items = vec!(
            (2, &b";;"[..]),
            (3, &b";;"[..]),
            );
        let items_wrong = vec!(
            (2, &b";"[..]),
            (3, &b""[..]),
            );
        let converted = vec!(
            (2, Item::new_default()),
            (3, Item::new_default()),
            );
        assert_eq!(parser::convert_vec_items(items), converted);
        assert_eq!(parser::convert_vec_items(items_wrong), vec!());
    }
    
    #[test]
    fn parse_and_convert_lines() {
        use types::Item;
        let lines = b"--;;\n----;;";
        assert_eq!(parser::read_lines_and_parse(lines), Done(&b""[..], vec!((1, Item::new_default()), (2, Item::new_default()))));
    }

    #[test]
    fn test_import() {
        let included = include_bytes!("test_todo");
        assert_eq!(parser::read_lines_and_parse(included), Done(&b""[..], vec!()));
    }
}
