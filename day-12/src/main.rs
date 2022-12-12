#[cfg(test)]
mod tests {

    use crate::{build_graph, can_go_to, can_go_to_char, is_in_bounds, Position};

    #[test]
    fn in_bounds_should_work() {
        let field = vec![vec!['a', 'a'], vec!['a', 'a']];

        assert_eq!(is_in_bounds(0, 0, &field), true);
        assert_eq!(is_in_bounds(0, 1, &field), true);
        assert_eq!(is_in_bounds(1, 0, &field), true);
        assert_eq!(is_in_bounds(1, 1, &field), true);

        assert_eq!(is_in_bounds(-2, -2, &field), false);
        assert_eq!(is_in_bounds(-1, -1, &field), false);
        assert_eq!(is_in_bounds(2, 2, &field), false);
    }

    #[test]
    fn can_go_to_should_work() {
        let field = vec![vec!['a', 'z'], vec!['a', 'a']];

        assert_eq!(
            can_go_to(Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, &field),
            false
        );
        assert_eq!(
            can_go_to(Position { x: 1, y: 0 }, Position { x: 0, y: 0 }, &field),
            true
        );
    }

    #[test]
    fn can_go_to_should_work_with_start_char() {
        let field = vec![vec!['S', 'z'], vec!['a', 'a']];

        assert_eq!(
            can_go_to(Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, &field),
            false
        );
        assert_eq!(
            can_go_to(Position { x: 1, y: 0 }, Position { x: 0, y: 0 }, &field),
            true
        );
    }

    #[test]
    fn build_graph_test() {
        let field = vec![vec!['S', 'a'], vec!['a', 'b']];

        let res = build_graph((0, 0), &field);
        assert_eq!(res.len(), 2);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 1, y: 0 }), true);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 0, y: 1 }), true);
    }

    #[test]
    fn build_graph_test_2() {
        let field = vec![vec!['S', 'a'], vec!['a', 'b'], vec!['a', 'c']];

        let res = build_graph((0, 1), &field);
        assert_eq!(res.len(), 3);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 1, y: 1 }), true);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 0, y: 2 }), true);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 0, y: 0 }), true);
    }

    #[test]
    fn build_graph_test_3() {
        let field = vec![
            vec!['q', 'p', 'o'],
            vec!['r', 'y', 'x'],
            vec!['s', 'z', 'E'],
        ];

        let res = build_graph((1, 1), &field);
        assert_eq!(res.len(), 4);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 1, y: 0 }), true);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 2, y: 1 }), true);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 0, y: 1 }), true);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 1, y: 2 }), true);
    }

    #[test]
    fn build_graph_4() {
        let field = vec![vec!['S', 'a'], vec!['a', 'E']];

        let res = build_graph((0, 1), &field);
        assert_eq!(res.len(), 1);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 0, y: 0 }), true);
        assert_eq!(res.iter().any(|n| n.pos == Position { x: 1, y: 1 }), false);
    }

    #[test]
    fn can_got_to_char_test() {
        assert_eq!(can_go_to_char('z', 'E'), true);
        assert_eq!(can_go_to_char('x', 'E'), false);
        assert_eq!(can_go_to_char('v', 'E'), false);
    }
}

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let rows = input.split("\n").collect::<Vec<&str>>();

    let field: Vec<Vec<char>> = rows
        .iter()
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let height = field.len();
    let width = field[0].len();

    let mut visited_nodes: HashMap<Position, Vec<Edge>> = HashMap::new();
    for row in 0..height {
        for col in 0..width {
            if field[row][col] == 'S' {
                start = (col, row);
            }

            if field[row][col] == 'E' {
                end = (col, row);
            }

            visited_nodes.insert(
                Position { x: col, y: row },
                build_graph((col as i32, row as i32), &field),
            );
        }
    }

    // create graph
    let start_node = Edge {
        pos: Position {
            x: start.0,
            y: start.1,
        },
        parent: Position {
            x: start.0,
            y: start.1,
        },
    };
    let end_node = Position { x: end.0, y: end.1 };

    let counter = find_way(start_node, end_node.clone(), &visited_nodes);
    println!("Part 1: {}", counter.unwrap());

    let mut start_points: Vec<Edge> = vec![];
    for row in 0..field.len() {
        for col in 0..field[0].len() {
            if field[row][col] == 'a' || field[row][col] == 'S' {
                start_points.push(Edge {
                    pos: Position { x: col, y: row },
                    parent: Position { x: col, y: row },
                });
            }
        }
    }

    let mut minimum = width as i32 * height as i32;
    for starting_pos in start_points.iter() {
        let counter = find_way(
            starting_pos.clone(),
            end_node.clone(),
            &visited_nodes,
        );

        if counter.is_some() {
            minimum = minimum.min(counter.unwrap());
        }
    }

    println!("Part 2: {}", minimum);
}

fn find_way(
    start_node: Edge,
    end_node: Position,
    visited_nodes: &HashMap<Position, Vec<Edge>>,
) -> Option<i32> {
    let mut que: VecDeque<Edge> = VecDeque::new();
    let mut visited: HashSet<Edge> = HashSet::new();
    que.push_back(start_node.clone());
    visited.insert(start_node.clone());

    let mut backtrack_list: Vec<Edge> = vec![];
    backtrack_list.push(start_node.clone());

    //println!("-- {:?}", start_node);

    while que.len() > 0 {
        let v = que.pop_front().unwrap();

        /*if v.pos == end_node {
            found_end = true;
        } else {*/
        let edges = visited_nodes.get(&v.pos).unwrap();

        for edge in edges.iter() {
            if !visited.contains(&edge) {
                // println!("{} == {}", field[edge.y][edge.x], field[end_node.y][end_node.x]);
                // println!("Adding: {:?}", edge);
                visited.insert(edge.clone());

                let mut backtrack = edge.clone();
                backtrack.parent = v.pos.clone();
                backtrack_list.push(backtrack);

                que.push_back(edge.clone());
            }
        }
        //}
    }

    // println!("{:?}, len: {}", end_node, backtrack_list.len());

    let end_point_result = backtrack_list.iter().find(|e| e.pos == end_node);
    let mut end_point: &Edge;

    // We do not have the endpoint in our backtracking list
    // this means there is no way from start to end - ignore this
    if end_point_result.is_none() {
        return None;
    } else {
        end_point = end_point_result.unwrap();
    }

    let mut counter = 0;
    loop {
        //println!("{}", field[end_point.pos.y][end_point.pos.x]);
        //println!("{:?} - {:?}", end_point.pos, start_node.pos);
        if end_point.pos == start_node.pos {
            break;
        } else {
            counter += 1;
            end_point = backtrack_list
                .iter()
                .find(|e| e.pos == end_point.parent)
                .unwrap();
        }
    }

    return Some(counter);
}
fn build_graph(current_pos: (i32, i32), field: &Vec<Vec<char>>) -> Vec<Edge> {
    let paths = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut nodes: Vec<Edge> = vec![];
    for p in paths {
        let x = current_pos.0 + p.0;
        let y = current_pos.1 + p.1;

        println!("InBounds: {} | CanGoTo: -", is_in_bounds(x, y, field));
        if is_in_bounds(x, y, field) {
            if can_go_to(
                Position::from(current_pos.0, current_pos.1),
                Position::from(x, y),
                &field, /* && !visited_nodes.contains_key(&(x, y) */
            ) {
                //println!("Adding: ({}/{})", x, y);
                nodes.push(Edge {
                    pos: Position::from(x, y),
                    parent: Position { x: 0, y: 0 },
                });
            }
        }
    }

    return nodes;
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Edge {
    pos: Position,
    parent: Position,
}

impl Position {
    fn from(x: i32, y: i32) -> Self {
        Self {
            x: x as usize,
            y: y as usize,
        }
    }
}

fn is_in_bounds(x: i32, y: i32, field: &Vec<Vec<char>>) -> bool {
    let height = field.len() as i32;
    let width = field[0].len() as i32;

    if x >= 0 && x < width && y >= 0 && y < height {
        return true;
    }

    return false;
}

fn can_go_to(current: Position, target: Position, field: &Vec<Vec<char>>) -> bool {
    let current_char = field[current.y][current.x];
    let target_char = field[target.y][target.x];

    return can_go_to_char(current_char, target_char);
}

fn can_go_to_char(current_input: char, target_input: char) -> bool {
    let mut current = current_input;
    let mut target = target_input;

    if current == 'S' {
        current = 'a'
    }

    if target == 'E' {
        target = 'z'
    }

    if current == target || target as u8 <= current as u8 + 1 {
        return true;
    }

    return false;
}
