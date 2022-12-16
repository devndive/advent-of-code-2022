use std::{collections::{HashMap, HashSet}, fs, hash::Hash, vec};

use regex::Regex;

struct Valve {
    flow_rate: u32,
    nodes: Vec<String>,
}

impl Valve {
    fn from(rate: &str, connected_rooms: &str) -> Self {
        Self {
            flow_rate: rate.parse::<u32>().unwrap(),
            nodes: connected_rooms
                .split(", ")
                .map(|r| r.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Node {
    name: String,
    released_steam: u32,
    minutes_remaining: u32,
    is_open: bool,
    leafes: Vec<(Node, u32)>,
}

fn main() {
    let input = fs::read_to_string("./src/test_input").unwrap();
    let rooms = input.split("\n").collect::<Vec<&str>>();

    let re =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z,\s]*)")
            .unwrap();

    let mut map: HashMap<String, Valve> = HashMap::new();
    for room in rooms {
        let matches = re.captures(room).unwrap();

        map.insert(
            matches[1].to_string(),
            Valve::from(&matches[2], &matches[3]),
        );
    }

    /*
    let root_node = Node {
        name: String::from("AA"),
        leafes: HashMap::new(),
    };
    */

    let mut total_steam = 0;
    let mut nodes_visited_and_open = HashMap::new();
    nodes_visited_and_open.insert(String::from("AA"), Node {
        name: String::from("AA"),
        released_steam: 0,
        is_open: false,
        minutes_remaining: 30,
        leafes: vec![]
    });

    let root_node = process_leaf(&String::from("AA"), &map, 30, nodes_visited_and_open.clone());
    let mut cur_node = root_node.0;

    while cur_node.leafes.len() > 0 {
        let highest = cur_node.leafes.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
        println!("{}: {}", highest.0.name, highest.0.minutes_remaining);

        total_steam = total_steam + highest.0.released_steam;

        cur_node = highest.0.clone();
    }

    println!("Part 1: {}", total_steam);

}

fn process_leaf(
    node_name: &String,
    map: &HashMap<String, Valve>,
    minutes_left: u32,
    mut nodes_visited: HashMap<String, Node>
) -> (Node, u32) {
    let node = map.get(node_name).unwrap();
    let mut minutes_remaining = minutes_left;

    let mut released_steam = 0;
    // let mut open_valve_time_cost = 0;
    if node.flow_rate > 0 && minutes_remaining > 1 {
        // open_valve_time_cost = 1;
        released_steam = node.flow_rate * (minutes_left - 1);
        minutes_remaining -= 1;
    }

    let mut result_node = Node {
        name: node_name.clone(),
        is_open: node.flow_rate > 0 && released_steam > 0,
        released_steam: released_steam,
        minutes_remaining: minutes_remaining,
        leafes: vec![],
    };

    nodes_visited.insert(node_name.clone(), result_node.clone());

    if minutes_remaining > 1 {
        let mut max_released_steam = 0;
        // let ret = vec![];
        for leaf in node.nodes.iter() {
            // let leaf_node = map.get(leaf).unwrap();
            // let released_steam = leaf_node.flow_rate * (minutes_left - 1 - open_valve_time_cost);

            let visited = nodes_visited.get(leaf);
            if visited.is_none() {
                let next_leaf_released_steam =
                    process_leaf(leaf, map, minutes_remaining - 1, nodes_visited.clone());
                // ret.push((leaf, next_leaf_released_steam));

                max_released_steam = max_released_steam.max(next_leaf_released_steam.1);
                result_node
                    .leafes
                    .push((next_leaf_released_steam.0.clone(), next_leaf_released_steam.1));
            } else {
                result_node
                    .leafes
                    .push((visited.unwrap().clone(), 0));
            }

        }
    nodes_visited.insert(node_name.clone(), result_node.clone());

        return (result_node, max_released_steam + released_steam);
    } else {
        return (result_node, released_steam);
    }
}
