extern crate pam;
extern crate sqlite;
extern crate guid_create;

pub mod traits;
pub mod echo_module;
pub mod authentication_module;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
