mod kyu;
mod kyu7;
mod kyu8;
mod problem;

use crate::kyu7::summing_a_numbers_digits::SummingANumbersDigits;
use crate::kyu7::vowel_count::VowelCount;
use crate::kyu8::count_of_positives_sum_of_negatives::CountOfPositivesSumOfNegatives;
use crate::kyu8::invert_values::InvertValues;
use crate::kyu8::make_upper_case::MakeUpperCase;
use crate::kyu8::multiplication_table_for_number::MultiplicationTableForNumber;
use crate::kyu8::opposite_number::OppositeNumber;
use crate::kyu8::plural::Plural;
use crate::kyu8::quarter_of_the_year::QuarterOfTheYear;
use crate::kyu8::sum_of_positive::SumOfPositive;
use crate::kyu8::the_if_function::TheIfFunction;
use crate::problem::Problem;

fn main() {
    println!("         Codewars Rust          ");
    println!("_____________SOLVED_____________");
    // Kyu 8
    print_problem_solved(TheIfFunction);
    print_problem_solved(Plural);
    print_problem_solved(MakeUpperCase);
    print_problem_solved(MultiplicationTableForNumber);
    print_problem_solved(QuarterOfTheYear);
    print_problem_solved(OppositeNumber);
    print_problem_solved(SumOfPositive);
    print_problem_solved(CountOfPositivesSumOfNegatives);
    print_problem_solved(InvertValues);
    // Kyu 7
    print_problem_solved(SummingANumbersDigits);
    print_problem_solved(VowelCount);
}

fn print_problem_solved<P: Problem>(problem: P) {
    println!("✅ \"{}\", Kyu {}", problem.problem_name(), problem.kyu());
}
