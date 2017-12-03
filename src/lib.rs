pub mod vm;
pub mod moo;

type Program = Vec<Command>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
