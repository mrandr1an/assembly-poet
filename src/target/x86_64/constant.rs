use std::fmt::Display;

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

struct StringConstant<'a>(&'a str);

pub trait Constant {
    fn generate(&self) -> impl Display;
}

pub struct List<C, const N: usize>
where
    C: Constant,
{
    constants: [C; N],
}

impl Constant for Numeric {
    fn generate(&self) -> impl Display {
        match self.base {
            Base::Binary => String::from("0b") + &self.value.to_string(),
            Base::Octal => String::from("0o") + &self.value.to_string(),
            Base::Dec => String::from("0d") + &self.value.to_string(),
            Base::Hex => String::from("0x") + &self.value.to_string(),
        }
    }
}

impl<'a> Constant for CharacterString<'a> {
    fn generate(&self) -> impl Display {
        String::from("`") + self.value + &String::from("`")
    }
}

impl<'a> Constant for CharacterConstant<'a> {
    fn generate(&self) -> impl Display {
        String::from("\'") + self.value + &String::from("\'")
    }
}

impl<C, const N: usize> Constant for List<C, N>
where
    C: Constant,
{
    fn generate(&self) -> impl Display {
        let mut open_paren = String::from("%(");
        let mut it = self.constants.iter().peekable();
        while let Some(constant) = it.next() {
            open_paren = open_paren + &constant.generate().to_string();
            if it.peek().is_none() {
                open_paren += ")";
                break;
            } else {
                open_paren += ",";
            }
        }
        open_paren
    }
}
