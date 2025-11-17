use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct Plural;

impl Plural {
    fn plural(n: f64) -> bool {
        n != 1.0
    }
}

impl Problem for Plural {
    fn problem_name(&self) -> &'static str {
        "Plural"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        We need a simple function that determines if a plural is needed or not. It should take a number, and return true if a plural should be used with that number or false if not. This would be useful when printing out a string such as 5 minutes, 14 apples, or 1 sun.

        You only need to worry about english grammar rules for this kata, where anything that isn't singular (one of something), it is plural (not one of something).

        All values will be positive integers or floats, or zero.
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
    fn test_plural() {
        assert_eq!(Plural::plural(0.0), true);
        assert_eq!(Plural::plural(0.5), true);
        assert_eq!(Plural::plural(1.0), false);
        assert_eq!(Plural::plural(100.0), true);
    }
}
