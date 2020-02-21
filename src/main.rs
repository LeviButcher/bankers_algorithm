extern crate bankers;
use bankers::bankers_algorithm;

use std::env;
use std::fs;
mod lib;

fn main() {
    // Parse text files into bankers algorithms required structure
    // Could run the parse of the file and bankers algorithm in threads, return bankers result from thread,
    // Then print the result

    let mut flat_vec: Vec<Vec<u32>> = mapper("input1.txt");
    let mut available: Vec<u32> = flat_vec[0].to_vec();
    // remove first item (available line)
    flat_vec.drain(0..1);

    let half_count = flat_vec.len() / 2;
    let mut i = 0;
    let mut alloc: Vec<Vec<u32>> = Vec::new();
    let mut max: Vec<Vec<u32>> = Vec::new();
    let mut first_half = true;
    while i < flat_vec.len() {
        if first_half {
            alloc.push(flat_vec[i].to_vec());
        } else {
            max.push(flat_vec[i].to_vec());
        }
        if i == half_count - 1 && first_half {
            first_half = false;
        }
        i += 1;
    }
    // a couple tests
    assert_eq!(alloc.len(), max.len());
    // check length against input file length
    assert_eq!(flat_vec.len() / 2, max.len());

    let mut processes: Vec<Process> = Vec::new();
    i = 0;
    while i < half_count {
        processes.push(Process::new(alloc[i], max[i]));
        i += 1;
    }
    // EXECUTE ALG
    bankers_algorithm(available, processes);
}
fn mapper(filename: &str) -> Vec<Vec<u32>> {
    let dir = "src/BankerData/";
    let filename = format!("{}{}", dir, filename);
    let input_file_str = fs::read_to_string(filename).expect("Error happened.");

    let all_vec: Vec<Vec<u32>> = input_file_str
        .lines()
        .map(|line| {
            line.split(char::is_whitespace)
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    return all_vec;
}
