use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Register {
    Number(usize, usize),
    Receiver,
    Result,
}
impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Number(level, index) => write!(f, "r{}-{}", level, index),
            Register::Receiver => write!(f, "@receiver"),
            Register::Result => write!(f, "@result"),
        }
    }
}

impl PartialEq for Register {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Register::Result, Register::Number(0, 0)) => true,
            (Register::Result, Register::Result) => true,
            (Register::Receiver, Register::Number(0, 1)) => true,
            (Register::Receiver, Register::Receiver) => true,
            (Register::Number(0, 0), Register::Result) => true,
            (Register::Number(0, 1), Register::Receiver) => true,
            (Register::Number(a1, b1), Register::Number(a2, b2)) => a1 == a2 && b1 == b2,
            _ => false,
        }
    }
}
