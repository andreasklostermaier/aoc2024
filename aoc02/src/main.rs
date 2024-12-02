use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    part_one();
    part_two();

}


fn part_one() {

    let mut safe_reports_count = 0i32;
    let mut direction          = 0i32;
    let mut prev_level         = 0i32;

    if let Ok(lines) = read_lines("./aoc02/data/input.txt") {
        for report in lines.map_while(Result::ok) {
            let levels: Vec<i32> = 
                report.split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();

            let safe = 'outer: {
                for (count, level) in levels.into_iter().enumerate() {
                    match count {
                        0 => { },
                        1 => {
                            if !(1..=3).contains(&prev_level.abs_diff(level)) {
                                break 'outer false;
                            } else {
                                direction = (prev_level - level).signum();
                            }
                        },
                        _ => {
                            if !(1..=3).contains(&prev_level.abs_diff(level)) || 
                                direction != (prev_level - level).signum() {
                                break 'outer false;
                            }
                        },
                    }
                    prev_level = level;
                }
                break 'outer true
            };
            if safe { safe_reports_count += 1 }
        };
    }

    println!("--- Part One ---");
    println!("The number of safe reports is: {:?}", safe_reports_count);

}


fn part_two() {

    let mut safe_reports_count = 0i32;

    if let Ok(lines) = read_lines("./aoc02/data/input.txt") {
        for report in lines.map_while(Result::ok) {
            let levels: Vec<i32> = 
                report.split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();

            let mut safe_counter       = 0i32;

            for v in 0..=levels.len() {
                let mut variant = levels.clone();
                if v < levels.len() { variant.remove(v); }

                let mut direction          = 0i32;
                let mut prev_level         = 0i32;

                let safe = 'outer: {

                    for (count, level) in variant.into_iter().enumerate() {
                        match count {
                            0 => { },
                            1 => {
                                if !(1..=3).contains(&prev_level.abs_diff(level)) {
                                    break 'outer false;
                                } else {
                                    direction = (prev_level - level).signum();
                                }
                            },
                            _ => {
                                if !(1..=3).contains(&prev_level.abs_diff(level)) || 
                                    direction != (prev_level - level).signum() {
                                    break 'outer false;
                                }
                            },
                        }
                        prev_level = level;
                    }
                    break 'outer true;
                };
                if safe { safe_counter += 1 }
            }
            if safe_counter > 0 { safe_reports_count += 1 }
        }
    }

    println!("--- Part Two ---");
    println!("The number of safe reports is now: {:?}", safe_reports_count);

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}