use std::{collections::{HashSet, HashMap}, fs};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Stone {
    fields: HashSet<Position>,
}

impl Stone {
    fn move_to(&mut self, offset: Position) {
        self.fields = self
            .fields
            .iter()
            .map(|f| Position {
                x: offset.x + f.x,
                y: offset.y + f.y,
            })
            .collect::<HashSet<Position>>();
    }

    fn height(&self) -> i32 {
        self.fields.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y
    }

    fn can_move_right(&self, occupied_fields: &HashSet<Position>) -> bool {
        let moved_fields = self
            .fields
            .iter()
            .map(|f| Position { x: f.x + 1, y: f.y })
            .collect::<HashSet<Position>>();

        let is_in_bounds = moved_fields.iter().all(|f| f.x < 7);

        let is_blocked = moved_fields
            .iter()
            .any(|moved_field| occupied_fields.contains(moved_field));

        return is_in_bounds && !is_blocked;
    }

    fn can_move_left(&self, occupied_fields: &HashSet<Position>) -> bool {
        let moved_fields = self
            .fields
            .iter()
            .map(|f| Position {
                x: f.x + -1,
                y: f.y,
            })
            .collect::<HashSet<Position>>();

        let is_in_bounds = moved_fields.iter().all(|f| f.x >= 0);

        let is_blocked = moved_fields
            .iter()
            .any(|moved_field| occupied_fields.contains(moved_field));

        return is_in_bounds && !is_blocked;
    }

    fn move_left(&mut self) {
        self.move_x(-1);
    }

    fn move_right(&mut self) {
        self.move_x(1);
    }

    fn move_x(&mut self, x_diff: i32) {
        self.fields = self
            .fields
            .iter()
            .map(|f| Position {
                x: f.x + x_diff,
                y: f.y,
            })
            .collect::<HashSet<Position>>()
    }

    fn move_down(&mut self) {
        self.fields = self
            .fields
            .iter()
            .map(|f| Position { x: f.x, y: f.y + 1 })
            .collect::<HashSet<Position>>()
    }

    fn can_move_down(&self, occupied_fields: &HashSet<Position>) -> bool {
        // move all fields down by one
        let moved_fields = self
            .fields
            .iter()
            .map(|f| Position { x: f.x, y: f.y + 1 })
            .collect::<HashSet<Position>>();

        if moved_fields.iter().any(|moved_field| moved_field.y > 3) {
            return false;
        }

        if moved_fields
            .iter()
            .any(|moved_field| occupied_fields.contains(moved_field))
        {
            return false;
        }

        return true;
    }
}

fn let_stone_fall(
    mut steps: usize,
    mut starting_point: i32,
    mut falling_stone: Stone,
    occupied_fields: &mut HashSet<Position>,
    occupied_fields_2: &mut HashMap<usize, HashSet<usize>>,
    stream: &String,
) -> (usize, i32) {
    let drawing_point = starting_point - falling_stone.height();
    // print field 7x4
    falling_stone.move_to(Position {
        x: 2,
        y: drawing_point,
    });

    //println!("Start");
    //draw_field(drawing_point, &occupied_fields, &falling_stone);

    //while falling_stone.can_move_down(&occupied_fields) {
    loop {
        let jet_stream = stream.chars().nth(steps % stream.len()).unwrap();
        if jet_stream == '>' {
            if falling_stone.can_move_right(&occupied_fields) {
                falling_stone.move_right();
            }
        } else {
            if falling_stone.can_move_left(&occupied_fields) {
                falling_stone.move_left();
            }
        }

        //println!("Wind: ");
        //draw_field(drawing_point, &occupied_fields, &falling_stone);

        if falling_stone.can_move_down(&occupied_fields) {
            falling_stone.move_down();
            steps += 1;
            //println!("Down: ");
            //draw_field(drawing_point, &occupied_fields, &falling_stone);
        } else {
            break;
        }

        //   if falling_stone.can_move_down(&occupied_fields) {
        //  } else {
        //    break;
        // }

        //draw_field(starting_point, &occupied_fields, &falling_stone);
    }

    falling_stone.fields.iter().for_each(|f| {
        let sanity_check = occupied_fields.insert(f.clone());
        if sanity_check == false {
            println!("{} - {}", steps, stream.len());
            println!("{:?}", f.clone());
            panic!("Uhoh!");
        }
    });

    let highest_point = occupied_fields
        .iter()
        .min_by(|x, y| x.y.cmp(&y.y))
        .unwrap();

    return (steps + 1, highest_point.y - 4);
}

fn main() {
    let stream = fs::read_to_string("./src/puzzle_input").unwrap();

    let mut starting_point = 0;

    let mut stone_one = Stone {
        fields: HashSet::new(),
    };

    stone_one.fields.insert(Position { x: 0, y: 0 });
    stone_one.fields.insert(Position { x: 1, y: 0 });
    stone_one.fields.insert(Position { x: 2, y: 0 });
    stone_one.fields.insert(Position { x: 3, y: 0 });

    let mut stone_two = Stone {
        fields: HashSet::new(),
    };

    stone_two.fields.insert(Position { x: 1, y: 0 });
    stone_two.fields.insert(Position { x: 0, y: 1 });
    stone_two.fields.insert(Position { x: 1, y: 1 });
    stone_two.fields.insert(Position { x: 2, y: 1 });
    stone_two.fields.insert(Position { x: 1, y: 2 });

    let mut stone_three = Stone {
        fields: HashSet::new(),
    };

    stone_three.fields.insert(Position { x: 2, y: 0 });
    stone_three.fields.insert(Position { x: 2, y: 1 });
    stone_three.fields.insert(Position { x: 0, y: 2 });
    stone_three.fields.insert(Position { x: 1, y: 2 });
    stone_three.fields.insert(Position { x: 2, y: 2 });

    let mut stone_four = Stone {
        fields: HashSet::new(),
    };

    stone_four.fields.insert(Position { x: 0, y: 0 });
    stone_four.fields.insert(Position { x: 0, y: 1 });
    stone_four.fields.insert(Position { x: 0, y: 2 });
    stone_four.fields.insert(Position { x: 0, y: 3 });

    let mut stone_five = Stone {
        fields: HashSet::new(),
    };

    stone_five.fields.insert(Position { x: 0, y: 0 });
    stone_five.fields.insert(Position { x: 1, y: 0 });
    stone_five.fields.insert(Position { x: 0, y: 1 });
    stone_five.fields.insert(Position { x: 1, y: 1 });

    let mut stone_six = Stone {
        fields: HashSet::new(),
    };

    stone_six.fields.insert(Position { x: 0, y: 0 });
    stone_six.fields.insert(Position { x: 1, y: 0 });
    stone_six.fields.insert(Position { x: 2, y: 0 });
    stone_six.fields.insert(Position { x: 3, y: 0 });

    let all_stones = vec![
        stone_one,
        stone_two,
        stone_three,
        stone_four,
        stone_five,
        stone_six,
    ];

    let mut steps = 0;
    let mut occupied_fields = HashSet::new();

    let mut occupied_fields_2: HashMap<usize, HashSet<usize>> = HashMap::new();

    //let mut stone_counter = 0;
    for stone_counter in 0..1000000000000 {
        println!("Left over {}", 1000000000000 - stone_counter);
        let res = let_stone_fall(
            steps,
            starting_point,
            all_stones[stone_counter % 5].clone(),
            &mut occupied_fields,
            &mut occupied_fields_2,
            &stream,
        );

        steps = res.0;
        starting_point = res.1;
        //println!("New starting point {}", starting_point);
    }

    let highest_point = occupied_fields
        .iter()
        .min_by(|x, y| x.y.cmp(&y.y))
        .unwrap();

    println!("Part 1:  {}", (highest_point.y - 4).abs());
    /*
    let res = let_stone_fall(
        steps,
        starting_point,
        all_stones[0].clone(),
        &mut occupied_fields,
        &stream,
    );

    steps = res.0;
    starting_point = res.1;
    println!("New starting point {}", starting_point);

    let res = let_stone_fall(
        steps,
        starting_point,
        all_stones[1].clone(),
        &mut occupied_fields,
        &stream,
    );

    steps = res.0;
    starting_point = res.1;
    println!("New starting point {}", starting_point);

    let res = let_stone_fall(
        steps,
        starting_point,
        all_stones[2].clone(),
        &mut occupied_fields,
        &stream,
    );

    steps = res.0;
    starting_point = res.1;
    println!("New starting point {}", starting_point);

    let res = let_stone_fall(
        steps,
        starting_point,
        all_stones[3].clone(),
        &mut occupied_fields,
        &stream,
    );

    steps = res.0;
    starting_point = res.1;
    println!("New starting point {}", starting_point);

    let res = let_stone_fall(
        steps,
        starting_point,
        all_stones[4].clone(),
        &mut occupied_fields,
        &stream,
    );

    steps = res.0;
    starting_point = res.1;
    println!("New starting point {}", starting_point);

    let res = let_stone_fall(
        steps,
        starting_point,
        all_stones[5].clone(),
        &mut occupied_fields,
        &stream,
    ); */
    /*
    // print field 7x4
    /*
    let mut falling_stone = stone_one.clone();
    falling_stone.move_to(Position {
        x: 2,
        y: starting_point,
    });
    */

    draw_field(starting_point, &occupied_fields, &falling_stone);

    let jet_stream = stream.chars().nth(steps % stream.len()).unwrap();
    if jet_stream == '>' {
        if falling_stone.can_move_right() {
            falling_stone.move_right();
        }
    } else {
        if falling_stone.can_move_left() {
            falling_stone.move_left();
        }
    }

    draw_field(starting_point, &occupied_fields, &falling_stone);

    while falling_stone.can_move_down(&occupied_fields) {
        steps += 1;
        falling_stone.move_down();

        let jet_stream = stream.chars().nth(steps % stream.len()).unwrap();
        if jet_stream == '>' {
            if falling_stone.can_move_right() {
                falling_stone.move_right();
            }
        } else {
            if falling_stone.can_move_left() {
                falling_stone.move_left();
            }
        }
        draw_field(starting_point, &occupied_fields, &falling_stone);
    }

    falling_stone.fields.iter().for_each(|f| {
        occupied_fields.insert(f.clone());
    });

    let highest_point = falling_stone
        .fields
        .iter()
        .min_by(|x, y| x.y.cmp(&y.y))
        .unwrap();
    starting_point = starting_point - highest_point.y;

    println!("Starting point {}", starting_point);

    let mut f_stone_two = stone_two.clone();
    f_stone_two.move_to(Position {
        x: 2,
        y: starting_point,
    });

    draw_field(starting_point, &occupied_fields, &f_stone_two);
    while f_stone_two.can_move_down(&occupied_fields) {
        f_stone_two.move_down();

        draw_field(starting_point, &occupied_fields, &f_stone_two);
    } */
}

fn draw_field(starting_point: i32, occupied_fields: &HashSet<Position>, falling_stone: &Stone) {
    /*
    for y in starting_point..4 {
        for x in 0..7 {
            let field = occupied_fields.get(&Position { x, y });
            let falling_stone_field = falling_stone.fields.get(&Position { x, y });

            if falling_stone_field.is_some() {
                print!("@");
            } else if field.is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
    */
}
