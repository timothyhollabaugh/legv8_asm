use bit::Bit;

#[derive(Debug, PartialEq)]
pub struct Immediate6(pub u8);

impl From<u8> for Immediate6 {
    fn from(number: u8) -> Immediate6 {
        Immediate6(number)
    }
}

impl From<Immediate6> for [Bit; 6] {
    fn from(immediate: Immediate6) -> [Bit; 6] {
        let mut result: [Bit; 6] = [Bit::Zero; 6];
        for i in 0..6 {
            result[i] = Bit::from_u8((immediate.0 >> i) as u8 & 1_u8).unwrap();
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Immediate9(pub u16);

impl From<u16> for Immediate9 {
    fn from(number: u16) -> Immediate9 {
        Immediate9(number)
    }
}

impl From<Immediate9> for [Bit; 9] {
    fn from(immediate: Immediate9) -> [Bit; 9] {
        let mut result: [Bit; 9] = [Bit::Zero; 9];
        for i in 0..9 {
            result[i] = Bit::from_u8((immediate.0 >> i) as u8 & 1_u8).unwrap();
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Immediate12(pub u16);

impl From<u16> for Immediate12 {
    fn from(number: u16) -> Immediate12 {
        Immediate12(number)
    }
}

impl From<Immediate12> for [Bit; 12] {
    fn from(immediate: Immediate12) -> [Bit; 12] {
        let mut result: [Bit; 12] = [Bit::Zero; 12];
        for i in 0..12 {
            result[i] = Bit::from_u8((immediate.0 >> i) as u8 & 1_u8).unwrap();
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Immediate16(pub u16);

impl From<u16> for Immediate16 {
    fn from(number: u16) -> Immediate16 {
        Immediate16(number)
    }
}

impl From<Immediate16> for [Bit; 16] {
    fn from(immediate: Immediate16) -> [Bit; 16] {
        let mut result: [Bit; 16] = [Bit::Zero; 16];
        for i in 0..16 {
            result[i] = Bit::from_u8((immediate.0 >> i) as u8 & 1_u8).unwrap();
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Immediate19(pub u32);

impl From<u32> for Immediate19 {
    fn from(number: u32) -> Immediate19 {
        Immediate19(number)
    }
}

impl From<Immediate19> for [Bit; 19] {
    fn from(immediate: Immediate19) -> [Bit; 19] {
        let mut result: [Bit; 19] = [Bit::Zero; 19];
        for i in 0..19 {
            result[i] = Bit::from_u8((immediate.0 >> i) as u8 & 1_u8).unwrap();
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Immediate26(pub u32);

impl From<u32> for Immediate26 {
    fn from(number: u32) -> Immediate26 {
        Immediate26(number)
    }
}

impl From<Immediate26> for [Bit; 26] {
    fn from(immediate: Immediate26) -> [Bit; 26] {
        let mut result: [Bit; 26] = [Bit::Zero; 26];
        for i in 0..26 {
            result[i] = Bit::from_u8((immediate.0 >> i) as u8 & 1_u8).unwrap();
        }
        result
    }
}
