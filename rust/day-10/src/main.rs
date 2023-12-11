use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

struct NoCircularPathError;
struct WrongCharError;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

struct Diagram {
    map: HashMap<Pos, Vec<Pos>>,
    start_pos: Pos,
    max_x: i32,
    max_y: i32,
}

fn try_path(d: &mut Diagram) -> Result<(i32, Vec<Pos>), NoCircularPathError> {
    let mut cur_pos: Pos = d.start_pos;
    let mut last_pos: Pos = cur_pos;
    let mut steps: i32 = 0;
    let mut path: Vec<Pos> = vec![];
    'move_loop: loop {
        path.push(cur_pos);
        if steps > 0 && cur_pos == d.start_pos {
            return Ok((steps, path));
        }

        for option in d.map.get(&cur_pos).expect("Pos should be mapped") {
            if option.x < 0 || option.x > d.max_x || option.y < 0 || option.y > d.max_y {
                return Err(NoCircularPathError);
            }
            if *option != last_pos && d.map.get(option).unwrap().contains(&cur_pos) {
                last_pos = cur_pos;
                cur_pos = *option;
                steps += 1;
                continue 'move_loop;
            }
        }
    }
}

fn try_paths(d: &mut Diagram) -> Result<(i32, Vec<Pos>), NoCircularPathError> {
    for c in ['|', '-', 'L', 'J', '7', 'F'] {
        let mut targets: Vec<Pos> = vec![];
        match c {
            '|' | '-' | 'L' | 'J' | '7' | 'F' => {
                if let Ok(v) = get_offset(c, d.start_pos.x, d.start_pos.y) {
                    targets.extend_from_slice(&v);
                }
            }
            _ => (),
        }
        d.map.insert(d.start_pos, targets);
        match try_path(d) {
            Ok(v) => return Ok(v),
            Err(_) => (),
        }
    }
    Err(NoCircularPathError)
}

fn expand_path_by_2x(path: &Vec<Pos>) -> HashSet<Pos> {
    let mut pipes: HashSet<Pos> = HashSet::new();
    for i in 0..(path.len() - 1) {
        let new_p1 = Pos {
            x: path[i].x * 2,
            y: path[i].y * 2,
        };
        let new_p2 = Pos {
            x: path[i + 1].x * 2,
            y: path[i + 1].y * 2,
        };
        let new_p3 = Pos {
            x: (new_p1.x + new_p2.x) / 2,
            y: (new_p1.y + new_p2.y) / 2,
        };
        pipes.insert(new_p1);
        pipes.insert(new_p3);
    }
    pipes
}

fn get_area(path: &Vec<Pos>, max_x: i32, max_y: i32) -> i32 {
    let larger_path: HashSet<Pos> = expand_path_by_2x(path);
    let mut filled_map: HashSet<Pos> = larger_path.clone();
    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(Pos { x: -1, y: -1 });
    filled_map.insert(Pos { x: -1, y: -1 });

    while queue.len() > 0 {
        let pos = queue.pop_front().expect("Queue should have a pos");

        for offset in [
            Pos { x: 0, y: -1 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: 1 },
            Pos { x: -1, y: 0 },
        ] {
            let new_pos = Pos {
                x: pos.x + offset.x,
                y: pos.y + offset.y,
            };
            if new_pos.x < -1
                || new_pos.x > 2 * max_x + 1
                || new_pos.y < -1
                || new_pos.y > max_y * 2 + 1
            {
                continue;
            }
            if !filled_map.contains(&new_pos) {
                filled_map.insert(new_pos);
                queue.push_back(new_pos);
            }
        }
    }

    let mut area = 0;
    for x in 0..(max_x * 2 + 1) {
        for y in 0..(max_y * 2 + 1) {
            if x % 2 == 0 && y % 2 == 0 && !filled_map.contains(&Pos { x, y }) {
                area += 1;
            }
        }
    }
    area
}

fn get_offset(c: char, x: i32, y: i32) -> Result<[Pos; 2], WrongCharError> {
    match c {
        '|' => Ok([
            Pos {
                x: x as i32,
                y: y as i32 - 1,
            },
            Pos {
                x: x as i32,
                y: y as i32 + 1,
            },
        ]),
        '-' => Ok([
            Pos {
                x: x as i32 - 1,
                y: y as i32,
            },
            Pos {
                x: x as i32 + 1,
                y: y as i32,
            },
        ]),
        'L' => Ok([
            Pos {
                x: x as i32,
                y: y as i32 - 1,
            },
            Pos {
                x: x as i32 + 1,
                y: y as i32,
            },
        ]),
        'J' => Ok([
            Pos {
                x: x as i32,
                y: y as i32 - 1,
            },
            Pos {
                x: x as i32 - 1,
                y: y as i32,
            },
        ]),
        '7' => Ok([
            Pos {
                x: x as i32 - 1,
                y: y as i32,
            },
            Pos {
                x: x as i32,
                y: y as i32 + 1,
            },
        ]),
        'F' => Ok([
            Pos {
                x: x as i32 + 1,
                y: y as i32,
            },
            Pos {
                x: x as i32,
                y: y as i32 + 1,
            },
        ]),
        _ => Err(WrongCharError),
    }
}

fn main() {
    let mut diagram_map: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let mut start_pos = Pos { x: -1, y: -1 };
    let (mut max_x, mut max_y) = (0, 0);

    for (y, line) in fs::read_to_string("input.txt")
        .expect("File should exist")
        .lines()
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            let mut targets: Vec<Pos> = vec![];
            match c {
                '|' | '-' | 'L' | 'J' | '7' | 'F' => {
                    if let Ok(v) = get_offset(c, x as i32, y as i32) {
                        targets.extend_from_slice(&v);
                    }
                }
                'S' => {
                    start_pos = Pos {
                        x: x as i32,
                        y: y as i32,
                    };
                }
                _ => (),
            }
            max_x = x as i32;
            max_y = y as i32;
            diagram_map.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                targets,
            );
        }
    }

    let mut diagram: Diagram = Diagram {
        map: diagram_map,
        start_pos,
        max_x,
        max_y,
    };
    let result = try_paths(&mut diagram);

    match result {
        Ok(v) => {
            println!("Part 1: {:?}", v.0 / 2);
            println!("Part 2: {}", get_area(&v.1, diagram.max_x, diagram.max_y));
        }
        Err(_) => println!("Didn't find a path"),
    }
}
