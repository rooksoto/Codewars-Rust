use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct MakeUpperCase;

impl MakeUpperCase {
    fn make_upper_case(s: &str) -> String {
        String::from(s.to_uppercase())
    }
}

impl Problem for MakeUpperCase {
    fn problem_name(&self) -> &'static str {
        "MakeUpperCase"
    }

    fn problem_description(&self) -> &'static str {
        "Write a function which converts the input string to uppercase."
    }

    fn kyu(&self) -> Kyu {
        Kyu::Kyu8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_upper_case() {
        assert_eq!(MakeUpperCase::make_upper_case("hello"), "HELLO");
    }
}
