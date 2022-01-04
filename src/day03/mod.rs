use core::num;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve(path: &Path) {

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    const BITS: usize = 12;
    
    // counts the number of 0s
    let mut counts = [0; BITS];
    let mut entries = 0;

    
    let mut numbers = Vec::new(); 
    
    for result in reader.lines() {
        let line = result.unwrap();
        numbers.push(usize::from_str_radix(&line.trim_end(), 2).unwrap());

        let mut index = 0;
        for char in line.chars() {
            match char {
                '0' => counts[index] += 1,
                _ => {}
            }
            index += 1;
        }
        entries += 1;
    }
    
    let mut gamma = 0;
    let mut eps = 0;

    for i in 0..BITS {
        
        // most common bit is 1
        gamma <<= 1;
        if counts[i] < entries - counts[i] {
            gamma += 1;
        }

        // most common bit is 0
        eps <<= 1;
        if counts[i] > entries - counts[i] {
            eps += 1;
        }
    }

    // part 2
    let mut gammas = numbers.clone();
    let mut epsilons = numbers; 

    let mut bit = BITS;
    while gammas.len() != 1 {
        bit -= 1;

        let mut ones = Vec::new();
        let mut zeroes = Vec::new();

        for number in &gammas {
            if *number & (1 << bit) != 0 {
                ones.push(*number);
            } else {
                zeroes.push(*number);
            }
        }

        if ones.len() >= zeroes.len() {
            gammas = ones;
        } else {
            gammas = zeroes;
        }
    }

    bit = BITS;
    while epsilons.len() != 1 {
        bit -= 1;

        let mut ones = Vec::new();
        let mut zeroes = Vec::new();

        for number in &epsilons {
            if *number & (1 << bit) != 0 {
                ones.push(*number);
            } else {
                zeroes.push(*number);
            }
        }

        if zeroes.len() <= ones.len() {
            epsilons = zeroes;
        } else {
            epsilons = ones;
        }
    }
    let gamma2 = gammas[0];
    let eps2 = epsilons[0];

    println!("{} {}", gamma * eps, gamma2 * eps2);

}