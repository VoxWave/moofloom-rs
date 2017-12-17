use moo::{parse_program, parse_program_from_string, parse_param};
use vm::Param;
#[test]
fn parse_param_valid_float_test() {
    if let Ok(Param::FConstant(f)) = parse_param("1.045f") {
        assert_eq!(f, 1.045f64);
    }
}