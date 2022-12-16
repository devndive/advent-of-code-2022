#[cfg(test)]
mod Tests {
    use crate::{Pair, Position};

    #[test]
    fn first_test() {
        let p = Pair::from("Sensor at x=8, y=7: closest beacon is at x=2, y=10");
        assert_eq!(p.is_in_range(Position { x: -1, y: 7 }), true);
    }

    fn calc_boundaries() {
        // ---- 4 ----- Sensor
        // ---- 5 ----- 
        // -----8 -----

        // => min_y = bottom - sensor.y => 1
        // => max_y = top - sensor.y => 4

        // ---- 5 ----- 
        // ---- 6 ----- Sensor
        // -----8 -----

        // => min_y = bottom (5) - sensor.y (6) => -1
        // => max_y = top (8) - sensor.y (6) => 2

        // ---- 5 ----- 
        // ---- 6 ----- 
        // -----8 ----- Sensor

        // => min_y = bottom - sensor.y => -3
        // => max_y = top - sensor.y => -2
    }
}

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Pair {
    beacon: Position,
    sensor: Position,
}

impl Position {
    fn from(x: &str, y: &str) -> Self {
        Self {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
        }
    }
}

impl Pair {
    fn from(input: &str) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let matches = re.captures(input).unwrap();

        Self {
            sensor: Position::from(&matches[1], &matches[2]),
            beacon: Position::from(&matches[3], &matches[4]),
        }
    }

    fn distance(&self) -> i32 {
        (self.sensor.x - self.beacon.x).abs() + (self.sensor.y - self.beacon.y).abs()
    }

    fn is_in_range(&self, target: Position) -> bool {
        let distance = self.distance();
        (self.sensor.x - target.x).abs() + (self.sensor.y - target.y).abs() <= distance
    }
}

fn part1() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let pairs = input
        .split("\n")
        .map(|row| Pair::from(row))
        .collect::<Vec<Pair>>();

    let mut blocks = HashMap::new();
    let target_row = 2000000;
    //let target_row = 10;

    for pair in pairs.iter() {
        println!("Processing {}/{}", pair.sensor.x, pair.sensor.y);
        let distance = pair.distance();
        if pair.sensor.y == target_row {
            blocks.insert(pair.sensor, "S");
        }

        if pair.beacon.y == target_row {
            blocks.insert(pair.beacon, "B");
        }

        let mut y = target_row - pair.sensor.y;
        if pair.sensor.y > target_row {
            y = target_row - pair.sensor.y;
        }

        for x in -distance..=distance {
            //for y in -distance..=distance {
            let pos = Position {
                x: pair.sensor.x + x,
                y: pair.sensor.y + y,
            };

            if pair.is_in_range(pos) && !blocks.contains_key(&pos) {
                // println!("Adding a block");
                // There can be no beacon in this position
                blocks.insert(pos, "#");
            }
            //}
        }
    }

    let items_in_row = blocks.iter().filter(|item| {
        if item.1 == &"#" {
            return true;
        } else {
            return false;
        }
    });

    println!("Part 1: {}", items_in_row.count());
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let pairs = input
        .split("\n")
        .map(|row| Pair::from(row))
        .collect::<Vec<Pair>>();

    let LB = 0;
    let RB = 4000000;

    for pair in pairs.iter() {
        let distance = pair.distance();

        // let min_x = LB - pair.sensor.x + distance;
        // let max_x = pair.sensor.x + distance -RB;

        let mut min_x = LB - pair.sensor.x;
        if pair.sensor.x > LB  {
            min_x = LB - pair.sensor.x;
        }
        let mut max_x = RB - pair.sensor.x;
        if pair.sensor.x > RB  {
            max_x = RB - pair.sensor.x;
        }

        println!("{} - {}", min_x, max_x);
        for x in min_x..=max_x {
            //println!("Round {}", x);
            // let min_y = LB - pair.sensor.y + distance;
            // let max_y = pair.sensor.y + distance - RB;

            let mut min_y = LB - pair.sensor.y;
            let mut max_y = RB - pair.sensor.y;

        println!("  {} - {}", min_y, max_y);
            for y in min_y..=max_y {
                let all_out_of_range = pairs.iter().all(|p| !p.is_in_range(Position { x, y }));
                if all_out_of_range {
                    println!("Found ya! ({}/{})", x, y);
                }
            }
        }
    }
}
