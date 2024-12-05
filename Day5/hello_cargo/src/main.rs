use std::fs;
use std::collections::HashMap;

fn main() {
    //PART 1 with some of part 2 Solutions
    let file_path = "input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total = 0;
    let mut incorrect_lines = Vec::new();
    let mut num_to_nums_after:HashMap<i32, Vec<i32>> = HashMap::new();
    '_lines:for line in contents.lines() {
        //Create map of num to numbers that should be after it
        if line.contains('|') {
            let nums = line.split('|');
            let nums_collection = nums.collect::<Vec<&str>>();
            let num1 = nums_collection[0].parse::<i32>().unwrap();
            let num2 = nums_collection[1].parse::<i32>().unwrap();
            num_to_nums_after.entry(num1).or_default().push(num2);
        }

        let mut bad_line = false;
        if line.contains(',') {
            let nums = line.split(',');
            let mut vec_of_nums = Vec::new();
            for num in nums {
                let num_as_int = num.parse::<i32>().unwrap();
                if num_to_nums_after.contains_key(&num_as_int) {
                    for value in num_to_nums_after.get_mut(&num_as_int).unwrap() {
                        if vec_of_nums.contains(value) {
                            //Bad line
                            bad_line = true;
                        }
                    }
                }
                vec_of_nums.push(num_as_int);
            }

            if !bad_line {
                let middle_value = vec_of_nums[(vec_of_nums.len()+1)/2 - 1]; //Assume all lines odd lengths
                total = total + middle_value;
            }
            else {
                incorrect_lines.push(vec_of_nums.clone());
            }
        }
    }

    println!("Total Part1: {}", total);

    //All Part 2 from now on
    let mut total_part2 = 0;

    for vector in incorrect_lines {
        let mut line_solved = false;
        let mut vector_dup = vector.clone();
        while !line_solved {
            let mut vec_of_nums_so_far = Vec::new();
            let mut new_vector = Vec::new();
            let mut bad_line = false;
            for index in 0..vector_dup.len() {
                let num = vector_dup[index];
                if !bad_line {
                    if num_to_nums_after.contains_key(&num) {
                        for value in num_to_nums_after.get_mut(&num).unwrap() {
                            if vec_of_nums_so_far.contains(value) {
                                //Bad line

                                //Move current value to start of the line and start again when we find a bad value
                                //Not the most efficient sorting algorithm, but it works :)
                                new_vector.push(num);
                                new_vector.append(&mut vec_of_nums_so_far);
                                bad_line = true;
                            }
                        }
                    }
                    vec_of_nums_so_far.push(num);
                } else {
                    new_vector.push(num);
                }
            }

            if !bad_line {
                //println!("Final vector {vector_dup:?}");
                //println!("Vector len {}", vector_dup.len());
                let middle_value = vector_dup[(vector_dup.len()+1)/2 - 1]; //Assume all lines odd lengths
                //println!("Mid val {middle_value}");
                total_part2 = total_part2 + middle_value;

                line_solved = true;
            }
            else {
                vector_dup = new_vector;
                //println!("new vector {vector_dup:?}")
            }
        }
    }

    println!("Total Part2: {}", total_part2);
}
