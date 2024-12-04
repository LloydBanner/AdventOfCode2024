use std::fs;

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

    //Prints grid
    //for x in 0..char_grid.len() {
    //    for y in 0..char_grid[x].len() {
    //        print!("{}", char_grid[x][y]);
    //    }
    //    print!("\n");
    //}

    let y_len = char_grid.len();
    let x_len = char_grid[0].len();
    let mut total = 0;
    for y_pos in 0..y_len {
        for x_pos in 0..x_len {
            //Forward
            if char_grid[y_pos][x_pos] == 'X' && (x_pos+3)<x_len {
                if char_grid[y_pos][x_pos+1] == 'M'
                    && char_grid[y_pos][x_pos+2] == 'A'
                    && char_grid[y_pos][x_pos+3] == 'S' {
                    total += 1;
                }
            }
            //Backwards
            if char_grid[y_pos][x_pos] == 'X' && x_pos.checked_add_signed(-3)!=None {
                if char_grid[y_pos][x_pos-1] == 'M'
                    && char_grid[y_pos][x_pos-2] == 'A'
                    && char_grid[y_pos][x_pos-3] == 'S' {
                    total += 1;
                }
            }
            //Down
            if char_grid[y_pos][x_pos] == 'X' && (y_pos+3)<y_len {
                if char_grid[y_pos+1][x_pos] == 'M'
                    && char_grid[y_pos+2][x_pos] == 'A'
                    && char_grid[y_pos+3][x_pos] == 'S' {
                    total += 1;
                }
            }
            //Up
            if char_grid[y_pos][x_pos] == 'X' && y_pos.checked_add_signed(-3)!=None {
                if char_grid[y_pos-1][x_pos] == 'M'
                    && char_grid[y_pos-2][x_pos] == 'A'
                    && char_grid[y_pos-3][x_pos] == 'S' {
                    total += 1;
                }
            }
            //Down-Right
            if char_grid[y_pos][x_pos] == 'X' && (x_pos+3)<x_len && (y_pos+3)<y_len {
                if char_grid[y_pos+1][x_pos+1] == 'M'
                    && char_grid[y_pos+2][x_pos+2] == 'A'
                    && char_grid[y_pos+3][x_pos+3] == 'S' {
                    total += 1;
                }
            }
            //Down-left
            if char_grid[y_pos][x_pos] == 'X' && x_pos.checked_add_signed(-3)!=None && (y_pos+3)<y_len {
                if char_grid[y_pos+1][x_pos-1] == 'M'
                    && char_grid[y_pos+2][x_pos-2] == 'A'
                    && char_grid[y_pos+3][x_pos-3] == 'S' {
                    total += 1;
                }
            }
            //Up-Right
            if char_grid[y_pos][x_pos] == 'X' && (x_pos+3)<x_len && y_pos.checked_add_signed(-3)!=None {
                if char_grid[y_pos-1][x_pos+1] == 'M'
                    && char_grid[y_pos-2][x_pos+2] == 'A'
                    && char_grid[y_pos-3][x_pos+3] == 'S' {
                    total += 1;
                }
            }
            //Up-Left
            if char_grid[y_pos][x_pos] == 'X' && x_pos.checked_add_signed(-3)!=None && y_pos.checked_add_signed(-3)!=None {
                if char_grid[y_pos-1][x_pos-1] == 'M'
                    && char_grid[y_pos-2][x_pos-2] == 'A'
                    && char_grid[y_pos-3][x_pos-3] == 'S' {
                    total += 1;
                }
            }
        }
    }

    println!("Total part 1: {total}");

    //Part 2
    let y_len = char_grid.len();
    let x_len = char_grid[0].len();
    let mut total = 0;
    for y_pos in 0..y_len {
        for x_pos in 0..x_len {
            if char_grid[y_pos][x_pos] == 'A'
                && (x_pos + 1) < x_len
                && (y_pos + 1) < y_len
                && x_pos.checked_add_signed(-1) != None
                && y_pos.checked_add_signed(-1) != None {

                //Check Diagonals
                if (char_grid[y_pos-1][x_pos-1] == 'M' && char_grid[y_pos+1][x_pos+1] == 'S')
                    || (char_grid[y_pos-1][x_pos-1] == 'S' && char_grid[y_pos+1][x_pos+1] == 'M'){
                    if (char_grid[y_pos-1][x_pos+1] == 'M' && char_grid[y_pos+1][x_pos-1] == 'S')
                        || (char_grid[y_pos-1][x_pos+1] == 'S' && char_grid[y_pos+1][x_pos-1] == 'M') {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("Total part 2: {total}");
}
