use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct OppositeNumber;

impl OppositeNumber {
    fn opposite(number: i32) -> i32 {
        -number
    }
}

impl Problem for OppositeNumber {
    fn problem_name(&self) -> &'static str {
        "Opposite number"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        Very simple, given a number, find its opposite (additive inverse).
        
        Examples:
        
        - 1: -1
        - 14: -14
        - -34: 34
        "#
    }

    fn kyu(&self) -> Kyu {
        Kyu::Kyu8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(OppositeNumber::opposite(1), -1);
        assert_eq!(OppositeNumber::opposite(-1), 1);
    }
}
