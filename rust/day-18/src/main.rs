use std::fs;
use std::time::Instant;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

struct DigPlan {
    points: Vec<Pos>,
}

struct Instruction {
    direction: Direction,
    steps: i64,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl DigPlan {
    fn new(instructions: &Vec<Instruction>) -> DigPlan {
        let mut cur = Pos { x: 0, y: 0 };
        let mut points: Vec<Pos> = vec![cur];

        for i in 0..instructions.len() {
            let mut extra = 0;
            if is_clockwise_corner(
                &instructions[(i as i32 - 1).rem_euclid(instructions.len() as i32) as usize]
                    .direction,
                &instructions[i].direction,
            ) && is_clockwise_corner(
                &instructions[i as usize].direction,
                &instructions[(i as i32 + 1).rem_euclid(instructions.len() as i32) as usize]
                    .direction,
            ) {
                extra += 1;
            }

            if !is_clockwise_corner(
                &instructions[(i as i32 - 1).rem_euclid(instructions.len() as i32) as usize]
                    .direction,
                &instructions[i].direction,
            ) && !is_clockwise_corner(
                &instructions[i as usize].direction,
                &instructions[(i as i32 + 1).rem_euclid(instructions.len() as i32) as usize]
                    .direction,
            ) {
                extra -= 1;
            }

            match instructions[i as usize].direction {
                Direction::Up => cur.y -= instructions[i as usize].steps + extra,
                Direction::Right => cur.x += instructions[i as usize].steps + extra,
                Direction::Down => cur.y += instructions[i as usize].steps + extra,
                Direction::Left => cur.x -= instructions[i as usize].steps + extra,
            }
            points.push(cur);
        }

        DigPlan { points }
    }

    fn find_area(&self) -> i64 {
        let mut area = 0;
        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];
            area += determinant_2x2_matrix([[p1.x, p2.x], [p1.y, p2.y]]);
        }
        let p_last = self.points.last().expect("Should have points");
        let p_first = self.points.first().expect("Should have points");
        area += determinant_2x2_matrix([[p_last.x, p_first.x], [p_last.y, p_first.y]]);
        area / 2
    }
}

fn determinant_2x2_matrix(mat: [[i64; 2]; 2]) -> i64 {
    mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
}

fn is_clockwise_corner(dir_1: &Direction, dir_2: &Direction) -> bool {
    match (dir_1, dir_2) {
        (Direction::Up, Direction::Right) => true,
        (Direction::Up, Direction::Left) => false,
        (Direction::Right, Direction::Up) => false,
        (Direction::Right, Direction::Down) => true,
        (Direction::Down, Direction::Right) => false,
        (Direction::Down, Direction::Left) => true,
        (Direction::Left, Direction::Up) => true,
        (Direction::Left, Direction::Down) => false,
        _ => panic!("Invalid turn in directions"),
    }
}

fn is_clockwise(instructions: &Vec<Instruction>) -> bool {
    let (mut right, mut left) = (0, 0);
    for i in 0..(instructions.len() - 1) {
        match (&instructions[i].direction, &instructions[i + 1].direction) {
            (Direction::Up, Direction::Right) => right += 1,
            (Direction::Up, Direction::Left) => left += 1,
            (Direction::Right, Direction::Up) => left += 1,
            (Direction::Right, Direction::Down) => right += 1,
            (Direction::Down, Direction::Right) => left += 1,
            (Direction::Down, Direction::Left) => right += 1,
            (Direction::Left, Direction::Up) => right += 1,
            (Direction::Left, Direction::Down) => left += 1,
            _ => panic!("Invalid turn in directions"),
        }
    }
    right > left
}

fn parse_part_1_instructions() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    for line in fs::read_to_string("input.txt")
        .expect("File should exist")
        .split("\n")
    {
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap();
        let steps: i64 = parts.next().unwrap().parse().unwrap();

        instructions.push(Instruction {
            direction: match dir {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!("Error parsing directions"),
            },
            steps,
        })
    }
    instructions
}

fn parse_part_2_instructions() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    for line in fs::read_to_string("input.txt")
        .expect("File should exist")
        .split("\n")
    {
        let mut parts = line.split(" ");
        let hex = parts
            .nth(2)
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(")");
        let steps = i64::from_str_radix(&hex[..5], 16).unwrap();

        instructions.push(Instruction {
            direction: match &hex[5..] {
                "3" => Direction::Up,
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                _ => panic!("Error parsing directions"),
            },
            steps,
        })
    }
    instructions
}

fn main() {
    let start = Instant::now();
    let part_1_instructions = parse_part_1_instructions();
    let part_2_instructions = parse_part_2_instructions();

    if !is_clockwise(&part_1_instructions) || !is_clockwise(&part_2_instructions) {
        panic!("Counter clockwise solution not implemented")
    }

    let dig_plan = DigPlan::new(&part_1_instructions);
    println!("Part 1: {}", dig_plan.find_area());

    let dig_plan_2 = DigPlan::new(&part_2_instructions);
    println!("Part 2: {}", dig_plan_2.find_area());

    println!("Time elapsed: {:?}", start.elapsed());
}
