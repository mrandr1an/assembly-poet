use std::fmt::Display;

use super::constant::Constant;

pub trait Instruction<const A: usize = 2> {}

pub trait DX<C>
where
    C: Constant,
{
    fn generate(&self, arg: C) -> impl Display;
}

pub trait ResX {
    fn generate(&self, amount: usize) -> impl Display;
}

pub enum DeclareUninitialized {
    RESB,
    RESW,
    RESD,
    RESQ,
    REST,
    RESO,
    RESY,
    RESZ,
}

pub enum DeclareInitialized {
    DB,
    DW,
    DD,
    DQ,
    DT,
    DO,
    DY,
    DZ,
}

impl<C> DX<C> for DeclareInitialized
where
    C: Constant,
{
    fn generate(&self, arg: C) -> impl Display {
        match self {
            DeclareInitialized::DB => String::from("db ") + &arg.generate().to_string(),
            DeclareInitialized::DD => String::from("dd ") + &arg.generate().to_string(),
            DeclareInitialized::DW => String::from("dw ") + &arg.generate().to_string(),
            DeclareInitialized::DQ => String::from("dq ") + &arg.generate().to_string(),
            DeclareInitialized::DT => String::from("dw ") + &arg.generate().to_string(),
            DeclareInitialized::DO => String::from("do ") + &arg.generate().to_string(),
            DeclareInitialized::DY => String::from("dy ") + &arg.generate().to_string(),
            DeclareInitialized::DZ => String::from("dz ") + &arg.generate().to_string(),
        }
    }
}

impl ResX for DeclareUninitialized {
    fn generate(&self, amount: usize) -> impl Display {
        match self {
            DeclareUninitialized::RESB => String::from("resb ") + &amount.to_string(),
            DeclareUninitialized::RESW => String::from("resw ") + &amount.to_string(),
            DeclareUninitialized::RESD => String::from("resd ") + &amount.to_string(),
            DeclareUninitialized::RESQ => String::from("resq ") + &amount.to_string(),
            DeclareUninitialized::REST => String::from("rest ") + &amount.to_string(),
            DeclareUninitialized::RESO => String::from("reso ") + &amount.to_string(),
            DeclareUninitialized::RESY => String::from("resy ") + &amount.to_string(),
            DeclareUninitialized::RESZ => String::from("resz ") + &amount.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::DeclareInitialized;
    use crate::target::x86_64::constant::{CharacterConstant, Numeric};
    use crate::target::x86_64::instruction::DX;

    #[test]
    fn declare_characterconst() {
        let char_const: CharacterConstant = "hello".into();
        let db = DeclareInitialized::DB;
        assert_eq!(db.generate(char_const).to_string(), "db 'hello'");
    }

    #[test]
    fn declare_numeric() {
        let number: Numeric = 183.into();
        let db = DeclareInitialized::DB;
        assert_eq!(db.generate(number).to_string(), "db 0d183");
    }
}
