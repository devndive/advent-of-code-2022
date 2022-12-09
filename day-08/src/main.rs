#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{calculate_scenic_score, Tree};

    #[test]
    fn scenic_score() {
        let input = fs::read_to_string("./src/test_input").unwrap();
        let lines = input.split("\n").collect::<Vec<&str>>();

        let field: Vec<Vec<Tree>> = lines
            .iter()
            .map(|l| {
                let v = l
                    .chars()
                    .map(|c| Tree {
                        height: c.to_string().parse::<u32>().unwrap(),
                        is_visible: false,
                    })
                    .collect::<Vec<Tree>>();
                return v;
            })
            .collect::<Vec<Vec<Tree>>>();

        let score = calculate_scenic_score(&field, 1, 2);
        assert_eq!(score, 4);
        let score = calculate_scenic_score(&field, 3, 2);
        assert_eq!(score, 8);
    }
}

use std::fs;

struct Tree {
    height: u32,
    is_visible: bool,
}

fn calculate_scenic_score(field: &Vec<Vec<Tree>>, row_idx: usize, col_idx: usize) -> u32 {
    let height = field.len();
    let width = field[0].len();
    let cur_height = field[row_idx][col_idx].height;

    let mut left_count = 0;
    for left_idx in (0..col_idx).rev() {
        let height_to_compare = field[row_idx][left_idx].height;
        left_count += 1;

        if height_to_compare == cur_height {
            break;
        }
    }

    let mut right_count = 0;
    for right_idx in col_idx + 1..width {
        if field[row_idx][right_idx].height < cur_height {
            right_count += 1;
        } else if field[row_idx][right_idx].height >= cur_height {
            right_count += 1;
            break;
        } else {
            break;
        }
    }

    let mut up_count = 0;
    for up_idx in (0..row_idx).rev() {
        if field[up_idx][col_idx].height < cur_height {
            up_count += 1;
        } else if field[up_idx][col_idx].height >= cur_height {
            up_count += 1;
            break;
        } else {
            break;
        }
    }

    let mut down_count = 0;
    for down_idx in row_idx + 1..height {
        if field[down_idx][col_idx].height < cur_height {
            down_count += 1;
        } else if field[down_idx][col_idx].height >= cur_height {
            down_count += 1;
            break;
        } else {
            break;
        }
    }

    left_count * right_count * up_count * down_count
}

impl Tree {
    fn is_visible(&self, tree_height: u32) -> bool {
        tree_height < self.height
    }

    fn set_visible(&mut self, tree_height: u32) -> u32 {
        if self.is_visible(tree_height) {
            self.is_visible = true;
            return self.height
        }

        tree_height
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut field: Vec<Vec<Tree>> = lines
        .iter()
        .map(|l| {
            let v = l
                .chars()
                .map(|c| Tree {
                    height: c.to_string().parse::<u32>().unwrap(),
                    is_visible: false,
                })
                .collect::<Vec<Tree>>();
            return v;
        })
        .collect::<Vec<Vec<Tree>>>();

    let height = field.len();
    let width = field[0].len();

    let mut total_visible = height * 2 + width * 2 - 4;

    // check left to right and right to left
    for row_idx in 1..height - 1 {
        let mut tree_height = field[row_idx][0].height;

        for col_idx in 1..width - 1 {
            tree_height = field[row_idx][col_idx].set_visible(tree_height);
        }

        tree_height = field[row_idx][width - 1].height;
        for col_idx in (1..width - 1).rev() {
            tree_height = field[row_idx][col_idx].set_visible(tree_height);
        }
    }

    for col_idx in 1..width - 1 {
        let mut tree_height = field[0][col_idx].height;

        for row_idx in 1..height - 1 {
            tree_height = field[row_idx][col_idx].set_visible(tree_height);
        }

        let mut tree_height = field[height - 1][col_idx].height;
        for row_idx in (1..height - 1).rev() {
            tree_height = field[row_idx][col_idx].set_visible(tree_height);
        }
    }

    for row in field.iter() {
        for tree in row {
            if tree.is_visible {
                total_visible += 1;
            }
        }
    }

    println!("Part 1: {}", total_visible);

    let mut heighest_scenic_score = 0;
    for row_idx in 1..height - 1 {
        for col_idx in 1..width - 1 {
            let scenic_score = calculate_scenic_score(&field, row_idx, col_idx);
            if heighest_scenic_score < scenic_score {
                heighest_scenic_score = scenic_score;
            }
        }
    }

    println!("{}", heighest_scenic_score);
}
