use std::fs;

fn main() {
    //PART 1 Solution
    let file_path = "input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total = 0;
    //Create 2d vector of chars
    for line in contents.lines() {
        let test_val_and_other_vals = line.split(": ").collect::<Vec<&str>>();
        let test_val = test_val_and_other_vals[0];
        let other_vals = test_val_and_other_vals[1];
        let other_vals_vec = other_vals.split(" ").collect::<Vec<&str>>();
        //println!("{:?}", other_vals_vec);

        let mut potential_outputs:Vec<i64> = Vec::new();
        potential_outputs.push(0);
        for other_val in other_vals_vec {
            let mut new_potential_outputs = Vec::new();
            for output_val in potential_outputs {
                let other_val_int = other_val.parse::<i64>().unwrap();
                //println!("other_val_int {other_val_int} output_val {output_val}");
                new_potential_outputs.push(output_val + other_val_int);
                new_potential_outputs.push(output_val * other_val_int);
            }
            potential_outputs = new_potential_outputs;
        }

        for result in potential_outputs {
            let test_val_int = test_val.parse::<i64>().unwrap();
            if result == test_val_int {
                total += test_val_int;
                break;
            }
        }
    }

    println!("Total Part1: {}", total);

    //Part 2
    let mut total = 0;
    //Create 2d vector of chars
    for line in contents.lines() {
        let test_val_and_other_vals = line.split(": ").collect::<Vec<&str>>();
        let test_val = test_val_and_other_vals[0];
        let other_vals = test_val_and_other_vals[1];
        let other_vals_vec = other_vals.split(" ").collect::<Vec<&str>>();
        //println!("{:?}", other_vals_vec);

        let mut potential_outputs:Vec<i64> = Vec::new();
        potential_outputs.push(0);
        for other_val in other_vals_vec {
            let mut new_potential_outputs = Vec::new();
            for output_val in potential_outputs {
                let other_val_int = other_val.parse::<i64>().unwrap();
                //println!("other_val_int {other_val_int} output_val {output_val}");
                new_potential_outputs.push(output_val + other_val_int);
                new_potential_outputs.push(output_val * other_val_int);

                //New bit for part 2
                let string_output_val:String = output_val.to_string();
                let new_potential_output:String = string_output_val + other_val;
                let concatenated_val = new_potential_output.parse::<i64>().unwrap();
                new_potential_outputs.push(concatenated_val);
            }
            potential_outputs = new_potential_outputs;
        }

        for result in potential_outputs {
            let test_val_int = test_val.parse::<i64>().unwrap();
            if result == test_val_int {
                total += test_val_int;
                break;
            }
        }
    }

    println!("Total Part2: {}", total);
}
