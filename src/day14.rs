use std::collections::HashMap;
use itertools::max;
use AoC2023::*;
use AoC2023::Direction::North;

pub fn day14(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 14");


    let data = AoC2023::read_input(14, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }

    // Vec or HashMap? Not sure yet
    let mut platform_vec = Vec::new();
    let mut platform_map = HashMap::new();

    for i  in 0..data.len() {
        // let row = &data[i];
        let my_row = string_to_chars(&data[i][0]);
        platform_vec.push(my_row.clone()); // cloning here to keep it afterwards
        for j in 0..my_row.len() {
            platform_map.insert(Point{r: i as i32, c:j as i32}, my_row[j]);
        }
    }

    let num_rows = data.len();
    let num_cols = data[0][0].len();
    let tilt_direction = North;
    println!("rows {}", &num_rows);
    println!("cols {}", &num_cols);

    // skip the top row because we can't move them North
    for r in 1..num_rows {
        // println!("{r}");
        for c in 0..num_cols {
            let mut still_room = true;

            let mut r_i = r as i32;
            let mut c_i = c as i32;
            while r_i >= 1 {
                let mut current = Point { r: r_i, c: c_i };
                // println!("{:?}", current);
                if *platform_map.get(&current).unwrap() == 'O' {
                    // this is a rock, so we move it
                    let target = move_point(&current, tilt_direction);
                    if *platform_map.get(&target).unwrap() == '.' {
                        // we can move it!
                        // platform_map[&current] = '.';
                        platform_map.insert(current, '.');
                        // platform_map[&target] = 'O';
                        platform_map.insert(target, 'O');
                        // update the local indeces so we can go around again
                        r_i -= 1;
                        // c_i += 1;
                    } else {
                        still_room = false;
                        break;
                    }
                }
                else {
                    // not a rock!
                    break;
                }
                // check whether we have reached the end

            }
        }
    }

    // print updated map
    // println!("Map");
    // for r in 0..num_rows {
    //     // println!("{r}");
    //     for c in 0..num_cols {
    //         print!("{}", platform_map.get(&Point{r: r as i32, c: c as i32}).unwrap());
    //     }
    //     print!("\n");
    // }

    let mut max_score = num_rows;
    let mut total_load = 0;
    for r in 0..num_rows {
        let mut O_in_row : usize = 0;
        for c in 0..num_cols {
            if *platform_map.get(&Point{r:r as i32, c: c as i32}).unwrap() == 'O' {
                O_in_row += 1;
            }
        }
        total_load += max_score * O_in_row;
        max_score -= 1;
    }

    println!("Part 1: {}", total_load);
    // 102497 not bad!

    Ok(())
}