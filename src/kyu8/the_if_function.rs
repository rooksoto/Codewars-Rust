use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct TheIfFunction;

impl TheIfFunction {
    fn _if<T, F1, F2>(cond: bool, mut then: F1, mut els: F2) -> T
    where
        F1: FnMut() -> T,
        F2: FnMut() -> T,
    {
        if cond { then() } else { els() }
    }
}

impl Problem for TheIfFunction {
    fn problem_name(&self) -> &'static str {
        "The 'if' function"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        Create a function that takes three arguments:

        1. a value to be evaluated for truthiness.
        2. a function to execute if the first argument is truthy.
        3. a function to execute if the first argument is falsy.

        If the first argument evaluates to truthy, call the second argument (a function). If it evaluates to falsy, call the third argument instead (also a function).

        In statically-typed languages, the first argument will be a boolean. In dynamically-typed languages that attribute a truth value to all expressions, it may be of any type.
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
    fn should_support_true() {
        assert_eq!(TheIfFunction::_if(true, || 1, || 2), 1);
    }

    #[test]
    fn should_support_false() {
        assert_eq!(TheIfFunction::_if(false, || 1, || 2), 2);
    }

    #[test]
    fn should_support_false_other_type() {
        assert_eq!(TheIfFunction::_if(false, || 'a', || 'b'), 'b');
        assert_eq!(TheIfFunction::_if(true, || "True", || "False"), "True");
    }

    #[test]
    fn should_support_closures() {
        let mut true_was_set = false;
        let result = TheIfFunction::_if(true, || true_was_set = true, || panic!("Fail"));
        assert!(true_was_set);
        assert_eq!(result, ())
    }
}
