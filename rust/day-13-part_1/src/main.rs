use std::fs;

struct Pattern {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn new(map: Vec<Vec<char>>) -> Pattern {
        let height = map.len();
        let width = map.first().expect("Should have at least 1 row").len();
        Pattern { map, width, height}
    }

    fn find_refl_across_hor(&self) -> Option<usize> {
        'find_loop: for y_line in 1..self.height {
            let edge_dist = y_line.min(self.height - y_line);

            for x in 0..self.width {
                for y_offset in 0..=edge_dist {
                    if self.map[y_line-y_offset][x] != self.map[y_line+y_offset-1][x] {
                        continue 'find_loop;
                    }
                }
            }

            return Some(y_line);
        }
        None
    }

    fn find_refl_across_vert(&self) -> Option<usize> {
        'find_loop: for x_line in 1..self.width {
            let edge_dist = x_line.min(self.width - x_line);

            for y in 0..self.height {
                for x_offset in 0..=edge_dist {
                    if self.map[y][x_line - x_offset] != self.map[y][x_line+x_offset - 1] {
                        continue 'find_loop;
                    }
                }
            }

            return Some(x_line);
        }
        None
    }
}

fn main() {
    let mut total = 0;

    for (_, section) in fs::read_to_string("input.txt").expect("File should exist").split("\n\n").enumerate() {
        let map: Vec<Vec<char>> = section.split("\n").map(|s|s.chars().collect::<Vec<char>>()).collect();
        let pattern = Pattern::new(map);

        match pattern.find_refl_across_vert() {
            Some(v) => {
                total += v;
            },
            None => (),
        }

        match pattern.find_refl_across_hor() {
            Some(v) => {
                total += 100 * v;
            },
            None => (),
        }
    }
    println!("Part 1: {}", total);
}
