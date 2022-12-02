use std::collections::HashMap;
use std::fs::File;

use std::io::{BufRead, BufReader};

use std::collections::binary_heap::BinaryHeap;


/*
    Function to solve problem 1 of advent of code 2022:
    https://adventofcode.com/2022/day/1

    Params: 
        - filename: an optional name to a file from which to 
                    read problem input

    Output:
        - solution to problem 1

*/
pub fn solve_problem_1(filename: Option<&str>, num_elves: i32) -> i32{
    let data = parse_file(filename.unwrap_or("src/inputs/problem_1.txt"));
    count_calories(data, num_elves)

}

fn count_calories(data: HashMap<i32, Vec<i32>>, num_elves: i32) -> i32 {
    let mut heap = BinaryHeap::new();

    for val in data.values() {
        heap.push(val.iter().sum::<i32>())
    }

    let mut cal_sum = 0;

    for _ in 0..num_elves {
        match heap.pop() {
            Some(x) => cal_sum += x,
            None => panic!("Less than {} elves in the group", num_elves),

        }
    }

    cal_sum
}

fn parse_file(filename: &str) -> HashMap<i32, Vec<i32>> {
    let file = File::open(filename).expect("file wasn't found.");
    let lines = BufReader::new(file).lines();

    let mut data: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut current_elf = 0;

    for line in lines{
        if let Ok(next) = line {
            if !next.is_empty() {
                data.entry(current_elf).or_insert(Vec::new()).push(next.parse::<i32>().unwrap());
            } else {
                current_elf+=1;
            }
        }
    }


    data
}