#![allow(unused_imports)]
use nom::digit;
use nom::line_ending;
use nom::IResult;
use nom::types::CompleteStr;

use register::Register;
use shift16::Shift16;
use condition::Condition;
use instruction::Instruction;

use immediate::Immediate6;
use immediate::Immediate9;
use immediate::Immediate12;
use immediate::Immediate16;
use immediate::Immediate19;
use immediate::Immediate26;

#[derive(PartialEq, Debug)]
pub enum AsmLine<'a> {
    Instruction(Instruction),
    Comment(&'a str),
    Blank,
    Error,
}

/// Parse a register in the form `X23` to an `instruction::Register`
named!(
    parse_register<CompleteStr, Register>,
    do_parse!(
        tag!("X") >>
        register: switch!(alt!(digit | tag!("ZR")),
            CompleteStr("0")  => value!(Register::X0)  |
            CompleteStr("1")  => value!(Register::X1)  |
            CompleteStr("2")  => value!(Register::X2)  |
            CompleteStr("3")  => value!(Register::X3)  |
            CompleteStr("4")  => value!(Register::X4)  |
            CompleteStr("5")  => value!(Register::X5)  |
            CompleteStr("6")  => value!(Register::X6)  |
            CompleteStr("7")  => value!(Register::X7)  |
            CompleteStr("8")  => value!(Register::X8)  |
            CompleteStr("9")  => value!(Register::X9)  |
            CompleteStr("10") => value!(Register::X10) |
            CompleteStr("11") => value!(Register::X11) |
            CompleteStr("12") => value!(Register::X12) |
            CompleteStr("13") => value!(Register::X13) |
            CompleteStr("14") => value!(Register::X14) |
            CompleteStr("15") => value!(Register::X15) |
            CompleteStr("16") => value!(Register::X16) |
            CompleteStr("17") => value!(Register::X17) |
            CompleteStr("18") => value!(Register::X18) |
            CompleteStr("19") => value!(Register::X19) |
            CompleteStr("20") => value!(Register::X20) |
            CompleteStr("21") => value!(Register::X21) |
            CompleteStr("22") => value!(Register::X22) |
            CompleteStr("23") => value!(Register::X23) |
            CompleteStr("24") => value!(Register::X24) |
            CompleteStr("25") => value!(Register::X25) |
            CompleteStr("26") => value!(Register::X26) |
            CompleteStr("27") => value!(Register::X27) |
            CompleteStr("28") => value!(Register::X28) |
            CompleteStr("29") => value!(Register::X29) |
            CompleteStr("30") => value!(Register::X30) |
            CompleteStr("ZR") => value!(Register::XZR)
        ) >>
        (register)
    )
);

/// Parse a 6 bit immidiate value
named!(
    parse_immediate_6<CompleteStr, Immediate6>,
    do_parse!(
        s: opt!(tag!("-")) >>
        i: map_res!(digit, |d: CompleteStr| d.0.parse::<i8>()) >>
        (Immediate6(if s.is_some() { -i } else { i }))
    )
);

/// Parse a 9 bit immidiate value
named!(
    parse_immediate_9<CompleteStr, Immediate9>,
    do_parse!(
        s: opt!(tag!("-")) >>
        i: map_res!(digit, |d: CompleteStr| d.0.parse::<i16>()) >>
        (Immediate9(if s.is_some() { -i } else { i }))
    )
);

/// Parse a 12 bit immidiate value
named!(
    parse_immediate_12<CompleteStr, Immediate12>,
    do_parse!(
        s: opt!(tag!("-")) >>
        i: map_res!(digit, |d: CompleteStr| d.0.parse::<i16>()) >>
        (Immediate12(if s.is_some() { -i } else { i }))
    )
);

/// Parse a 16 bit immidiate value
named!(
    parse_immediate_16<CompleteStr, Immediate16>,
    do_parse!(
        s: opt!(tag!("-")) >>
        i: map_res!(digit, |d: CompleteStr| d.0.parse::<i16>()) >>
        (Immediate16(if s.is_some() { -i } else { i }))
    )
);

/// Parse a 19 bit immidiate value
named!(
    parse_immediate_19<CompleteStr, Immediate19>,
    do_parse!(
        s: opt!(tag!("-")) >>
        i: map_res!(digit, |d: CompleteStr| d.0.parse::<i32>()) >>
        (Immediate19(if s.is_some() { -i } else { i }))
    )
);

/// Parse a 26 bit immidiate value
named!(
    parse_immediate_26<CompleteStr, Immediate26>,
    do_parse!(
        s: opt!(tag!("-")) >>
        i: map_res!(digit, |d: CompleteStr| d.0.parse::<i32>()) >>
        (Immediate26(if s.is_some() { -i } else { i }))
    )
);

/// Parse a shift amount in the form `LSL 16` to an `instruction::Shift16`
named!(
    parse_shift<CompleteStr, Shift16>,
    do_parse!(
        ws!(tag!("LSL")) >>
        shift: switch!(digit,
            CompleteStr("0") => value!(Shift16::Shift0) |
            CompleteStr("16") => value!(Shift16::Shift16) |
            CompleteStr("32") => value!(Shift16::Shift32) |
            CompleteStr("48") => value!(Shift16::Shift48)
        ) >>
        (shift)
    )
);

/// Parse a condition into an `instruction::Condition`
named!(
    parse_condition<CompleteStr, Condition>,
    do_parse!(
        condition: switch!(take!(2),
            CompleteStr("EQ") => value!(Condition::Equal) |
            CompleteStr("NE") => value!(Condition::NotEqual) |
            CompleteStr("HS") => value!(Condition::HigherSame) |
            CompleteStr("LO") => value!(Condition::Lower) |
            CompleteStr("MI") => value!(Condition::Minus) |
            CompleteStr("PL") => value!(Condition::PositiveZero) |
            CompleteStr("VS") => value!(Condition::SignedOverflow) |
            CompleteStr("VC") => value!(Condition::NoSignedOverflow) |
            CompleteStr("HI") => value!(Condition::Higher) |
            CompleteStr("LS") => value!(Condition::LowerSame) |
            CompleteStr("GE") => value!(Condition::GreaterThanEqual) |
            CompleteStr("LT") => value!(Condition::LessThan) |
            CompleteStr("GT") => value!(Condition::GreaterThan) |
            CompleteStr("LE") => value!(Condition::LessThanEqual) |
            CompleteStr("AL") => value!(Condition::Always) |
            CompleteStr("NV") => value!(Condition::Reserved)
        ) >>
        (condition)
    )
);

/// Parse an instruction and its arguments into an `instruction::Instruction`
named!(
    parse_instruction<CompleteStr, Instruction>,
    do_parse!(
        instruction: switch!(take_until_either_and_consume!(" ."),
            CompleteStr("ADD") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::Add { n: n, m: m, destination: d })
            ) |
            CompleteStr("SUB") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::Subtract { n: n, m: m, destination: d })
            ) |
            CompleteStr("ADDI") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::AddImmediate { n: n, m: i, destination: d })
            ) |
            CompleteStr("SUBI") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::SubtractImmediate { n: n, m: i, destination: d })
            ) |
            CompleteStr("ADDS") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::AddSetFlags { n: n, m: m, destination: d })
            ) |
            CompleteStr("SUBS") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::SubtractSetFlags { n: n, m: m, destination: d })
            ) |
            CompleteStr("ADDIS") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::AddImmediateSetFlags { n: n, m: i, destination: d })
            ) |
            CompleteStr("SUBIS") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::SubtractImmediateSetFlags { n: n, m: i, destination: d })
            ) |
            CompleteStr("STUR") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >> ws!(tag!("[")) >>
                a: parse_register >> ws!(tag!(",")) >>
                o: parse_immediate_9 >> ws!(tag!("]")) >>
                (Instruction::Store { address: a, offset: o, data: d })
            ) |
            CompleteStr("LDUR") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >> ws!(tag!("[")) >>
                a: parse_register >> ws!(tag!(",")) >>
                o: parse_immediate_9 >> ws!(tag!("]")) >>
                (Instruction::Load { address: a, offset: o, data: d })
            ) |
            CompleteStr("STURB") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >> ws!(tag!("[")) >>
                a: parse_register >> ws!(tag!(",")) >>
                o: parse_immediate_9 >> ws!(tag!("]")) >>
                (Instruction::StoreByte{ address: a, offset: o, data: d })
            ) |
            CompleteStr("LDURB") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >> ws!(tag!("[")) >>
                a: parse_register >> ws!(tag!(",")) >>
                o: parse_immediate_9 >> ws!(tag!("]")) >>
                (Instruction::LoadByte { address: a, offset: o, data: d })
            ) |
            CompleteStr("MOVZ") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_16 >>
                s: alt!(complete!(preceded!(ws!(tag!(",")), parse_shift)) | value!(Shift16::Shift0)) >>
                (Instruction::MoveZero { immediate: i, shift: s, destination: d })
            ) |
            CompleteStr("MOVK") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_16 >>
                s: alt!(complete!(preceded!(ws!(tag!(",")), parse_shift)) | value!(Shift16::Shift0)) >>
                (Instruction::MoveKeep { immediate: i, shift: s, destination: d })
            ) |
            CompleteStr("AND") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::And{ n: n, m: m, destination: d })
            ) |
            CompleteStr("ORR") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::Or{ n: n, m: m, destination: d })
            ) |
            CompleteStr("EOR") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::Xor{ n: n, m: m, destination: d })
            ) |
            CompleteStr("ANDI") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::AndImmediate { n: n, m: i, destination: d })
            ) |
            CompleteStr("ORRI") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::OrImmediate { n: n, m: i, destination: d })
            ) |
            CompleteStr("EORI") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::XorImmediate { n: n, m: i, destination: d })
            ) |
            CompleteStr("ANDS") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::AndSetFlags { n: n, m: m, destination: d })
            ) |
            CompleteStr("ANDIS") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::AndImmediateSetFlags { n: n, m: i, destination: d })
            ) |
            CompleteStr("LSR") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_6 >>
                (Instruction::LogicalShiftRight { n: n, m: i, destination: d })
            ) |
            CompleteStr("LSL") => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_6 >>
                (Instruction::LogicalShiftLeft { n: n, m: i, destination: d })
            ) |
            CompleteStr("CBZ") => do_parse!(
                r: parse_register >> ws!(tag!(",")) >>
                a: parse_immediate_19 >>
                (Instruction::CompareBranchZero { address: a, r: r })
            ) |
            CompleteStr("CBNZ") => do_parse!(
                r: parse_register >> ws!(tag!(",")) >>
                a: parse_immediate_19 >>
                (Instruction::CompareBranchNotZero { address: a, r: r })
            ) |
            CompleteStr("B") => alt!(
                do_parse!(
                    c: ws!(parse_condition) >>
                    a: ws!(parse_immediate_19) >>
                    (Instruction::ConditionalBranch { address: a, condition: c })
                ) |
                do_parse!(
                    a: ws!(parse_immediate_26) >>
                    (Instruction::Branch { address: a })
                )
            ) |
            CompleteStr("BR") => do_parse!(
                r: parse_register >>
                (Instruction::BranchRegister { r: r })
            ) |
            CompleteStr("BL") => do_parse!(
                a: parse_immediate_26 >>
                (Instruction::BranchLink { address: a })
            )
        ) >>
        (instruction)
    )
);

named!(
    parse_line<CompleteStr, AsmLine>,
    alt!(
        parse_instruction => { |i| AsmLine::Instruction(i) } |
        exact!(ws!(tag!(""))) => { |_| AsmLine::Blank }
        // the closure takes the result as arguNone ment if the parser is successful
    )
);

/// Parse lines to instructions
pub fn parse_lines(lines: &str) -> Vec<AsmLine> {
    lines.lines().map(|line| {
        if let Ok((_, asm_line)) = parse_line(CompleteStr(line)) {
            asm_line
        } else {
            AsmLine::Error
        }
    }).collect()
}

#[test]
fn test_lines_parse() {
    assert_eq!(
        parse_lines("ADD X1, X2, X3\n\nSUB X4, X5, X6"),
        vec![
            AsmLine::Instruction(Instruction::Add { n: Register::X2, m: Register::X3, destination: Register::X1 }),
            AsmLine::Blank,
            AsmLine::Instruction(Instruction::Subtract { n: Register::X5, m: Register::X6, destination: Register::X4 })
        ]
    );
}

#[test]
fn test_line_instruction_parse() {
    assert_eq!(
        parse_line(CompleteStr("ADD X1, X2, X3")),
        Ok((
            CompleteStr(""),
            AsmLine::Instruction(Instruction::Add { n: Register::X2, m: Register::X3, destination: Register::X1 }),
        ))
    );
}

#[test]
fn test_line_empty_parse() {
    assert_eq!(
        parse_line(CompleteStr(" ")),
        Ok((
            CompleteStr(""),
            AsmLine::Blank
        ))
    );
}

#[test]
fn test_line_error_parse() {
    assert!(
        parse_line(CompleteStr("asdfasdf")).is_err()
    );
}

#[test]
fn test_branch_link_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("BL 2500")),
        Ok((
            CompleteStr(""),
            Instruction::BranchLink {
                address: Immediate26(2500_i32),
            }
        ))
    );
}

#[test]
fn test_branch_register_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("BR X30")),
        Ok((CompleteStr(""), Instruction::BranchRegister { r: Register::X30 }))
    );
}

#[test]
fn test_branch_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("B 25")),
        Ok((
            CompleteStr(""),
            Instruction::Branch {
                address: Immediate26(25_i32),
            }
        ))
    );
}

#[test]
fn test_conditional_branch_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("B.LT 25")),
        Ok((
            CompleteStr(""),
            Instruction::ConditionalBranch {
                address: Immediate19(25_i32),
                condition: Condition::LessThan,
            }
        ))
    );
}

#[test]
fn test_compare_branch_not_zero_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("CBNZ X1, 25")),
        Ok((
            CompleteStr(""),
            Instruction::CompareBranchNotZero {
                address: Immediate19(25_i32),
                r: Register::X1,
            }
        ))
    );
}

#[test]
fn test_compare_branch_zero_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("CBZ X1, 25")),
        Ok((
            CompleteStr(""),
            Instruction::CompareBranchZero {
                address: Immediate19(25_i32),
                r: Register::X1,
            }
        ))
    );
}

#[test]
fn test_logical_shift_left_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("LSL X1, X2, 10")),
        Ok((
            CompleteStr(""),
            Instruction::LogicalShiftLeft {
                n: Register::X2,
                m: Immediate6(10_i8),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_logical_shift_right_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("LSR X1, X2, 10")),
        Ok((
            CompleteStr(""),
            Instruction::LogicalShiftRight {
                n: Register::X2,
                m: Immediate6(10_i8),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_and_immidiate_set_flags_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ANDIS X1, X2, 40")),
        Ok((
            CompleteStr(""),
            Instruction::AndImmediateSetFlags {
                n: Register::X2,
                m: Immediate12(40_i16),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_and_set_flags_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ANDS X1, X2, X3")),
        Ok((
            CompleteStr(""),
            Instruction::AndSetFlags {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_and_immidiate_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ANDI X1, X2, 40")),
        Ok((
            CompleteStr(""),
            Instruction::AndImmediate {
                n: Register::X2,
                m: Immediate12(40_i16),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_or_immidiate_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ORRI X1, X2, 40")),
        Ok((
            CompleteStr(""),
            Instruction::OrImmediate {
                n: Register::X2,
                m: Immediate12(40_i16),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_xor_immidiate_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("EORI X1, X2, 40")),
        Ok((
            CompleteStr(""),
            Instruction::XorImmediate {
                n: Register::X2,
                m: Immediate12(40_i16),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_and_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("AND X1, X2, X3")),
        Ok((
            CompleteStr(""),
            Instruction::And {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_or_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ORR X1, X2, X3")),
        Ok((
            CompleteStr(""),
            Instruction::Or {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_xor_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("EOR X1, X2, X3")),
        Ok((
            CompleteStr(""),
            Instruction::Xor {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_move_keep_without_shift_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("MOVK X9, 255")),
        Ok((
            CompleteStr(""),
            Instruction::MoveKeep {
                immediate: Immediate16(255_i16),
                shift: Shift16::Shift0,
                destination: Register::X9,
            }
        ))
    );
}

#[test]
fn test_move_keep_with_shift_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("MOVK X9, 255, LSL 16")),
        Ok((
            CompleteStr(""),
            Instruction::MoveKeep {
                immediate: Immediate16(255_i16),
                shift: Shift16::Shift16,
                destination: Register::X9,
            }
        ))
    );
}

#[test]
fn test_move_zero_without_shift_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("MOVZ X9, 255")),
        Ok((
            CompleteStr(""),
            Instruction::MoveZero {
                immediate: Immediate16(255_i16),
                shift: Shift16::Shift0,
                destination: Register::X9,
            }
        ))
    );
}

#[test]
fn test_move_zero_with_shift_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("MOVZ X9, 255, LSL 16")),
        Ok((
            CompleteStr(""),
            Instruction::MoveZero {
                immediate: Immediate16(255_i16),
                shift: Shift16::Shift16,
                destination: Register::X9,
            }
        ))
    );
}

#[test]
fn test_load_byte_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("LDURB X1, [X2, 40]")),
        Ok((
            CompleteStr(""),
            Instruction::LoadByte {
                address: Register::X2,
                offset: Immediate9(40_i16),
                data: Register::X1,
            }
        ))
    );
}

#[test]
fn test_store_byte_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("STURB X1, [X2, 40]")),
        Ok((
            CompleteStr(""),
            Instruction::StoreByte {
                address: Register::X2,
                offset: Immediate9(40_i16),
                data: Register::X1,
            }
        ))
    );
}

#[test]
fn test_load_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("LDUR X1, [X2, 40]")),
        Ok((
            CompleteStr(""),
            Instruction::Load {
                address: Register::X2,
                offset: Immediate9(40_i16),
                data: Register::X1,
            }
        ))
    );
}

#[test]
fn test_store_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("STUR X1, [X2, 40]")),
        Ok((
            CompleteStr(""),
            Instruction::Store {
                address: Register::X2,
                offset: Immediate9(40_i16),
                data: Register::X1,
            }
        ))
    );
}

#[test]
fn test_add_immidiate_set_flags_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ADDIS X1, X2, 40")),
        Ok((
            CompleteStr(""),
            Instruction::AddImmediateSetFlags {
                n: Register::X2,
                m: Immediate12(40_i16),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_sub_immidiate_set_flags_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("SUBIS X1, X2, 40")),
        Ok((
            CompleteStr(""),
            Instruction::SubtractImmediateSetFlags {
                n: Register::X2,
                m: Immediate12(40_i16),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_add_set_flags_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ADDS X1, X2, X3")),
        Ok((
            CompleteStr(""),
            Instruction::AddSetFlags {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_subract_set_flags_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("SUBS X1, X2, X3")),
        Ok((
            CompleteStr(""),
            Instruction::SubtractSetFlags {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_add_immidiate_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ADDI X1, X2, 40")),
        Ok((
            CompleteStr(""),
            Instruction::AddImmediate {
                n: Register::X2,
                m: Immediate12(40_i16),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_sub_immidiate_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("SUBI X1, X2, 40")),
        Ok((
            CompleteStr(""),
            Instruction::SubtractImmediate {
                n: Register::X2,
                m: Immediate12(40_i16),
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_add_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("ADD X1, X2, X3")),
        Ok((
            CompleteStr(""),
            Instruction::Add {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_subract_parse() {
    assert_eq!(
        parse_instruction(CompleteStr("SUB X1, X2, X3")),
        Ok((
            CompleteStr(""),
            Instruction::Subtract {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        ))
    );
}

#[test]
fn test_shift0_parse() {
    assert_eq!(parse_shift(CompleteStr("LSL 0")), Ok((CompleteStr(""), Shift16::Shift0,)));
}

#[test]
fn test_shift16_parse() {
    assert_eq!(parse_shift(CompleteStr("LSL 16")), Ok((CompleteStr(""), Shift16::Shift16,)));
}

#[test]
fn test_shift32_parse() {
    assert_eq!(parse_shift(CompleteStr("LSL 32")), Ok((CompleteStr(""), Shift16::Shift32,)));
}

#[test]
fn test_shift48_parse() {
    assert_eq!(parse_shift(CompleteStr("LSL 48")), Ok((CompleteStr(""), Shift16::Shift48,)));
}

#[test]
fn test_condition_equal_parse() {
    assert_eq!(parse_condition(CompleteStr("EQ")), Ok((CompleteStr(""), Condition::Equal)));
}

#[test]
fn test_condition_not_equal_parse() {
    assert_eq!(
        parse_condition(CompleteStr("NE")),
        Ok((CompleteStr(""), Condition::NotEqual))
    );
}

#[test]
fn test_condition_higher_same_parse() {
    assert_eq!(
        parse_condition(CompleteStr("HS")),
        Ok((CompleteStr(""), Condition::HigherSame))
    );
}

#[test]
fn test_condition_lower_parse() {
    assert_eq!(parse_condition(CompleteStr("LO")), Ok((CompleteStr(""), Condition::Lower)));
}

#[test]
fn test_condition_minus_parse() {
    assert_eq!(parse_condition(CompleteStr("MI")), Ok((CompleteStr(""), Condition::Minus)));
}

#[test]
fn test_condition_positive_zero_parse() {
    assert_eq!(
        parse_condition(CompleteStr("PL")),
        Ok((CompleteStr(""), Condition::PositiveZero))
    );
}

#[test]
fn test_condition_signed_overflow_parse() {
    assert_eq!(
        parse_condition(CompleteStr("VS")),
        Ok((CompleteStr(""), Condition::SignedOverflow))
    );
}

#[test]
fn test_condition_no_signed_overflow_parse() {
    assert_eq!(
        parse_condition(CompleteStr("VC")),
        Ok((CompleteStr(""), Condition::NoSignedOverflow))
    );
}

#[test]
fn test_condition_higher_parse() {
    assert_eq!(parse_condition(CompleteStr("HI")), Ok((CompleteStr(""), Condition::Higher)));
}

#[test]
fn test_condition_lower_same_parse() {
    assert_eq!(
        parse_condition(CompleteStr("LS")),
        Ok((CompleteStr(""), Condition::LowerSame))
    );
}

#[test]
fn test_condition_greater_than_equal_parse() {
    assert_eq!(
        parse_condition(CompleteStr("GE")),
        Ok((CompleteStr(""), Condition::GreaterThanEqual))
    );
}

#[test]
fn test_condition_less_than_parse() {
    assert_eq!(
        parse_condition(CompleteStr("LT")),
        Ok((CompleteStr(""), Condition::LessThan))
    );
}

#[test]
fn test_condition_greater_than_parse() {
    assert_eq!(
        parse_condition(CompleteStr("GT")),
        Ok((CompleteStr(""), Condition::GreaterThan))
    );
}

#[test]
fn test_condition_less_than_equal_parse() {
    assert_eq!(
        parse_condition(CompleteStr("LE")),
        Ok((CompleteStr(""), Condition::LessThanEqual))
    );
}

#[test]
fn test_condition_always_parse() {
    assert_eq!(parse_condition(CompleteStr("AL")), Ok((CompleteStr(""), Condition::Always)));
}

#[test]
fn test_condition_reserved_parse() {
    assert_eq!(
        parse_condition(CompleteStr("NV")),
        Ok((CompleteStr(""), Condition::Reserved))
    );
}

#[test]
fn test_immediate6_parse() {
    assert_eq!(
        parse_immediate_6(CompleteStr("25")),
        Ok((CompleteStr(""), Immediate6(25_i8)))
    );
}

#[test]
fn test_immediate9_parse() {
    assert_eq!(
        parse_immediate_9(CompleteStr("25")),
        Ok((CompleteStr(""), Immediate9(25_i16)))
    );
}

#[test]
fn test_immediate12_parse() {
    assert_eq!(
        parse_immediate_12(CompleteStr("25")),
        Ok((CompleteStr(""), Immediate12(25_i16)))
    );
}

#[test]
fn test_immediate16_parse() {
    assert_eq!(
        parse_immediate_16(CompleteStr("25")),
        Ok((CompleteStr(""), Immediate16(25_i16)))
    );
}

#[test]
fn test_immediate19_parse() {
    assert_eq!(
        parse_immediate_19(CompleteStr("25")),
        Ok((CompleteStr(""), Immediate19(25_i32)))
    );
}

#[test]
fn test_immediate26_parse() {
    assert_eq!(
        parse_immediate_26(CompleteStr("25")),
        Ok((CompleteStr(""), Immediate26(25_i32)))
    );
}

#[test]
fn test_negitive_immediate6_parse() {
    assert_eq!(
        parse_immediate_6(CompleteStr("-25")),
        Ok((CompleteStr(""), Immediate6(-25_i8)))
    );
}

#[test]
fn test_negitive_immediate9_parse() {
    assert_eq!(
        parse_immediate_9(CompleteStr("-25")),
        Ok((CompleteStr(""), Immediate9(-25_i16)))
    );
}

#[test]
fn test_negitive_immediate12_parse() {
    assert_eq!(
        parse_immediate_12(CompleteStr("-25")),
        Ok((CompleteStr(""), Immediate12(-25_i16)))
    );
}

#[test]
fn test_negitive_immediate16_parse() {
    assert_eq!(
        parse_immediate_16(CompleteStr("-25")),
        Ok((CompleteStr(""), Immediate16(-25_i16)))
    );
}

#[test]
fn test_negitive_immediate19_parse() {
    assert_eq!(
        parse_immediate_19(CompleteStr("-25")),
        Ok((CompleteStr(""), Immediate19(-25_i32)))
    );
}

#[test]
fn test_negitive_immediate26_parse() {
    assert_eq!(
        parse_immediate_26(CompleteStr("-25")),
        Ok((CompleteStr(""), Immediate26(-25_i32)))
    );
}

#[test]
fn test_register_x1_parse() {
    assert_eq!(parse_register(CompleteStr("X0")), Ok((CompleteStr(""), Register::X0)));
}
#[test]
fn test_register_x2_parse() {
    assert_eq!(parse_register(CompleteStr("X1")), Ok((CompleteStr(""), Register::X1)));
}
#[test]
fn test_register_x3_parse() {
    assert_eq!(parse_register(CompleteStr("X2")), Ok((CompleteStr(""), Register::X2)));
}
#[test]
fn test_register_x4_parse() {
    assert_eq!(parse_register(CompleteStr("X3")), Ok((CompleteStr(""), Register::X3)));
}
#[test]
fn test_register_x5_parse() {
    assert_eq!(parse_register(CompleteStr("X4")), Ok((CompleteStr(""), Register::X4)));
}
#[test]
fn test_register_x6_parse() {
    assert_eq!(parse_register(CompleteStr("X5")), Ok((CompleteStr(""), Register::X5)));
}
#[test]
fn test_register_x7_parse() {
    assert_eq!(parse_register(CompleteStr("X6")), Ok((CompleteStr(""), Register::X6)));
}
#[test]
fn test_register_x8_parse() {
    assert_eq!(parse_register(CompleteStr("X7")), Ok((CompleteStr(""), Register::X7)));
}
#[test]
fn test_register_x9_parse() {
    assert_eq!(parse_register(CompleteStr("X8")), Ok((CompleteStr(""), Register::X8)));
}
#[test]
fn test_register_x10_parse() {
    assert_eq!(parse_register(CompleteStr("X9")), Ok((CompleteStr(""), Register::X9)));
}
#[test]
fn test_register_x11_parse() {
    assert_eq!(parse_register(CompleteStr("X10")), Ok((CompleteStr(""), Register::X10)));
}
#[test]
fn test_register_x12_parse() {
    assert_eq!(parse_register(CompleteStr("X11")), Ok((CompleteStr(""), Register::X11)));
}
#[test]
fn test_register_x13_parse() {
    assert_eq!(parse_register(CompleteStr("X12")), Ok((CompleteStr(""), Register::X12)));
}
#[test]
fn test_register_x14_parse() {
    assert_eq!(parse_register(CompleteStr("X13")), Ok((CompleteStr(""), Register::X13)));
}
#[test]
fn test_register_x15_parse() {
    assert_eq!(parse_register(CompleteStr("X14")), Ok((CompleteStr(""), Register::X14)));
}
#[test]
fn test_register_x16_parse() {
    assert_eq!(parse_register(CompleteStr("X15")), Ok((CompleteStr(""), Register::X15)));
}
#[test]
fn test_register_x17_parse() {
    assert_eq!(parse_register(CompleteStr("X16")), Ok((CompleteStr(""), Register::X16)));
}
#[test]
fn test_register_x18_parse() {
    assert_eq!(parse_register(CompleteStr("X17")), Ok((CompleteStr(""), Register::X17)));
}
#[test]
fn test_register_x19_parse() {
    assert_eq!(parse_register(CompleteStr("X18")), Ok((CompleteStr(""), Register::X18)));
}
#[test]
fn test_register_x20_parse() {
    assert_eq!(parse_register(CompleteStr("X19")), Ok((CompleteStr(""), Register::X19)));
}
#[test]
fn test_register_x21_parse() {
    assert_eq!(parse_register(CompleteStr("X20")), Ok((CompleteStr(""), Register::X20)));
}
#[test]
fn test_register_x22_parse() {
    assert_eq!(parse_register(CompleteStr("X21")), Ok((CompleteStr(""), Register::X21)));
}
#[test]
fn test_register_x23_parse() {
    assert_eq!(parse_register(CompleteStr("X22")), Ok((CompleteStr(""), Register::X22)));
}
#[test]
fn test_register_x24_parse() {
    assert_eq!(parse_register(CompleteStr("X23")), Ok((CompleteStr(""), Register::X23)));
}
#[test]
fn test_register_x25_parse() {
    assert_eq!(parse_register(CompleteStr("X24")), Ok((CompleteStr(""), Register::X24)));
}
#[test]
fn test_register_x26_parse() {
    assert_eq!(parse_register(CompleteStr("X25")), Ok((CompleteStr(""), Register::X25)));
}
#[test]
fn test_register_x27_parse() {
    assert_eq!(parse_register(CompleteStr("X26")), Ok((CompleteStr(""), Register::X26)));
}
#[test]
fn test_register_x28_parse() {
    assert_eq!(parse_register(CompleteStr("X27")), Ok((CompleteStr(""), Register::X27)));
}
#[test]
fn test_register_x29_parse() {
    assert_eq!(parse_register(CompleteStr("X28")), Ok((CompleteStr(""), Register::X28)));
}
#[test]
fn test_register_x30_parse() {
    assert_eq!(parse_register(CompleteStr("X29")), Ok((CompleteStr(""), Register::X29)));
}
#[test]
fn test_register_x31_parse() {
    assert_eq!(parse_register(CompleteStr("X30")), Ok((CompleteStr(""), Register::X30)));
}
#[test]
fn test_register_xzr_parse() {
    assert_eq!(parse_register(CompleteStr("XZR")), Ok((CompleteStr(""), Register::XZR)));
}
