use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

#[derive(Debug)]
struct Galaxy {
    id: usize,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Space {
    map: Vec<Vec<char>>,
    galaxies: Vec<Galaxy>,
}

impl Space {
    fn expand_vert(&mut self) {
        let mut expanded: Vec<Vec<char>> = vec![];
        for row in self.map.iter() {
            if !row.contains(&'#') {
                for _ in 0..1_000_000 {
                    expanded.push(row.to_vec());
                }
            }
            expanded.push(row.to_vec());
        }
        self.map = expanded;
    }

    fn expand_hor(&mut self) {
        let mut expanded: Vec<Vec<char>> = vec![vec![];self.map.len()];
        for i in 0..self.map.first().expect("Map row should not be empty").len() {
            if self.map.iter().all(|r|r.iter().nth(i).unwrap() != &'#') {
                for _ in 0..1_000_000 {
                    for j in 0..expanded.len()  {
                        expanded[j].push('.');
                    }
                }
            }
            for j in 0..expanded.len()  {
                expanded[j].push(self.map[j][i]);
            }
        }
        self.map = expanded;
    }

    fn add_galaxies(&mut self) {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut id: usize = 1;
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    galaxies.push(Galaxy { id, x, y});
                    id += 1;
                }
            }
        }
        self.galaxies = galaxies;
    }

    fn get_distances(&self) -> usize {
        let mut length_sum = 0;
        for pair in self.galaxies.iter().combinations(2) {
            let x_diff = pair[0].x.max(pair[1].x) - pair[0].x.min(pair[1].x);
            let y_diff = pair[0].y.max(pair[1].y) - pair[0].y.min(pair[1].y);
            length_sum += x_diff + y_diff;
        }
        length_sum
    }
}

fn main() {
    let mut map: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let row: Vec<char> = ip.chars().collect();
                map.push(row);
            }
        }
    }
    let mut space: Space = Space{ map, galaxies: vec![] };
    space.expand_vert();
    space.expand_hor();
    space.add_galaxies();
    //println!("Space {:?}", space);
    println!("Part 1: {}", space.get_distances());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
