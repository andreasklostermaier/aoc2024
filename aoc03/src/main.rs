// © 2024 Andreas Klostermaier
// MIT LICENSE

use std::fs;
use regex::Regex;

fn main() {
    part_one();
    part_two();
}


fn part_one() {

    let memory    = fs::read_to_string("./aoc03/data/input.txt").unwrap();
    let instr_pat = Regex::new(r"mul\((?<op_a>[0-9]+)\,(?<op_b>[0-9]+)\)").unwrap();

    let result: u32 = instr_pat.captures_iter(&memory).map(|caps| {
        let op_a = caps.name("op_a").unwrap().as_str().parse::<u32>().unwrap();
        let op_b = caps.name("op_b").unwrap().as_str().parse::<u32>().unwrap();
        op_a * op_b
    }).sum();

    println!("--- Part One ---");
    println!("The added up result of legal mul-instructions is: {:#?}", result);

}


fn part_two() {

    let memory        = fs::read_to_string("./aoc03/data/input.txt").unwrap();
    let do_pattern    = Regex::new(r"(^|do\(\)).*?don't\(\)").unwrap();
    let mul_pattern   = Regex::new(r"mul\((?<op_a>[0-9]+)\,(?<op_b>[0-9]+)\)").unwrap();
    let mut total_sum = 0u32;

    let do_chunks: Vec<&str> = do_pattern.find_iter(&memory).map(|m| m.as_str()).collect();
    for do_chunk in do_chunks {
        let chunk_sum: u32 = mul_pattern.captures_iter(do_chunk).map(|caps| {
            let op_a = caps.name("op_a").unwrap().as_str().parse::<u32>().unwrap();
            let op_b = caps.name("op_b").unwrap().as_str().parse::<u32>().unwrap();
            op_a * op_b
        }).sum();
        total_sum += chunk_sum;
    }

    println!("--- Part Two ---");
    println!("The added up result of all mul-instructions is: {:#?}", total_sum);

}

