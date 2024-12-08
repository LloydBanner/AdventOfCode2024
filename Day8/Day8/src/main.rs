use std::fs;
use std::collections::HashMap;

fn main() {
    //PART 1 Solution
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

    let mut character_and_places_it_appears:HashMap<String, Vec<Vec<usize>>> = HashMap::new();
    for y_pos in 0..char_grid.len() {
        for x_pos in 0..char_grid[y_pos].len() {
            if char_grid[y_pos][x_pos].is_alphanumeric() {
                let character = char_grid[y_pos][x_pos].to_string();
                let mut position = Vec::new();
                position.push(y_pos);
                position.push(x_pos);
                character_and_places_it_appears.entry(character).or_default().push(position);
            }
        }
    }
    //println!("{:?}", character_and_places_it_appears);

    let mut antinodes:Vec<Vec<usize>> = Vec::new();
    for character_and_positions in character_and_places_it_appears.clone() {
        //Find antinodes for positions with same character
        let character_positions = character_and_positions.1;
        //println!("positions {character_positions:?}");
        for char_pos1 in 0..character_positions.len() {
            for char_pos2 in 0..character_positions.len() {
                if char_pos1 != char_pos2 {
                    let position1 = &character_positions[char_pos1];
                    let position2 = &character_positions[char_pos2];
                    let diff_y = position1[0] as i8 - position2[0] as i8;
                    let diff_x = position1[1] as i8 - position2[1] as i8;

                    // if either none then outside of map so ignore
                    let potential_antinode1 = find_antinode(position1, position2, diff_y, diff_x, antinodes.clone(), char_grid.clone());
                    let potential_antinode2 = find_antinode(position1, position2, -diff_y, -diff_x, antinodes.clone(), char_grid.clone());
                    let potential_antinode3 = find_antinode(position2, position1, diff_y, diff_x, antinodes.clone(), char_grid.clone());
                    let potential_antinode4 = find_antinode(position2, position1, -diff_y, -diff_x, antinodes.clone(), char_grid.clone());

                    if !potential_antinode1.is_empty() {
                        antinodes.push(potential_antinode1);
                    }
                    if !potential_antinode2.is_empty() {
                        antinodes.push(potential_antinode2);
                    }
                    if !potential_antinode3.is_empty() {
                        antinodes.push(potential_antinode3);
                    }
                    if !potential_antinode4.is_empty() {
                        antinodes.push(potential_antinode4);
                    }
                }
            }
        }
    }

    println!("Antinodes length is {}", antinodes.len());

    //Part 2 Solution
    let mut antinodes:Vec<Vec<usize>> = Vec::new();
    for character_and_positions in character_and_places_it_appears.clone() {
        //Find antinodes for positions with same character
        let character_positions = character_and_positions.1;
        //println!("positions {character_positions:?}");
        for char_pos1 in 0..character_positions.len() {
            for char_pos2 in 0..character_positions.len() {
                if char_pos1 != char_pos2 {
                    let position1 = &character_positions[char_pos1];
                    let position2 = &character_positions[char_pos2];
                    let diff_y = position1[0] as i8 - position2[0] as i8;
                    let diff_x = position1[1] as i8 - position2[1] as i8;

                    // if either none then outside of map so ignore
                    let potential_antinodes1 = find_antinode_consecutive(position1, diff_y, diff_x, char_grid.clone());
                    let potential_antinodes2 = find_antinode_consecutive(position2, diff_y, diff_x, char_grid.clone());

                    for antinode in potential_antinodes1 {
                        if !antinodes.contains(&antinode) {
                            antinodes.push(antinode);
                        }
                    }
                    for antinode in potential_antinodes2 {
                        if !antinodes.contains(&antinode) {
                            antinodes.push(antinode);
                        }
                    }
                }
            }
        }
    }

    //println!("Antinodes: {antinodes:?}");
    println!("Antinodes part 2 length is {}", antinodes.len());
}

fn find_antinode(position1: &Vec<usize>, position2: &Vec<usize>, diff_y:i8, diff_x:i8, antinodes:Vec<Vec<usize>>, char_grid:Vec<Vec<char>>) -> Vec<usize> {
    let mut check_y = true;
    let mut check_x = true;
    let abs_diff_y = diff_y.abs();
    if diff_y.is_negative(){
        if position1[0].checked_sub(abs_diff_y as usize) == None {
            check_y = false;
        }
    };
    let abs_diff_x = diff_x.abs();
    if diff_x.is_negative(){
        if position1[1].checked_sub(abs_diff_x as usize) == None {
            check_x = false;
        }
    }

    //println!("diff_x: {diff_x} diff_y: {diff_y} pos: {position1:?} check_y: {check_y:?} check_x: {check_x:?}");
    if check_y {
        if check_x {
            let antinode_y;
            let antinode_x;
            if diff_y.is_negative() {
                antinode_y = position1[0] - diff_y.abs() as usize;
            }
            else {
                antinode_y = position1[0] + diff_y.abs() as usize;
            }

            if diff_x.is_negative() {
                antinode_x = position1[1] - diff_x.abs() as usize;
            }
            else {
                antinode_x = position1[1] + diff_x.abs() as usize;
            }

            //println!("antinode_y {antinode_y}, antinode_x {antinode_x}");
            if antinode_y < char_grid.len() && antinode_x < char_grid[0].len() {
                let new_position = vec![antinode_y, antinode_x];
                let result_is_ant_1 = position1[0] == new_position[0] && position1[1] == new_position[1];
                let result_is_ant_2 = position2[0] == new_position[0] && position2[1] == new_position[1];

                if !antinodes.contains(&new_position)
                && !result_is_ant_1 && !result_is_ant_2 {
                    //println!("position added {new_position:?}");
                    return new_position;
                }
            }
        }
    }
    let new_position = Vec::new();
    return new_position;
}

fn find_antinode_consecutive(position: &Vec<usize>, diff_y:i8, diff_x:i8, char_grid:Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut inside_map = true;
    let mut iteration_starting_pos = position.clone();
    let mut new_antinodes:Vec<Vec<usize>> = Vec::new();
    while inside_map {
        let mut check_y = true;
        let mut check_x = true;
        let abs_diff_y = diff_y.abs();
        if diff_y.is_negative(){
            if iteration_starting_pos[0].checked_sub(abs_diff_y as usize) == None {
                check_y = false;
                inside_map = false;
            }
        };
        let abs_diff_x = diff_x.abs();
        if diff_x.is_negative(){
            if iteration_starting_pos[1].checked_sub(abs_diff_x as usize) == None {
                check_x = false;
                inside_map = false;
            }
        }

        //println!("diff_x: {diff_x} diff_y: {diff_y} pos: {iteration_starting_pos:?} check_y: {check_y:?} check_x: {check_x:?}");
        if check_y {
            if check_x {
                let antinode_y;
                let antinode_x;
                if diff_y.is_negative() {
                    antinode_y = iteration_starting_pos[0] - diff_y.abs() as usize;
                }
                else {
                    antinode_y = iteration_starting_pos[0] + diff_y.abs() as usize;
                }

                if diff_x.is_negative() {
                    antinode_x = iteration_starting_pos[1] - diff_x.abs() as usize;
                }
                else {
                    antinode_x = iteration_starting_pos[1] + diff_x.abs() as usize;
                }

                //println!("antinode_y {antinode_y}, antinode_x {antinode_x}");
                if antinode_y < char_grid.len() && antinode_x < char_grid[0].len() {
                    let new_position = vec![antinode_y, antinode_x];
                    //println!("position added {new_position:?}");
                    new_antinodes.push(new_position.clone());
                    iteration_starting_pos = new_position;
                }
                else {
                    inside_map = false;
                }
            }
        }

    }
    return new_antinodes;
}