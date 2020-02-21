extern crate bankers;
use bankers::bankers_algorithm;

use std::env;
use std::fs;
mod lib;

fn main() {
    // Parse text files into bankers algorithms required structure
    // Could run the parse of the file and bankers algorithm in threads, return bankers result from thread,
    // Then print the result

    //munger("input1.txt");
    let mut flat_vec: Vec<Vec<u32>> = mapper("input1.txt");

    // ** OUTPUT ** //
    // Process is one line in the input file
    // vector max = [1,1,1,1]
    // vector avail = [1,1,1,1]

    let mut available: Vec<u32> = flat_vec[0].to_vec();
    // remove first item
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
    // check length against input file length
    assert_eq!(alloc.len(), max.len());
    assert_eq!(flat_vec.len() / 2, max.len());

    // processes = Vec<Process>
    // available = Vec<u32>
    let mut processes: Vec<Process> = Vec::new();
    i = 0;
    while i < half_count {
        processes.push(Process::new(alloc[i], max[i]));
        i += 1;
    }
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
fn munger(filename: &str) {
    let filename_dir = "src/BankerData/";
    let filename_type = ".txt";
    let filename = format!("{}{}{}", filename_dir, filename, filename_type);
    let input_file_str = fs::read_to_string(filename).expect("Error happened.");
    // ** Input ** //
    // line 1: available
    // 1st half: alloc
    // 2nd half: max

    // Break down into each line of the input file.
    let mut input_file_vec: Vec<&str> = (input_file_str.split("\r\n")).collect();

    // -> Available vector extraction
    let mut available: Vec<u32> =
        convert_str_vec_to_int_vec((input_file_vec[0].split(" ")).collect());
    // Remove first line
    input_file_vec.drain(0..1);
    // <- ENDS: Available vector extraction

    let mut alloc: Vec<Vec<u32>> = Vec::new();
    let mut max: Vec<Vec<u32>> = Vec::new();

    let half_count = input_file_vec.len() / 2;
    let mut first_half = true;
    let mut i = 0;
    let mut this_line: Vec<u32> = Vec::new();
    while i < input_file_vec.len() {
        this_line = convert_str_vec_to_int_vec((input_file_vec[i].split(" ")).collect());
        if first_half {
            alloc.push(this_line);
        } else {
            max.push(this_line);
        }
        if i == half_count - 1 && first_half {
            first_half = false;
        }
        i += 1;
    }

    // a couple tests
    // check length against input file length
    assert_eq!(alloc.len(), max.len());
    assert_eq!(input_file_vec.len() / 2, max.len());

    // ** Output ** //
    // 3 vectors: max, alloc, available
    // need is derived from max and alloc
}
// Converts str-vec to int-vec
fn convert_str_vec_to_int_vec(str_vec: Vec<&str>) -> Vec<u32> {
    let mut output_vec: Vec<u32> = Vec::new();

    // Converts available str-vec to int-vec
    for str_num in str_vec {
        let pure_num: u32 = str_num.parse().expect("Conversion failed.");
        output_vec.push(pure_num);
    }
    return output_vec;
}
