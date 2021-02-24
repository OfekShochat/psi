// value.rs - Representation and operations of data types
use std::ops::{Add, Sub, Mul, Div, Neg, Rem, BitXor};
use round::round;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(f) => write!(fmt, "'{}'", round(*f, 5)),
        }
    }
}

impl Add for Value {
    type Output = Value;
    fn add(self, other: Value) -> Self::Output {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a + b),
            _ => panic!("Impossible Operation"),
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Self::Output {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a - b),
            _ => panic!("Impossible Operation"),
        }
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Self::Output {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a * b),
            _ => panic!("Impossible Operation"),
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, other: Value) -> Self::Output {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a / b),
            _ => panic!("Impossible Operation"),
        }
    }
}

impl Rem for Value {
    type Output = Value;
    fn rem(self, other: Value) -> Self::Output {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a % b),
            _ => panic!("Impossible Operation"),
        }
    }
}

impl BitXor for Value {
    type Output = Value;
    fn bitxor(self, other: Value) -> Self::Output {
        // Actually acts as a power operator, not a bitxor
        // We just want to trick rust into using it like this, for code clarity
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a.powf(b)),
            _ => panic!("Impossible Operation"),
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        match self {
            Self::Number(num) => Self::Number(-num),
            _ => panic!("Impossible Operation"),
        }
    }
}
