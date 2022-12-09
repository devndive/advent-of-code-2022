#[cfg(test)]
mod tests {
    use crate::{Field};


    #[test]
    fn follow_should_work() {
        let head = Field { row_idx: 0, col_idx: 0, display: String::from("H"), };
        let mut tail = Field { row_idx: 0, col_idx: 0, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 0);
        assert_eq!(tail.col_idx, 0);
    } 

    #[test]
    fn follow_should_work_1() {
        let head = Field { row_idx: 2, col_idx: 0, display: String::from("H"), };
        let mut tail = Field { row_idx: 0, col_idx: 0, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 1);
        assert_eq!(tail.col_idx, 0);
    }
    
    #[test]
    fn follow_should_work_2() {
        let head = Field { row_idx: 2, col_idx: 0, display: String::from("H"), };
        let mut tail = Field { row_idx: 4, col_idx: 0, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 3);
        assert_eq!(tail.col_idx, 0);
    }

    #[test]
    fn follow_should_work_3() {
        let head = Field { row_idx: 0, col_idx: 2, display: String::from("H"), };
        let mut tail = Field { row_idx: 0, col_idx: 0, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 0);
        assert_eq!(tail.col_idx, 1);
    }

    #[test]
    fn follow_should_work_4() {
        let head = Field { row_idx: 0, col_idx: 2, display: String::from("H"), };
        let mut tail = Field { row_idx: 0, col_idx: 4, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 0);
        assert_eq!(tail.col_idx, 3);
    }

    #[test]
    fn follow_should_work_5() {
        let head = Field { row_idx: 2, col_idx: 1, display: String::from("H"), };
        let mut tail = Field { row_idx: 0, col_idx: 0, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 1);
        assert_eq!(tail.col_idx, 1);
    }

    #[test]
    fn follow_should_work_6() {
        let head = Field { row_idx: 2, col_idx: 2, display: String::from("H"), };
        let mut tail = Field { row_idx: 0, col_idx: 0, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 1);
        assert_eq!(tail.col_idx, 1);
    }
    
    #[test]
    fn follow_should_work_7() {
        let head = Field { row_idx: 2, col_idx: 2, display: String::from("H"), };
        let mut tail = Field { row_idx: 4, col_idx: 4, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 3);
        assert_eq!(tail.col_idx, 3);
    }

     #[test]
    fn follow_should_work_17() {
        let head = Field { row_idx: 5, col_idx: 5, display: String::from("H"), };
        let mut tail = Field { row_idx: 6, col_idx: 7, display: String::from("T"), };

        tail.follow(head);

        assert_eq!(tail.row_idx, 6);
        assert_eq!(tail.col_idx, 6);
    }
}

use std::fs;

struct Step {
    visited: bool,
}

#[derive(Clone, Debug)]
struct Field {
    row_idx: usize,
    col_idx: usize,
    display: String,
}

fn change(input: i32) -> i32 {
    if input < -1 {
        return -1;
    } else if input > 1 {
        return 1;
    }

    return input;
}

impl Field {
    fn move_dir(&mut self, dir: String) {
        if dir.eq(&String::from('R')) {
            self.col_idx += 1;
        }

        if dir == String::from('L') {
            self.col_idx -= 1;
        }

        if dir == String::from('U') {
            self.row_idx -= 1;
        }

        if dir == String::from('D') {
            self.row_idx += 1;
        }
    }

    fn follow(&mut self, head: Field) {
        let col_diff = head.col_idx as i32 - self.col_idx as i32;
        let row_diff = head.row_idx as i32 - self.row_idx as i32;
        //println!("row_diff: {} | col_diff: {}", row_diff, col_diff);

        if col_diff.abs() > 1 || row_diff.abs() > 1 {
            // return Field {
            self.col_idx = (self.col_idx as i32 + change(col_diff)) as usize;
            self.row_idx = (self.row_idx as i32 + change(row_diff)) as usize;
            // };
        } else {
            //self.col_idx = (self.col_idx as i32 + col_diff) as usize;
            //self.row_idx = (self.row_idx as i32 + row_diff) as usize;
        }

        // return Field {
        //};
    }
}


#[derive(Clone)]
struct Instruction {
    direction: String,
    count: usize,
}

impl Instruction {
    pub fn new(input: String) -> Self {
        let parts = input.split_at(2);

        Self {
            direction: String::from(parts.0.trim()),
            count: parts.1.parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let instructions = lines
        .iter()
        .map(|l| Instruction::new(String::from(*l)))
        .collect::<Vec<Instruction>>();

    let width = 1000;
    let height = 1000;

    let mut bridge: Vec<Vec<Step>> = vec![];
    for _ in 0..height {
        bridge.push(
            (0..width)
                .map(|_| Step { visited: false })
                .collect::<Vec<Step>>(),
        );
    }

    let mut head = Field {
        row_idx: (width / 2) - 1,
        col_idx: height / 2 - 1,
        display: String::from("H"),
    };
    let mut tail: Field = Field {
        row_idx: (width / 2) - 1,
        col_idx: height / 2 - 1,
        display: String::from("T"),
    };

    bridge[tail.row_idx][tail.col_idx].visited = true;

    for i in instructions.iter() {
        // println!("== {} {} ==", i.direction, i.count);

        for _ in 0..i.count {
            //print_bridge(&bridge, &head, &tail);
            //println!("");

            head.move_dir(i.direction.clone());
            tail.follow(head.clone());
            bridge[tail.row_idx][tail.col_idx].visited = true;
        }
    }

    let count = bridge.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |a, col| if col.visited { a + 1 } else { a })
    });
    println!("Part 1: {}", count);

    let mut bridge: Vec<Vec<Step>> = vec![];
    for _ in 0..height {
        bridge.push(
            (0..width)
                .map(|_| Step { visited: false })
                .collect::<Vec<Step>>(),
        );
    }

    let mut snake: Vec<Field> = vec![
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("H"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("1"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("2"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("3"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("4"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("5"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("6"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("7"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("8"),
        },
        Field {
            row_idx: (width / 2) - 1,
            col_idx: height / 2 - 1,
            display: String::from("9"),
        },
    ];
    bridge[snake[0].row_idx][snake[0].col_idx].visited = true;

    for i in instructions.iter() {
        //println!("== {} {} ==", i.direction, i.count);

        for _ in 0..i.count {

            // let cur_head = snake[0].clone();
            snake[0].move_dir(i.direction.clone());
            //let h = snake[0].clone();
            //let mut temp = snake[1].clone();
            //temp.follow(h);

            /*
            head.move_dir(i.direction.clone());
            tail.follow(&head);
            bridge[tail.row_idx][tail.col_idx].visited = true;
            */
            for idx in 1..snake.len() {
                 let h = snake[idx-1].clone();
                 snake[idx].follow(h);

                /*
                if snake[idx - 1].needs_to_follow(&snake[idx], width, height) {
                    let next_field = next_field(&snake[idx - 1], &snake[idx], width, height);

                    snake[idx].row_idx = next_field.row_idx;
                    snake[idx].col_idx = next_field.col_idx;
                }
                */
            }
            //print_bridge(&bridge, &snake);
            //println!("");

            bridge[snake[snake.len()-1].row_idx][snake[snake.len()-1].col_idx].visited = true;
        }
    }

    //print_bridge(&bridge, &snake);
    let count = bridge.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |a, col| if col.visited { a + 1 } else { a })
    });

    println!("Part 2: {}", count);
}

fn print_bridge(bridge: &Vec<Vec<Step>>, snake: &Vec<Field>) {
    let mut y = 0;

    for row in bridge {
        let mut x = 0;

        for column in row {
            let s = snake.iter().find(|f| f.col_idx == x && f.row_idx == y);
            if s.is_some() {
                print!("{}", s.unwrap().display);
            } else if column.visited {
                print!("#")
            } else {
                print!(".")
            }

            x += 1;
        }

        y += 1;
        println!("");
    }
}
