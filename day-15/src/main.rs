#[cfg(test)]
mod tests {
    use crate::{calculate_points, Pair, Position};

    #[test]
    fn simple_test() {
        let p = Pair {
            sensor: Position { x: 0, y: -3 },
            distance: 7,
        };

        let points = calculate_points(&p);

        for y in -10..=10 {
            for x in -10..=10 {
                if points.contains(&Position { x, y }) {
                    print!("x");
                } else {
                    print!(".");
                }
            }

            println!("");
        }

        assert!(false);
    }
}

use std::{
    collections::{HashMap, HashSet},
    fs,
    thread::{self},
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

fn calculate_points(p: &Pair) -> Vec<Position> {
    let mut points_on_circle = vec![];

    for x in p.sensor.x - p.distance..=p.sensor.x + p.distance {
        // (x1 - x2).abs() + (y1 - y2).abs() = distance
        // x1 = p.sensor.x
        // y1 = p.sensor.y
        // x2 = x
        // y2 = ?
        // (0 - -5) + (0 - y2) = 5
        // => 5     + (-y2).abs() = 5
        // => y2 = 0

        // (0 - -4) + (0 - y2) = 5
        // 4 + (-y2).abs() = 5
        // y2 = 1

        let x_diff = (p.sensor.x - x).abs();
        let y = p.distance - x_diff;

        points_on_circle.push(Position { x: x, y: y });
        points_on_circle.push(Position { x: x, y: y * -1 });
    }

    return points_on_circle;
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

    let mut all_points = vec![];
    for p in pairs.iter() {
        let mut points = calculate_points(p);
        all_points.append(&mut points);
    }

    println!("{}", all_points.len());
    /*
    let mut min_x = 4000000;
    let mut max_x = 0;
    let mut min_y = 4000000;
    let mut max_y = 0;
    */

    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut min_y = 4000000;
    let mut max_y = 0;

    for point in all_points.iter() {
        let mut min_x = 4000000;
        let mut max_x = 0;

        if map.contains_key(&point.y) {
            let values = map.get(&point.y).unwrap();

            min_x = values.0;
            max_x = values.1;
        }

        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);

        map.insert(point.y, (min_x, max_x));
    }

    if min_y < 0 {
        min_y = 0;
    }

    if max_y > 4000000 {
        max_y = 4000000;
    }

    /*
    for y in -15..=15 {
        for x in -40..=40 {
            if all_points.contains(&Position { x, y }) {
                print!("x");
            } else {
                print!(".");
            }
        }

        println!("");
    }
    */

    /*
       let mut hash_map = vec![];
       for point in all_points {
           let pt = [
               (-1, -1),
               (0, -1),
               (1, -1),
               (-1, 0),
               (1, 0),
               (-1, 1),
               (0, 1),
               (1, 1),
           ];

           pt.iter().for_each(|t| {
               let x_1 = point.x + t.0;
               let y_1 = point.y + t.1;

               hash_map.push(Position { x: x_1, y: y_1 });
           });
       }
    */
    /*
    for y in -15..=15 {
        for x in -40..=40 {
            if hash_map.contains(&Position { x, y }) {
                print!("x");
            } else {
                print!(".");
            }
        }

        println!("");
    } */

    //println!("{}", hash_map.len());
    /*
    hash_map.iter().for_each(|p_check| {
        //if p_check.x >= lb && p_check.x <= rb && p_check.y >= lb && p_check.y <= rb {
        /*
        if p_check.x <= 20 && p_check.y <= 20 {
            if !pairs.iter().any(|p| p.is_in_range(p_check)) {
                println!("Found you: {:?}", p_check);
            }
        }
             */

        let pt = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let mut result = false;
        if p_check.x <= 20 && p_check.y <= 20 {
            result = pt.iter().all(|test| {
                return hash_map.contains(&Position {
                    x: p_check.x + test.0,
                    y: p_check.y + test.1,
                });
            });
        }

        if result {
            println!("Found you {:?}", p_check);
        }
    }); */

    for y in min_y - 1..=max_y + 1 {
        //for point_check in hash_map.iter() {
        //    if point_check.x == 14 {
        //        println!("Checking {}/{}", point_check.x, point_check.y);
        //    }
        let values = map.get(&min_y);
        if values.is_some() {
            let (mut min_x, mut max_x) = values.unwrap();

            if min_x < 0 {
                min_x = 0;
            }

            if max_x > 4000000 {
                max_x = 4000000;
            }

            println!("{y}: {min_x} - {max_x}");
            for x in min_x - 1..=max_x + 1 {
                if !pairs
                    .iter()
                    .any(|pair| pair.is_in_range(&Position { x, y }))
                {
                    let pt = [
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ];

                    let result = pt.iter().all(|test| {
                        return pairs.iter().any(|pair| {
                            return pair.is_in_range(&Position {
                                x: x + test.0,
                                y: y + test.1,
                            });
                        });
                    });

                    //let point_check_out_of_range = pairs.iter().any(|pair| pair.is_in_range(&point_check));

                    if result
                    /*&& !point_check_out_of_range*/
                    {
                        println!("Found {}/{}", x, y);
                        panic!("Done");
                    }
                }
            }
        }
    }
}
