use std::fs;

fn is_race_win(race_time: u64, acc_time: u64, record_dist: u64) -> bool {
    acc_time * (race_time - acc_time) > record_dist
}

fn parse_line(line: &str) -> Vec<u64> {
    line.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn join_numbers(nums: &Vec<u64>) -> u64 {
    nums.iter()
        .map(|t| t.to_string())
        .fold(String::new(), |a, c| a + &c)
        .parse()
        .expect("Should parse to a number")
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File should exist");
    let mut lines = file.split("\n");

    let times = parse_line(lines.nth(0).unwrap());
    let distances = parse_line(lines.next().unwrap());

    let mut nr_ways_to_win_mult: u64 = 1;

    for i in 0..times.len() {
        let mut ways_to_win: u64 = 0;

        for button_time in 1..times[i] {
            if is_race_win(times[i], button_time, distances[i]) {
                ways_to_win += 1;
            }
        }

        nr_ways_to_win_mult *= ways_to_win;
    }

    let part_2_time: u64 = join_numbers(&times);
    let part_2_distance: u64 = join_numbers(&distances);

    let mut ways_to_win_2 = 0;

    for button_time in 1..part_2_time {
        if is_race_win(part_2_time, button_time, part_2_distance) {
            ways_to_win_2 += 1;
        }
    }
    println!("Part 1: {}", nr_ways_to_win_mult);
    println!("Part 2: {}", ways_to_win_2);
}
