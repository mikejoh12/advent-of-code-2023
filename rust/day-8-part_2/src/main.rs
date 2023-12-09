use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let re = Regex::new(r"[A-Z0-9]{3}").unwrap();
    let mut node_map: HashMap<String, [String; 2]> = HashMap::new();
    let mut start_nodes: Vec<String> = Vec::new();

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

        if matches[0].ends_with('A') {
            start_nodes.push(matches[0].to_string());
        }
    }

    let mut cycle_lengths: Vec<usize> = vec![];

    for start_node in start_nodes {
        let mut node = start_node.clone();
        let mut steps: usize = 0;

        loop {
            let cur_instr = instructions[steps % instructions.len()];

            if cur_instr == 'L' {
                node = node_map.get(&node).expect("Node should exist")[0].to_string();
            } else if cur_instr == 'R' {
                node = node_map.get(&node).expect("Node should exist")[1].to_string();
            }

            steps += 1;

            if node.ends_with('Z') {
                cycle_lengths.push(steps);
                break;
            }
        }
    }

    let lcm_of_cycles = cycle_lengths
        .iter()
        .fold(cycle_lengths[0], |a, c| lcm(a, *c));
    println!("Part 2: {}", lcm_of_cycles);
}
