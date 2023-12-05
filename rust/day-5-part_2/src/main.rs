use std::{fs, collections::VecDeque};

#[derive(Debug, Copy, Clone)]
struct Range {
    min: i64,
    max: i64,
}

struct RangeCmpResult {
    no_overlap: Vec<Range>,
    overlap: Range,
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        !(self.min > other.max || self.max < other.min)
    }

    fn get_overlap(&self, other: &Range) -> Range {
        Range {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }

    fn compare_ranges(&self, other: &Range) -> RangeCmpResult {
        let mut outside: Vec<Range> = Vec::new();

        if self.min < other.min {
            outside.push(Range {
                min: self.min,
                max: other.min - 1,
            })
        }
        if self.max > other.max {
            outside.push(Range {
                min: other.max + 1,
                max: self.max,
            })
        }

        RangeCmpResult {
            no_overlap: outside,
            overlap: self.get_overlap(other),
        }
    }
}

struct PlantMap {
    source_range: Range,
    correction: i64,
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
        maps.push(PlantMap {
            source_range: Range {
                min: nrs[1],
                max: nrs[1] + nrs[2],
            },
            correction: nrs[0] - nrs[1],
        });
    }

    maps
}

fn main() {
    let sections: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let seeds_1: Vec<i64> = sections[0]
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut seeds_2: Vec<Range> = Vec::new();
    for i in (0..seeds_1.len()).step_by(2) {
        seeds_2.push(Range { min: seeds_1[i], max: seeds_1[i] + seeds_1[i+1] - 1 })
    }

    let mut maps: Vec<Vec<PlantMap>> = vec![];

    for idx in 1..sections.len() {
        maps.push(make_map(&sections[idx]));
    }

    let mut results: Vec<Range> = Vec::new();

    for seed_rng in seeds_2 {
        let mut ranges = vec![seed_rng];

        for map in &maps {
            let mut queue = VecDeque::from(ranges.clone());
            let mut unmapped: Vec<Range> = Vec::new();
            let mut mapped: Vec<Range> = Vec::new();

            while queue.len() > 0 {
                let cur_range = queue.pop_front().expect("Queue should not be empty");
                let mut is_overlapping = false;

                for m in map {
                    if cur_range.overlaps(&m.source_range) {
                        is_overlapping = true;
                        let result = cur_range.compare_ranges(&m.source_range);
                        mapped.push(Range { min: result.overlap.min + m.correction, max: result.overlap.max + m.correction });
                        for no_overlap_rng in result.no_overlap {
                            queue.push_back(no_overlap_rng);
                        }
                        break;
                    }
                }
                if !is_overlapping {
                    unmapped.push(cur_range);
                }

            }

            ranges = vec![unmapped, mapped].concat();
        }
        results.extend(&ranges);

    }
    let lowest: i64 = results.iter().map(|r|r.min).min().expect("Should have a result");
    println!("{}", lowest);    
}
