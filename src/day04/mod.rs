use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::Path;

pub fn solve(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut draws = Vec::new();
    let mut boards = Vec::new();
    let mut adj = vec![Vec::new(); 100];

    let mut board = Vec::with_capacity(5);
    let mut first = true;
    for result in reader.lines() {
        let line = result.unwrap();
        if draws.len() == 0 {
            draws = line
                .trim_end()
                .split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            continue;
        }

        if first {
            first = false;
            continue;
        }

        if line.is_empty() {
            boards.push(board);
            board = Vec::new();
            continue;
        }

        let row: Vec<_> = line
            .trim_end()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        for i in 0..row.len() {
            let num = row[i];
            adj[num as usize].push((boards.len(), board.len(), i));
        }
        board.push(row);
    }
    boards.push(board);

    let mut row_counters = vec![[0; 5]; boards.len()];
    let mut column_counters = vec![[0; 5]; boards.len()];

    let mut i = 0;
    let winning_board: usize;

    'main_loop: loop {
        let draw = draws[i];

        for (board, row, column) in &adj[draw as usize] {
            row_counters[*board][*row] += 1;
            column_counters[*board][*column] += 1;

            if row_counters[*board][*row] == 5 {
                winning_board = *board;
                break 'main_loop;
            }

            if column_counters[*board][*column] == 5 {
                winning_board = *board;
                break 'main_loop;
            }
        }
        i += 1;
    }
    let flattened = flatten_board(&boards[winning_board]);
    let mut subset_draws: Vec<_> = draws[..=i].into_iter().map(|x| *x).collect();
    subset_draws.sort();

    let diff = difference(&flattened, &subset_draws);
    let sum = diff.iter().fold(0, |acc, val| acc + *val);
    let p1 = sum * draws[i];

    // p2
    let mut remaining_boards: HashSet<_> = (0..boards.len()).collect();
    i = 0;

    let mut row_counters_2 = vec![[0; 5]; boards.len()];
    let mut column_counters_2 = vec![[0; 5]; boards.len()];

    'main_loop: while remaining_boards.len() != 1 {
        let draw = draws[i];
        for (board, row, column) in &adj[draw as usize] {
            row_counters_2[*board][*row] += 1;
            column_counters_2[*board][*column] += 1;

            if row_counters_2[*board][*row] == 5 {
                remaining_boards.remove(board);
            }

            if column_counters_2[*board][*column] == 5 {
                remaining_boards.remove(board);
            }
        }
        i += 1;
    }
    let losing_board = remaining_boards.iter().next().unwrap();

    // figure out when board wins
    'main_loop: loop {
        let draw = draws[i];
        for (board, row, column) in &adj[draw as usize] {
            if *board != *losing_board {
                continue;
            }

            row_counters_2[*board][*row] += 1;
            column_counters_2[*board][*column] += 1;

            if row_counters_2[*board][*row] == 5 {
                break 'main_loop;
            }

            if column_counters_2[*board][*column] == 5 {
                break 'main_loop;
            }
        }
        i += 1;
    }

    let flattened = flatten_board(&boards[*losing_board]);
    let mut subset_draws: Vec<_> = draws[..=i].into_iter().map(|x| *x).collect();
    subset_draws.sort();

    let diff = difference(&flattened, &subset_draws);
    let sum = diff.iter().fold(0, |acc, val| acc + *val);
    let p2 = sum * draws[i];

    println!("{} {}", p1, p2);
}

pub fn flatten_board(board: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut flattened = Vec::with_capacity(25);
    for row in board {
        for num in row {
            flattened.push(*num);
        }
    }
    flattened.sort();
    flattened
}

pub fn difference(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
    let size = std::cmp::max(lhs.len(), rhs.len());
    let mut result = Vec::with_capacity(size);

    let mut ptr_lhs = 0;
    let mut ptr_rhs = 0;

    while ptr_lhs < lhs.len() && ptr_rhs < rhs.len() {
        if lhs[ptr_lhs] == rhs[ptr_rhs] {
            ptr_lhs += 1;
            ptr_rhs += 1;
        } else if lhs[ptr_lhs] < rhs[ptr_rhs] {
            result.push(lhs[ptr_lhs]);
            ptr_lhs += 1;
        } else
        /*if lhs[ptr_lhs] > rhs[ptr_rhs]*/
        {
            ptr_rhs += 1;
        }
    }

    for i in ptr_lhs..lhs.len() {
        result.push(lhs[i]);
    }
    result.shrink_to_fit();
    result
}
