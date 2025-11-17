mod kyu;
mod kyu8;
mod problem;

use crate::kyu8::make_upper_case::MakeUpperCase;
use crate::kyu8::multiplication_table_for_number::MultiplicationTableForNumber;
use crate::kyu8::plural::Plural;
use crate::kyu8::quarter_of_the_year::QuarterOfTheYear;
use crate::kyu8::the_if_function::TheIfFunction;
use crate::problem::Problem;

fn main() {
    println!("         Codewars Rust          ");
    println!("_____________SOLVED_____________");
    print_problem_solved(TheIfFunction);
    print_problem_solved(Plural);
    print_problem_solved(MakeUpperCase);
    print_problem_solved(MultiplicationTableForNumber);
    print_problem_solved(QuarterOfTheYear);
}

fn print_problem_solved<P: Problem>(problem: P) {
    println!("✅ \"{}\", Kyu {}", problem.problem_name(), problem.kyu());
}
