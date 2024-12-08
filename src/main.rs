use std::env;
use std::fs;

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();

    // check for filename
    if args.len() < 2 {
        println!("No file name specified!");
        return;
    }

    // get filename from the first argument
    let file_path = &args[1];
    let buffer = fs::read_to_string(file_path).unwrap();
    let lines = buffer.lines();
    let mut part1_sum = 0;
    
    for line in lines {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let test_value_string = parts[0];
        let test_value = test_value_string.parse::<u64>().unwrap();
        let operator_strings = parts[1].split(" ").collect::<Vec<&str>>();
        let mut operators: Vec<u64> = Vec::new();

        for operator_string in operator_strings {
            operators.push(operator_string.parse::<u64>().unwrap());
        }
        let solutions = find_all_combinations(&operators, test_value);

        if solutions.len() > 0 {
            part1_sum += test_value;
        }
    }

    println!("part1: {}", part1_sum);
}

/// Represents the type of operation for each number
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Multiply,
}


pub fn find_all_combinations(numbers: &[u64], target: u64) -> Vec<Vec<(u64, Operation)>> {
        
    // Vector to store all valid combinations
    let mut all_solutions = Vec::new();
    
    fn backtrack(
        numbers: &[u64], 
        target: u64, 
        current_value: u64, 
        current_index: usize,
        current_combination: &mut Vec<(u64, Operation)>,
        all_solutions: &mut Vec<Vec<(u64, Operation)>>
    ) {
        // Base case: if we've used all numbers
        if current_index == numbers.len() {
            // Check if we've reached the exact target
            if current_value == target {
                all_solutions.push(current_combination.clone());
            }
            return;
        }
        
        // Current number we're processing
        let num = numbers[current_index];
        
        // Try addition
        current_combination.push((num, Operation::Add));
        backtrack(
            numbers, 
            target, 
            current_value + num, 
            current_index + 1,
            current_combination,
            all_solutions
        );
        current_combination.pop();
        
        // Try multiplication
        current_combination.push((num, Operation::Multiply));
        backtrack(
            numbers, 
            target, 
            if current_value == 0 { num } else { current_value * num }, 
            current_index + 1,
            current_combination,
            all_solutions
        );
        current_combination.pop();
    }
    
    // Start the backtracking process
    let mut current_combination = Vec::new();
    backtrack(numbers, target, 0, 0, &mut current_combination, &mut all_solutions);
    
    all_solutions
}
