use std::{fs, vec};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let lines = input.split("\n\n").collect::<Vec<&str>>();

    let stacks = lines[0];
    let instructions = lines[1];

    // build stacks
    let rows_in_stacks = stacks.split("\n").collect::<Vec<&str>>();
    let number_of_stacks = rows_in_stacks
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let number_of_stacks = number_of_stacks.last().unwrap().parse::<usize>().unwrap();

    let mut stacks_with_crates: Vec<Vec<String>> = vec![];
    for _ in 0..number_of_stacks {
        stacks_with_crates.push(vec![]);
    }

    // start from the bottom, skip the stack indexes
    for row in rows_in_stacks.iter().rev().skip(1) {
        for idx in 0..number_of_stacks {
            // peak into each position, if it contains a character
            // we found a crate
            let crate_indicator = row.chars().nth((idx * 4) + 1).unwrap();
            if crate_indicator != ' ' {
                stacks_with_crates[idx].push(String::from(crate_indicator));
            }
        }
    }

    // Process instructions
    let re = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
    for instruction in instructions.split("\n") {
        let captures = re.captures(instruction).unwrap();

        let count = captures[1].parse::<usize>().unwrap();
        let from = captures[2].parse::<usize>().unwrap() - 1;
        let to = captures[3].parse::<usize>().unwrap() - 1;

        // moving for challenge one
        /*
        for _ in 1..count+1 {
            let temp = stacks_with_crates[from].pop().unwrap();
            stacks_with_crates[to].push(temp);
        }
        */

        let mut temp_stack: Vec<String> = vec![];
        for _ in 1..count + 1 {
            temp_stack.push(stacks_with_crates[from].pop().unwrap());
        }

        for _ in 1..count + 1 {
            let temp = temp_stack.pop().unwrap();
            stacks_with_crates[to].push(temp);
        }
    }

    let mut res = String::new();
    for idx in 0..number_of_stacks {
        res = format!("{}{}", res, stacks_with_crates[idx].pop().unwrap());
    }

    println!("{}", res);
}
