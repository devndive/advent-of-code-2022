use core::fmt;
use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn from(input: &str) -> Self {
        let parts = input.split(",").collect::<Vec<&str>>();

        Self {
            x: parts[0].parse::<i32>().unwrap(),
            y: parts[1].parse::<i32>().unwrap(),
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}/{})", self.x, self.y)
    }
}

struct Path {
    positions: Vec<Position>,
}

impl Path {
    fn from(input: &str) -> Self {
        let parts = input
            .split(" -> ")
            .map(|pos| Position::from(pos))
            .collect::<Vec<Position>>();

        Self { positions: parts }
    }
}

fn part_one() {
    let input = fs::read_to_string("./src/test_input").unwrap();
    let paths = input
        .split("\n")
        .map(|input| Path::from(input))
        .collect::<Vec<Path>>();

    // Find minimum and maximum values
    // This will help drawing the field
    let mut min_x: i32 = 10000;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 10000;
    let mut max_y: i32 = 0;

    for path in paths.iter() {
        for pos in path.positions.iter() {
            min_x = pos.x.min(min_x);
            max_x = pos.x.max(max_x);
            min_y = pos.y.min(min_y);
            max_y = pos.y.max(max_y);
        }
    }

    min_x = 500.min(min_x);
    min_y = 0.min(min_y);

    println!("{} - {}", min_x, max_x);
    println!("{} - {}", min_y, max_y);

    // recalculate x and y values
    let paths = paths
        .iter()
        .map(|path| {
            return Path {
                positions: path
                    .positions
                    .iter()
                    .map(|pos| {
                        return Position {
                            x: pos.x - min_x,
                            y: pos.y - min_y,
                        };
                    })
                    .collect::<Vec<Position>>(),
            };
        })
        .collect::<Vec<Path>>();

    let sand_start = Position {
        x: 500 - min_x,
        y: 0,
    };

    let mut field: Vec<Vec<char>> = vec![];
    for _ in 0..(max_y - min_y + 1) {
        field.push((0..(max_x - min_x + 1)).map(|_| '.').collect::<Vec<char>>());
    }

    for path in paths.iter() {
        for window in path.positions.windows(2) {
            let pos_one = &window[0];
            let pos_two = &window[1];

            println!("{:?} - {:?}", pos_one, pos_two);

            // go left or right
            if pos_one.y == pos_two.y {
                println!("Y is equal");
                if pos_one.x < pos_two.x {
                    for x in 0..pos_two.x - pos_one.x + 1 {
                        field[pos_one.y as usize][(pos_one.x + x) as usize] = '#';
                    }
                } else {
                    for x in 0..pos_one.x - pos_two.x + 1 {
                        field[pos_one.y as usize][(pos_one.x - x) as usize] = '#';
                    }
                }
            } else {
                println!("X is equal");
                if pos_one.y < pos_two.y {
                    println!("One less than two");
                    for y in 0..pos_two.y - pos_one.y + 1 {
                        println!("Should mark ({}/{})", pos_one.x, pos_one.y + 1);
                        field[(pos_one.y + y) as usize][pos_one.x as usize] = '#';
                    }
                } else {
                    println!("Two less than one");
                    for y in 0..pos_one.y - pos_two.y + 1 {
                        field[(pos_one.y - y) as usize][pos_one.x as usize] = '#';
                    }
                }
            }
        }
    }

    field[sand_start.y as usize][sand_start.x as usize] = '+';

    print_field(&field);
    println!("");

    let mut is_sand_falling_into_void = false;
    while !is_sand_falling_into_void {
        let mut next_move = (sand_start.x, sand_start.y + 1);
        let mut can_move = true;
        while can_move {
            let possible_fields = [(0, 1), (-1, 1), (1, 1)];

            let any_out_of_bounds = possible_fields
                .iter()
                .any(|f| !is_in_bounds(next_move.0 + f.0, next_move.1 + f.1, &field));

            if any_out_of_bounds {
                println!("Sand is falling into the void");
                is_sand_falling_into_void = true;
                break;
            }

            let possible_next_move = possible_fields
                .iter()
                .find(|f| !is_blocked(next_move.0 + f.0, next_move.1 + f.1, &field));

            if possible_next_move.is_some() {
                let res = *possible_next_move.unwrap();
                next_move = (next_move.0 + res.0, next_move.1 + res.1);
                println!("Next move {:?}", next_move);
            } else {
                field[next_move.1 as usize][next_move.0 as usize] = 'o';
                can_move = false;
            }
        }
    }

    print_field(&field);
    println!("");

    println!(
        "Part 1: {}",
        field.iter().fold(0, |acc, row| acc
            + row
                .iter()
                .fold(0, |i_acc, field| i_acc + field_count(*field)))
    );
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let paths = input
        .split("\n")
        .map(|input| Path::from(input))
        .collect::<Vec<Path>>();

    // Find minimum and maximum values
    // This will help drawing the field
    //let mut min_x: i32 = 10000;
    //let mut max_x: i32 = 0;
    //let mut min_y: i32 = 10000;
    let mut max_y: i32 = 0;

    for path in paths.iter() {
        for pos in path.positions.iter() {
            //min_x = pos.x.min(min_x);
            //max_x = pos.x.max(max_x);
            //min_y = pos.y.min(min_y);
            max_y = pos.y.max(max_y);
        }
    }

    //min_x = 500.min(min_x);
    //min_y = 0.min(min_y);

    //println!("{} - {}", min_x, max_x);
    //println!("{} - {}", min_y, max_y);

    // recalculate x and y values
    /*let paths = paths
    .iter()
    .map(|path| {
        return Path {
            positions: path
                .positions
                .iter()
                .map(|pos| {
                    return Position {
                        x: pos.x - min_x,
                        y: pos.y - min_y,
                    };
                })
                .collect::<Vec<Position>>(),
        };
    })
    .collect::<Vec<Path>>();
    */

    let sand_start = Position { x: 500, y: 0 };

    let mut fields: HashMap<Position, char> = HashMap::new();
    //let mut field: Vec<Vec<char>> = vec![];
    //for _ in 0..(max_y - min_y + 1) {
    //    field.push((0..(max_x - min_x + 1)).map(|_| '.').collect::<Vec<char>>());
    //}

    for path in paths.iter() {
        for window in path.positions.windows(2) {
            let pos_one = &window[0];
            let pos_two = &window[1];

            println!("{:?} - {:?}", pos_one, pos_two);

            // go left or right
            if pos_one.y == pos_two.y {
                println!("Y is equal");
                if pos_one.x < pos_two.x {
                    for x in 0..pos_two.x - pos_one.x + 1 {
                        // field[pos_one.y as usize][(pos_one.x + x) as usize] = '#';
                        fields.insert(
                            Position {
                                x: pos_one.x + x,
                                y: pos_one.y,
                            },
                            '#',
                        );
                    }
                } else {
                    for x in 0..pos_one.x - pos_two.x + 1 {
                        // field[pos_one.y as usize][(pos_one.x - x) as usize] = '#';
                        fields.insert(
                            Position {
                                x: pos_one.x - x,
                                y: pos_one.y,
                            },
                            '#',
                        );
                    }
                }
            } else {
                if pos_one.y < pos_two.y {
                    for y in 0..pos_two.y - pos_one.y + 1 {
                        // field[(pos_one.y + y) as usize][pos_one.x as usize] = '#';
                        fields.insert(
                            Position {
                                x: pos_one.x,
                                y: pos_one.y + y,
                            },
                            '#',
                        );
                    }
                } else {
                    for y in 0..pos_one.y - pos_two.y + 1 {
                        // field[(pos_one.y - y) as usize][pos_one.x as usize] = '#';
                        fields.insert(
                            Position {
                                x: pos_one.x,
                                y: pos_one.y - y,
                            },
                            '#',
                        );
                    }
                }
            }
        }
    }

    fields.insert(
        Position {
            x: sand_start.x,
            y: sand_start.y,
        },
        '+',
    );

    let mut is_sand_at_start = false;
    while !is_sand_at_start {
        let mut next_move = (sand_start.x, sand_start.y);
        let mut can_move = true;
        while can_move {
            let possible_fields = [(0, 1), (-1, 1), (1, 1)];

            let possible_next_move = possible_fields
                .iter()
                .find(|f| !is_blocked_2(next_move.0 + f.0, next_move.1 + f.1, max_y, &fields));

            if possible_next_move.is_some() {
                let res = *possible_next_move.unwrap();
                next_move = (next_move.0 + res.0, next_move.1 + res.1);


                // We still have places to go
                // println!("Next move {:?}", next_move);
            } else {
                // field[next_move.1 as usize][next_move.0 as usize] = 'o';
                // println!("No more next move {:?}", next_move);

                // No more way to go, we stay here
                fields.insert(
                    Position {
                        x: next_move.0,
                        y: next_move.1,
                    },
                    'o',
                );

                // If our next move will land us on the starting point, we stop
                if next_move.0 == sand_start.x && next_move.1 == sand_start.y {
                    println!("Next move is {:?}, and that is our sand start", next_move);
                    is_sand_at_start = true;
                }

                can_move = false;
            }
        }
    }

    println!(
        "Part 2: {}",
        fields.iter().fold(0, |acc, row| acc + field_count(*row.1))
    );
}

fn field_count(field: char) -> i32 {
    if field == 'o' {
        1
    } else {
        0
    }
}
fn print_field(field: &Vec<Vec<char>>) {
    for y in 0..field.len() {
        for x in 0..field[0].len() {
            print!("{}", field[y][x]);
        }

        println!("");
    }
}

fn is_in_bounds(x: i32, y: i32, field: &Vec<Vec<char>>) -> bool {
    let width = field[0].len() as i32;
    let height = field.len() as i32;

    x >= 0 && x < width && y >= 0 && y < height
}

fn is_blocked(x: i32, y: i32, field: &Vec<Vec<char>>) -> bool {
    field[y as usize][x as usize] == '#' || field[y as usize][x as usize] == 'o'
}

fn is_blocked_2(x: i32, y: i32, max_y: i32, fields: &HashMap<Position, char>) -> bool {
    if y == max_y + 2 {
        return true;
    }

    fields.contains_key(&Position { x, y })

    // fields.iter().any(|(field, _)| field.x == x && field.y == y)
}

fn is_in_bounds_2(x: i32, y: i32, field: &Vec<Vec<char>>) -> bool {
    let width = field[0].len() as i32;
    let height = field.len() as i32;

    x >= 0 && x < width && y >= 0 && y < height
}
