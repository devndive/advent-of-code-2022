use std::collections::VecDeque;
use std::fs;

use regex::Regex;

struct Monkey {
    name: String,
    items: VecDeque<u128>,
    operation: String,
    test_divisible_by: u128,
    if_true_throw_to: usize,
    if_false_throw_to: usize,

    processing_counter: u128,
}

impl Monkey {
    pub fn from(input: &str) -> Self {
        let re_name = Regex::new(r"Monkey (\d+):").unwrap();

        let mut name = String::from("");
        let mut items: VecDeque<u128> = VecDeque::new();
        let mut operation = String::from("");
        let mut test_divisible_by = 0;
        let mut if_true_throw_to = 0;
        let mut if_false_throw_to = 0;

        for line in input.split("\n").collect::<Vec<&str>>() {
            if re_name.is_match(line) {
                let captures = re_name.captures(line).unwrap();
                name = captures[1].to_string();
            }

            if line.starts_with("  Starting items: ") {
                let parts = line.split("  Starting items: ").collect::<Vec<&str>>();

                items = parts[1]
                    .split(", ")
                    .map(|item| item.parse::<u128>().unwrap())
                    .collect::<VecDeque<u128>>();
            }

            if line.starts_with("  Operation: new = ") {
                let parts = line.split("  Operation: new = ").collect::<Vec<&str>>();

                operation = String::from(parts[1]);
            }

            if line.starts_with("  Test: divisible by ") {
                let parts = line.split("  Test: divisible by ").collect::<Vec<&str>>();

                test_divisible_by = parts[1].parse::<u128>().unwrap();
            }
            if line.starts_with("    If true: throw to monkey ") {
                let parts = line
                    .split("    If true: throw to monkey ")
                    .collect::<Vec<&str>>();

                if_true_throw_to = parts[1].parse::<usize>().unwrap();
            }

            if line.starts_with("    If false: throw to monkey ") {
                let parts = line
                    .split("    If false: throw to monkey ")
                    .collect::<Vec<&str>>();

                if_false_throw_to = parts[1].parse::<usize>().unwrap();
            }
        }

        Self {
            name: name,
            items,
            operation,
            test_divisible_by,
            if_true_throw_to,
            if_false_throw_to,
            processing_counter: 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let monkey_text = input.split("\n\n").collect::<Vec<&str>>();

    let mut monkeys = monkey_text
        .iter()
        .map(|group| Monkey::from(*group))
        .collect::<Vec<Monkey>>();

    let worry_modifier = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test_divisible_by);

    for round in 1..(10000 + 1) {
        for idx in 0..monkeys.len() {
            //for _ in monkey.items.iter_mut() {
            // let monkey = monkeys[idx];
            //println!("Monkey {}", monkeys[idx].name);
            let mut item_option = monkeys[idx].items.pop_front();
            while item_option.is_some() {
                let item = item_option.unwrap();
                // println!("Processing: {}", item);
                monkeys[idx].processing_counter += 1;
                //println!("  Item {}", item);

                let mut result = item;
                let process_step = monkeys[idx].operation.split(" ").collect::<Vec<&str>>();

                if process_step[1] == "+" {
                    if process_step[2] == "old" {
                        result += item;
                    } else {
                        result += process_step[2].parse::<u128>().unwrap();
                    }
                }

                if process_step[1] == "*" {
                    if process_step[2] == "old" {
                        result *= item;
                    } else {
                        result *= process_step[2].parse::<u128>().unwrap();
                    }
                }
                //println!("  Processed {}", result);

                result = result % worry_modifier;
                //println!("  Processed {} checked", result);

                let throw_true_idx = monkeys[idx].if_true_throw_to;
                let throw_false_idx = monkeys[idx].if_false_throw_to;

                let remainder = result % monkeys[idx].test_divisible_by;

                if remainder == 0 {
                    monkeys[throw_true_idx].items.push_back(result);
                } else {
                    monkeys[throw_false_idx].items.push_back(result);
                }

                item_option = monkeys[idx].items.pop_front();
            }
        }
        /*
        println!("== After round {} ==", round);
        for m in monkeys.iter() {
            println!(
                "Monkey {} inspected items {} times.",
                m.name, m.processing_counter
            );
        }
        println!(""); */
    }

    monkeys.sort_by_key(|m| m.processing_counter);

    let idx_one = monkeys.len() - 1;
    let idx_two = monkeys.len() - 2;
    println!(
        "{}*{}={}",
        monkeys[idx_one].processing_counter,
        monkeys[idx_two].processing_counter,
        monkeys[idx_two].processing_counter * monkeys[idx_one].processing_counter
    );
}
