use std::fs;
use std::collections::HashMap;

fn main() {
    //PART 1 Solution
    let file_path = "input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //println!("With text:\n{contents}")

    //Split each list of numbers in to separate vectors first
    let lines = contents.lines();
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();
    for line in lines {
        let parts = line.split("   ");
        let collection = parts.collect::<Vec<&str>>();
        left_vec.push(collection[0].parse::<i32>().unwrap());
        right_vec.push(collection[1].parse::<i32>().unwrap());
    }

    //Sort vectors
    left_vec.sort();
    right_vec.sort();

    let size = right_vec.len();
    let mut total_distance = 0;
    for index in 0..size {
        let distance = (left_vec[index] - right_vec[index]).abs();
        total_distance = total_distance + distance;
    }

    println!("Total Distance between lists is: {}", total_distance);

    //PART 2 Solution
    //left_vec and right_vec are still used from part 1
    //Create map of value of int in right vec to frequency
    let mut num_to_freq:HashMap<i32, i32> = HashMap::new();
    for value in right_vec {
        *num_to_freq.entry(value).or_default() += 1;
    }

    let mut total_similarity = 0;
    for value in left_vec {
        let similarity = value * *num_to_freq.entry(value).or_default();
        total_similarity = total_similarity + similarity;
    }

    println!("Total similarity between lists is: {}", total_similarity);
}
