use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

struct Schematic {
    map: Vec<Vec<char>>,
    symbols: HashSet<Pos>,
    gears: HashMap<Pos, Vec<i32>>,
    offsets: [[i32; 2]; 8],
}

impl Schematic {
    fn new() -> Self {
        let mut schematic: Vec<Vec<char>> = vec![];

        for line in fs::read_to_string("input.txt").unwrap().lines() {
            schematic.push(line.chars().collect());
        }

        let mut symbols: HashSet<Pos> = HashSet::new();
        let mut gears: HashMap<Pos, Vec<i32>> = HashMap::new();

        for y in 0..schematic.len() {
            for x in 0..schematic.first().unwrap().len() {
                if !schematic[y][x].is_ascii_digit() && schematic[y][x] != '.' {
                    symbols.insert(Pos {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                if schematic[y][x] == '*' {
                    gears.insert(
                        Pos {
                            x: x as i32,
                            y: y as i32,
                        },
                        vec![],
                    );
                }
            }
        }

        Schematic {
            map: schematic,
            symbols,
            gears,
            offsets: [[-1, -1],[-1, 0],[-1, 1],[0, -1],[0, 1],[1, -1],[1, 0],[1, 1]],
        }
    }

    fn get_gear_ratio_sum(&self) -> i32 {
        self.gears
            .values()
            .fold(0, |a, c| if c.len() == 2 { a + c[0] * c[1] } else { a } )
    }

    fn mark_nr_in_gears(&mut self, nr: i32, locations: &Vec<Pos>) {
        let mut neighbor_gears: HashSet<Pos> = HashSet::new();

        for location in locations {
            for offset in &self.offsets {
                if self.gears.contains_key(&Pos {
                    x: location.x + offset[1],
                    y: location.y + offset[0],
                }) {
                    neighbor_gears.insert(Pos {
                        x: location.x + offset[1],
                        y: location.y + offset[0],
                    });
                }
            }
        }

        for gear in neighbor_gears {
            self.gears.entry(gear).or_insert(Vec::new()).push(nr);
        }
    }

    fn is_valid(&self, nr_locations: &Vec<Pos>) -> bool {
        for nr_location in nr_locations {
            for offset in &self.offsets {
                if self.symbols.contains(&Pos {
                    x: 0.max(nr_location.x + offset[1]),
                    y: 0.max(nr_location.y + offset[0]),
                }) {
                    return true;
                }
            }
        }
        false
    }

    fn calculate(&mut self) -> (i32, i32) {
        let mut total: i32 = 0;

        for y in 0..self.map.len() {
            let mut digit_str = String::new();
            let mut nr_locations: Vec<Pos> = vec![];

            for x in 0..self.map.first().unwrap().len() {
                if self.map[y][x].is_ascii_digit() {
                    digit_str.push(self.map[y][x]);
                    nr_locations.push(Pos {
                        y: y as i32,
                        x: x as i32,
                    });
                } else if digit_str.len() > 0 {
                    if self.is_valid(&nr_locations) {
                        let nr = digit_str.parse::<i32>().unwrap();
                        total += nr;
                        self.mark_nr_in_gears(nr, &nr_locations);
                    }
                    digit_str.clear();
                    nr_locations.clear();
                }
            }
            if digit_str.len() > 0 && self.is_valid(&nr_locations) {
                let nr = digit_str.parse::<i32>().unwrap();
                total += nr;
                self.mark_nr_in_gears(nr, &nr_locations);
            }
            digit_str.clear();
            nr_locations.clear();
        }

        (total, self.get_gear_ratio_sum())
    }
}

fn main() {
    let mut schematic = Schematic::new();
    let answers = schematic.calculate();

    println!("Part 1: {}", answers.0);
    println!("Part 2: {}", answers.1);
}
