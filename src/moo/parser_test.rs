use moo::{parse_program, parse_program_from_string, parse_param, MooParseError};
use vm::Param;

#[test]
fn these_should_parse_into_float_params() {
    if let Ok(Param::FConstant(f)) = parse_param("1.045f") {
        assert_eq!(f, 1.045f64);
    } else {
        panic!();
    };

    if let Ok(Param::FConstant(f)) = parse_param("   20.22586f  \n") {
        assert_eq!(f, 20.22586f64);
    } else {
        panic!();
    };

    if let Ok(Param::FConstant(f)) = parse_param("1000f") {
        assert_eq!(f, 1000f64);
    } else {
        panic!();
    }
}

#[ignore]
#[test]
fn invalid_param_test_1() {
    match parse_param("1.1.1254f") {
        Ok(a) => panic!("\"1.1.1254f\" was parsed into {:?}", a),
        Err(e) => {
            match e {
                MooParseError::InvalidParam(_) => {},
                _ => panic!("invalid error {:?} was returned", e),
            };
        },
    };
}

#[test]
fn invalid_param_test_2() {
    if let Ok(a) = parse_param("1.45ff") {
        panic!("\"1.45ff\" was parsed into {:?}", a);
    }
}

#[test]
fn invalid_param_test_3() {
    if let Ok(a) = parse_param("1.53f053") {
        panic!("\"1.53f053\" was parsed into {:?}", a);
    }
}

#[test]
fn invalid_param_test_4() {
    if let Ok(a) = parse_param("R1.5f") {
        panic!("\"R1.5f\" was parsed into {:?}", a);
    }
}

#[test]
fn invalid_param_test_5() {
    if let Ok(a) = parse_param("10.053") {
        panic!("\"10.053\" was parsed into {:?}", a);
    }
}