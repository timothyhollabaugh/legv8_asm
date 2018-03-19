use bit::Bit;

#[derive(Debug, PartialEq)]
pub enum Shift16 {
    Shift0 = 0,
    Shift16,
    Shift32,
    Shift48,
}

impl From<Shift16> for [Bit; 2] {
    fn from(shift: Shift16) -> [Bit; 2] {
        let shift_number = shift as usize as u8;

        let mut result = [Bit::Zero; 2];

        for i in 0..2 {
            result[i] = Bit::from_u8((shift_number >> i as u8) & 1_u8).unwrap();
        }

        result
    }
}

#[test]
fn test_shift0() {
    assert_eq!(<[Bit; 2]>::from(Shift16::Shift0), [Bit::Zero, Bit::Zero]);
}

#[test]
fn test_shift32() {
    assert_eq!(<[Bit; 2]>::from(Shift16::Shift32), [Bit::Zero, Bit::One]);
}

