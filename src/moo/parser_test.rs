use moo::{parse_param, parse_program, parse_program_from_string, MooParseError};
use vm::{Command, Param};

fn valid_float_param_check(param: &str, expected_float: f64) {
    if let Ok(Param::FConstant(f)) = parse_param(param) {
        assert_eq!(f, expected_float);
    } else {
        panic!("{} did not parse into a float", param);
    }
}

#[test]
fn valid_float_param_1() {
    valid_float_param_check("1.045f", 1.045f64);
}

#[test]
fn valid_float_param_2() {
    valid_float_param_check("   20.22586f  \n", 20.22586f64);
}

#[test]
fn valid_float_param_3() {
    valid_float_param_check("1000f", 1000f64);
}

fn invalid_param_check(param: &str) {
    match parse_param(param) {
        Ok(a) => panic!("\"{}\" was parsed into {:?}", param, a),
        Err(e) => {
            match e {
                MooParseError::InvalidParam(_) => {}
                _ => panic!("invalid error {:?} was returned", e),
            };
        }
    };
}

#[test]
fn invalid_param_test_1() {
    invalid_param_check("1.1.1254f");
}

#[test]
fn invalid_param_test_2() {
    invalid_param_check("1.45ff");
}

#[test]
fn invalid_param_test_3() {
    invalid_param_check("1.53f053");
}

#[test]
fn invalid_param_test_4() {
    invalid_param_check("R1.5f");
}

#[test]
fn invalid_param_test_5() {
    invalid_param_check("10.053");
}

#[test]
fn fadd_parsing_test() {
    let source = "fadd 1f 2f R0;";
    let program = parse_program_from_string(source).unwrap();
    assert_eq!(
        program,
        vec![Command::FAdd(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )]
    );
}

#[test]
fn fsub_parsing_test() {
    let source = "fsub 1f 2f R0;";
    let program = parse_program_from_string(source).unwrap();
    assert_eq!(
        program,
        vec![Command::FSub(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )]
    );
}

#[test]
fn fmul_parsing_test() {
    let source = "fmul 1f 2f R0";
    let program = parse_program_from_string(source).unwrap();
    assert_eq!(
        program,
        vec![Command::FMul(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )]
    );
}

#[test]
fn fdiv_parsing_test() {
    let source = "fdiv 1f 2f R0;";
    let program = parse_program_from_string(source).unwrap();
    assert_eq!(
        program,
        vec![Command::FDiv(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )]
    );
}

#[test]
fn load_parsing_test() {
    let source = "load 1f R0;";
    let program = parse_program_from_string(source).unwrap();
    assert_eq!(
        program,
        vec![Command::Load(Param::FConstant(1.), Param::Register(0))]
    );
}

#[test]
fn program_with_all_commands_parsing_test() {
    let source = r#"fadd 1f 2f R0;
    fsub 1f 2f R0
;   fmul 1f 2f R0;
fdiv 1f 2f R0;
load 1f R0;"#;
    let program = parse_program_from_string(source).unwrap();
    assert_eq!(
        program,
        vec![
            Command::FAdd(
                Param::FConstant(1.),
                Param::FConstant(2.),
                Param::Register(0),
            ),
            Command::FSub(
                Param::FConstant(1.),
                Param::FConstant(2.),
                Param::Register(0),
            ),
            Command::FMul(
                Param::FConstant(1.),
                Param::FConstant(2.),
                Param::Register(0),
            ),
            Command::FDiv(
                Param::FConstant(1.),
                Param::FConstant(2.),
                Param::Register(0),
            ),
            Command::Load(Param::FConstant(1.), Param::Register(0)),
        ]
    );
}
