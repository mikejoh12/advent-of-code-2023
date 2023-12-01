use std::{collections::HashMap, fs};

fn find_calibration_part_2(s: &str) -> usize {
    let mut first_idx: usize = s.len() - 1;
    let mut last_idx: usize = 0;
    let mut first_digit: Option<usize> = Option::None;
    let mut last_digit: Option<usize> = Option::None;

    let digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    for (digit, value) in digits {
        match (s.find(digit), s.rfind(digit)) {
            (Some(l_idx), Some(r_idx)) => {
                if r_idx >= last_idx {
                    last_idx = r_idx.max(last_idx);
                    last_digit = Some(value);
                }
                if l_idx <= first_idx {
                    first_idx = l_idx.min(first_idx);
                    first_digit = Some(value);
                }
            }
            _ => (),
        }
    }

    first_digit.expect("Should have a digit") * 10 + last_digit.expect("Should have a digit")
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("Error opening file");
    let (mut total_1, mut total_2) = (0, 0);

    for line in text.split("\n") {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        total_1 += digits.first().unwrap() * 10 + digits.last().unwrap();

        total_2 += find_calibration_part_2(&line);
    }

    println!("Part 1: {}", total_1);
    println!("Part 2: {}", total_2);
}
