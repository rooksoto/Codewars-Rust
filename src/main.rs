mod kyu;
mod problem;

use crate::problem::Problem;

fn main() {
    println!("         Codewars Rust          ");
    println!("_____________SOLVED_____________");
}

fn print_problem_solved<P: Problem>(problem: P) {
    println!("✅ \"{}\", Kyu {}", problem.problem_name(), problem.kyu());
}
