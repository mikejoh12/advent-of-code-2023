use std::{
    collections::{HashSet, VecDeque},
    fs,
};

struct Contraption {
    positions: Vec<Vec<Point>>,
    size: i32,
    start_beam: Beam,
}

impl Contraption {
    fn energize(&mut self) {
        let mut visited: HashSet<Beam> = HashSet::new();
        let mut queue: VecDeque<Beam> = VecDeque::new();
        queue.push_back(self.start_beam.clone());

        while !queue.is_empty() {
            let beam = queue.pop_front().expect("Queue should not be empty");

            if visited.contains(&beam)
                || beam.x < 0
                || beam.x >= self.size
                || beam.y < 0
                || beam.y >= self.size
            {
                continue;
            }

            visited.insert(beam.clone());
            self.positions[beam.y as usize][beam.x as usize].is_energized = true;

            match self.positions[beam.y as usize][beam.x as usize].point_content {
                PointContent::EmptySpace => {
                    let new_beam = match beam.direction {
                        Direction::Up => Beam {
                            x: beam.x,
                            y: beam.y - 1,
                            direction: beam.direction,
                        },
                        Direction::Right => Beam {
                            x: beam.x + 1,
                            y: beam.y,
                            direction: beam.direction,
                        },
                        Direction::Down => Beam {
                            x: beam.x,
                            y: beam.y + 1,
                            direction: beam.direction,
                        },
                        Direction::Left => Beam {
                            x: beam.x - 1,
                            y: beam.y,
                            direction: beam.direction,
                        },
                    };
                    queue.push_back(new_beam);
                }
                PointContent::VertSplitter => {
                    if beam.direction == Direction::Right || beam.direction == Direction::Left {
                        queue.push_back(Beam {
                            x: beam.x,
                            y: beam.y - 1,
                            direction: Direction::Up,
                        });
                        queue.push_back(Beam {
                            x: beam.x,
                            y: beam.y + 1,
                            direction: Direction::Down,
                        });
                    } else if beam.direction == Direction::Up {
                        queue.push_back(Beam {
                            x: beam.x,
                            y: beam.y - 1,
                            direction: beam.direction,
                        });
                    } else if beam.direction == Direction::Down {
                        queue.push_back(Beam {
                            x: beam.x,
                            y: beam.y + 1,
                            direction: beam.direction,
                        });
                    }
                }
                PointContent::HorSplitter => {
                    if beam.direction == Direction::Up || beam.direction == Direction::Down {
                        queue.push_back(Beam {
                            x: beam.x - 1,
                            y: beam.y,
                            direction: Direction::Left,
                        });
                        queue.push_back(Beam {
                            x: beam.x + 1,
                            y: beam.y,
                            direction: Direction::Right,
                        });
                    } else if beam.direction == Direction::Left {
                        queue.push_back(Beam {
                            x: beam.x - 1,
                            y: beam.y,
                            direction: beam.direction,
                        });
                    } else if beam.direction == Direction::Right {
                        queue.push_back(Beam {
                            x: beam.x + 1,
                            y: beam.y,
                            direction: beam.direction,
                        });
                    }
                }
                PointContent::LeftLeaningMirror => {
                    let new_beam = match beam.direction {
                        Direction::Up => Beam {
                            x: beam.x - 1,
                            y: beam.y,
                            direction: Direction::Left,
                        },
                        Direction::Right => Beam {
                            x: beam.x,
                            y: beam.y + 1,
                            direction: Direction::Down,
                        },
                        Direction::Left => Beam {
                            x: beam.x,
                            y: beam.y - 1,
                            direction: Direction::Up,
                        },
                        Direction::Down => Beam {
                            x: beam.x + 1,
                            y: beam.y,
                            direction: Direction::Right,
                        },
                    };
                    queue.push_back(new_beam);
                }
                PointContent::RightLeaningMirror => {
                    let new_beam = match beam.direction {
                        Direction::Up => Beam {
                            x: beam.x + 1,
                            y: beam.y,
                            direction: Direction::Right,
                        },
                        Direction::Right => Beam {
                            x: beam.x,
                            y: beam.y - 1,
                            direction: Direction::Up,
                        },
                        Direction::Left => Beam {
                            x: beam.x,
                            y: beam.y + 1,
                            direction: Direction::Down,
                        },
                        Direction::Down => Beam {
                            x: beam.x - 1,
                            y: beam.y,
                            direction: Direction::Left,
                        },
                    };
                    queue.push_back(new_beam);
                }
            }
        }
    }

    fn count_energized(&self) -> i32 {
        let mut count = 0;
        for row in &self.positions {
            for pos in row {
                if pos.is_energized {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone)]
struct Point {
    point_content: PointContent,
    is_energized: bool,
}

#[derive(Debug, Clone)]
enum PointContent {
    EmptySpace,
    VertSplitter,       // Splits horizontal beam to up/down beams -> |
    HorSplitter,        // Splits vertical beam to left/rigth beams -> -
    RightLeaningMirror, // Reflects between left/up and right/down -> /
    LeftLeaningMirror,  // Reflects between up/right and left/down -> \
}

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Beam {
    x: i32,
    y: i32,
    direction: Direction,
}

fn main() {
    let mut positions: Vec<Vec<Point>> = vec![];

    for row in fs::read_to_string("input.txt")
        .expect("File should exist")
        .lines()
    {
        let mut points_row: Vec<Point> = vec![];
        for c in row.chars() {
            let point_content = match c {
                '|' => PointContent::VertSplitter,
                '-' => PointContent::HorSplitter,
                '/' => PointContent::RightLeaningMirror,
                '\\' => PointContent::LeftLeaningMirror,
                _ => PointContent::EmptySpace,
            };
            points_row.push(Point {
                point_content,
                is_energized: false,
            });
        }
        positions.push(points_row);
    }

    let size = positions.len() as i32;
    let mut contraption = Contraption {
        positions: positions.clone(),
        size,
        start_beam: Beam {
            x: 0,
            y: 0,
            direction: Direction::Right,
        },
    };

    contraption.energize();
    let count = contraption.count_energized();

    println!("Part 1: {}", count);

    let mut max_energized = 0;

    for i in 0..100 {
        let mut left_edge = Contraption {
            positions: positions.clone(),
            size,
            start_beam: Beam {
                x: 0,
                y: i,
                direction: Direction::Right,
            },
        };

        let mut right_edge = Contraption {
            positions: positions.clone(),
            size,
            start_beam: Beam {
                x: size - 1,
                y: i,
                direction: Direction::Left,
            },
        };

        let mut top_edge = Contraption {
            positions: positions.clone(),
            size,
            start_beam: Beam {
                x: i,
                y: 0,
                direction: Direction::Down,
            },
        };

        let mut bottom_edge = Contraption {
            positions: positions.clone(),
            size,
            start_beam: Beam {
                x: i,
                y: size - 1,
                direction: Direction::Up,
            },
        };

        left_edge.energize();
        let left_edge_count = left_edge.count_energized();
        right_edge.energize();
        let right_edge_count = right_edge.count_energized();
        top_edge.energize();
        let top_edge_count = top_edge.count_energized();
        bottom_edge.energize();
        let bottom_edge_count = bottom_edge.count_energized();

        max_energized = *[
            max_energized,
            left_edge_count,
            right_edge_count,
            top_edge_count,
            bottom_edge_count,
        ]
        .iter()
        .max()
        .expect("Should have values");
    }

    println!("Part 2: {}", max_energized);
}
