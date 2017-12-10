pub mod vm;
pub mod moo;

use vm::Command;

type Program = Vec<Command>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
