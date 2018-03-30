use bit::Bit;
use register::Register;
use shift16::Shift16;
use condition::Condition;
use immediate::Immediate6;
use immediate::Immediate9;
use immediate::Immediate12;
use immediate::Immediate16;
use immediate::Immediate19;
use immediate::Immediate26;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add {
        n: Register,
        m: Register,
        destination: Register
    },
    Subtract {
        n: Register,
        m: Register,
        destination: Register
    },

    AddImmediate {
        n: Register,
        m: Immediate12,
        destination: Register
    },
    SubtractImmediate {
        n: Register,
        m: Immediate12,
        destination: Register
    },

    AddSetFlags {
        n: Register,
        m: Register,
        destination: Register
    },
    SubtractSetFlags {
        n: Register,
        m: Register,
        destination: Register
    },

    AddImmediateSetFlags {
        n: Register,
        m: Immediate12,
        destination: Register
    },
    SubtractImmediateSetFlags {
        n: Register,
        m: Immediate12,
        destination: Register
    },

    Store {
        address: Register,
        offset: Immediate9,
        data: Register
    },
    Load {
        address: Register,
        offset: Immediate9,
        data: Register
    },

    StoreByte {
        address: Register,
        offset: Immediate9,
        data: Register
    },
    LoadByte {
        address: Register,
        offset: Immediate9,
        data: Register
    },

    MoveZero {
        immediate: Immediate16,
        shift: Shift16,
        destination: Register
    },
    MoveKeep {
        immediate: Immediate16,
        shift: Shift16,
        destination: Register
    },

    And {
        n: Register,
        m: Register,
        destination: Register
    },
    Or {
        n: Register,
        m: Register,
        destination: Register
    },
    Xor {
        n: Register,
        m: Register,
        destination: Register
    },

    AndImmediate {
        n: Register,
        m: Immediate12,
        destination: Register
    },
    OrImmediate {
        n: Register,
        m: Immediate12,
        destination: Register
    },
    XorImmediate {
        n: Register,
        m: Immediate12,
        destination: Register
    },

    AndSetFlags {
        n: Register,
        m: Register,
        destination: Register
    },
    AndImmediateSetFlags {
        n: Register,
        m: Immediate12,
        destination: Register
    },

    LogicalShiftRight {
        n: Register,
        m: Immediate6,
        destination: Register
    },
    LogicalShiftLeft {
        n: Register,
        m: Immediate6,
        destination: Register
    },

    CompareBranchZero {
        address: Immediate19,
        r: Register
    },
    CompareBranchNotZero {
        address: Immediate19,
        r: Register
    },

    ConditionalBranch {
        address: Immediate19,
        condition: Condition
    },

    Branch {
        address: Immediate26
    },
    BranchRegister {
        r: Register
    },
    BranchLink {
        address: Immediate26
    }
}

impl From<Instruction> for [Bit; 32] {
    fn from(instruction: Instruction) -> [Bit; 32] {
        let mut result: [Bit; 32] = [Bit::Zero; 32];

        match instruction {
            Instruction::Add {
                m: m,
                n: n,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[16..21].copy_from_slice(&<[Bit; 5]>::from(m));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1]);
            }

            Instruction::Subtract {
                m: m,
                n: n,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[16..21].copy_from_slice(&<[Bit; 5]>::from(m));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1]);
            }

            Instruction::AddImmediate {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..22].copy_from_slice(&<[Bit; 12]>::from(i));
                result[22..32].copy_from_slice(&bit_array![0, 0, 1, 0, 0, 0, 1, 0, 0, 1]);
            }
            Instruction::SubtractImmediate {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..22].copy_from_slice(&<[Bit; 12]>::from(i));
                result[22..32].copy_from_slice(&bit_array![0, 0, 1, 0, 0, 0, 1, 0, 1, 1]);
            }
            Instruction::AddSetFlags {
                n: n,
                m: m,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[16..21].copy_from_slice(&<[Bit; 5]>::from(m));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1]);
            }
            Instruction::SubtractSetFlags {
                n: n,
                m: m,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[16..21].copy_from_slice(&<[Bit; 5]>::from(m));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1]);
            }
            Instruction::AddImmediateSetFlags {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..22].copy_from_slice(&<[Bit; 12]>::from(i));
                result[22..32].copy_from_slice(&bit_array![0, 0, 1, 0, 0, 0, 1, 1, 0, 1]);
            }
            Instruction::SubtractImmediateSetFlags {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..22].copy_from_slice(&<[Bit; 12]>::from(i));
                result[22..32].copy_from_slice(&bit_array![0, 0, 1, 0, 0, 0, 1, 1, 1, 1]);
            }
            Instruction::Store {
                address: a,
                offset: o,
                data: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(a));
                result[12..21].copy_from_slice(&<[Bit; 9]>::from(o));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);
            }
            Instruction::Load {
                address: a,
                offset: o,
                data: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(a));
                result[12..21].copy_from_slice(&<[Bit; 9]>::from(o));
                result[21..32].copy_from_slice(&bit_array![0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1]);
            }
            Instruction::StoreByte {
                address: a,
                offset: o,
                data: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(a));
                result[12..21].copy_from_slice(&<[Bit; 9]>::from(o));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0]);
            }
            Instruction::LoadByte {
                address: a,
                offset: o,
                data: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(a));
                result[12..21].copy_from_slice(&<[Bit; 9]>::from(o));
                result[21..32].copy_from_slice(&bit_array![0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0]);
            }
            Instruction::MoveZero {
                immediate: i,
                shift: s,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..21].copy_from_slice(&<[Bit; 16]>::from(i));
                result[21..23].copy_from_slice(&<[Bit; 2]>::from(s));
                result[23..32].copy_from_slice(&bit_array![1, 0, 1, 0, 0, 1, 0, 1, 1]);
            }
            Instruction::MoveKeep {
                immediate: i,
                shift: s,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..21].copy_from_slice(&<[Bit; 16]>::from(i));
                result[21..23].copy_from_slice(&<[Bit; 2]>::from(s));
                result[23..32].copy_from_slice(&bit_array![1, 0, 1, 0, 0, 1, 1, 1, 1]);
            }
            Instruction::And {
                n: n,
                m: m,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[16..21].copy_from_slice(&<[Bit; 5]>::from(m));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1]);
            }
            Instruction::Or {
                n: n,
                m: m,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[16..21].copy_from_slice(&<[Bit; 5]>::from(m));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1]);
            }
            Instruction::Xor {
                n: n,
                m: m,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[16..21].copy_from_slice(&<[Bit; 5]>::from(m));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1]);
            }
            Instruction::AndImmediate {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..22].copy_from_slice(&<[Bit; 12]>::from(i));
                result[22..32].copy_from_slice(&bit_array![0, 0, 0, 1, 0, 0, 1, 0, 0, 1]);
            }
            Instruction::OrImmediate {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..22].copy_from_slice(&<[Bit; 12]>::from(i));
                result[22..32].copy_from_slice(&bit_array![0, 0, 0, 1, 0, 0, 1, 1, 0, 1]);
            }
            Instruction::XorImmediate {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..22].copy_from_slice(&<[Bit; 12]>::from(i));
                result[22..32].copy_from_slice(&bit_array![0, 0, 0, 1, 0, 0, 1, 0, 1, 1]);
            }
            Instruction::AndSetFlags {
                n: n,
                m: m,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[16..21].copy_from_slice(&<[Bit; 5]>::from(m));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1]);
            }
            Instruction::AndImmediateSetFlags {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..22].copy_from_slice(&<[Bit; 12]>::from(i));
                result[22..32].copy_from_slice(&bit_array![0, 0, 0, 1, 0, 0, 1, 1, 1, 1]);
            }
            Instruction::LogicalShiftRight {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..16].copy_from_slice(&<[Bit; 6]>::from(i));
                result[21..32].copy_from_slice(&bit_array![0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1]);
            }
            Instruction::LogicalShiftLeft {
                n: n,
                m: i,
                destination: d
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(d));
                result[5..10].copy_from_slice(&<[Bit; 5]>::from(n));
                result[10..16].copy_from_slice(&<[Bit; 6]>::from(i));
                result[21..32].copy_from_slice(&bit_array![1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1]);
            }
            Instruction::CompareBranchZero { address: a, r: r } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(r));
                result[5..24].copy_from_slice(&<[Bit; 19]>::from(a));
                result[24..32].copy_from_slice(&bit_array![0, 0, 1, 0, 1, 1, 0, 1]);
            }
            Instruction::CompareBranchNotZero { address: a, r: r } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(r));
                result[5..24].copy_from_slice(&<[Bit; 19]>::from(a));
                result[24..32].copy_from_slice(&bit_array![1, 0, 1, 0, 1, 1, 0, 1]);
            }
            Instruction::ConditionalBranch {
                address: a,
                condition: c
            } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(c));
                result[5..24].copy_from_slice(&<[Bit; 19]>::from(a));
                result[24..32].copy_from_slice(&bit_array![0, 0, 1, 0, 1, 0, 1, 0]);
            }
            Instruction::Branch { address: a } => {
                result[0..26].copy_from_slice(&<[Bit; 26]>::from(a));
                result[26..32].copy_from_slice(&bit_array![1, 0, 1, 0, 0, 0]);
            }
            Instruction::BranchRegister { r: r } => {
                result[0..5].copy_from_slice(&<[Bit; 5]>::from(r));
                result[21..32].copy_from_slice(&bit_array![0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1]);
            }
            Instruction::BranchLink { address: a } => {
                result[0..26].copy_from_slice(&<[Bit; 26]>::from(a));
                result[26..32].copy_from_slice(&bit_array![1, 0, 1, 0, 0, 1]);
            }
        };

        result
    }
}

#[test]
fn test_add() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::Add {
            m: Register::X1,
            n: Register::X2,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0,
            0, 0, 1
        ]
    );
}

#[test]
fn test_subtract() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::Subtract {
            m: Register::X1,
            n: Register::X2,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0,
            0, 1, 1
        ]
    );
}

#[test]
fn test_add_immediate() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::AddImmediate {
            m: Immediate12(3_u16),
            n: Register::X1,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            0, 0, 1
        ]
    );
}

#[test]
fn test_subtract_immediate() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::SubtractImmediate {
            m: Immediate12(3_u16),
            n: Register::X1,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            0, 1, 1
        ]
    );
}

#[test]
fn test_add_set_flags() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::AddSetFlags {
            m: Register::X1,
            n: Register::X2,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0,
            1, 0, 1
        ]
    );
}

#[test]
fn test_subtract_set_flags() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::SubtractSetFlags {
            m: Register::X1,
            n: Register::X2,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0,
            1, 1, 1
        ]
    );
}

#[test]
fn test_add_immediate_set_flags() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::AddImmediateSetFlags {
            m: Immediate12(3_u16),
            n: Register::X1,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            1, 0, 1
        ]
    );
}

#[test]
fn test_subtract_immediate_set_flags() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::SubtractImmediateSetFlags {
            m: Immediate12(3_u16),
            n: Register::X1,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            1, 1, 1
        ]
    );
}

#[test]
fn test_store() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::Store {
            address: Register::X1,
            offset: Immediate9(4_u16),
            data: Register::X2
        }),
        bit_array![
            0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 1
        ]
    );
}

#[test]
fn test_load() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::Load {
            address: Register::X1,
            offset: Immediate9(4_u16),
            data: Register::X2
        }),
        bit_array![
            0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1,
            1, 1, 1
        ]
    );
}

#[test]
fn test_store_byte() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::StoreByte {
            address: Register::X1,
            offset: Immediate9(4_u16),
            data: Register::X2
        }),
        bit_array![
            0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 0, 0
        ]
    );
}

#[test]
fn test_load_byte() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::LoadByte {
            address: Register::X1,
            offset: Immediate9(4_u16),
            data: Register::X2
        }),
        bit_array![
            0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1,
            1, 0, 0
        ]
    );
}

#[test]
fn test_move_zero() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::MoveZero {
           immediate: Immediate16(5_u16),
           shift: Shift16::Shift16,
           destination: Register::X1
       }),
        bit_array![
            1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1,
            0, 1, 1
        ]
    );
}

#[test]
fn test_move_keep() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::MoveKeep {
           immediate: Immediate16(5_u16),
           shift: Shift16::Shift16,
           destination: Register::X1
       }),
        bit_array![
            1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1,
            1, 1, 1
        ]
    );
}

#[test]
fn test_and() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::And{
            m: Register::X1,
            n: Register::X2,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 1
        ]
    );
}

#[test]
fn test_or() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::Or{
            m: Register::X1,
            n: Register::X2,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0,
            1, 0, 1
        ]
    );
}

#[test]
fn test_xor() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::Xor{
            m: Register::X1,
            n: Register::X2,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0,
            0, 1, 1
        ]
    );
}

#[test]
fn test_and_immediate() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::AndImmediate {
            m: Immediate12(3_u16),
            n: Register::X1,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 1
        ]
    );
}

#[test]
fn test_or_immediate() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::OrImmediate {
            m: Immediate12(3_u16),
            n: Register::X1,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
            1, 0, 1
        ]
    );
}

#[test]
fn test_xor_immediate() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::XorImmediate {
            m: Immediate12(3_u16),
            n: Register::X1,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
            0, 1, 1
        ]
    );
}

#[test]
fn test_and_set_flags() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::AndSetFlags {
            m: Register::X1,
            n: Register::X2,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0,
            1, 1, 1
        ]
    );
}

#[test]
fn test_and_immediate_set_flags() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::AndImmediateSetFlags {
            m: Immediate12(3_u16),
            n: Register::X1,
            destination: Register::X3
        }),
        bit_array![
            1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
            1, 1, 1
        ]
    );
}

#[test]
fn test_logical_shift_right() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::LogicalShiftRight {
            n: Register::X1,
            m: Immediate6(6_u8),
            destination: Register::X2
        }),
        bit_array![
            0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1,
            0, 1, 1
        ]
    );
}

#[test]
fn test_logical_shift_left() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::LogicalShiftLeft {
            n: Register::X1,
            m: Immediate6(6_u8),
            destination: Register::X2
        }),
        bit_array![
            0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1,
            0, 1, 1
        ]
    );
}

#[test]
fn test_compare_branch_zero() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::CompareBranchZero{
            address: Immediate19(12_u32),
            r: Register::X1
        }),
        bit_array![
            1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            1, 0, 1
        ]
    );
}

#[test]
fn test_compare_branch_not_zero() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::CompareBranchNotZero{
            address: Immediate19(12_u32),
            r: Register::X1
        }),
        bit_array![
            1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1,
            1, 0, 1
        ]
    );
}

#[test]
fn test_conditional_branch() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::ConditionalBranch{
            address: Immediate19(12_u32),
            condition: Condition::NoSignedOverflow
        }),
        bit_array![
            1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            0, 1, 0
        ]
    );
}

#[test]
fn test_branch() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::Branch{
            address: Immediate26(12_u32)
        }),
        bit_array![
            0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            0, 0, 0
        ]
    );
}

#[test]
fn test_branch_register() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::BranchRegister {
            r: Register::X1,
        }),
        bit_array![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1,
            0, 1, 1
        ]
    );
}

#[test]
fn test_branch_link() {
    assert_eq!(
        <[Bit; 32]>::from(Instruction::BranchLink{
            address: Immediate26(12_u32)
        }),
        bit_array![
            0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            0, 0, 1
        ]
    );
}
