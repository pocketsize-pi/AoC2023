use std::collections::HashMap;
use AoC2023::InputType;

pub fn day03(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 3");


    let data = AoC2023::read_input(3, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }

    // --- Day 3: Gear Ratios ---
    // You and the Elf eventually reach a gondola lift station; he says the gondola lift will take
    // you up to the water source, but this is as far as he can bring you. You go inside.

    // It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

    // "Aaah!"

    // You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry,
    // I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while
    // before I can fix it." You offer to help.

    // The engineer explains that an engine part seems to be missing from the engine, but nobody
    // can figure out which one. If you can add up all the part numbers in the engine schematic,
    // it should be easy to work out which part is missing.

    // The engine schematic (your puzzle input) consists of a visual representation of the engine.
    // There are lots of numbers and symbols you don't really understand, but apparently any number
    // adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum.
    // (Periods (.) do not count as a symbol.)

    // Here is an example engine schematic:

    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..
    // In this schematic, two numbers are not part numbers because they are not adjacent to a symbol:
    // 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is
    // a part number; their sum is 4361.

    // Of course, the actual engine schematic is much larger. What is the sum of all of the part
    // numbers in the engine schematic?

    // reconfigure data into chars
    let mut new_data : Vec<Vec<char>> = Vec::new();
    for i in 0..data.len() {
        new_data.push(data[i][0].chars().collect());
    }
    // for row in &new_data {
    //     println!("{:?}", row);
    // }

    let width = new_data[0].len();
    let height = new_data.len();
    let nothing = '.';

    #[derive(Hash)]
    struct Coords {
        r: u32,
        c: u32,
    }

    // utility function to get the diagonals
    // started as a function, the IDE told me to convert to a closure
    // I understood closures when I did the course, not any more!
    let get_diagonal_values = |r: i32, c: i32| -> (Vec<char>,Vec<(u32, u32)>) {
        let mut return_values: Vec<char> = Vec::new();
        let mut return_coords: Vec<(u32, u32)> = Vec::new();
        let my_range : Vec<i32> = vec!(-1, 0, 1);
        for r_i in &my_range {
            for c_i in &my_range {
                let new_r = r + r_i;
                let new_c = c + c_i;
                if ! ((new_r < 0) | (new_r >= height as i32) | (new_c < 0) | (new_c >= width as i32)) {
                    return_values.push(new_data[new_r as usize][new_c as usize]);
                    return_coords.push((new_r as u32, new_c as u32));
                }

            }
        }
        (return_values, return_coords)
    };

    let mut sum_of_parts : u32 = 0;



    // let symbol_coords : Vec<Coords> = Vec::new();
    let mut symbol_db  = HashMap::new();

    for r in 0..height {
        let mut in_a_number = false;
        let mut found_symbol = false;
        let mut number_string = String::new();

        for c in 0..width {

            let val = new_data[r][c];


            if val.is_digit(10) {
                in_a_number = true;
                number_string.push(val);
                // ok, it's a number, now what
                // check all diagonals, if it is not a match already!
                if !found_symbol {
                    let (diagonals, diag_coord) = get_diagonal_values(r as i32,c as i32);
                    // println!("{:?}", diagonals);
                    for d_i in 0..diagonals.len() {
                    // for d_vals in diagonals {
                        let d_vals = &diagonals[d_i];
                        let d_coords = &diag_coord[d_i];
                        if !((*d_vals==nothing) | d_vals.is_digit(10)) {
                            // it's a symbol! we are good
                            found_symbol = true;

                            // we've found a symbol, let's see what we need to do with it
                            if !symbol_db.contains_key(d_coords) {
                                symbol_db.insert(d_coords.clone(), 1);
                            }
                            else {
                                // the symbol already exists
                                // symbol_db[d_coords] = symbol_db[d_coords] +1;
                                symbol_db.insert(d_coords.clone(), symbol_db[d_coords] +1);
                            }

                            // we dont' need to stay here longer
                            break;
                        }
                    }
                }
            }
            // if val is not a digit, or we are at the end of a row
            // we break not just in dots, but in a symbol!
            if !(val.is_digit(10)) | (c == width-1) {
                // ok, this is a dot or the end of the row. is this the end of a number?
                if in_a_number & found_symbol {
                    // this number had a symbol, and is good to keep
                    // println!("{}", number_string);
                    sum_of_parts += number_string.parse::<u32>().unwrap();
                }
                // reset our values and continue
                in_a_number = false;
                number_string = "".parse().unwrap();
                found_symbol = false;
                continue;
            }

        }
    }

    println!("{}", sum_of_parts);
    // 13466731 is too high -> I wasn't breaking the number when it had a symbol in the middle 90&12
    // 553079 is correct!

    // --- Part Two ---
    // The engineer finds the missing part and installs it in the engine! As the engine springs to
    // life, you jump in the closest gondola, finally ready to ascend to the water source.

    // You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately,
    // the gondola has a phone labeled "help", so you pick it up and the engineer answers.

    // Before you can explain the situation, she suggests that you look out the window. There stands
    // the engineer, holding a phone in one hand and waving with the other. You're going so slowly
    // that you haven't even left the station. You exit the gondola.

    // The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear
    // is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of
    // multiplying those two numbers together.

    // This time, you need to find the gear ratio of every gear and add them all up so that the
    // engineer can figure out which gear needs to be replaced.

    // Consider the same engine schematic again:

    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..
    // In this schematic, there are two gears. The first is in the top left; it has part numbers
    // 467 and 35, so its gear ratio is 16345. The second gear is in the lower right;
    // its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only
    // adjacent to one part number.) Adding up all of the gear ratios produces 467835.

    // What is the sum of all of the gear ratios in your engine schematic?

    // Ok, so these are all the symbols with only two numbers adjacent
    let mut gear_sum : u32 = 0;
    for (s_coord, s_num) in &symbol_db {
        if *s_num == 2 {
            // println!("{} {}", s_coord.0, s_coord.1);

            // let's get the diagonals again!
            let mut visited_coords : Vec<(u32, u32)> = Vec::new();
            let (diagonals, diag_coord) = get_diagonal_values(s_coord.0 as i32,s_coord.1 as i32);
            let mut local_product : u32 = 1;
            for d_i in 0..diagonals.len() {
                // for d_vals in diagonals {

                let d_vals = &diagonals[d_i];
                let d_coords = &diag_coord[d_i];
                // is this a non-visited coordinate?
                if !visited_coords.contains(d_coords) {
                    if d_vals.is_digit(10) {
                        let mut build_num = String::default();

                        build_num.insert(0,new_data[d_coords.0 as usize][d_coords.1 as usize]);
                        visited_coords.push(*d_coords);
                        // go back
                        let mut new_c = d_coords.1;
                        if d_coords.1 >=1 {
                            new_c = d_coords.1 - 1;
                            while new_data[d_coords.0 as usize][new_c as usize].is_digit(10) {
                                build_num.insert(0, new_data[d_coords.0 as usize][new_c as usize]);
                                visited_coords.push((d_coords.0, new_c));
                                if new_c >=1 { new_c = new_c - 1;}
                                else {break;}
                            }
                        }
                        // now go forwards
                        if d_coords.1 < (width as u32 -1) {
                            new_c = d_coords.1 + 1;

                            while new_data[d_coords.0 as usize][new_c as usize].is_digit(10) {
                                build_num.insert(build_num.len(), new_data[d_coords.0 as usize][new_c as usize]);
                                visited_coords.push((d_coords.0, new_c));

                                if new_c < (width as u32 -1) { new_c = new_c + 1;}
                                else {break;}
                            }
                        }

                        // we are done with the number, let's store it
                        // println!("{}", build_num);
                        local_product = local_product * build_num.parse::<u32>().unwrap();
                    }

                }

            }

            //  and now we add all the local products
            gear_sum += local_product;

        }
    }

    println!("Part 2: {}", gear_sum);
    // 84363105 is the right answer

    Ok(())
}