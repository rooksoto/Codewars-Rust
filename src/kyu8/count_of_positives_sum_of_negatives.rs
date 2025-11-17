use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct CountOfPositivesSumOfNegatives;

impl CountOfPositivesSumOfNegatives {
    fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
        if input.is_empty() {
            return vec![];
        }
        let (count, sum) = input
            .iter()
            .fold((0, 0), |(count, sum), &x| match x.signum() {
                1 => (count + 1, sum),
                -1 => (count, sum + x),
                _ => (count, sum),
            });
        vec![count, sum]
    }
}

impl Problem for CountOfPositivesSumOfNegatives {
    fn problem_name(&self) -> &'static str {
        "Count of positives / sum of negatives"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        Given an array of integers.

        Return an array, where the first element is the count of positives numbers and the second element is sum of negative numbers. 0 is neither positive nor negative.

        If the input is an empty array or is null, return an empty array.

        Example

        For input [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], you should return [10, -65].
        "#
    }

    fn kyu(&self) -> Kyu {
        Kyu::Kyu8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a: &[i32], expected: &[i32]) {
        let actual = CountOfPositivesSumOfNegatives::count_positives_sum_negatives(a.to_vec());
        assert_eq!(
            actual, expected,
            "With input = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15],
            &[10, -65],
        );
        dotest(&[], &[]);
        dotest(
            &[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14],
            &[8, -50],
        );
        dotest(&[0, 1, 2, 3, 4, 5], &[5, 0]);
        dotest(&[1, 2, 3, 4, 5], &[5, 0]);
        dotest(&[0, -1, -2, -3, -4, -5], &[0, -15]);
        dotest(&[-1, -2, -3, -4, -5], &[0, -15]);
        dotest(&[0, 0, 0, 0], &[0, 0]);
        dotest(&[0], &[0, 0]);
    }
}
