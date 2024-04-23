use std::fmt::Display;

pub trait Token {
    fn generate(&self) -> impl Display;
}
