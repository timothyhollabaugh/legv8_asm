
/// A single bit
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bit {
    One,
    Zero,
}

impl Bit {
    pub fn from_u8(number: u8) -> Option<Bit> {
        match number {
            0 => Some(Bit::Zero),
            1 => Some(Bit::One),
            _ => None,
        }
    }

    pub fn from_bool(boolean: bool) -> Bit {
        match boolean {
            false => Bit::Zero,
            true => Bit::One,
        }
    }
}

#[macro_export]
macro_rules! bit_array {
    ($($x:expr),*) => {
        [ $( $crate::bit::Bit::from_u8($x as u8).unwrap(), )* ]
    };
}

//#[macro_export]
//macro_rules! bits {
//    ($($x:expr),*) => {
//        [ $( Some($x), )* ]
//    };
//}

/// Convert a bit to a boolean where
/// one => true and
/// zero => false
impl From<Bit> for bool {
    fn from(bit: Bit) -> bool {
        match bit {
            Bit::One => true,
            Bit::Zero => false,
        }
    }
}

/// Convert a bit to a u8
impl From<Bit> for u8 {
    fn from(bit: Bit) -> u8 {
        match bit {
            Bit::One => 1_u8,
            Bit::Zero => 0_u8,
        }
    }
}

#[test]
fn test_bits() {
    assert_eq!(bit_array![1, 0, 1, 1], [Bit::One, Bit::Zero, Bit::One, Bit::One]);
}

#[test]
fn test_bool_bit_true_from() {
    assert_eq!(Bit::One, Bit::from_bool(true));
}

#[test]
fn test_bool_bit_false_from() {
    assert_eq!(Bit::Zero, Bit::from_bool(false));
}

#[test]
fn test_u8_bit_true_from() {
    assert_eq!(Some(Bit::One), Bit::from_u8(1_u8));
}

#[test]
fn test_u8_bit_false_from() {
    assert_eq!(Some(Bit::Zero), Bit::from_u8(0_u8));
}

#[test]
fn test_u8_bit_none_from() {
    assert_eq!(None, Bit::from_u8(27_u8));
}

#[test]
fn test_bit_bool_true_from() {
    assert_eq!(bool::from(Bit::One), true);
}

#[test]
fn test_bit_bool_true_into() {
    let true_bit: bool = Bit::One.into();
    assert_eq!(true_bit, true);
}

#[test]
fn test_bit_bool_false_from() {
    assert_eq!(bool::from(Bit::Zero), false);
}

#[test]
fn test_bit_bool_false_into() {
    let false_bit: bool = Bit::Zero.into();
    assert_eq!(false_bit, false);
}

#[test]
fn test_bit_u8_true_from() {
    assert_eq!(u8::from(Bit::One), 1_u8);
}

#[test]
fn test_bit_u8_true_into() {
    let true_bit: u8 = Bit::One.into();
    assert_eq!(true_bit, 1_u8);
}

#[test]
fn test_bit_u8_false_from() {
    assert_eq!(u8::from(Bit::Zero), 0_u8);
}

#[test]
fn test_bit_u8_false_into() {
    let false_bit: u8 = Bit::Zero.into();
    assert_eq!(false_bit, 0_u8);
}

