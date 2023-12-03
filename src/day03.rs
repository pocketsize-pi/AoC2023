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

    // utility function to get the diagonals
    // started as a function, the IDE told me to convert to a closure
    // I understood closures when I did the course, not any more!
    let get_diagonal_values = |r: i32, c: i32| -> Vec<char> {
        let mut return_values: Vec<char> = Vec::new();
        let my_range : Vec<i32> = vec!(-1, 0, 1);
        for r_i in &my_range {
            for c_i in &my_range {
                let new_r = r + r_i;
                let new_c = c + c_i;
                if ! ((new_r < 0) | (new_r >= height as i32) | (new_c < 0) | (new_c >= width as i32)) {
                    return_values.push(new_data[new_r as usize][new_c as usize]);
                }

            }
        }
        return_values
    };

    let mut sum_of_parts : u32 = 0;

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
                    let diagonals = get_diagonal_values(r as i32,c as i32);
                    // println!("{:?}", diagonals);
                    for d_vals in diagonals {
                        if !((d_vals==nothing) | d_vals.is_digit(10)) {
                            // it's a symbol! we are good
                            found_symbol = true;
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


    Ok(())
}