use std::fs;
use std::collections::HashMap;

fn main() {
    //PART 1 Solution
    let file_path = "smallInput.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut char_grid:Vec<Vec<char>> = Vec::new();

    //Create 2d vector of chars
    for line in contents.lines() {
        let line_vec:Vec<char> = line.chars().collect();
        char_grid.push(line_vec);
    }

    //Prints grid
    //for x in 0..char_grid.len() {
    //    for y in 0..char_grid[x].len() {
    //        print!("{}", char_grid[x][y]);
    //    }
    //    print!("\n");
    //}

    let mut x_pos = 0;
    let mut y_pos = 0;
    for y_pos_check in 0..char_grid.len() {
        for x_pos_check in 0..char_grid[0].len() {
            if char_grid[y_pos_check][x_pos_check] == '^' {
                //Initial position
                x_pos = x_pos_check;
                y_pos = y_pos_check;
            }
        }
    }


    char_grid[y_pos][x_pos] = 'X'; //X denotes a location has been visited

    let mut in_map = true;
    let mut total_distinct_locations_visited = 1;
    let mut facing_direction = "North".to_string();
    while in_map == true {
        if facing_direction == "North".to_string() {
            if !y_pos.checked_sub(1).is_none()
                && char_grid[y_pos-1][x_pos] != '#'{
                y_pos -= 1;
            }
            else if !y_pos.checked_sub(1).is_none()
                && char_grid[y_pos-1][x_pos] == '#' {
                facing_direction = "East".to_string();
            }
            else if y_pos.checked_sub(1).is_none() {
                in_map = false;
            }
        }
        else if facing_direction == "East".to_string() {
            if  x_pos+1 < char_grid[0].len()
                && char_grid[y_pos][x_pos+1] != '#'{
                x_pos += 1;
            }
            else if x_pos+1 < char_grid[0].len()
                &&char_grid[y_pos][x_pos+1] == '#' {
                facing_direction = "South".to_string();
            }
            else if x_pos+1 >= char_grid[0].len() {
                in_map = false;
            }
        }
        else if facing_direction == "South".to_string(){
            if  y_pos+1 < char_grid.len()
                && char_grid[y_pos+1][x_pos] != '#'{
                y_pos += 1;
            }
            else if y_pos+1 < char_grid.len()
                &&char_grid[y_pos+1][x_pos] == '#' {
                facing_direction = "West".to_string();
            }
            else if y_pos+1 >= char_grid.len() {
                in_map = false;
            }
        }
        if facing_direction == "West".to_string() {
            if !x_pos.checked_sub(1).is_none()
                && char_grid[y_pos][x_pos-1] != '#'{
                x_pos -= 1;
            }
            else if !x_pos.checked_sub(1).is_none()
                && char_grid[y_pos][x_pos-1] == '#' {
                facing_direction = "North".to_string();
            }
            else if x_pos.checked_sub(1).is_none() {
                in_map = false;
            }
        }

        if char_grid[y_pos][x_pos] != 'X' {
            total_distinct_locations_visited += 1;
            char_grid[y_pos][x_pos] = 'X';
        }
    }

    println!("Total locations visited, part 1: {}", total_distinct_locations_visited);

    //Part 2 Solution
    let file_path = "input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut char_grid:Vec<Vec<char>> = Vec::new();

    //Create 2d vector of chars
    for line in contents.lines() {
        let line_vec:Vec<char> = line.chars().collect();
        char_grid.push(line_vec);
    }

    //Prints grid
    //for x in 0..char_grid.len() {
    //    for y in 0..char_grid[x].len() {
    //        print!("{}", char_grid[x][y]);
    //    }
    //    print!("\n");
    //}

    let mut x_pos = 0;
    let mut y_pos = 0;
    for y_pos_check in 0..char_grid.len() {
        for x_pos_check in 0..char_grid[0].len() {
            if char_grid[y_pos_check][x_pos_check] == '^' {
                //Initial position
                x_pos = x_pos_check;
                y_pos = y_pos_check;
            }
        }
    }
    let initial_x_pos = x_pos;
    let initial_y_pos = y_pos;

    let mut total_loops = 0;
    for change_y in 0..char_grid.len() {
        for change_x in 0..char_grid[0].len() {
            //try placing an obstacle at each location and see the outcome
            let old_pos = char_grid[change_y][change_x];
            if old_pos == '^' || old_pos == '#' {
                continue; //skip positions with obstacles or guards
            }
            char_grid[change_y][change_x] = '#';

            let mut y_pos = initial_y_pos;
            let mut x_pos = initial_x_pos;
            //println!("y_pos: {y_pos}, x_pos: {x_pos}");

            let mut in_map = true;
            let mut proven_loop = false;
            let mut visited_locations_and_directions:HashMap<String, Vec<String>> = HashMap::new();
            let mut facing_direction = "North".to_string();
            while in_map == true && proven_loop == false {
                if facing_direction == "North".to_string() {
                    if !y_pos.checked_sub(1).is_none()
                        && char_grid[y_pos-1][x_pos] != '#'{
                        y_pos -= 1;
                    }
                    else if !y_pos.checked_sub(1).is_none()
                        && char_grid[y_pos-1][x_pos] == '#' {
                        facing_direction = "East".to_string();
                    }
                    else if y_pos.checked_sub(1).is_none() {
                        in_map = false;
                    }
                }
                else if facing_direction == "East".to_string() {
                    if  x_pos+1 < char_grid[0].len()
                        && char_grid[y_pos][x_pos+1] != '#'{
                        x_pos += 1;
                    }
                    else if x_pos+1 < char_grid[0].len()
                        && char_grid[y_pos][x_pos+1] == '#' {
                        facing_direction = "South".to_string();
                    }
                    else if x_pos+1 >= char_grid[0].len() {
                        in_map = false;
                    }
                }
                else if facing_direction == "South".to_string(){
                    if  y_pos+1 < char_grid.len()
                        && char_grid[y_pos+1][x_pos] != '#'{
                        y_pos += 1;
                    }
                    else if y_pos+1 < char_grid.len()
                        && char_grid[y_pos+1][x_pos] == '#' {
                        facing_direction = "West".to_string();
                    }
                    else if y_pos+1 >= char_grid.len() {
                        in_map = false;
                    }
                }
                if facing_direction == "West".to_string() {
                    if !x_pos.checked_sub(1).is_none()
                        && char_grid[y_pos][x_pos-1] != '#'{
                        x_pos -= 1;
                    }
                    else if !x_pos.checked_sub(1).is_none()
                        && char_grid[y_pos][x_pos-1] == '#' {
                        facing_direction = "North".to_string();
                    }
                    else if x_pos.checked_sub(1).is_none() {
                        in_map = false;
                    }
                }


                if char_grid[y_pos][x_pos] != 'X' {
                    total_distinct_locations_visited += 1;
                    char_grid[y_pos][x_pos] = 'X';
                }

                let position = y_pos.to_string() + "," + &x_pos.to_string();
                if visited_locations_and_directions.contains_key(&position) {
                    let map_entry = visited_locations_and_directions.get_mut(&position).unwrap();
                    if map_entry.contains(&facing_direction) {
                        //println!("position: {:?}", position);
                        //println!("facing_direction: {:?}", facing_direction);
                        proven_loop = true;
                        //println!("visited: {:?}", visited_locations_and_directions);
                    }
                }

                visited_locations_and_directions.entry(position).or_default().push(facing_direction.clone());

            }
            //restore position to have old value
            char_grid[change_y][change_x] = old_pos;
            if proven_loop && in_map {
                total_loops += 1;
                println!("Loop Found, change_x: {change_x}, change_y: {change_y}");
            }
        }
    }

    println!("Total loops, Part 2: {}", total_loops);
}
