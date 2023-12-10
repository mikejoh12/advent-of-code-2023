use std::{collections::HashMap, fs, vec};

#[derive(Debug)]
enum HandType {
    FiveOFAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn hand_to_value(hand: &HandType) -> usize {
    match hand {
        HandType::FiveOFAKind => 6,
        HandType::FourOfAKind => 5,
        HandType::FullHouse => 4,
        HandType::ThreeOfAKind => 3,
        HandType::TwoPair => 2,
        HandType::OnePair => 1,
        HandType::HighCard => 0,
    }
}

fn cards_to_values(cards: &Vec<char>) -> Vec<i32> {
    let mapping: HashMap<char, i32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);
    let mut values: Vec<i32> = vec![];
    for card in cards {
        values.push(*mapping.get(card).unwrap())
    }
    values
}

struct Hand {
    value: i32,
    hand_type: HandType,
    card_values: Vec<i32>,
}

impl Hand {
    fn new(cards: Vec<char>, value: i32) -> Self {
        let counts = count_cards(&cards);
        let hand_type = get_hand_type(&counts);
        let card_values = cards_to_values(&cards);

        Self {
            value,
            hand_type,
            card_values,
        }
    }
}

fn count_cards(cards: &Vec<char>) -> HashMap<char, i32> {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in cards {
        *counts.entry(*c).or_insert(0) += 1;
    }
    counts
}

fn is_5_of_a_kind(card_counts: &HashMap<char, i32>) -> bool {
    card_counts.len() == 1
}

fn is_4_of_a_kind(card_counts: &HashMap<char, i32>) -> bool {
    card_counts.values().any(|&c| c == 4)
}

fn is_full_house(card_counts: &HashMap<char, i32>) -> bool {
    card_counts.values().any(|&c| c == 3) && card_counts.values().any(|&c| c == 2)
}

fn is_3_of_a_kind(card_counts: &HashMap<char, i32>) -> bool {
    card_counts.values().any(|&c| c == 3)
}

fn is_2_pair(card_counts: &HashMap<char, i32>) -> bool {
    card_counts.values().filter(|&&c| c == 2).count() == 2
}

fn is_1_pair(card_counts: &HashMap<char, i32>) -> bool {
    card_counts.values().any(|&c| c == 2)
}

fn get_hand_type(card_counts: &HashMap<char, i32>) -> HandType {
    if is_5_of_a_kind(card_counts) {
        HandType::FiveOFAKind
    } else if is_4_of_a_kind(card_counts) {
        HandType::FourOfAKind
    } else if is_full_house(card_counts) {
        HandType::FullHouse
    } else if is_3_of_a_kind(card_counts) {
        HandType::ThreeOfAKind
    } else if is_2_pair(card_counts) {
        HandType::TwoPair
    } else if is_1_pair(card_counts) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn main() {
    let mut hands: Vec<Hand> = Vec::new();

    for line in fs::read_to_string("input.txt")
        .expect("File should open")
        .lines()
    {
        let mut parts = line.split(" ");
        let cards: Vec<char> = parts
            .nth(0)
            .expect("Should parse 1st column")
            .chars()
            .collect();
        let value: i32 = parts
            .next()
            .expect("Should parse 2nd column")
            .parse()
            .expect("Should be nr");

        hands.push(Hand::new(cards, value));
    }

    hands.sort_unstable_by_key(|h| {
        (
            hand_to_value(&h.hand_type),
            h.card_values[0],
            h.card_values[1],
            h.card_values[2],
            h.card_values[3],
            h.card_values[4],
        )
    });

    let mut part_1: i32 = 0;
    for (idx, hand) in hands.iter().enumerate() {
        part_1 += (idx as i32 + 1) * hand.value;
    }
    println!("Part 1: {}", part_1);
}
