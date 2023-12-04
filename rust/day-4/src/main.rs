use std::{collections::HashSet, fs};

struct Card {
    instances: i32,
    winning: HashSet<i32>,
    numbers: Vec<i32>,
}

fn main() {
    let (mut total_1, mut total_2) = (0, 0);
    let mut cards: Vec<Card> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts: Vec<String> = line.split(": ").map(|s| s.to_string()).collect();
        let nr_sections: Vec<String> = parts[1].split(" | ").map(|s| s.to_string()).collect();

        cards.push(Card {
            instances: 1,
            winning: nr_sections[0]
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            numbers: nr_sections[1]
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        })
    }

    for i in 0..cards.len() {
        total_2 += cards[i].instances;

        let winning_nrs = cards[i]
            .numbers
            .iter()
            .filter(|n| cards[i].winning.contains(n))
            .count();
        if winning_nrs > 0 {
            total_1 += 2usize.pow(winning_nrs as u32 - 1);
        }

        for j in (i + 1)..(i + 1 + winning_nrs) {
            cards[j].instances += cards[i].instances;
        }
    }

    println!("{}", total_1);
    println!("{}", total_2);
}
