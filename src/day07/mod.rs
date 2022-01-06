use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solve(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut crab_positions = HashMap::new();
    let mut position_set = HashSet::new();

    for result in reader.lines() {
        let line = result.unwrap();
        let positions: Vec<_> = line
            .trim_end()
            .split(',')
            .map(|pos| pos.parse::<usize>().unwrap())
            .collect();

        for position in positions {
            if let Some(data) = crab_positions.get_mut(&position) {
                *data += 1
            } else {
                crab_positions.insert(position, 1);
            }
            position_set.insert(position);
        }
    }

    let mut queue: Vec<_> = position_set.clone().into_iter().collect();
    queue.sort();

    let min = *queue.first().unwrap();
    let max = *queue.last().unwrap();

    let mut best_cost_1 = usize::MAX;
    for i in min..=max {
        let mut left_cost = 0;

        for k in 0..queue.len() {
            let pos = queue[k];
            if pos >= i {
                break;
            }
            left_cost += (i - pos) * *crab_positions.get(&pos).unwrap();
        }

        let mut right_cost = 0;
        for k in 0..queue.len() {
            let pos = queue[k];
            if pos <= i {
                continue;
            }
            right_cost += *crab_positions.get(&pos).unwrap() * (pos - i);
        }

        if best_cost_1 > left_cost + right_cost {
            best_cost_1 = left_cost + right_cost;
        }
    }

    let mut best_cost_2 = usize::MAX;
    for i in min..=max {
        let mut left_cost = 0;

        for k in 0..queue.len() {
            let pos = queue[k];
            if pos >= i {
                break;
            }
            left_cost += cost_squared(i - pos) * *crab_positions.get(&pos).unwrap();
        }

        let mut right_cost = 0;
        for k in 0..queue.len() {
            let pos = queue[k];
            if pos <= i {
                continue;
            }
            right_cost += *crab_positions.get(&pos).unwrap() * cost_squared(pos - i);
        }

        if best_cost_2 > left_cost + right_cost {
            best_cost_2 = left_cost + right_cost;
        }
    }

    println!("{} {}", best_cost_1, best_cost_2);
}

fn cost_squared(dist: usize) -> usize {
    dist * (dist + 1) / 2
}
