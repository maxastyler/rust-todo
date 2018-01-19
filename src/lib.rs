//! Library for parsing todo files

#[macro_use]
extern crate nom;

pub mod types;
use types::Datetime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
