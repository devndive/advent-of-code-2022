use std::fs;

fn score_for_shape(me: &str) -> i32 {
    match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,

        _ => { panic!("Invalid input: {}", me); }
    }
}

fn calculate_score_first(opponent: &str, me: &str) -> i32 {
    // opponent        | me
    // A => Rock        X
    // B => Paper       Y
    // C => Scissors    Z
    let mut total_score = score_for_shape(me);

    if opponent == "A" {
        if me == "X" {
            total_score += 3;
        }

        if me =="Y" {
            total_score += 6;
        }

        if me == "Z" {
            total_score += 0;
        }
    }

    if opponent == "B" {
        if me == "X" {
            total_score += 0;
        }

        if me =="Y" {
            total_score += 3;
        }

        if me == "Z" {
            total_score += 6;
        }
    }

    if opponent == "C" {
        if me == "X" {
            total_score += 6;
        }

        if me =="Y" {
            total_score += 0;
        }

        if me == "Z" {
            total_score += 3;
        }
    }

    total_score
}

fn calculate_score_second(opponent: &str, round_outcome: &str) -> i32 {
    // opponent        | round_outcome
    // A => Rock        X => should loose
    // B => Paper       Y => should draw
    // C => Scissors    Z => should win
    let mut selected_shape = "";

    if opponent == "A" {
        if round_outcome == "X" {
            selected_shape = "Z";
        }

        if round_outcome =="Y" {
            selected_shape = "X";
        }

        if round_outcome == "Z" {
            selected_shape = "Y";
        }
    }

    if opponent == "B" {
        if round_outcome == "X" {
            selected_shape = "X";
        }

        if round_outcome =="Y" {
            selected_shape = "Y";
        }

        if round_outcome == "Z" {
            selected_shape = "Z";
        }
    }

    if opponent == "C" {
        if round_outcome == "X" {
            selected_shape = "Y";
        }

        if round_outcome =="Y" {
            selected_shape = "Z";
        }

        if round_outcome == "Z" {
            selected_shape = "X";
        }
    }

    let mut total_score = score_for_shape(selected_shape);
    if round_outcome == "Y" {
        total_score += 3;
    }

    if round_outcome == "Z" {
        total_score += 6;
    }

    total_score
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();

    let total = input
        .split("\n")
        .map(|round| {
            let choices: Vec<&str> = round.split_whitespace().collect();

            calculate_score_second(choices[0], choices[1])
        })
        .sum::<i32>();

    println!("{}", total);
}
