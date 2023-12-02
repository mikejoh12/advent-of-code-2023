use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (mut possible, mut power_sum) = (0, 0);

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut parts = ip.split(": ");
                let game_nr: i32 = parts
                    .clone()
                    .nth(0)
                    .unwrap()
                    .split(" ")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .expect("Should be a nr");

                let mut is_possible: bool = true;
                let (mut red_max, mut green_max, mut blue_max) = (0, 0, 0);
                let hands = parts.nth(1).unwrap().split("; ");

                for hand in hands {
                    for color in hand.split(", ") {
                        let mut color_parts = color.split(" ");
                        let nr: i32 = color_parts.nth(0).unwrap().parse().unwrap();
                        let col = color_parts.next().unwrap();

                        match col {
                            "red" => red_max = red_max.max(nr),
                            "green" => green_max = green_max.max(nr),
                            "blue" => blue_max = blue_max.max(nr),
                            _ => (),
                        }

                        if (col == "red" && nr > 12)
                            || (col == "green" && nr > 13)
                            || col == "blue" && nr > 14
                        {
                            is_possible = false;
                        }
                    }
                }

                if is_possible {
                    possible += game_nr;
                }

                power_sum += red_max * green_max * blue_max;
            }
        }
    }

    println!("Part 1: {}", possible);
    println!("Part 2: {}", power_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
