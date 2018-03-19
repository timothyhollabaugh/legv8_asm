use bit::Bit;

#[derive(Debug, PartialEq)]
pub enum Condition {
    Equal = 0,
    NotEqual,
    HigherSame,
    Lower,
    Minus,
    PositiveZero,
    SignedOverflow,
    NoSignedOverflow,
    Higher,
    LowerSame,
    GreaterThanEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    Always,
    Reserved,
}

impl From<Condition> for [Bit; 5] {
    fn from(condition: Condition) -> [Bit; 5] {
        let condition_number = condition as usize as u8;

        let mut result = [Bit::Zero; 5];

        for i in 0..5 {
            result[i] = Bit::from_u8((condition_number >> i as u8) & 1_u8).unwrap();
        }

        result
    }
}

#[test]
fn test_condition_eq() {
    assert_eq!(<[Bit; 5]>::from(Condition::Equal), [Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero]);
}

#[test]
fn test_condition_hi() {
    assert_eq!(<[Bit; 5]>::from(Condition::Higher), [Bit::Zero, Bit::Zero, Bit::Zero, Bit::One, Bit::Zero]);
}

