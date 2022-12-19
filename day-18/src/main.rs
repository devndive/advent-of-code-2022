use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, vec,
};

#[derive(PartialEq, Eq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn from(input: &str) -> Self {
        let parts = input.split(",").collect::<Vec<&str>>();

        Self {
            x: parts[0].parse::<i32>().unwrap(),
            y: parts[1].parse::<i32>().unwrap(),
            z: parts[2].parse::<i32>().unwrap(),
        }
    }

    fn sourrounding_fields(&self) -> Vec<(i32, i32, i32)> {
        vec![
            (self.x + 1, self.y, self.z),
            (self.x - 1, self.y, self.z),
            (self.x, self.y + 1, self.z),
            (self.x, self.y - 1, self.z),
            (self.x, self.y, self.z + 1),
            (self.x, self.y, self.z - 1),
        ]
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();

    let cubes = input
        .split("\n")
        .map(|c| Cube::from(c))
        .collect::<Vec<Cube>>();

    let mut free_sides = 0;

    cubes.iter().for_each(|c| {
        // check x-axis -1
        let checked = vec![
            cubes
                .iter()
                .any(|inner_c| inner_c.x == c.x - 1 && inner_c.y == c.y && inner_c.z == c.z),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == c.x + 1 && inner_c.y == c.y && inner_c.z == c.z),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == c.x && inner_c.y == c.y - 1 && inner_c.z == c.z),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == c.x && inner_c.y == c.y + 1 && inner_c.z == c.z),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == c.x && inner_c.y == c.y && inner_c.z == c.z - 1),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == c.x && inner_c.y == c.y && inner_c.z == c.z + 1),
        ];

        free_sides += 6 - checked.iter().filter(|c| **c).collect::<Vec<&bool>>().len();
    });

    println!("Part 1: {}", free_sides);

    let mut sourrounding_fields = HashSet::new();

    for c in cubes.iter() {
        let fields = c.sourrounding_fields();

        for f in fields {
            if !cubes
                .iter()
                .any(|inner_c| inner_c.x == f.0 && inner_c.y == f.1 && inner_c.z == f.2)
                /*
                && cubes
                    .iter()
                    .any(|inner_c| f.0 > inner_c.x && f.1 == inner_c.y && f.2 == inner_c.z)
                && cubes
                    .iter()
                    .any(|inner_c| f.0 < inner_c.x && f.1 == inner_c.y && f.2 == inner_c.z)
                && cubes
                    .iter()
                    .any(|inner_c| f.1 > inner_c.y && f.0 == inner_c.x && f.2 == inner_c.z)
                && cubes
                    .iter()
                    .any(|inner_c| f.1 < inner_c.y && f.0 == inner_c.x && f.2 == inner_c.z)
                && cubes
                    .iter()
                    .any(|inner_c| f.2 > inner_c.z && f.0 == inner_c.x && f.1 == inner_c.y)
                && cubes
                    .iter()
                    .any(|inner_c| f.2 < inner_c.z && f.0 == inner_c.x && f.1 == inner_c.y)
                 */
            {
                sourrounding_fields.insert(f);
            }
        }
    }

    let mut enclosed_count = 0;
    // find all fields that are directly enclosed by cubes

    let mut not_directly_enclosed = vec![];
    sourrounding_fields.iter().for_each(|f| {
        println!("Checking {:?}", f);

        let checked = vec![
            cubes
                .iter()
                .any(|inner_c| inner_c.x == f.0 - 1 && inner_c.y == f.1 && inner_c.z == f.2),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == f.0 + 1 && inner_c.y == f.1 && inner_c.z == f.2),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == f.0 && inner_c.y == f.1 - 1 && inner_c.z == f.2),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == f.0 && inner_c.y == f.1 + 1 && inner_c.z == f.2),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == f.0 && inner_c.y == f.1 && inner_c.z == f.2 - 1),
            cubes
                .iter()
                .any(|inner_c| inner_c.x == f.0 && inner_c.y == f.1 && inner_c.z == f.2 + 1),
        ];

        if checked.iter().all(|c| c == &true) {
            println!("{:?}", f);
            enclosed_count += 1;
        } else {
            not_directly_enclosed.push(f);
        }
    });

    not_directly_enclosed.iter().for_each(|f| {
        let f = **f;

        let mut visited: Vec<(i32, i32, i32)> = vec![];
        visited.push(f);

        let mut que: VecDeque<(i32, i32, i32)> = VecDeque::new();
        que.push_back(f);

        let mut did_abort = false;

        while !que.is_empty() {
            let item = que.pop_front().unwrap();

            println!("{} vs {}", que.len(), not_directly_enclosed.len());
            if que.len() > not_directly_enclosed.len() + 20 {
                // Break, it seems we are adding more fields than we have enclosed
                // fields. Probably leaking somewhere
                did_abort = true;
                break;
            }

            let fields = vec![
                (item.0 + 1, item.1, item.2),
                (item.0 - 1, item.1, item.2),
                (item.0, item.1 + 1, item.2),
                (item.0, item.1 - 1, item.2),
                (item.0, item.1, item.2 + 1),
                (item.0, item.1, item.2 - 1),
            ];

            for f in fields.iter() {
                if !visited.contains(&&f)
                    && !cubes.contains(&Cube {
                        x: f.0,
                        y: f.1,
                        z: f.2,
                    })
                {
                    que.push_back(*f);
                }
            }
        }

        if !did_abort {
            // que should be empty
            if !que.is_empty() {
                println!("Que still has items {}", que.len());
            }

            for item in visited {
                let fields = vec![
                    (item.0 + 1, item.1, item.2),
                    (item.0 - 1, item.1, item.2),
                    (item.0, item.1 + 1, item.2),
                    (item.0, item.1 - 1, item.2),
                    (item.0, item.1, item.2 + 1),
                    (item.0, item.1, item.2 - 1),
                ];

                let cubes_around = fields.iter().filter(|f| cubes.contains(&Cube { x: f.0, y: f.1, z: f.2 }));
                let cubes_around = cubes_around.collect::<Vec<&(i32, i32, i32)>>().len();

                free_sides = free_sides - cubes_around;
            }
        }
    });

    println!("Part 2: {}", free_sides - (enclosed_count * 6));
}
