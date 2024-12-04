// © 2024 Andreas Klostermaier
// MIT LICENSE

use std::fs;
use regex::Regex;

fn main() {
    part_one();
    part_two();
}


fn part_one() {

    let mut xmas_occurences = 0u32;

    let input_raw  = fs::read_to_string("./aoc04/data/input.txt").unwrap();
    let nr_of_rows = input_raw.lines().count();
    let nr_of_cols = input_raw.lines().next().unwrap_or("").chars().count();

    let input = input_raw.replace("\n", "");

    let reversed_input: String = input_raw
        .lines()
        .map(|line| line.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(""); 

    // horizontal rows
    let mut list: Vec<String> = input_raw.lines().map(|s| s.to_string()).collect();

    // vertical columns
    for c in 1..=nr_of_cols {
        let mut col_string = "".to_string();
        for r in 0..nr_of_rows {
            col_string.push(input.chars().nth((r*nr_of_cols+c)-1).unwrap());
        }
        list.push(col_string.clone());
    }

    // diagonal columns
    let nr_of_diagonals = nr_of_rows + nr_of_cols - 1;
    let step_width      = nr_of_cols+1;
    let mut start_index = 0;

    let center_rows_start = nr_of_cols+1;
    let center_rows_end   = nr_of_diagonals - nr_of_cols;

    for d in 0..nr_of_diagonals {
        let mut col_string = "".to_string();
        let mut col_string_reversed = "".to_string();
        let row_length;
        if d < center_rows_start-1 {
            row_length = nr_of_cols - d;
        } else if d > center_rows_end {
            row_length = nr_of_diagonals-d;
        } else {
            row_length = nr_of_cols;
        }
        let mut char_index = start_index;
        for _ in 1..=row_length {
            col_string.push(input.chars().nth(char_index).unwrap());
            col_string_reversed.push(reversed_input.chars().nth(char_index).unwrap());
            char_index += step_width;
        }
        list.push(col_string.clone());
        list.push(col_string_reversed.clone());
        if d < center_rows_end+1 { start_index += 1 } else { start_index += step_width-1 }
    }

    let xmas_pattern = Regex::new(r"(XMAS|SAMX)").unwrap();
    for e in list {
        xmas_occurences += overlap_match_count(&xmas_pattern, &e);
    }

    println!("--- Part One ---");
    println!("XMAS appears {:#?} times", xmas_occurences);

}


fn part_two() {

    fn coord2idx(x: usize, y:usize, w:usize) -> usize {
        y * w + x
    }

    let mut xmas_occurences = 0u32;

    let input_raw  = fs::read_to_string("./aoc04/data/input.txt").unwrap();
    let nr_of_rows = input_raw.lines().count();
    let nr_of_cols = input_raw.lines().next().unwrap_or("").chars().count();
    let input = input_raw.replace("\n", "");
    let pattern = "MMASS MSAMS SMASM SSAMM";
    println!("rows: {nr_of_rows}, cols: {nr_of_cols}");

    for x in 0..nr_of_cols-2 {
        for y in 0..nr_of_rows-2 {
            let mut test = "".to_string();
            test.push(input.chars().nth(coord2idx(x,y,nr_of_cols)).unwrap());
            test.push(input.chars().nth(coord2idx(x+2,y,nr_of_cols)).unwrap());
            test.push(input.chars().nth(coord2idx(x+1,y+1,nr_of_cols)).unwrap());
            test.push(input.chars().nth(coord2idx(x,y+2,nr_of_cols)).unwrap());
            test.push(input.chars().nth(coord2idx(x+2,y+2,nr_of_cols)).unwrap());
            if pattern.contains(&test) {xmas_occurences += 1}
        }
    }

    println!("--- Part Two ---");
    println!("X-MAS appears {:#?} times", xmas_occurences);

}


/// Rust regex cannot do overlapping search, so we have to repeatedly
/// search substrings by shifting the starting position.
/// This program is not going to be fast :-(
fn overlap_match_count(re: &Regex, txt:&str) -> u32 {
    let mut count = 0;
    let mut posit = 0;
    while let Some(hit) = re.find_at(txt, posit) {
        count += 1;
        posit = hit.start() + 1;
    }
    count
}


