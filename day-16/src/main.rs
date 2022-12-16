use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fs, vec,
};

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

fn find_routes(
    start_point: &String,
    /*end_point: &String,*/
    valves: &HashMap<String, Valve>,
) -> HashMap<String, u32> {
    let mut distances: HashMap<String, u32> = HashMap::new();
    println!("{} -> ", start_point);

    let mut que = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();

    visited.insert(start_point.clone());
    que.push_back(start_point);

    let mut distance = 0;

    while !que.is_empty() {
        let item = que.pop_front().unwrap();
        //if item == end_point {
        //    break;
        //}

        distance += 1;
        for valve in valves.get(item).unwrap().nodes.iter() {
            if !visited.contains(valve) {
                println!("      {}: {}", valve, distance);
                visited.insert(valve.clone());

                distances.insert(valve.clone(), distance);
                que.push_back(valve);
            }
        }
    }

    return distances;
}

fn main() {
    let input = fs::read_to_string("./src/test_input").unwrap();
    let rooms = input.split("\n").collect::<Vec<&str>>();

    let re =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z,\s]*)")
            .unwrap();

    let mut valves: HashMap<String, Valve> = HashMap::new();
    for room in rooms {
        let matches = re.captures(room).unwrap();

        valves.insert(
            matches[1].to_string(),
            Valve::from(&matches[2], &matches[3]),
        );
    }

    // Build two HashMaps to hold oll possible ways from each to each point
    let mut distances: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for v in valves.iter() {
        // ignore all paths to valves that do not add value (release pressure)
        if v.1.flow_rate == 0 && v.0 != &String::from("AA") {
            continue;
        }

        distances.insert(v.0.clone(), find_routes(&v.0.clone(), &valves));
    }

    let open_valves: HashSet<String> = HashSet::new();

    let res = process_leaf(String::from("AA"), 10, &valves, distances, open_valves);
    println!(
        "Part 1: {} - {}",
        res.0,
        res.1.join(",")
    );
}

fn process_leaf(
    leaf: String,
    mut remaining_time: u32,
    valves: &HashMap<String, Valve>,
    distances: HashMap<String, HashMap<String, u32>>,
    mut open_valves: HashSet<String>,
) -> (u32, Vec<String>) {
        let valve = valves.get(&leaf).unwrap();
    if open_valves.contains(&leaf)  || valve.flow_rate == 0 {
        println!("------------- should not have done this");
    }
    
    if remaining_time < 2 {
        // we have no more time todo anything meaningful here
        println!("Early return - 0");
        return (0, vec![leaf]);
    } else if remaining_time == 2 {
        // Open valve and let it run, no more need to go anywhere
        println!("Early return - {}", valve.flow_rate);
        return (valve.flow_rate, vec![leaf]);
    } else {
        // open the valve

        let mut released_steam = valve.flow_rate;
        if valve.flow_rate > 0 {
            remaining_time -= 1;

            println!(
                "{}: {} * {} = {}",
                leaf,
                remaining_time,
                valve.flow_rate,
                remaining_time * valve.flow_rate
            );
            released_steam = remaining_time * valve.flow_rate;
            open_valves.insert(leaf.clone());
        }

        // get distances to valves
        let distances_form_leaf = distances.get(&leaf);

        // No place to go from here
        if distances_form_leaf.is_none() {
            println!("No places to go: {}", released_steam);
            return (released_steam, vec![leaf]);
        }
        let distances_form_leaf = distances_form_leaf.unwrap();

        //
        //print!("{} ({}) ->", leaf, remaining_time);
        let mut next_valves = vec![];
        for (next_valve, distance) in distances_form_leaf.iter() {
            //print!(" {}", next_valve);
            let next_valve_node = valves.get(next_valve).unwrap();

            //let distance_to_next_leaf = distances_form_leaf.get(next_valve).unwrap();

            println!("{} => {} - {} - {}", leaf, next_valve, distance, remaining_time);
            // if the distance to next leaf is greater or equal to the time remaining no need to go there
            // if the valve is already open, no need to go there
            if *distance <= remaining_time
                && !open_valves.contains(next_valve)
                && next_valve_node.flow_rate > 0
            {
                let res = process_leaf(
                    next_valve.clone(),
                    remaining_time - distance,
                    valves,
                    distances.clone(),
                    open_valves.clone(),
                );
                //println!(
                //    "Remaining time: {} - {} - {}",
                //    remaining_time - distance, next_valve, value
                //);
                if res.0 > 0 {
                    next_valves.push((next_valve, res));
                }
            }
        }

        next_valves.iter().for_each(|(next_valve, value)| {
            println!("Options: {} -> {} | {}", leaf, next_valve, value.0);
        });

        let most_valuable_step = next_valves.iter().max_by(|x, y| x.1.cmp(&y.1));
        if most_valuable_step.is_some() {
            let most_valuable_step = most_valuable_step.unwrap();
            println!(
                "  Chosen: {} -> {} | {}",
                leaf, most_valuable_step.0, most_valuable_step.1.0
            );
            let mut ret_vec = (most_valuable_step.1).1.clone();
            ret_vec.push(format!("{} ({})", leaf, most_valuable_step.1.0));
            return (released_steam + most_valuable_step.1.0, ret_vec);
        }

        println!("Released steam {}", released_steam);
        return (released_steam, vec![format!("{} (.{})",leaf, released_steam)]);
    }
}
