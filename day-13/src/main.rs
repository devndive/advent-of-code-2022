#[cfg(test)]
mod tests {
    use crate::{do_parse, Packet};

    #[test]
    fn parse_signal_test() {
        let result = do_parse("[1,1,3,1,1]");
        println!("Res {:?}", result);

        match result {
            Packet::Int(value) => {
                assert!(false);
            }
            Packet::List(value) => {
                assert_eq!(value.len(), 5);
            }
        }
    }
}

use std::{cmp::Ordering, fs, vec};

fn is_ordered(left_value: &Packet, right_value: &Packet) -> Ordering {
    match (left_value, right_value) {
        (Packet::Int(a), Packet::Int(b)) => a.cmp(&b),
        (Packet::List(left), Packet::List(right)) => {
            for (p1, p2) in left.iter().zip(right.iter()) {
                let order = is_ordered(p1, p2);
                if order != Ordering::Equal {
                    return order;
                }
            }
            return left.len().cmp(&right.len());
        }
        (Packet::List(_), Packet::Int(right)) => {
            return is_ordered(left_value, &Packet::List(vec![Packet::Int(*right)]));
        }
        (Packet::Int(left), Packet::List(_)) => {
            return is_ordered(&Packet::List(vec![Packet::Int(*left)]), right_value);
        }
    }
}

#[derive(Debug, Clone)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

fn do_parse(signal: &str) -> Packet {
    if !signal.starts_with("[") {
        return Packet::Int(signal.parse().unwrap());
    } else {
        let mut inner_packets = Vec::new();
        let mut depth = 0;
        let mut start = 1;

        for i in 1..signal.len() - 1 {
            let c = signal.chars().nth(i).unwrap();
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {
                    if depth == 0 {
                        let inner_packet = do_parse(&signal[start..i]);
                        inner_packets.push(inner_packet);
                        start = i + 1;
                    }
                }

                _ => (),
            }
        }

        if signal.len() > 2 {
            inner_packets.push(do_parse(&signal[start..signal.len() - 1]));
        }

        return Packet::List(inner_packets);
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let signal_groups = input.split("\n\n").collect::<Vec<&str>>();

    let mut result = 0;
    for (idx, group) in signal_groups.iter().enumerate() {
        let signals = group.split("\n").collect::<Vec<&str>>();

        let signal_one = do_parse(signals[0]);
        let signal_two = do_parse(signals[1]);

        if is_ordered(&signal_one, &signal_two) != Ordering::Greater {
            println!("Idx {}", idx);
            result += idx + 1;
        }
    }

    println!("Part 1: {}", result);

    let mut list = vec![];
    for group in signal_groups.iter() {
        let signals = group.split("\n").collect::<Vec<&str>>();

        let signal_one = do_parse(signals[0]);
        let signal_two = do_parse(signals[1]);

        list.push(signal_one);
        list.push(signal_two);
    }
    let first_divider = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let second_divider = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    list.push(first_divider.clone());
    list.push(second_divider.clone());

    list.sort_by(|left, right| is_ordered(left, right));

    let first = list.iter().enumerate().find(|l| {
        println!("{:?} - {:?}", &l.1, &first_divider);
        return is_ordered(l.1, &first_divider) == Ordering::Equal;
    });

    let second = list
        .iter()
        .enumerate()
        .find(|l| is_ordered(l.1, &second_divider) == Ordering::Equal);

    println!("{}", (first.unwrap().0 + 1) * (second.unwrap().0 + 1));
}
