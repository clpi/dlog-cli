#[macro_use]
extern crate pest_derive;
extern crate pest;

pub mod parse;
pub mod ast;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
