use std::collections::{HashMap, VecDeque};
use ndarray::{Array2, Axis};
use proceed::any_or_quit_with;
use AoC2023::*;
use AoC2023::Direction::{East, North, South, West};

pub fn day16(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 16");


    let data = AoC2023::read_input(16, input_type, manual_name)?;

    // let mut contraption : Vec<Array2<char>> = Vec::new();
    let mut temp_field : Vec<Vec<char>> = Vec::new();
    for row in &data {
        // println!("{:?}", row);

        temp_field.push(string_to_chars(&row[0]));
    }

    let num_rows = temp_field.len();
    let num_cols = temp_field[0].len();

    let mut contraption = Array2::<char>::default((num_rows, num_cols));
    for (i, mut row) in contraption.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = temp_field[i][j];
        }
    }

    // println!("{:?}", contraption);

    fn light_out (point: &Point, nr: usize, nc: usize) -> bool {
        (point.r < 0) | (point.c < 0) | (point.r >= nr as i32) | (point.c >= nc as i32)
    }

    // let mut light_pos = Point {r:0, c:0};
    // let mut light_dir = West;
    // let mut energised = HashMap::new();
    // let mut energised = Array2::<char>::default((num_rows, num_cols));
    let mut energised_vec = Vec::new();
    let mut beam_list = VecDeque::new();
    let mut beam_list_done = VecDeque::new();
    let mut visited_nodes = Vec::new();
    // let mut split_handled = Vec::new();

    // start conditions
    beam_list.push_back((Point {r:0, c:-1}, East));
    // beam_list_done.push_back((Point {r:0, c:0}, East));
    // energised_vec.push(Point {r:0, c:0});

    while !beam_list.is_empty() {
        // println!("{:?}", beam_list);
        let current_light = beam_list.pop_front().unwrap();
        let mut light_position = current_light.0;
        let mut light_direction = current_light.1;
        // println!("New beam");


        while !light_out(&move_point(&light_position, light_direction), num_rows, num_cols) &&
            !visited_nodes.contains(&(move_point(&light_position, light_direction), light_direction)){
            // move light along the direction
            let new_point = move_point(&light_position, light_direction);
            visited_nodes.push((new_point, light_direction));
            // println!("new point: {:?}, direction {:?}", new_point, light_direction);
            // check contents of new point
            let new_entry = contraption[(new_point.r as usize,new_point.c as usize)];
            if new_entry == '.' {
                // good, continue along
                light_position = new_point;
                if !energised_vec.contains(&light_position) {
                    energised_vec.push(light_position);
                }
            }
            else if new_entry == '|' {
                if (light_direction == North) | (light_direction == South) {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                }
                else {
                    // split! one goes north, the other south
                    light_position = new_point;
                    light_direction = North;
                    beam_list.push_back((new_point, South));
                    // if !beam_list_done.contains(&(new_point, South)) {
                        beam_list.push_back((new_point, South));
                        beam_list_done.push_back((new_point, South));
                    // }
                    // split_handled.push((new_point, South));
                    // println!("adding to beam list");
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                }
            }
            else if new_entry == '-' {
                if (light_direction == East) | (light_direction == West) {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                }
                else {
                    // split! one goes east, the other west
                    light_position = new_point;
                    light_direction = East;
                    // if !beam_list_done.contains(&(new_point, West)) {
                        beam_list.push_back((new_point, West));
                        beam_list_done.push_back((new_point, West));
                    // }
                    // split_handled.push((new_point, West));
                    // println!("adding to beam list");
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                }
            }
            else if new_entry == '\\' {
                // println!("found this?");
                light_position = new_point;
                if !energised_vec.contains(&light_position) {
                    energised_vec.push(light_position);
                }
                if light_direction == North {
                    light_direction = West;
                }
                else if light_direction == East {
                    light_direction = South;
                }
                else if light_direction == South {
                    light_direction = East;
                }
                else if light_direction == West {
                    light_direction = North;
                }
            }
            else if new_entry == '/' {
                light_position = new_point;
                if !energised_vec.contains(&light_position) {
                    energised_vec.push(light_position);
                }
                if light_direction == North {
                    light_direction = East;
                }
                else if light_direction == East {
                    light_direction = North;
                }
                else if light_direction == South {
                    light_direction = West;
                }
                else if light_direction == West {
                    light_direction = South;
                }
            }
            // print!("Press any key to continue, or 'q' to quit.");
            // for ri in 0..num_rows {
            //     let mut contr = String::new();
            //     for ci in 0..num_cols {
            //         if (light_position.r == ri as i32) & (light_position.c == ci as i32) {
            //             contr.push('@')
            //         }
            //         else if energised_vec.contains(&Point{r:ri as i32, c:ci as i32}) {
            //             contr.push('#');
            //         }
            //         else {
            //             contr.push('.');
            //         }
            //     }
            //     println!("{}", contr);
            // }
            // println!();
            // if !any_or_quit_with('q') {
            //     println!("Quitting.");
            //     return Ok(());
            // }

        }
        // println!("Light has left the field");
    }

    // for ri in 0..num_rows {
    //     let mut contr = String::new();
    //     for ci in 0..num_cols {
    //         // if (light_position.r == ri as i32) & (light_position.c == ci as i32) {
    //         //     contr.push('@')
    //         // }
    //         // else
    //         if energised_vec.contains(&Point{r:ri as i32, c:ci as i32}) {
    //             contr.push('#');
    //         }
    //         else {
    //             contr.push('.');
    //         }
    //     }
    //     println!("{}", contr);
    // }
    // println!();


    println!("PArt 1: {}", energised_vec.len());
    // 5297 too low
    // 7996 is right

    // Part 2 brute force copy paste
    let mut configuration_energised = Vec::new();

    for start_r in 0..num_rows {
        let mut energised_vec = Vec::new();
        let mut beam_list = VecDeque::new();
        let mut beam_list_done = VecDeque::new();
        let mut visited_nodes = Vec::new();
        let start_point = Point {r: start_r as i32, c:-1};
        let start_dir = East;

        beam_list.push_back((start_point.clone(), start_dir.clone()));

        while !beam_list.is_empty() {
            let current_light = beam_list.pop_front().unwrap();
            let mut light_position = current_light.0;
            let mut light_direction = current_light.1;

            while !light_out(&move_point(&light_position, light_direction), num_rows, num_cols) &&
                !visited_nodes.contains(&(move_point(&light_position, light_direction), light_direction)) {
                let new_point = move_point(&light_position, light_direction);
                visited_nodes.push((new_point, light_direction));
                let new_entry = contraption[(new_point.r as usize, new_point.c as usize)];
                if new_entry == '.' {
                    // good, continue along
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                } else if new_entry == '|' {
                    if (light_direction == North) | (light_direction == South) {
                        light_position = new_point;
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    } else {
                        light_position = new_point;
                        light_direction = North;
                        beam_list.push_back((new_point, South));
                        beam_list.push_back((new_point, South));
                        beam_list_done.push_back((new_point, South));
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    }
                } else if new_entry == '-' {
                    if (light_direction == East) | (light_direction == West) {
                        light_position = new_point;
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    } else {
                        light_position = new_point;
                        light_direction = East;
                        beam_list.push_back((new_point, West));
                        beam_list_done.push_back((new_point, West));
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    }
                } else if new_entry == '\\' {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                    if light_direction == North {
                        light_direction = West;
                    } else if light_direction == East {
                        light_direction = South;
                    } else if light_direction == South {
                        light_direction = East;
                    } else if light_direction == West {
                        light_direction = North;
                    }
                } else if new_entry == '/' {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                    if light_direction == North {
                        light_direction = East;
                    } else if light_direction == East {
                        light_direction = North;
                    } else if light_direction == South {
                        light_direction = West;
                    } else if light_direction == West {
                        light_direction = South;
                    }
                }
            }
        }
        configuration_energised.push(energised_vec.len());
    }

    for start_r in (0..num_rows).rev() {
        let mut energised_vec = Vec::new();
        let mut beam_list = VecDeque::new();
        let mut beam_list_done = VecDeque::new();
        let mut visited_nodes = Vec::new();
        let start_point = Point {r: start_r as i32, c:num_cols as i32};
        let start_dir = West;

        beam_list.push_back((start_point.clone(), start_dir.clone()));

        while !beam_list.is_empty() {
            let current_light = beam_list.pop_front().unwrap();
            let mut light_position = current_light.0;
            let mut light_direction = current_light.1;

            while !light_out(&move_point(&light_position, light_direction), num_rows, num_cols) &&
                !visited_nodes.contains(&(move_point(&light_position, light_direction), light_direction)) {
                let new_point = move_point(&light_position, light_direction);
                visited_nodes.push((new_point, light_direction));
                let new_entry = contraption[(new_point.r as usize, new_point.c as usize)];
                if new_entry == '.' {
                    // good, continue along
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                } else if new_entry == '|' {
                    if (light_direction == North) | (light_direction == South) {
                        light_position = new_point;
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    } else {
                        light_position = new_point;
                        light_direction = North;
                        beam_list.push_back((new_point, South));
                        beam_list.push_back((new_point, South));
                        beam_list_done.push_back((new_point, South));
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    }
                } else if new_entry == '-' {
                    if (light_direction == East) | (light_direction == West) {
                        light_position = new_point;
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    } else {
                        light_position = new_point;
                        light_direction = East;
                        beam_list.push_back((new_point, West));
                        beam_list_done.push_back((new_point, West));
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    }
                } else if new_entry == '\\' {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                    if light_direction == North {
                        light_direction = West;
                    } else if light_direction == East {
                        light_direction = South;
                    } else if light_direction == South {
                        light_direction = East;
                    } else if light_direction == West {
                        light_direction = North;
                    }
                } else if new_entry == '/' {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                    if light_direction == North {
                        light_direction = East;
                    } else if light_direction == East {
                        light_direction = North;
                    } else if light_direction == South {
                        light_direction = West;
                    } else if light_direction == West {
                        light_direction = South;
                    }
                }
            }
        }
        configuration_energised.push(energised_vec.len());
    }

    for start_c in 0..num_cols {
        let mut energised_vec = Vec::new();
        let mut beam_list = VecDeque::new();
        let mut beam_list_done = VecDeque::new();
        let mut visited_nodes = Vec::new();
        let start_point = Point {c: start_c as i32, r:-1};
        let start_dir = South;

        beam_list.push_back((start_point.clone(), start_dir.clone()));

        while !beam_list.is_empty() {
            let current_light = beam_list.pop_front().unwrap();
            let mut light_position = current_light.0;
            let mut light_direction = current_light.1;

            while !light_out(&move_point(&light_position, light_direction), num_rows, num_cols) &&
                !visited_nodes.contains(&(move_point(&light_position, light_direction), light_direction)) {
                let new_point = move_point(&light_position, light_direction);
                visited_nodes.push((new_point, light_direction));
                let new_entry = contraption[(new_point.r as usize, new_point.c as usize)];
                if new_entry == '.' {
                    // good, continue along
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                } else if new_entry == '|' {
                    if (light_direction == North) | (light_direction == South) {
                        light_position = new_point;
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    } else {
                        light_position = new_point;
                        light_direction = North;
                        beam_list.push_back((new_point, South));
                        beam_list.push_back((new_point, South));
                        beam_list_done.push_back((new_point, South));
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    }
                } else if new_entry == '-' {
                    if (light_direction == East) | (light_direction == West) {
                        light_position = new_point;
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    } else {
                        light_position = new_point;
                        light_direction = East;
                        beam_list.push_back((new_point, West));
                        beam_list_done.push_back((new_point, West));
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    }
                } else if new_entry == '\\' {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                    if light_direction == North {
                        light_direction = West;
                    } else if light_direction == East {
                        light_direction = South;
                    } else if light_direction == South {
                        light_direction = East;
                    } else if light_direction == West {
                        light_direction = North;
                    }
                } else if new_entry == '/' {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                    if light_direction == North {
                        light_direction = East;
                    } else if light_direction == East {
                        light_direction = North;
                    } else if light_direction == South {
                        light_direction = West;
                    } else if light_direction == West {
                        light_direction = South;
                    }
                }
            }
        }
        configuration_energised.push(energised_vec.len());
    }

    for start_c in (0..num_cols).rev() {
        let mut energised_vec = Vec::new();
        let mut beam_list = VecDeque::new();
        let mut beam_list_done = VecDeque::new();
        let mut visited_nodes = Vec::new();
        let start_point = Point {c: start_c as i32, r:num_rows as i32};
        let start_dir = North;

        beam_list.push_back((start_point.clone(), start_dir.clone()));

        while !beam_list.is_empty() {
            let current_light = beam_list.pop_front().unwrap();
            let mut light_position = current_light.0;
            let mut light_direction = current_light.1;

            while !light_out(&move_point(&light_position, light_direction), num_rows, num_cols) &&
                !visited_nodes.contains(&(move_point(&light_position, light_direction), light_direction)) {
                let new_point = move_point(&light_position, light_direction);
                visited_nodes.push((new_point, light_direction));
                let new_entry = contraption[(new_point.r as usize, new_point.c as usize)];
                if new_entry == '.' {
                    // good, continue along
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                } else if new_entry == '|' {
                    if (light_direction == North) | (light_direction == South) {
                        light_position = new_point;
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    } else {
                        light_position = new_point;
                        light_direction = North;
                        beam_list.push_back((new_point, South));
                        beam_list.push_back((new_point, South));
                        beam_list_done.push_back((new_point, South));
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    }
                } else if new_entry == '-' {
                    if (light_direction == East) | (light_direction == West) {
                        light_position = new_point;
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    } else {
                        light_position = new_point;
                        light_direction = East;
                        beam_list.push_back((new_point, West));
                        beam_list_done.push_back((new_point, West));
                        if !energised_vec.contains(&light_position) {
                            energised_vec.push(light_position);
                        }
                    }
                } else if new_entry == '\\' {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                    if light_direction == North {
                        light_direction = West;
                    } else if light_direction == East {
                        light_direction = South;
                    } else if light_direction == South {
                        light_direction = East;
                    } else if light_direction == West {
                        light_direction = North;
                    }
                } else if new_entry == '/' {
                    light_position = new_point;
                    if !energised_vec.contains(&light_position) {
                        energised_vec.push(light_position);
                    }
                    if light_direction == North {
                        light_direction = East;
                    } else if light_direction == East {
                        light_direction = North;
                    } else if light_direction == South {
                        light_direction = West;
                    } else if light_direction == West {
                        light_direction = South;
                    }
                }
            }
        }
        configuration_energised.push(energised_vec.len());
    }

    println!("Part 2: {:?}", configuration_energised.iter().max().unwrap());
    // 8236 too low?
    // 8239 brute force is awesome!
    // I know this would have been easy to parallelise, but maybe another day



    Ok(())
}