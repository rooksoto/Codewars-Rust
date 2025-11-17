use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct SummingANumbersDigits;

impl SummingANumbersDigits {
    fn sum_digits(number: i32) -> i32 {
        number
            .abs()
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum::<u32>() as i32
    }
}

impl Problem for SummingANumbersDigits {
    fn problem_name(&self) -> &'static str {
        "Summing a number's digits"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        Write a function which takes a number as input and returns the sum of the absolute value of each of the number's decimal digits.

        For example: (Input --> Output)

        - 10 --> 1
        - 99 --> 18
        - -32 --> 5

        Let's assume that all numbers in the input will be integer values.
        "#
    }

    fn kyu(&self) -> Kyu {
        Kyu::Kyu7
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_digits_10() {
        let n = 10;
        let expected = 1;
        let actual = SummingANumbersDigits::sum_digits(n);
        assert_eq!(
            actual, expected,
            "\nsum_digits({}) should be {}, got {}",
            n, expected, actual
        );
    }

    #[test]
    fn test_sum_digits_99() {
        let n = 99;
        let expected = 18;
        let actual = SummingANumbersDigits::sum_digits(n);
        assert_eq!(
            actual, expected,
            "\nsum_digits({}) should be {}, got {}",
            n, expected, actual
        );
    }

    #[test]
    fn test_sum_digits_neg_32() {
        let n = -32;
        let expected = 5;
        let actual = SummingANumbersDigits::sum_digits(n);
        assert_eq!(
            actual, expected,
            "\nsum_digits({}) should be {}, got {}",
            n, expected, actual
        );
    }
}
