use super::token::Token;

enum Base {
    Binary,
    Octal,
    Dec,
    Hex,
}

pub struct Numeric {
    base: Base,
    value: u64,
}

impl From<u64> for Numeric {
    fn from(value: u64) -> Self {
        Numeric {
            base: Base::Dec,
            value,
        }
    }
}

impl From<(Base, u64)> for Numeric {
    fn from(value: (Base, u64)) -> Self {
        Numeric {
            base: value.0,
            value: value.1,
        }
    }
}

pub struct CharacterConstant<'a> {
    value: &'a str,
}

impl<'a> From<&'a str> for CharacterConstant<'a> {
    fn from(value: &'a str) -> Self {
        if value.len() <= 8 {
            CharacterConstant { value }
        } else {
            panic!("Value of character constant cannot be more than 8 bytes");
        }
    }
}

pub struct CharacterString<'a> {
    value: &'a str,
}

impl<'a> From<&'a str> for CharacterString<'a> {
    fn from(value: &'a str) -> Self {
        if value.chars().count() <= 8 {
            CharacterString { value }
        } else {
            panic!("Value of character string cannot be more than 8 characters");
        }
    }
}

type StringConstant<'a> = &'a str;
