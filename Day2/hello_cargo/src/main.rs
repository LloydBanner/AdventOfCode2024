use std::fs;

fn main() {
    //PART 1 Solution
    let file_path = "input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.lines();
    let mut safe_lines = 0;
    '_lines:for line in lines {
        safe_lines += 1; //Assume line is safe until proven otherwise
        let parts = line.split(" ");
        let collection = parts.collect::<Vec<&str>>();

        let mut last_value = 0;
        let mut incrementing = true;
        let mut direction_known = false;
        '_values:for value in collection {
            if last_value != 0 {
                let mut good_value = true;
                let difference = value.parse::<i32>().unwrap() - last_value;
                if difference > 0 { //Incrementing
                    if !direction_known {
                        incrementing = true;
                        direction_known = true;
                    } else {
                        if incrementing { //Good

                        } else { //Bad
                            good_value = false;
                        }
                    }
                } else if difference < 0 { //Decrementing
                    if !direction_known {
                        incrementing = false;
                        direction_known = true;
                    } else {
                        if !incrementing { //Good

                        } else { //Bad
                            good_value = false;
                        }
                    }
                } else { //Value is not increasing or decreasing so must be bad
                    good_value = false;
                }

                let abs_difference = difference.abs();

                if abs_difference > 3 {
                    good_value = false;
                }

                if !good_value { //If bad remove a safe line and move on to next line
                    safe_lines = safe_lines - 1;
                    break '_values;
                }
            }
            last_value = value.parse::<i32>().unwrap();
        }
    }

    println!("Safe lines = {safe_lines}");






    //PART 2 Solution
    let file_path = "input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.lines();
    let mut safe_lines = 0;
    '_lines:for line in lines {
        safe_lines += 1; //Assume line is safe until proven otherwise
        let parts = line.split(" ");
        let collection = parts.collect::<Vec<&str>>();
        let dup_collection = collection.iter().cloned().collect::<Vec<&str>>();
        let configurations = collection.len();

        '_configurations:for conf in 0..(configurations+1) {
            //On conf 0 try all, on later confs try excluding each num
            let dup_collection2 = dup_collection.iter().cloned().collect::<Vec<&str>>();
            let mut last_value = 0;
            let mut incrementing = true;
            let mut direction_known = false;
            let exclude_index = conf;
            let mut index = 0;
            //println!("conf: {conf} exclude_index: {exclude_index}");
            '_values: for value in dup_collection2 {
                //println!("index: {index} exclude_index: {exclude_index}");
                if exclude_index != 0 {
                    if index == exclude_index-1 {
                        index += 1;
                        continue '_values;
                    }
                }

                if last_value != 0 {
                    let mut good_value = true;
                    let difference = value.parse::<i32>().unwrap() - last_value;
                    if difference > 0 { //Incrementing
                        if !direction_known {
                            incrementing = true;
                            direction_known = true;
                        } else {
                            if incrementing { //Good

                            } else { //Bad
                                good_value = false;
                            }
                        }
                    } else if difference < 0 { //Decrementing
                        if !direction_known {
                            incrementing = false;
                            direction_known = true;
                        } else {
                            if !incrementing { //Good

                            } else { //Bad
                                good_value = false;
                            }
                        }
                    } else { //Value is not increasing or decreasing so must be bad
                        good_value = false;
                    }

                    let abs_difference = difference.abs();

                    if abs_difference > 3 {
                        good_value = false;
                    }

                    if !good_value { //Only bad if all configurations are bad
                        if conf == configurations {
                            safe_lines = safe_lines - 1;
                        }
                        continue '_configurations;
                    }
                    else if index == configurations-1 {
                        continue '_lines;
                    }
                }
                index += 1;
                last_value = value.parse::<i32>().unwrap();
            }
        }
    }

    println!("Safe lines = {safe_lines}");
}
