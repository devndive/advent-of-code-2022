use std::{
    fs,
    thread::{self, Thread},
};

use regex::Regex;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Pair {
    // beacon: Position,
    sensor: Position,
    distance: i32,
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
        let sensor = Position::from(&matches[1], &matches[2]);
        let beacon = Position::from(&matches[3], &matches[4]);

        Self {
            sensor,
            // beacon,
            distance: (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs(),
        }
    }

    /*
    fn distance(&self) -> i32 {
        (self.sensor.x - self.beacon.x).abs() + (self.sensor.y - self.beacon.y).abs()
    } */

    fn is_in_range(&self, target: &Position) -> bool {
        (self.sensor.x - target.x).abs() + (self.sensor.y - target.y).abs() <= self.distance
    }
}

/*
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
        let distance = pair.distance;
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

            if pair.is_in_range(&pos) && !blocks.contains_key(&pos) {
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
*/

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let pairs = input
        .split("\n")
        .map(|row| Pair::from(row))
        .collect::<Vec<Pair>>();

    let lb = 0;
    let rb = 4000000;

    let range = (lb..=rb).collect::<Vec<i32>>();

    let windows = range.chunks((rb / 10) as usize);
    println!("Chunks: {} ", windows.len());

    let mut handles = vec![];
    for window in windows {
        println!("Hä");
        let p = pairs.clone();
        let owned_window = window.to_vec();

        let handle = thread::spawn(move || {
            println!("Thread - start {}", owned_window.len());
            for y in owned_window {
                println!("y: {}", y);
                for x in lb..=rb {
                    let pos = Position { x, y: y.clone() };

                    let any_in_range = p.iter().any(|p| p.is_in_range(&pos));
                    if !any_in_range {
                        panic!("Found ya! ({}/{})", x, y);
                    }
                }
            }
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    /*
    for y in lb..=rb {
        println!("y: {}", y);
        for x in lb..=rb {
            let pos = Position { x, y };

            let any_in_range = pairs.iter().any(|p| p.is_in_range(&pos));
            if !any_in_range {
                panic!("Found ya! ({}/{})", x, y);
            }
        }
    }
    */
}
