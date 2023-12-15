use regex::Regex;
use std::{collections::VecDeque, fs, hash, ops::Index};

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Clone)]
struct Box {
    lenses: VecDeque<Lens>,
}

impl Box {
    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|l| l.label != *label);
    }

    fn add_lens(&mut self, label: &str, focal_length: u32) {
        for lens in &mut self.lenses {
            if lens.label == label {
                *lens = Lens {
                    label: label.to_string(),
                    focal_length,
                };
                return;
            }
        }
        self.lenses.push_back(Lens {
            label: label.to_string(),
            focal_length,
        });
    }
}

fn hash_label(l: &String) -> u32 {
    let mut cur_v = 0;
    for c in l.chars() {
        cur_v = (cur_v + c as u32) * 17 % 256;
    }
    cur_v
}

fn part_1(input: &Vec<String>) -> u32 {
    input.iter().fold(0, |a, c| a + hash_label(c))
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .expect("file should exist")
        .split(",")
        .map(|s| s.to_string())
        .collect();

    println!("Part 1: {}", part_1(&input));

    let mut boxes: Vec<Box> = vec![
        Box {
            lenses: VecDeque::new()
        };
        256
    ];

    for instr in input {
        let re = Regex::new(r"[a-zA-Z]+").unwrap();
        let label = re.find(&instr).unwrap().as_str();
        let box_idx = hash_label(&label.to_string());

        if instr.contains("-") {
            boxes[box_idx as usize].remove_lens(label);
        } else {
            let parts: Vec<String> = instr.split("=").map(|s| s.to_string()).collect();
            let focal_length: u32 = parts[1].parse().expect("Number should parse");
            boxes[box_idx as usize].add_lens(label, focal_length)
        }
    }

    let mut total_2 = 0;

    for (box_idx, cur_box) in boxes.iter().enumerate() {
        for (lens_idx, lens) in cur_box.lenses.iter().enumerate() {
            total_2 += (box_idx + 1) * (lens_idx + 1) * lens.focal_length as usize
        }
    }

    println!("Part 2: {:}", total_2);
}
