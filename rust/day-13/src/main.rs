use std::fs;

#[derive(PartialEq)]
enum Line {
    Vertical(usize),
    Horizontal(usize),
}

struct Pattern {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    part_1_line: Option<Line>,
}

impl Pattern {
    fn new(map: Vec<Vec<char>>) -> Pattern {
        let height = map.len();
        let width = map.first().expect("Should have at least 1 row").len();
        Pattern {
            map,
            width,
            height: height,
            part_1_line: None,
        }
    }

    fn reverse_point(&mut self, y: usize, x: usize) {
        if self.map[y][x] == '.' {
            self.map[y][x] = '#'
        } else {
            self.map[y][x] = '.'
        }
    }

    fn check_smudges(&mut self) -> Option<usize> {
        for y in 0..self.height {
            for x in 0..self.width {

                self.reverse_point(y, x);
                if let Some(v) = self.find_refl_across_vert() {
                    return Some(v);
                }
                if let Some(v) = self.find_refl_across_hor() {
                    return Some(v * 100);
                }

                self.reverse_point(y, x);
            }
        }
        None
    }

    fn find_refl_across_hor(&mut self) -> Option<usize> {
        'find_loop: for y_line in 1..self.height {
            let edge_dist = y_line.min(self.height - y_line);

            for x in 0..self.width {
                for y_offset in 0..=edge_dist {
                    if self.map[y_line - y_offset][x] != self.map[y_line + y_offset - 1][x] {
                        continue 'find_loop;
                    }
                }
            }

            if self.part_1_line == None {
                self.part_1_line = Some(Line::Horizontal(y_line));
                return Some(y_line);
            }
            if Some(Line::Horizontal(y_line)) != self.part_1_line {
                return Some(y_line);
            }
        }
        None
    }

    fn find_refl_across_vert(&mut self) -> Option<usize> {
        'find_loop: for x_line in 1..self.width {
            let edge_dist = x_line.min(self.width - x_line);

            for y in 0..self.height {
                for x_offset in 0..=edge_dist {
                    if self.map[y][x_line - x_offset] != self.map[y][x_line + x_offset - 1] {
                        continue 'find_loop;
                    }
                }
            }

            if self.part_1_line == None {
                self.part_1_line = Some(Line::Vertical(x_line));
                return Some(x_line);
            }
            if Some(Line::Vertical(x_line)) != self.part_1_line {
                return Some(x_line);
            }
        }
        None
    }
}

fn main() {
    let (mut total_1, mut total_2) = (0, 0);

    for (_, section) in fs::read_to_string("input.txt")
        .expect("File should exist")
        .split("\n\n")
        .enumerate()
    {
        let map: Vec<Vec<char>> = section
            .split("\n")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let mut pattern = Pattern::new(map);

        match pattern.find_refl_across_vert() {
            Some(v) => {
                total_1 += v;
            }
            None => match pattern.find_refl_across_hor() {
                Some(v) => {
                    total_1 += 100 * v;
                }
                None => println!("No line found for part 1"),
            },
        }

        if let Some(v) = pattern.check_smudges() {
            total_2 += v;
        }
    }
    println!("Part 1: {}", total_1);
    println!("Part 2: {}", total_2);
}