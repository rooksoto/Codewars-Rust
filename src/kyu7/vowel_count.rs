use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct VowelCount;

impl VowelCount {
    fn get_count(string: &str) -> usize {
        string
            .chars()
            .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .count()
    }
}

impl Problem for VowelCount {
    fn problem_name(&self) -> &'static str {
        "Vowel Count"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        Return the number (count) of vowels in the given string.

        We will consider a, e, i, o, u as vowels for this Kata (but not y).

        The input string will only consist of lower case letters and/or spaces.
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
    fn my_tests() {
        assert_eq!(VowelCount::get_count("abracadabra"), 5);
    }
}
