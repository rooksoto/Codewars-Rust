use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Kyu {
    Kyu8 = 8,
    Kyu7 = 7,
    Kyu6 = 6,
    Kyu5 = 5,
    Kyu4 = 4,
    Kyu3 = 3,
    Kyu2 = 2,
    Kyu1 = 1,
}

impl Display for Kyu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i8)
    }
}
