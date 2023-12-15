use std::collections::HashMap;
use itertools::max;
use AoC2023::*;
use AoC2023::Direction::{East, North, South, West};

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
                    } else {
                        break;
                    }
                }
                else {
                    // not a rock!
                    break;
                }
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

    // Part two (I could functionise, but I like copy paste)
    let num_cycles = 1000000000;
    // let tilt_cycle = [North, West, South, East];

    fn reached_edge (dir: Direction, point: Point, rows: i32, cols: i32) -> bool {
        let mut edge = false;
        if dir == North {
            edge = (point.r < 1);
        }
        else if dir == West {
            edge = (point.c < 1);
            // println!("? {:?} {}", edge, c_i);
        }
        else if dir == South {
            edge = (point.r >= rows-1);
        }
        else if dir == East {
            edge = (point.c >= cols-1);
        }

        edge
    }

    let mut prev_map = platform_map.clone();
    let mut full_list = Vec::new();
    let mut repeat = 0;
    let mut final_cycle = 0;
    let mut found_pos = 0;

    for cycle in 0..num_cycles {
        // println!("cycle {cycle}");
        // for tilt in tilt_cycle {
        //     println!("{:?}", tilt);
        // NORTH
        let mut tilt = North;
        // println!("North");
        for r in 0..num_rows {
            // println!("{r}");
            for c in 0..num_cols {
                // println!("{c}");
                let mut r_i = r as i32;
                let mut c_i = c as i32;
                let mut current = Point { r: r_i, c: c_i };

                while !reached_edge(tilt, current, num_rows as i32, num_cols as i32) {

                    // println!("{:?}", current);
                    if *platform_map.get(&current).unwrap() == 'O' {
                        let target = move_point(&current, tilt);
                        if *platform_map.get(&target).unwrap() == '.' {
                            platform_map.insert(current, '.');
                            platform_map.insert(target, 'O');
                            current = move_point(&current, tilt);

                        } else {break;}
                    }
                    else {break;}
                }
            }
        }

        // WEST
        tilt = West;
        // println!("West");
        for c in 0..num_cols {
            // println!("{c}");
            for r in 0..num_rows {
                // println!("{r}");
                let mut r_i = r as i32;
                let mut c_i = c as i32;
                let mut current = Point { r: r_i, c: c_i };

                while !reached_edge(tilt, current, num_rows as i32, num_cols as i32) {

                    // println!("{:?}", current);
                    if *platform_map.get(&current).unwrap() == 'O' {
                        let target = move_point(&current, tilt);
                        if *platform_map.get(&target).unwrap() == '.' {
                            platform_map.insert(current, '.');
                            platform_map.insert(target, 'O');
                            current = move_point(&current, tilt);

                        } else {break;}
                    }
                    else {break;}
                }
            }
        }

        // SOUTH
        tilt = South;
        // println!("South");
        for r in (0..num_rows).rev() {
            // println!("{r}");
            for c in 0..num_cols {
                // println!("{c}");
                let mut r_i = r as i32;
                let mut c_i = c as i32;
                let mut current = Point { r: r_i, c: c_i };

                while !reached_edge(tilt, current, num_rows as i32, num_cols as i32) {

                    // println!("{:?}", current);
                    if *platform_map.get(&current).unwrap() == 'O' {
                        let target = move_point(&current, tilt);
                        if *platform_map.get(&target).unwrap() == '.' {
                            platform_map.insert(current, '.');
                            platform_map.insert(target, 'O');
                            current = move_point(&current, tilt);

                        } else {break;}
                    }
                    else {break;}
                }
            }
        }

        // EAST
        tilt = East;
        // println!("East");
        for c in (0..num_cols).rev() {
            // println!("{r}");
            for r in 0..num_rows {
                // println!("{c}");
                let mut r_i = r as i32;
                let mut c_i = c as i32;
                let mut current = Point { r: r_i, c: c_i };

                while !reached_edge(tilt, current, num_rows as i32, num_cols as i32) {

                    // println!("{:?}", current);
                    if *platform_map.get(&current).unwrap() == 'O' {
                        let target = move_point(&current, tilt);
                        if *platform_map.get(&target).unwrap() == '.' {
                            platform_map.insert(current, '.');
                            platform_map.insert(target, 'O');
                            current = move_point(&current, tilt);

                        } else {break;}
                    }
                    else {break;}
                }
            }
        }

        if full_list.contains(&platform_map) {
            println!("settled after {} cycles", cycle);
            final_cycle = cycle;
            found_pos = full_list.iter().position(|r| r == &platform_map).unwrap();
            repeat = final_cycle - found_pos;
            println!("entry from list: {}", repeat);
            break;

        }
        else {
            // prev_map == platform_map.clone();
            full_list.push(platform_map.clone());
        }

    }

    // get the right state
    let remainder = (num_cycles - final_cycle) % repeat;

    let final_id = remainder + found_pos -1;

    println!("final id? {}", final_id);

    // for check in 0..final_cycle {
        let mut max_score = num_rows;
        let mut total_load = 0;
        for r in 0..num_rows {
            let mut O_in_row: usize = 0;
            for c in 0..num_cols {
                if *full_list[final_id].get(&Point { r: r as i32, c: c as i32 }).unwrap() == 'O' {
                    O_in_row += 1;
                }
            }
            total_load += max_score * O_in_row;
            max_score -= 1;
        }
    //     println!("Part 2: {} {}", check, total_load);
    // }

    println!("Part 2: {}", total_load);
    // 105112 too high
    // 105008 is right, I had a by-one error

    Ok(())
}