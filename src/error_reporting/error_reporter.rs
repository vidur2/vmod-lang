use std::fmt::Display;

pub trait ErrorReport {
    fn print_error<E: Unwindable, T: Display + Literal>(error: E, literal: T);
}

pub trait Unwindable {
    fn get_value(&self) -> String;
}

pub trait Literal {
    fn get_line(&self) -> usize;
}
