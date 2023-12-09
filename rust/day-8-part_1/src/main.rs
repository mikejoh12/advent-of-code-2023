use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let re = Regex::new(r"[A-Z]{3}").unwrap();
    let mut node_map: HashMap<String, [String; 2]> = HashMap::new();

    let input = fs::read_to_string("input.txt").expect("File should open");
    let sections: Vec<&str> = input.split("\n\n").collect();
    let instructions: Vec<char> = sections[0].chars().collect();

    for line in sections[1].split("\n") {
        let matches: Vec<&str> = re
            .find_iter(line)
            .filter_map(|m| Some(m.as_str()))
            .collect();
        node_map.insert(
            matches[0].to_string(),
            [matches[1].to_string(), matches[2].to_string()],
        );
    }

    let mut steps: usize = 0;
    let mut pos: String = "AAA".to_string();
    loop {
        let cur_instr = instructions[steps % instructions.len()];
        if cur_instr == 'L' {
            pos = node_map.get(&pos).expect("Node should exist")[0].to_string();
        } else if cur_instr == 'R' {
            pos = node_map.get(&pos).expect("Node should exist")[1].to_string();
        }
        steps += 1;
        if pos == "ZZZ" {
            break;
        }
    }
    println!("Part 1: {}", steps);
}
