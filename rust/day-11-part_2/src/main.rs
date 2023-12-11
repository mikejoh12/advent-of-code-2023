use itertools::Itertools;
use std::fs;
use std::vec;

struct Galaxy {
    x: usize,
    y: usize,
}

struct Space {
    map: Vec<Vec<char>>,
    galaxies: Vec<Galaxy>,
    galaxies_expanded: Vec<Galaxy>,
    row_expansions: Vec<usize>,
    col_expansions: Vec<usize>,
}

impl Space {
    fn add_galaxies(&mut self) {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    galaxies.push(Galaxy { x, y });
                }
            }
        }
        self.galaxies = galaxies;
    }

    fn find_expansions(&mut self) {
        for (y, row) in self.map.iter().enumerate() {
            if !row.contains(&'#') {
                self.row_expansions.push(y);
            }
        }

        for x in 0..self.map.first().expect("Map row should not be empty").len() {
            if self.map.iter().all(|r| r.iter().nth(x).unwrap() != &'#') {
                self.col_expansions.push(x);
            }
        }
    }

    fn expand(&mut self, n: usize) {
        let mut galaxies: Vec<Galaxy> = vec![];
        for galaxy in &self.galaxies {
            let add_rows = self
                .row_expansions
                .iter()
                .filter(|&y| y < &galaxy.y)
                .count()
                * (n - 1);
            let add_cols = self
                .col_expansions
                .iter()
                .filter(|&x| x < &galaxy.x)
                .count()
                * (n - 1);
            galaxies.push(Galaxy {
                x: galaxy.x + add_cols,
                y: galaxy.y + add_rows,
            })
        }
        self.galaxies_expanded = galaxies
    }

    fn get_distances(&self) -> usize {
        let mut length_sum = 0;
        for pair in self.galaxies_expanded.iter().combinations(2) {
            let x_diff = pair[0].x.max(pair[1].x) - pair[0].x.min(pair[1].x);
            let y_diff = pair[0].y.max(pair[1].y) - pair[0].y.min(pair[1].y);
            length_sum += x_diff + y_diff;
        }
        length_sum
    }
}

fn main() {
    let map: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("File should open")
        .split("\n")
        .map(|r| r.chars().collect())
        .collect();
    let mut space = Space {
        map,
        galaxies: vec![],
        galaxies_expanded: vec![],
        row_expansions: vec![],
        col_expansions: vec![],
    };
    space.add_galaxies();
    space.find_expansions();

    space.expand(2);
    println!("Part 1: {}", space.get_distances());

    space.expand(1_000_000);
    println!("Part 2: {}", space.get_distances());
}
