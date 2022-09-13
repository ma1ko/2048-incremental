use std::{fmt::Display, ops::Deref};

use crate::*;

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Number {
    pub value: Option<usize>,
}

impl Deref for Number {
    type Target = Option<usize>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(number) = self.value {
            if 1 << number >= 128
                && Dispatch::<UpgradeableBoard>::new()
                    .get()
                    .scientific_notation
                    .get()
            {
                write!(f, "2e{}", number)
            } else {
                write!(f, "{}", 1 << number)
            }
        } else {
            write!(f, " ")
        }
    }
}
impl From<usize> for Number {
    fn from(n: usize) -> Self {
        if n == 0 {
            Self::none()
        } else {
            Self::new(n)
        }
    }
}
impl From<Option<usize>> for Number {
    fn from(value: Option<usize>) -> Self {
            Self {value}
    }
}
impl Number {
    pub fn value(&self) -> usize {
        if let Some(value) = self.value {
            1 << value
        }
        else {
            0
        }
    }
    pub fn set(&mut self, value: Option<usize>){
        self.value = value;

    }
    pub fn scientific(value: usize) -> Self {
        Number { value: Some(value) }
    }
    pub fn new(value: usize) -> Self {
        assert!(value >= 2);
        let x = Number {
            value: Some(1 << value.trailing_zeros() - 1),
        };
        // log::info!("{} become 2e{}", value, x.value.unwrap());
        x
    }
    pub fn none() -> Self {
        Number { value: None }
    }
    pub fn color(&self) -> &'static str {
        match self.value.unwrap_or(0) {
            0 => "bg-blue-50",
            1 => "bg-blue-100",
            2 => "bg-blue-400",
            3 => "bg-blue-700",
            4 => "bg-red-100",
            5 => "bg-red-400",
            6 => "bg-red-700",
            7 => "bg-green-100",
            8 => "bg-green-400",
            9 => "bg-green-700",
            10 => "bg-black-100",
            11 => "bg-black-400",
            12 => "bg-black-700",
            13 => "bg-yellow-100",
            14 => "bg-yellow-400",
            15 => "bg-yellow-700",
            _ => "",
        }
    }
}
