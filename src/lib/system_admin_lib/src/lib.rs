#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

pub mod messages;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
