use std::fs;

fn process_cycle(register: i32, cycle_count: i32, signal_strength: i32) -> (i32, i32) {
    // println!("{} - {}", cycle_count, register);
    let mut sig_str = signal_strength;

    if (cycle_count - 20) % 40 == 0 {
        // println!("{} * {} = {}", cycle_count, register, cycle_count * register);
        sig_str += cycle_count * register;
    }

    return (cycle_count + 1, sig_str);
}

fn process_cycle_2(register: i32, cycle_count: i32, row: &mut Vec<String>) -> i32 {
    // println!("{}", register);

    if cycle_count % 40 == 0 {
        row.iter().for_each(|f| print!("{}", f));
        println!("");
    } else {
        let pos_current_pixel = cycle_count % 40;

        if pos_current_pixel == register -1 || pos_current_pixel == register || pos_current_pixel == register +1 {
            row[pos_current_pixel as usize] = String::from("#");
        } else {
            row[pos_current_pixel as usize] = String::from(".");
        }

    }

    return cycle_count + 1;
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut register = 1;
    let mut cycle_count = 1;
    let mut signal_strength: i32 = 0;

    for line in lines.iter() {
        // println!("{}", line);
        let parts = line.split(" ").collect::<Vec<&str>>();

        match parts[0] {
            "noop" => {
                // Do nothing
                // increase cycle count
                (cycle_count, signal_strength) = process_cycle(register, cycle_count, signal_strength);
            }

            "addx" => {

                (cycle_count, signal_strength) = process_cycle(register, cycle_count, signal_strength);
                (cycle_count, signal_strength) = process_cycle(register, cycle_count, signal_strength);
                register += parts[1].parse::<i32>().unwrap();
            }

            _ => {
                panic!("Unknown command")
            }
        }
    }

    println!("Part 1: {}", signal_strength);

    let mut register = 1;
    let mut cycle_count = 1;
    let mut row: Vec<String> = (0..40).map(|_| String::from(".")).collect::<Vec<String>>();

    for line in lines.iter() {
        // println!("{}", line);
        let parts = line.split(" ").collect::<Vec<&str>>();

        match parts[0] {
            "noop" => {
                // Do nothing
                // increase cycle count
                cycle_count = process_cycle_2(register, cycle_count, &mut row);
            }

            "addx" => {

                cycle_count = process_cycle_2(register, cycle_count, &mut row);
                register += parts[1].parse::<i32>().unwrap();
                cycle_count = process_cycle_2(register, cycle_count, &mut row);
            }

            _ => {
                panic!("Unknown command")
            }
        }
    }
}
