use std::{ascii::AsciiExt, ops::ShrAssign};

use super::token::Token;
const DEFAULT_N: usize = 0;
///Represents a constant as defined in
/// https://www.nasm.us/doc/nasmdoc3.html#section-3.4
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Constant<'a, const N: usize = DEFAULT_N> {
    Numeric { base: Base, value: u64 },
    CharacterString([char; N]),
    CharacterConstant([u8; N]),
    StringConstant(&'a str),
    UnicodeString {},
    FloatingPointConstant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Base {
    Hex,
    Dec,
    Octal,
    Binary,
}

impl From<u8> for Base {
    fn from(value: u8) -> Self {
        match value {
            10 => Base::Dec,
            16 => Base::Hex,
            8 => Base::Octal,
            2 => Base::Binary,
            _ => panic!("Non-usable base"),
        }
    }
}

impl<'a, const N: usize> PartialEq<Self> for Constant<'a, N> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Constant::Numeric {
                    base: _lb,
                    value: lv,
                },
                Constant::Numeric {
                    base: _rb,
                    value: rv,
                },
            ) => lv == rv,
            _ => panic!("Canot compare two different constants"),
        }
    }
}

impl<'a, const N: usize> PartialEq<u64> for Constant<'a, N> {
    fn eq(&self, other: &u64) -> bool {
        match (self, other) {
            (
                Constant::Numeric {
                    base: _lb,
                    value: lv,
                },
                rv,
            ) => lv == rv,
            _ => panic!("Canot compare two different constants"),
        }
    }
}

impl<'a, const N: usize> From<(u8, u64)> for Constant<'a, N> {
    fn from(value: (u8, u64)) -> Self {
        Constant::Numeric {
            base: Base::from(value.0),
            value: value.1,
        }
    }
}

impl<'a, const N: usize> From<u64> for Constant<'a, N> {
    fn from(value: u64) -> Self {
        Constant::Numeric {
            base: Base::Dec,
            value,
        }
    }
}

impl<'a, const N: usize> From<&'a str> for Constant<'a, N> {
    fn from(value: &'a str) -> Self {
        Constant::StringConstant(value)
    }
}

impl<'a, const N: usize> From<[char; N]> for Constant<'a, N> {
    fn from(value: [char; N]) -> Self {
        Constant::CharacterString(value)
    }
}

impl<'a, const N: usize> From<[u8; N]> for Constant<'a, N> {
    fn from(value: [u8; N]) -> Self {
        Constant::CharacterConstant(value)
    }
}

impl<'a, const N: usize> Token for Constant<'a, N> {
    fn generate(&self) -> impl std::fmt::Display {
        match self {
            Constant::Numeric { base: b, value: v } => match b {
                Base::Dec => String::from("0d") + &v.to_string(),
                Base::Hex => String::from("0x") + &format!("{:x}", v),
                Base::Octal => String::from("0o") + &format!("{:o}", v),
                Base::Binary => String::from("0b") + &format!("{:b}", v),
            },
            Constant::CharacterString(char_arr) => char_arr.iter().collect(),
            Constant::CharacterConstant(byte_arr) => {
                String::from_utf8(byte_arr.to_vec()).expect("Bad")
            }
            Constant::StringConstant(s) => s.to_string(),
            e => panic!("Could not convert this {:#?}", e),
        }
    }
}

#[cfg(test)]
pub mod constant_tests {
    use crate::target::x86_64::{constant::Constant, token::Token};

    #[test]
    fn numeric() {
        let c1: Constant = Constant::from((16, 10));
        let c2: Constant = Constant::from(5);
        assert!(c1 == 10);
        assert!(c2 == 5);
    }

    #[test]
    fn token_trait() {
        let number_10 = "0d10";
        let c1: Constant = Constant::from(10);
        assert_eq!(number_10, c1.generate().to_string());

        let number_420 = "0x1a4";
        let c2: Constant = Constant::from((16, 420));
        assert_eq!(number_420, c2.generate().to_string());

        let number_69 = "0b1000101";
        let c3: Constant = Constant::from((2, 69));
        assert_eq!(number_69, c3.generate().to_string());

        let abcd = "abcd";
        let c4: Constant<4> = Constant::from(['a', 'b', 'c', 'd']);
        assert_eq!(abcd, c4.generate().to_string());

        let efg = "efg";
        let c5: Constant<3> = Constant::from([101, 102, 103]);
        assert_eq!(efg, c5.generate().to_string());
    }
}
