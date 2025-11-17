use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct InvertValues;

impl InvertValues {
    fn invert(values: &[i32]) -> Vec<i32> {
        values.iter().map(|x| -x).collect()
    }
}

impl Problem for InvertValues {
    fn problem_name(&self) -> &'static str {
        "Invert values"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        Given a set of numbers, return the additive inverse of each. Each positive becomes negatives, and the negatives become positives.

        - [1, 2, 3, 4, 5] --> [-1, -2, -3, -4, -5]
        - [1, -2, 3, -4, 5] --> [-1, 2, -3, 4, -5]
        - [] --> []
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
    fn basic_tests() {
        assert_eq!(
            InvertValues::invert(&vec![1, 2, 3, 4, 5]),
            vec![-1, -2, -3, -4, -5]
        );
        assert_eq!(
            InvertValues::invert(&vec![1, -2, 3, -4, 5]),
            vec![-1, 2, -3, 4, -5]
        );
        assert_eq!(InvertValues::invert(&vec![]), vec![]);
    }
}
