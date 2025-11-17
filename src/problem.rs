use crate::kyu;

pub trait Problem {
    fn problem_name(&self) -> &'static str;
    fn problem_description(&self) -> &'static str;
    fn kyu(&self) -> kyu::Kyu;
}
