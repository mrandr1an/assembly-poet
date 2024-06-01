use std::fmt::Display;

pub trait Token {
    fn generate(&self) -> impl Display;
}

pub trait Children<T> {}

pub trait Parent<T, const N: usize> {
    type Children: Children<T>;
    fn add(&mut self) -> &mut Self;
    fn get(&self, index: usize) -> T;
}
