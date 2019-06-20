use std::collections::HashMap;

use crate::moo::{parse_param, parse_program_from_string, MooParseError};
use crate::vm::{Command, Param};
use crate::program::Program;

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
    let expected_program = Program::new(
        vec![Command::FAdd(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )],
        HashMap::new(),
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn fsub_parsing_test() {
    let source = "fsub 1f 2f R0;";
    let program = parse_program_from_string(source).unwrap();
    let expected_program = Program::new(
        vec![Command::FSub(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )],
        HashMap::new(),
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn fmul_parsing_test() {
    let source = "fmul 1f 2f R0";
    let program = parse_program_from_string(source).unwrap();
    let expected_program = Program::new(
        vec![Command::FMul(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )],
        HashMap::new(),
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn fdiv_parsing_test() {
    let source = "fdiv 1f 2f R0;";
    let program = parse_program_from_string(source).unwrap();
    let expected_program = Program::new(
        vec![Command::FDiv(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )],
        HashMap::new(),
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn load_parsing_test() {
    let source = "load 1f R0;";
    let program = parse_program_from_string(source).unwrap();
    let expected_program = Program::new(
        vec![Command::Load(Param::FConstant(1.), Param::Register(0))],
        HashMap::new(),
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn program_with_all_float_arithmetic_operators_and_load_parsing_test() {
    let source = r#"fadd 1f 2f R0;
    fsub 1f 2f R0
;   fmul 1f 2f R0;
fdiv 1f 2f R0;
load 1f R0;"#;
    let program = parse_program_from_string(source).unwrap();
    let expected_program = Program::new(
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
        ],
        HashMap::new(),
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn program_with_all_signed_integer_arithmetic_operators_and_load_parsing_test() {
    let source = r#"iadd 1i 2i R0;
    isub 1i 2i R0
;   imul 1i 2i R0;
idiv 1i 2i R0;
load 1i R0;"#;
    let program = parse_program_from_string(source).unwrap();
    let expected_program = Program::new(
        vec![
            Command::IAdd(
                Param::IConstant(1),
                Param::IConstant(2),
                Param::Register(0),
            ),
            Command::ISub(
                Param::IConstant(1),
                Param::IConstant(2),
                Param::Register(0),
            ),
            Command::IMul(
                Param::IConstant(1),
                Param::IConstant(2),
                Param::Register(0),
            ),
            Command::IDiv(
                Param::IConstant(1),
                Param::IConstant(2),
                Param::Register(0),
            ),
            Command::Load(Param::IConstant(1), Param::Register(0)),
        ],
        HashMap::new(),
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn program_with_all_unsigned_integer_arithmetic_operators_and_load_parsing_test() {
    let source = r#"uadd 1u 2u R0;
    usub 1u 2u R0
;   umul 1u 2u R0;
udiv 1u 2u R0;
load 1u R0;"#;
    let program = parse_program_from_string(source).unwrap();
    let expected_program = Program::new(
        vec![
            Command::UAdd(
                Param::UConstant(1),
                Param::UConstant(2),
                Param::Register(0),
            ),
            Command::USub(
                Param::UConstant(1),
                Param::UConstant(2),
                Param::Register(0),
            ),
            Command::UMul(
                Param::UConstant(1),
                Param::UConstant(2),
                Param::Register(0),
            ),
            Command::UDiv(
                Param::UConstant(1),
                Param::UConstant(2),
                Param::Register(0),
            ),
            Command::Load(Param::UConstant(1), Param::Register(0)),
        ],
        HashMap::new(),
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn label_parsing_test() {
    let source = r#"label1:fadd 1f 2f R0;label2 :
    fsub 1f 2f R0
; label 3:  fmul 1f 2f R0;
fdiv 1f 2f R0;
label5: load 1f R0;"#;
    let program = parse_program_from_string(source).unwrap();
    let mut expected_labels = HashMap::new();
    expected_labels.insert("label1".to_string(), 0);
    expected_labels.insert("label2".to_string(), 1);
    expected_labels.insert("label 3".to_string(), 2);
    expected_labels.insert("label5".to_string(), 4);
    let expected_program = Program::new(
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
        ],
        expected_labels,
    );
    assert_eq!(
        program,
        expected_program,
    );
}

#[test]
fn invalid_label_test() {
    let invalid = "kokeilitko: tätä: fadd 1f 1f R0;";
    match parse_program_from_string(invalid) {
        Ok(_) => panic!("an Err should have been returned but and Ok was returned instead."),
        Err(MooParseError::InvalidLineStructure(_)) => {},
        Err(e) => panic!("an Err was returned but it was the wrong kind. Err returned was {:?}", e),
    }
}

#[test]
fn jump_parsing_janky_but_correct_test() {
    let source = r#"label1: jump label3;
        label2 :     jump    
        label1    ;
        
        label3:jump label2;"#;
    let program = parse_program_from_string(source).unwrap();
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), 0);
    labels.insert("label2".to_string(), 1);
    labels.insert("label3".to_string(), 2);
    let expected_program = Program::new(
        vec![
            Command::Jump("label3".to_string()),
            Command::Jump("label1".to_string()),
            Command::Jump("label2".to_string()),
        ],
        labels,
    );
    assert_eq!(program, expected_program);
}

#[test]
fn jump_parse_test() {
    let source = "label1:Jump label3\n; label2 : juMp label1;\n label3     :   jump label2;";
    let program = parse_program_from_string(source).unwrap();
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), 0);
    labels.insert("label2".to_string(), 1);
    labels.insert("label3".to_string(), 2);
    let expected_program = Program::new(
        vec![
            Command::Jump("label3".to_string()),
            Command::Jump("label1".to_string()),
            Command::Jump("label2".to_string()),
        ],
        labels,
    );
    assert_eq!(program, expected_program);
}

//TODO: do this test once parsing for u and i arithmetic operators work
// #[test]
// fn big_do_everything_loop_program() {
//     let source = r#"

//     "#;
// }
