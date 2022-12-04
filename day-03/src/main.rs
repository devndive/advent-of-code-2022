#[cfg(test)]
mod tests {
    use crate::{prio, Rucksack};

    #[test]
    fn create_rucksack() {
        let r = Rucksack::new(String::from("vJrwpWtwJgWrhcsFMMfFFhFp"));

        assert_eq!(
            r.compartment_one
                .iter()
                .fold(String::from(""), |acc, i| format!("{}{}", acc, i)),
            String::from("JrpWtwvg")
        );
    }

    #[test]
    fn intersect() {
        let r = Rucksack::new(String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        let res = r
            .compartment_one
            .intersection(&r.compartment_two)
            .collect::<Vec<&char>>();

        assert_eq!(prio(*res[0]), 38);
    }
}

use std::{collections::{HashSet, HashMap}, fs};

struct Rucksack {
    compartment_one: HashSet<char>,
    compartment_two: HashSet<char>,
}

impl Rucksack {
    pub fn new(input: String) -> Self {
        let middle = input.len() / 2;
        println!("len: {}", middle);

        let chars = input
            .as_bytes()
            .chunks(middle)
            .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
            .collect::<Vec<&str>>();

        Self {
            compartment_one: HashSet::from_iter(chars[0].chars().collect::<Vec<char>>()),
            compartment_two: HashSet::from_iter(chars[1].chars().collect::<Vec<char>>()),
        }
    }
}

pub fn prio(input: char) -> i32 {
    if input.is_lowercase() {
        return input as i32 - 96;
    }

    input as i32 - 38
}

fn main_one() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();

    let rucksacks: Vec<Rucksack> = input
        .split("\n")
        .map(|rucksack| Rucksack::new(String::from(rucksack)))
        .collect();

    let v = rucksacks
        .iter()
        .map(|r| {
            let res = r
                .compartment_one
                .intersection(&r.compartment_two)
                .collect::<Vec<&char>>();
            prio(*res[0])
        })
        .sum::<i32>();
    println!("{}", v);
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();

    let rucksacks: Vec<&str> = input
        .split("\n")
        .collect();

    let t2 = rucksacks.chunks(3).map(|group| {
        let one: HashSet<char> = HashSet::from_iter(group[0].chars().collect::<Vec<char>>());
        let two: HashSet<char> = HashSet::from_iter(group[1].chars().collect::<Vec<char>>());
        let three: HashSet<char> = HashSet::from_iter(group[2].chars().collect::<Vec<char>>());

        let intersection = one.intersection(&two).collect::<Vec<&char>>();
        println!("{:?}", intersection);

        let mut inter_char = intersection[0];
        for c in intersection {
            if three.contains(c) {
                // we found you
                inter_char = c;

                break;
            }
        }



        println!("inter: {}", inter_char);
        prio(*inter_char)
    }).sum::<i32>();
    println!("{}", t2);

    /*
    let v = rucksacks
        .iter()
        .map(|r| {
            let res = r
                .compartment_one
                .intersection(&r.compartment_two)
                .collect::<Vec<&char>>();
            prio(*res[0])
        })
        .sum::<i32>();
    println!("{}", v);
    */
}
