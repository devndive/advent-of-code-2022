use std::fs;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();

    let mut calories_per_elve = input
        .split("\n\n")
        .map(|elve_with_snacks| {
            elve_with_snacks
                .split("\n")
                .fold(0, |acc, number| acc + number.parse::<i32>().unwrap())
        })
        .collect::<Vec<i32>>();

    calories_per_elve.sort_by(|a, b| b.cmp(a));

    println!(
        "{}",
        calories_per_elve[0..3].iter().sum::<i32>()
    );
}
