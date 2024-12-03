use std::fs;

fn main() {
    //PART 1 Solution
    let file_path = "input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let string_check = "mul(";
    let indices:Vec<(usize, &str)> = contents.match_indices(string_check).collect();


    //println!("Locations {indices:?}");

    let mut total = 0;
    for location in indices {
        let index = location.0 + string_check.len();
        let mut num1_length = 1;
        if !contents.chars().nth(index).unwrap().is_numeric() {
           continue;
        }

        //Work out length of first num or continue if badly formed
        if contents.chars().nth(index+1).unwrap().is_numeric() {
            if contents.chars().nth(index+2).unwrap().is_numeric() {
                if contents.chars().nth(index+3).unwrap() == ',' {
                    num1_length = 3;
                }
                else {
                    continue;
                }
            }
            else {
                if contents.chars().nth(index+2).unwrap() == ',' {
                    num1_length = 2;
                }
                else {
                    continue;
                }
            }
        }
        else if contents.chars().nth(index+1).unwrap() != ',' {
            continue;
        }

        let num2_start = index + num1_length + 1;
        let mut num2_length = 1;
        if !contents.chars().nth(num2_start).unwrap().is_numeric() {
            continue;
        }

        //Work out length of second num or continue if badly formed
        if contents.chars().nth(num2_start+1).unwrap().is_numeric() {
            if contents.chars().nth(num2_start+2).unwrap().is_numeric() {
                if contents.chars().nth(num2_start+3).unwrap() == ')' {
                    num2_length = 3;
                }
                else {
                    continue;
                }
            }
            else {
                if contents.chars().nth(num2_start+2).unwrap() == ')' {
                    num2_length = 2;
                }
                else {
                    continue;
                }
            }
        }
        else if contents.chars().nth(num2_start+1).unwrap() != ')' {
            continue;
        }

        let num1 = &contents[index..index+num1_length];
        let num2 = &contents[num2_start..num2_start+num2_length];

        let multiplication = num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
        total += multiplication;
    }

    println!("Total:  {total}");







    //PART 2 Solution
    let file_path = "input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let string_check = "mul(";
    let indices:Vec<(usize, &str)> = contents.match_indices(string_check).collect();
    let indices_do:Vec<(usize, &str)> = contents.match_indices("do()").collect();
    let indices_dont:Vec<(usize, &str)> = contents.match_indices("don't()").collect();

    let mut total = 0;
    let mut should_do = true;
    for index in 0..contents.len() {
        if indices_do.contains(&(index, &"do()")) {
            should_do = true;
            continue;
        }
        else if indices_dont.contains(&(index, &"don't()")) {
            should_do = false;
            continue;
        }
        else if indices.contains(&(index, &string_check)) {
            if !should_do {
                continue;
            }
        }
        else {
            continue;
        }

        let num1_start = index + string_check.len();
        let mut num1_length = 1;
        if !contents.chars().nth(num1_start).unwrap().is_numeric() {
            continue;
        }

        //Work out length of first num or continue if badly formed
        if contents.chars().nth(num1_start+1).unwrap().is_numeric() {
            if contents.chars().nth(num1_start+2).unwrap().is_numeric() {
                if contents.chars().nth(num1_start+3).unwrap() == ',' {
                    num1_length = 3;
                }
                else {
                    continue;
                }
            }
            else {
                if contents.chars().nth(num1_start+2).unwrap() == ',' {
                    num1_length = 2;
                }
                else {
                    continue;
                }
            }
        }
        else if contents.chars().nth(num1_start+1).unwrap() != ',' {
            continue;
        }

        let num2_start = num1_start + num1_length + 1;
        let mut num2_length = 1;
        if !contents.chars().nth(num2_start).unwrap().is_numeric() {
            continue;
        }

        //Work out length of second num or continue if badly formed
        if contents.chars().nth(num2_start+1).unwrap().is_numeric() {
            if contents.chars().nth(num2_start+2).unwrap().is_numeric() {
                if contents.chars().nth(num2_start+3).unwrap() == ')' {
                    num2_length = 3;
                }
                else {
                    continue;
                }
            }
            else {
                if contents.chars().nth(num2_start+2).unwrap() == ')' {
                    num2_length = 2;
                }
                else {
                    continue;
                }
            }
        }
        else if contents.chars().nth(num2_start+1).unwrap() != ')' {
            continue;
        }

        let num1 = &contents[num1_start..num1_start+num1_length];
        let num2 = &contents[num2_start..num2_start+num2_length];

        let multiplication = num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
        total += multiplication;
    }

    println!("Total Part 2:  {total}");
}
