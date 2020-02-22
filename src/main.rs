extern crate bankers;
use bankers::{bankers_algorithm, Process};

use std::fs;

fn main() {
    // Parse text files into bankers algorithms required structure
    // Could run the parse of the file and bankers algorithm in threads, return bankers result from thread,
    // Then print the result

    execute("input1.txt");
    execute("input2.txt");
    execute("input3.txt");
}
fn execute(filename: &'static str) {
    let dir = "src/BankerData/";
    let full_filename = format!("{}{}", dir, filename);
    let input_file_str = fs::read_to_string(full_filename).expect("Error happened.");

    let mut vec: Vec<Vec<u32>> = input_file_str
        .lines()
        .map(|line| {
            line.split(char::is_whitespace)
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let available: Vec<u32> = vec[0].to_vec();
    // remove first item (available line)
    vec.drain(0..1);

    let half_count = vec.len() / 2;
    let mut i = 0;
    let mut alloc: Vec<Vec<u32>> = Vec::new();
    let mut max: Vec<Vec<u32>> = Vec::new();
    let mut first_half = true;
    while i < vec.len() {
        if first_half {
            alloc.push(vec[i].to_vec());
        } else {
            max.push(vec[i].to_vec());
        }
        if i == half_count - 1 && first_half {
            first_half = false;
        }
        i += 1;
    }
    // a couple tests
    assert_eq!(alloc.len(), max.len());
    // check length against input file length
    assert_eq!(vec.len() / 2, max.len());

    let mut processes: Vec<Process> = Vec::new();
    i = 0;
    while i < half_count {
        processes.push(
            Process::new(i as u32, alloc[i].to_vec(), max[i].to_vec())
                .expect("Failed to create Process"),
        );
        i += 1;
    }
    // EXECUTE ALG
    let result = bankers_algorithm(available, processes);
    // Test result

    if result.is_ok() {
        let unwrapped = result.unwrap();
        println!("\nfile :: {}", filename);
        for process in unwrapped {
            println!("{}", process);
        }
        println!("\n");
    } else {
        println!("\n\nfile :: {} :: No safe state exists\n\n", filename);
    }
}
