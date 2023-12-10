use std::{fs, vec};

fn add_rows(values: &mut Vec<Vec<i32>>) {
    while !values[values.len() - 1].iter().all(|&n| n == 0) {
        let mut new_values: Vec<i32> = vec![];

        for i in 0..values[values.len() - 1].len() - 1 {
            let diff = values[values.len() - 1][i + 1] - values[values.len() - 1][i];
            new_values.push(diff);
        }
        values.push(new_values);
    }
}

fn get_extrapolated_left(nums: &Vec<i32>) -> i32 {
    let mut values = vec![nums.clone()];
    add_rows(&mut values);
    fill_placeholders_left(&mut values);
    *values
        .first()
        .expect("Should have rows")
        .first()
        .expect("Should have a value")
}

fn get_extrapolated_right(nums: &Vec<i32>) -> i32 {
    let mut values = vec![nums.clone()];
    add_rows(&mut values);
    fill_placeholders_right(&mut values);
    *values
        .first()
        .expect("Should have rows")
        .last()
        .expect("Should have a value")
}

fn fill_placeholders_left(nums: &mut Vec<Vec<i32>>) {
    nums.last_mut().expect("Should have a vector").insert(0, 0);
    for i in (0..nums.len() - 1).rev() {
        let new_v = nums[i].first().expect("Should have elements")
            - nums[i + 1].first().expect("Should have elements");
        nums[i].insert(0, new_v);
    }
}

fn fill_placeholders_right(nums: &mut Vec<Vec<i32>>) {
    nums.last_mut().expect("Should have a vector").push(0);

    for i in (0..nums.len() - 1).rev() {
        let new_v = nums[i].last().expect("Should have elements")
            + nums[i + 1].last().expect("Should have elements");
        nums[i].push(new_v);
    }
}

fn main() {
    let (mut total_1, mut total_2) = (0, 0);

    for line in fs::read_to_string("input.txt")
        .expect("File should open")
        .lines()
    {
        let nums: Vec<i32> = line
            .split(" ")
            .map(|s| s.parse().expect("Should be a number"))
            .collect();

        total_1 += get_extrapolated_right(&nums);
        total_2 += get_extrapolated_left(&nums);
    }

    println!("Part 1: {}", total_1);
    println!("Part 2: {}", total_2);
}
