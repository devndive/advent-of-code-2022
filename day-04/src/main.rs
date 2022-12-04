#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::{contains};

    #[test]
    fn contain_should_work() {
        assert_eq!(contains(2, 4, 2, 8), true);
        assert_eq!(contains(2, 4, 6, 8), false);
        assert_eq!(contains(2, 6, 4, 8), false);
        assert_eq!(contains(6, 6, 4, 6), true);
        assert_eq!(contains(4, 6, 6, 6), false);
        // 21-82,22-81
        assert_eq!(contains(21, 82, 22, 81), false);
        // 21-82,22-81
        assert_eq!(contains(22, 81, 21, 82), true);
        // 39..96 - 39..97
        assert_eq!(contains(39, 96, 39, 97), true);
        assert_eq!(contains(6, 6, 6, 6), true);
    }
}

use std::{fs, ops::Range};

fn contains(start_one: i32, end_one: i32, start_two: i32, end_two: i32) -> bool {
    range_contains_range(
        &mut Range {
            start: start_one,
            end: end_one + 1,
        },
        Range {
            start: start_two,
            end: end_two + 1,
        },
    )
}

fn range_contains_range(first: &mut Range<i32>, second: Range<i32>) -> bool {
    print!("{:?} - {:?}", first, second);
    let res = first.any(|item| second.contains(&item));
    println!(" = {}", res);

    res
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut counter = 0;
    for line in lines {
        let elves = line.split(",").collect::<Vec<&str>>();
        let elve_one = elves[0];
        let elve_two = elves[1];

        let eo = elve_one.split("-").collect::<Vec<&str>>();
        let et = elve_two.split("-").collect::<Vec<&str>>();

        if contains(
            eo[0].parse::<i32>().unwrap(),
            eo[1].parse::<i32>().unwrap(),
            et[0].parse::<i32>().unwrap(),
            et[1].parse::<i32>().unwrap(),
        ) || contains(
            et[0].parse::<i32>().unwrap(),
            et[1].parse::<i32>().unwrap(),
            eo[0].parse::<i32>().unwrap(),
            eo[1].parse::<i32>().unwrap(),
        ) {
            counter += 1;
        }
    }
    println!("{}", counter);
}
