use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct QuarterOfTheYear;

impl QuarterOfTheYear {
    fn quarter_of(month: u8) -> u8 {
        (month + 2) / 3
    }
}

impl Problem for QuarterOfTheYear {
    fn problem_name(&self) -> &'static str {
        "Quarter of the year"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        Given a month as an integer from 1 to 12, return to which quarter of the year it belongs as an integer number.

        For example: month 2 (February), is part of the first quarter; month 6 (June), is part of the second quarter; and month 11 (November), is part of the fourth quarter.

        Constraint:

        - 1 <= month <= 12
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
    fn basic() {
        assert_eq!(QuarterOfTheYear::quarter_of(3), 1, "Month 3 = quarter 1");
        assert_eq!(QuarterOfTheYear::quarter_of(8), 3, "Month 8 = quarter 3");
        assert_eq!(QuarterOfTheYear::quarter_of(11), 4, "Month 11 = quarter 4");
    }
}
