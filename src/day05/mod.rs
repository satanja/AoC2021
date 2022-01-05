use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Gradient {
    HORIZONTAL,
    VERTICAL,
    DIAGONAL,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct Segment {
    left: Coordinate,
    right: Coordinate,
}

impl Segment {
    pub fn new(start: Coordinate, end: Coordinate) -> Segment {
        let (left, right) = if start.x <= end.x {
            (start, end)
        } else {
            (end, start)
        };
        Segment { left, right }
    }

    pub fn gradient(&self) -> Gradient {
        if self.left.y == self.right.y {
            return Gradient::HORIZONTAL;
        }
        if self.left.x == self.right.x {
            return Gradient::VERTICAL;
        }

        Gradient::DIAGONAL
    }

    pub fn occupies(&self) -> Vec<Coordinate> {
        let mut result = Vec::new();
        match self.gradient() {
            Gradient::HORIZONTAL => {
                let mut pos = self.left.clone();
                while pos.x != self.right.x {
                    result.push(pos.clone());
                    pos.x += 1;
                }
                result.push(pos.clone());
            }
            Gradient::VERTICAL => {
                let (mut pos, other) = if self.left.y <= self.right.y {
                    (self.left.clone(), self.right.clone())
                } else {
                    (self.right.clone(), self.left.clone())
                };
                while pos.y != other.y {
                    result.push(pos.clone());
                    pos.y += 1;
                }
                result.push(pos.clone());
            }
            Gradient::DIAGONAL => {
                let delta = if self.left.y < self.right.y {
                    1
                } else {
                    -1
                };
                let mut pos = self.left.clone();
                while pos != self.right {
                    result.push(pos.clone());
                    pos.x += 1;
                    pos.y += delta;
                }
                result.push(pos.clone());
            }
        }
        result
    }
}

pub fn solve(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut segments = Vec::new();
    for result in reader.lines() {
        let line = result.unwrap();
        let coords: Vec<_> = line.trim_end().split(" -> ").collect();
        let left: Vec<_> = coords[0]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let right: Vec<_> = coords[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        segments.push(Segment::new(
            Coordinate {
                x: left[0],
                y: left[1],
            },
            Coordinate {
                x: right[0],
                y: right[1],
            },
        ));
    }

    let mut grid_1 = HashMap::new();
    for segment in &segments {
        if segment.gradient() == Gradient::HORIZONTAL || segment.gradient() == Gradient::VERTICAL {
            let occupies = segment.occupies();
            for point in occupies {
                if grid_1.contains_key(&point) {
                    *grid_1.get_mut(&point).unwrap() += 1;
                } else {
                    grid_1.insert(point, 1);
                }
            }
        }
    }

    let p1 = grid_1.iter().fold(0, |acc, pair| {
        let increment = if *pair.1 >= 2 {
            1
        } else {
            0
        };
        acc + increment
    });

    let mut grid_2 = HashMap::new();
    for segment in &segments {
        let occupies = segment.occupies();
        for point in occupies {
            if grid_2.contains_key(&point) {
                *grid_2.get_mut(&point).unwrap() += 1;
            } else {
                grid_2.insert(point, 1);
            }
        }
    }

    let p2 = grid_2.iter().fold(0, |acc, pair| {
        let increment = if *pair.1 >= 2 {
            1
        } else {
            0
        };
        acc + increment
    });

    println!("{} {}", p1, p2);
}
