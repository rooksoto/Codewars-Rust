use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct SumOfPositive;

impl SumOfPositive {
    fn positive_sum(slice: &[i32]) -> i32 {
        slice.iter().filter(|&&x| x.is_positive()).sum()
    }
}

impl Problem for SumOfPositive {
    fn problem_name(&self) -> &'static str {
        "Sum of positive"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        Task

        You get an array of numbers, return the sum of all of the positives ones.

        Example

        - [1, -4, 7, 12] => 1 + 7 + 12 = 20

        Note

        If there is nothing to sum, the sum is default to 0.
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
    fn some_examples() {
        assert_eq!(SumOfPositive::positive_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(SumOfPositive::positive_sum(&[1, -2, 3, 4, 5]), 13);
        assert_eq!(SumOfPositive::positive_sum(&[-1, 2, 3, 4, -5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(SumOfPositive::positive_sum(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(SumOfPositive::positive_sum(&[-1, -2, -3, -4, -5]), 0);
    }
}
