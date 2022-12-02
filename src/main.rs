mod solution_1;


pub use crate::solution_1::solve_problem_1;

fn main() {
    println!("Hello, world!");
    println!("Day 1 - Part 1: {}", solve_problem_1(None, 1));
    println!("Day 1 - Part 2: {}", solve_problem_1(None, 3));
}
