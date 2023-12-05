use std::fs;

struct PlantMap {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

fn make_map(s: &String) -> Vec<PlantMap> {
    let soil_to_fert_str: String = s
        .chars()
        .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
        .collect::<String>()
        .trim_start()
        .to_string();

    let mut maps: Vec<PlantMap> = Vec::new();

    for line in soil_to_fert_str.split("\n") {
        let nrs: Vec<i64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        maps.push(PlantMap { destination_start: nrs[0], source_start: nrs[1], length: nrs[2] });
    }
    maps
}

fn main() {
    let sections: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let seeds: Vec<i64> = sections[0]
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut maps: Vec<Vec<PlantMap>> = vec![];

    for idx in 1..sections.len() {
        maps.push(make_map(&sections[idx]));
    }

    let mut results: Vec<i64> = Vec::new();

    for seed in seeds {
        let mut num: i64 = seed;

        for map in &maps {
            for m in map {
                if num >= m.source_start && num <=  m.source_start + m.length {
                    num += m.destination_start - m.source_start;
                    break;
                }
            }
        }

        results.push(num);
    }

    println!("{}", results.iter().min().expect("Should have result"));
}
