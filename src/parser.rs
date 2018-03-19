#![allow(unused_imports)]
use nom::digit;
use nom::IResult;

use register::Register;
use shift16::Shift16;
use condition::Condition;
use instruction::Instruction;

use immediate::Immediate9;
use immediate::Immediate12;
use immediate::Immediate16;
use immediate::Immediate19;

/// Parse a register in the form `X23` to an `instruction::Register`
named!(
    parse_register<&str, Register>,
    do_parse!(
        tag!("X") >>
        register: switch!(alt!(digit | tag!("ZR")),
            "0"  => value!(Register::X0)  |
            "1"  => value!(Register::X1)  |
            "2"  => value!(Register::X2)  |
            "3"  => value!(Register::X3)  |
            "4"  => value!(Register::X4)  |
            "5"  => value!(Register::X5)  |
            "6"  => value!(Register::X6)  |
            "7"  => value!(Register::X7)  |
            "8"  => value!(Register::X8)  |
            "9"  => value!(Register::X9)  |
            "10" => value!(Register::X10) |
            "11" => value!(Register::X11) |
            "12" => value!(Register::X12) |
            "13" => value!(Register::X13) |
            "14" => value!(Register::X14) |
            "15" => value!(Register::X15) |
            "16" => value!(Register::X16) |
            "17" => value!(Register::X17) |
            "18" => value!(Register::X18) |
            "19" => value!(Register::X19) |
            "20" => value!(Register::X20) |
            "21" => value!(Register::X21) |
            "22" => value!(Register::X22) |
            "23" => value!(Register::X23) |
            "24" => value!(Register::X24) |
            "25" => value!(Register::X25) |
            "26" => value!(Register::X26) |
            "27" => value!(Register::X27) |
            "28" => value!(Register::X28) |
            "29" => value!(Register::X29) |
            "30" => value!(Register::X30) |
            "ZR" => value!(Register::XZR)
        ) >>
        (register)
    )
);

/// Parse a 9 bit immidiate value
named!(
    parse_immediate_9<&str, Immediate9>,
    do_parse!(
        i: map_res!(digit, |d: &str| d.parse::<u16>())
        (Immediate9(i))
    )
);

/// Parse a 12 bit immidiate value
named!(
    parse_immediate_12<&str, Immediate16>,
    do_parse!(
        i: map_res!(digit, |d: &str| d.parse::<u16>())
        (Immediate12(i))
    )
);

/// Parse a 16 bit immidiate value
named!(
    parse_immediate_16<&str, Immediate16>,
    do_parse!(
        i: map_res!(digit, |d: &str| d.parse::<u16>())
        (Immediate16(i))
    )
);

/// Parse a 19 bit immidiate value
named!(
    parse_immediate_19<&str, Immediate19>,
    do_parse!(
        i: map_res!(digit, |d: &str| d.parse::<u32>())
        (Immediate19(i))
    )
);

/// Parse a shift amount in the form `LSL 16` to an `instruction::Shift16`
named!(
    parse_shift<&str, Shift16>,
    do_parse!(
        ws!(tag!("LSL")) >>
        shift: switch!(digit,
            "0" => value!(Shift16::Shift0) |
            "16" => value!(Shift16::Shift16) |
            "32" => value!(Shift16::Shift32) |
            "48" => value!(Shift16::Shift48)
        ) >>
        (shift)
    )
);

/// Parse a condition into an `instruction::Condition`
named!(
    parse_condition<&str, Condition>,
    do_parse!(
        condition: switch!(take!(2),
            "EQ" => value!(Condition::Equal) |
            "NE" => value!(Condition::NotEqual) |
            "HS" => value!(Condition::HigherSame) |
            "LO" => value!(Condition::Lower) |
            "MI" => value!(Condition::Minus) |
            "PL" => value!(Condition::PositiveZero) |
            "VS" => value!(Condition::SignedOverflow) |
            "VC" => value!(Condition::NoSignedOverflow) |
            "HI" => value!(Condition::Higher) |
            "LS" => value!(Condition::LowerSame) |
            "GE" => value!(Condition::GreaterThanEqual) |
            "LT" => value!(Condition::LessThan) |
            "GT" => value!(Condition::GreaterThan) |
            "LE" => value!(Condition::LessThanEqual) |
            "AL" => value!(Condition::Always) |
            "NV" => value!(Condition::Reserved)
        ) >>
        (condition)
    )
);

/// Parse an instruction and its arguments into an `instruction::Instruction`
named!(
    parse_instruction<&str, Instruction>,
    do_parse!(
        instruction: switch!(take_until_either_and_consume!(" ."),
            "ADD" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::Add { n: n, m: m, destination: d })
            ) |
            "SUB" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::Subtract { n: n, m: m, destination: d })
            ) |
            "ADDI" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::AddImmediate { n: n, m: i, destination: d })
            ) |
            "SUBI" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::SubtractImmediate { n: n, m: i, destination: d })
            ) |
            "ADDS" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::AddSetFlags { n: n, m: m, destination: d })
            ) |
            "SUBS" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::SubtractSetFlags { n: n, m: m, destination: d })
            ) |
            "ADDIS" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::AddImmediateSetFlags { n: n, m: i, destination: d })
            ) |
            "SUBIS" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::SubtractImmediateSetFlags { n: n, m: i, destination: d })
            ) |
            "STUR" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >> ws!(tag!("[")) >>
                a: parse_register >> ws!(tag!(",")) >>
                o: parse_immediate_9 >> ws!(tag!("]")) >>
                (Instruction::Store { address: a, offset: o, data: d })
            ) |
            "LDUR" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >> ws!(tag!("[")) >>
                a: parse_register >> ws!(tag!(",")) >>
                o: parse_immediate_9 >> ws!(tag!("]")) >>
                (Instruction::Load { address: a, offset: o, data: d })
            ) |
            "STURB" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >> ws!(tag!("[")) >>
                a: parse_register >> ws!(tag!(",")) >>
                o: parse_immediate_9 >> ws!(tag!("]")) >>
                (Instruction::StoreByte{ address: a, offset: o, data: d })
            ) |
            "LDURB" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >> ws!(tag!("[")) >>
                a: parse_register >> ws!(tag!(",")) >>
                o: parse_immediate_9 >> ws!(tag!("]")) >>
                (Instruction::LoadByte { address: a, offset: o, data: d })
            ) |
            "MOVZ" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_16 >>
                s: alt!(complete!(preceded!(ws!(tag!(",")), parse_shift)) | value!(Shift16::Shift0)) >>
                (Instruction::MoveZero { immediate: i, shift: s, destination: d })
            ) |
            "MOVK" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_16 >>
                s: alt!(complete!(preceded!(ws!(tag!(",")), parse_shift)) | value!(Shift16::Shift0)) >>
                (Instruction::MoveKeep { immediate: i, shift: s, destination: d })
            ) |
            "AND" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::And{ n: n, m: m, destination: d })
            ) |
            "ORR" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::Or{ n: n, m: m, destination: d })
            ) |
            "EOR" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::Xor{ n: n, m: m, destination: d })
            ) |
            "ANDI" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::AndImmediate { n: n, m: i, destination: d })
            ) |
            "ORRI" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::OrImmediate { n: n, m: i, destination: d })
            ) |
            "EORI" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::XorImmediate { n: n, m: i, destination: d })
            ) |
            "ANDS" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                m: parse_register >>
                (Instruction::AndSetFlags { n: n, m: m, destination: d })
            ) |
            "ANDIS" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::AndImmediateSetFlags { n: n, m: i, destination: d })
            ) |
            "LSR" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::LogicalShiftRight { n: n, m: i, destination: d })
            ) |
            "LSL" => do_parse!(
                d: parse_register >> ws!(tag!(",")) >>
                n: parse_register >> ws!(tag!(",")) >>
                i: parse_immediate_12 >>
                (Instruction::LogicalShiftLeft { n: n, m: i, destination: d })
            ) |
            "CBZ" => do_parse!(
                r: parse_register >> ws!(tag!(",")) >>
                a: parse_immediate_19 >>
                (Instruction::CompareBranchZero { address: a, r: r })
            ) |
            "CBNZ" => do_parse!(
                r: parse_register >> ws!(tag!(",")) >>
                a: parse_immediate_19 >>
                (Instruction::CompareBranchNotZero { address: a, r: r })
            ) |
            "B" => alt!(
                do_parse!(
                    c: ws!(parse_condition) >>
                    a: ws!(parse_immediate_19) >>
                    (Instruction::ConditionalBranch { address: a, condition: c })
                ) |
                do_parse!(
                    a: ws!(parse_immediate_19) >>
                    (Instruction::Branch { address: a })
                )
            ) |
            "BR" => do_parse!(
                r: parse_register >>
                (Instruction::BranchRegister { r: r })
            ) |
            "BL" => do_parse!(
                a: parse_immediate_32 >>
                (Instruction::BranchLink { address: a })
            )

        ) >>
        (instruction)
    )
);

#[test]
fn test_branch_link_parse() {
    assert_eq!(
        parse_instruction("BL 2500"),
        IResult::Done("", Instruction::BranchLink { address: 2500_u32 })
    );
}

#[test]
fn test_branch_register_parse() {
    assert_eq!(
        parse_instruction("BR X30"),
        IResult::Done("", Instruction::BranchRegister { r: Register::X30 })
    );
}

#[test]
fn test_branch_parse() {
    assert_eq!(
        parse_instruction("B 25"),
        IResult::Done("", Instruction::Branch { address: 25_u32 })
    );
}

#[test]
fn test_conditional_branch_parse() {
    assert_eq!(
        parse_instruction("B.LT 25"),
        IResult::Done(
            "",
            Instruction::ConditionalBranch {
                address: 25_u32,
                condition: Condition::LessThan,
            }
        )
    );
}

#[test]
fn test_compare_branch_not_zero_parse() {
    assert_eq!(
        parse_instruction("CBNZ X1, 25"),
        IResult::Done(
            "",
            Instruction::CompareBranchNotZero {
                address: 25_u32,
                r: Register::X1,
            }
        )
    );
}

#[test]
fn test_compare_branch_zero_parse() {
    assert_eq!(
        parse_instruction("CBZ X1, 25"),
        IResult::Done(
            "",
            Instruction::CompareBranchZero {
                address: 25_u32,
                r: Register::X1,
            }
        )
    );
}

#[test]
fn test_logical_shift_left_parse() {
    assert_eq!(
        parse_instruction("LSL X1, X2, 10"),
        IResult::Done(
            "",
            Instruction::LogicalShiftLeft {
                n: Register::X2,
                m: 10_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_logical_shift_right_parse() {
    assert_eq!(
        parse_instruction("LSR X1, X2, 10"),
        IResult::Done(
            "",
            Instruction::LogicalShiftRight {
                n: Register::X2,
                m: 10_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_and_immidiate_set_flags_parse() {
    assert_eq!(
        parse_instruction("ANDIS X1, X2, 40"),
        IResult::Done(
            "",
            Instruction::AndImmediateSetFlags {
                n: Register::X2,
                m: 40_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_and_set_flags_parse() {
    assert_eq!(
        parse_instruction("ANDS X1, X2, X3"),
        IResult::Done(
            "",
            Instruction::AndSetFlags {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_and_immidiate_parse() {
    assert_eq!(
        parse_instruction("ANDI X1, X2, 40"),
        IResult::Done(
            "",
            Instruction::AndImmediate {
                n: Register::X2,
                m: 40_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_or_immidiate_parse() {
    assert_eq!(
        parse_instruction("ORRI X1, X2, 40"),
        IResult::Done(
            "",
            Instruction::OrImmediate {
                n: Register::X2,
                m: 40_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_xor_immidiate_parse() {
    assert_eq!(
        parse_instruction("EORI X1, X2, 40"),
        IResult::Done(
            "",
            Instruction::XorImmediate {
                n: Register::X2,
                m: 40_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_and_parse() {
    assert_eq!(
        parse_instruction("AND X1, X2, X3"),
        IResult::Done(
            "",
            Instruction::And {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_or_parse() {
    assert_eq!(
        parse_instruction("ORR X1, X2, X3"),
        IResult::Done(
            "",
            Instruction::Or {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_xor_parse() {
    assert_eq!(
        parse_instruction("EOR X1, X2, X3"),
        IResult::Done(
            "",
            Instruction::Xor {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_move_keep_without_shift_parse() {
    assert_eq!(
        parse_instruction("MOVK X9, 255"),
        IResult::Done(
            "",
            Instruction::MoveKeep {
                immediate: 255_u16,
                shift: Shift16::Shift0,
                destination: Register::X9,
            }
        )
    );
}

#[test]
fn test_move_keep_with_shift_parse() {
    assert_eq!(
        parse_instruction("MOVK X9, 255, LSL 16"),
        IResult::Done(
            "",
            Instruction::MoveKeep {
                immediate: 255_u16,
                shift: Shift16::Shift16,
                destination: Register::X9,
            }
        )
    );
}

#[test]
fn test_move_zero_without_shift_parse() {
    assert_eq!(
        parse_instruction("MOVZ X9, 255"),
        IResult::Done(
            "",
            Instruction::MoveZero {
                immediate: 255_u16,
                shift: Shift16::Shift0,
                destination: Register::X9,
            }
        )
    );
}

#[test]
fn test_move_zero_with_shift_parse() {
    assert_eq!(
        parse_instruction("MOVZ X9, 255, LSL 16"),
        IResult::Done(
            "",
            Instruction::MoveZero {
                immediate: 255_u16,
                shift: Shift16::Shift16,
                destination: Register::X9,
            }
        )
    );
}

#[test]
fn test_load_byte_parse() {
    assert_eq!(
        parse_instruction("LDURB X1, [X2, 40]"),
        IResult::Done(
            "",
            Instruction::LoadByte {
                address: Register::X2,
                offset: 40_u16,
                data: Register::X1,
            }
        )
    );
}

#[test]
fn test_store_byte_parse() {
    assert_eq!(
        parse_instruction("STURB X1, [X2, 40]"),
        IResult::Done(
            "",
            Instruction::StoreByte {
                address: Register::X2,
                offset: 40_u16,
                data: Register::X1,
            }
        )
    );
}

#[test]
fn test_load_parse() {
    assert_eq!(
        parse_instruction("LDUR X1, [X2, 40]"),
        IResult::Done(
            "",
            Instruction::Load {
                address: Register::X2,
                offset: 40_u16,
                data: Register::X1,
            }
        )
    );
}

#[test]
fn test_store_parse() {
    assert_eq!(
        parse_instruction("STUR X1, [X2, 40]"),
        IResult::Done(
            "",
            Instruction::Store {
                address: Register::X2,
                offset: 40_u16,
                data: Register::X1,
            }
        )
    );
}

#[test]
fn test_add_immidiate_set_flags_parse() {
    assert_eq!(
        parse_instruction("ADDIS X1, X2, 40"),
        IResult::Done(
            "",
            Instruction::AddImmediateSetFlags {
                n: Register::X2,
                m: 40_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_sub_immidiate_set_flags_parse() {
    assert_eq!(
        parse_instruction("SUBIS X1, X2, 40"),
        IResult::Done(
            "",
            Instruction::SubtractImmediateSetFlags {
                n: Register::X2,
                m: 40_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_add_set_flags_parse() {
    assert_eq!(
        parse_instruction("ADDS X1, X2, X3"),
        IResult::Done(
            "",
            Instruction::AddSetFlags {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_subract_set_flags_parse() {
    assert_eq!(
        parse_instruction("SUBS X1, X2, X3"),
        IResult::Done(
            "",
            Instruction::SubtractSetFlags {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_add_immidiate_parse() {
    assert_eq!(
        parse_instruction("ADDI X1, X2, 40"),
        IResult::Done(
            "",
            Instruction::AddImmediate {
                n: Register::X2,
                m: 40_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_sub_immidiate_parse() {
    assert_eq!(
        parse_instruction("SUBI X1, X2, 40"),
        IResult::Done(
            "",
            Instruction::SubtractImmediate {
                n: Register::X2,
                m: 40_u16,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_add_parse() {
    assert_eq!(
        parse_instruction("ADD X1, X2, X3"),
        IResult::Done(
            "",
            Instruction::Add {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_subract_parse() {
    assert_eq!(
        parse_instruction("SUB X1, X2, X3"),
        IResult::Done(
            "",
            Instruction::Subtract {
                n: Register::X2,
                m: Register::X3,
                destination: Register::X1,
            }
        )
    );
}

#[test]
fn test_shift0_parse() {
    assert_eq!(parse_shift("LSL 0"), IResult::Done("", Shift16::Shift0,));
}

#[test]
fn test_shift16_parse() {
    assert_eq!(parse_shift("LSL 16"), IResult::Done("", Shift16::Shift16,));
}

#[test]
fn test_shift32_parse() {
    assert_eq!(parse_shift("LSL 32"), IResult::Done("", Shift16::Shift32,));
}

#[test]
fn test_shift48_parse() {
    assert_eq!(parse_shift("LSL 48"), IResult::Done("", Shift16::Shift48,));
}

#[test]
fn test_condition_equal_parse() {
    assert_eq!(parse_condition("EQ"), IResult::Done("", Condition::Equal));
}

#[test]
fn test_condition_not_equal_parse() {
    assert_eq!(
        parse_condition("NE"),
        IResult::Done("", Condition::NotEqual)
    );
}

#[test]
fn test_condition_higher_same_parse() {
    assert_eq!(
        parse_condition("HS"),
        IResult::Done("", Condition::HigherSame)
    );
}

#[test]
fn test_condition_lower_parse() {
    assert_eq!(parse_condition("LO"), IResult::Done("", Condition::Lower));
}

#[test]
fn test_condition_minus_parse() {
    assert_eq!(parse_condition("MI"), IResult::Done("", Condition::Minus));
}

#[test]
fn test_condition_positive_zero_parse() {
    assert_eq!(
        parse_condition("PL"),
        IResult::Done("", Condition::PositiveZero)
    );
}

#[test]
fn test_condition_signed_overflow_parse() {
    assert_eq!(
        parse_condition("VS"),
        IResult::Done("", Condition::SignedOverflow)
    );
}

#[test]
fn test_condition_no_signed_overflow_parse() {
    assert_eq!(
        parse_condition("VC"),
        IResult::Done("", Condition::NoSignedOverflow)
    );
}

#[test]
fn test_condition_higher_parse() {
    assert_eq!(parse_condition("HI"), IResult::Done("", Condition::Higher));
}

#[test]
fn test_condition_lower_same_parse() {
    assert_eq!(
        parse_condition("LS"),
        IResult::Done("", Condition::LowerSame)
    );
}

#[test]
fn test_condition_greater_than_equal_parse() {
    assert_eq!(
        parse_condition("GE"),
        IResult::Done("", Condition::GreaterThanEqual)
    );
}

#[test]
fn test_condition_less_than_parse() {
    assert_eq!(
        parse_condition("LT"),
        IResult::Done("", Condition::LessThan)
    );
}

#[test]
fn test_condition_greater_than_parse() {
    assert_eq!(
        parse_condition("GT"),
        IResult::Done("", Condition::GreaterThan)
    );
}

#[test]
fn test_condition_less_than_equal_parse() {
    assert_eq!(
        parse_condition("LE"),
        IResult::Done("", Condition::LessThanEqual)
    );
}

#[test]
fn test_condition_always_parse() {
    assert_eq!(parse_condition("AL"), IResult::Done("", Condition::Always));
}

#[test]
fn test_condition_reserved_parse() {
    assert_eq!(
        parse_condition("NV"),
        IResult::Done("", Condition::Reserved)
    );
}

#[test]
fn test_register_x1_parse() {
    assert_eq!(parse_register("X0"), IResult::Done("", Register::X0));
}
#[test]
fn test_register_x2_parse() {
    assert_eq!(parse_register("X1"), IResult::Done("", Register::X1));
}
#[test]
fn test_register_x3_parse() {
    assert_eq!(parse_register("X2"), IResult::Done("", Register::X2));
}
#[test]
fn test_register_x4_parse() {
    assert_eq!(parse_register("X3"), IResult::Done("", Register::X3));
}
#[test]
fn test_register_x5_parse() {
    assert_eq!(parse_register("X4"), IResult::Done("", Register::X4));
}
#[test]
fn test_register_x6_parse() {
    assert_eq!(parse_register("X5"), IResult::Done("", Register::X5));
}
#[test]
fn test_register_x7_parse() {
    assert_eq!(parse_register("X6"), IResult::Done("", Register::X6));
}
#[test]
fn test_register_x8_parse() {
    assert_eq!(parse_register("X7"), IResult::Done("", Register::X7));
}
#[test]
fn test_register_x9_parse() {
    assert_eq!(parse_register("X8"), IResult::Done("", Register::X8));
}
#[test]
fn test_register_x10_parse() {
    assert_eq!(parse_register("X9"), IResult::Done("", Register::X9));
}
#[test]
fn test_register_x11_parse() {
    assert_eq!(parse_register("X10"), IResult::Done("", Register::X10));
}
#[test]
fn test_register_x12_parse() {
    assert_eq!(parse_register("X11"), IResult::Done("", Register::X11));
}
#[test]
fn test_register_x13_parse() {
    assert_eq!(parse_register("X12"), IResult::Done("", Register::X12));
}
#[test]
fn test_register_x14_parse() {
    assert_eq!(parse_register("X13"), IResult::Done("", Register::X13));
}
#[test]
fn test_register_x15_parse() {
    assert_eq!(parse_register("X14"), IResult::Done("", Register::X14));
}
#[test]
fn test_register_x16_parse() {
    assert_eq!(parse_register("X15"), IResult::Done("", Register::X15));
}
#[test]
fn test_register_x17_parse() {
    assert_eq!(parse_register("X16"), IResult::Done("", Register::X16));
}
#[test]
fn test_register_x18_parse() {
    assert_eq!(parse_register("X17"), IResult::Done("", Register::X17));
}
#[test]
fn test_register_x19_parse() {
    assert_eq!(parse_register("X18"), IResult::Done("", Register::X18));
}
#[test]
fn test_register_x20_parse() {
    assert_eq!(parse_register("X19"), IResult::Done("", Register::X19));
}
#[test]
fn test_register_x21_parse() {
    assert_eq!(parse_register("X20"), IResult::Done("", Register::X20));
}
#[test]
fn test_register_x22_parse() {
    assert_eq!(parse_register("X21"), IResult::Done("", Register::X21));
}
#[test]
fn test_register_x23_parse() {
    assert_eq!(parse_register("X22"), IResult::Done("", Register::X22));
}
#[test]
fn test_register_x24_parse() {
    assert_eq!(parse_register("X23"), IResult::Done("", Register::X23));
}
#[test]
fn test_register_x25_parse() {
    assert_eq!(parse_register("X24"), IResult::Done("", Register::X24));
}
#[test]
fn test_register_x26_parse() {
    assert_eq!(parse_register("X25"), IResult::Done("", Register::X25));
}
#[test]
fn test_register_x27_parse() {
    assert_eq!(parse_register("X26"), IResult::Done("", Register::X26));
}
#[test]
fn test_register_x28_parse() {
    assert_eq!(parse_register("X27"), IResult::Done("", Register::X27));
}
#[test]
fn test_register_x29_parse() {
    assert_eq!(parse_register("X28"), IResult::Done("", Register::X28));
}
#[test]
fn test_register_x30_parse() {
    assert_eq!(parse_register("X29"), IResult::Done("", Register::X29));
}
#[test]
fn test_register_x31_parse() {
    assert_eq!(parse_register("X30"), IResult::Done("", Register::X30));
}

#[test]
fn test_register_xzr_parse() {
    assert_eq!(parse_register("XZR"), IResult::Done("", Register::XZR));
}
