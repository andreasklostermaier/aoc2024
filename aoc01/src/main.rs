use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {

    part_one();
    part_two();

}


fn part_one() {

    let mut total_distance = 0i32;
    let mut list_l: Vec<i32> = vec![];
    let mut list_r: Vec<i32> = vec![];

    // first we read the file line by line and build the left and right list
    // as individual Vectors
    if let Ok(lines) = read_lines("./aoc01/data/input.txt") {
        for line in lines.map_while(Result::ok) {
            let entry: Vec<&str> = line.split_whitespace().collect();
            list_l.push(entry[0].parse::<i32>().unwrap());
            list_r.push(entry[1].parse::<i32>().unwrap());
        }
    }

    // next we order the left and right list numerically
    list_l.sort();
    list_r.sort();

    // and finally we loop the left list and calculate the distance
    for (i, _) in list_l.iter().enumerate() {
        total_distance += (list_l[i] - list_r[i]).abs();
    }

    println!("--- Part One ---");
    println!("The total distance between the left list and the right list is: {:?}", total_distance);

}


fn part_two() {

    let mut similarity_score = 0i32;
    let mut index: i32;

    let mut list_l: Vec<i32> = vec![];
    let mut list_r: HashMap<i32, i32> = HashMap::new();

    // first we read the file line by line.
    // the left list goes in a vector like in part one,
    // the right list counts its entries in a HashMap
    if let Ok(lines) = read_lines("./aoc01/data/input.txt") {
        for line in lines.map_while(Result::ok) {
            let entry: Vec<&str> = line.split_whitespace().collect();
            list_l.push(entry[0].parse::<i32>().unwrap());
            index = entry[1].parse::<i32>().unwrap();
            if list_r.contains_key(&index) {
                let mut new_count = *list_r.get(&index).unwrap();
                new_count += 1;
                list_r.insert(index, new_count);
            } else {
                list_r.insert(index, 1);
            }
        }
    }

    for item in list_l.iter() {
        if list_r.contains_key(item) {
            similarity_score += item * list_r.get(item).unwrap();
        }
    }

    println!("--- Part Two ---");
    println!("The similarity score is: {:?}", similarity_score);

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}