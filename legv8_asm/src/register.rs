use bit::Bit;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register {
    X0 = 0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    XZR,
}

impl From<Register> for [Bit; 5] {
    fn from(register: Register) -> [Bit; 5] {
        let register_number = register as usize as u8;

        let mut result = [Bit::Zero; 5];

        for i in 0..5 {
            result[i] = Bit::from_u8((register_number >> i as u8) & 1_u8).unwrap();
        }

        result
    }
}

#[test]
fn test_resgister_bits_zero() {
    assert_eq!(<[Bit; 5]>::from(Register::X0), [Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero]);
}

#[test]
fn test_resgister_bits_one() {
    assert_eq!(<[Bit; 5]>::from(Register::X1), [Bit::One, Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero]);
}

#[test]
fn test_resgister_bits_xrz() {
    assert_eq!(<[Bit; 5]>::from(Register::XZR), [Bit::One, Bit::One, Bit::One, Bit::One, Bit::One]);
}

