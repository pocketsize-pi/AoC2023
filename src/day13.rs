use std::cmp::min;
use std::collections::HashMap;
use AoC2023::*;
use ndarray::{Array2, Axis};
// use array2d::{Array2D, Error};

pub fn day13(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 13");


    let data = AoC2023::read_input(13, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }

    // let's try ndarrays
    let mut full_field : Vec<Array2<char>> = Vec::new();
    let mut full_field_pt : Vec<Array2<char>> = Vec::new();
    let mut temp_field : Vec<Vec<char>> = Vec::new();
    let mut rows_in_field : usize = 0;
    let mut cols_in_field : usize = 0;
    for i in 0..data.len() {
        // println!("{:?}", row);
        let row: &Vec<String> = &data[i];
        if row.is_empty() | (i==(data.len()-1)) {
            let n_cols = data.len();
            // if last, let's get the data and not lose it
            if i == data.len()-1 {
                temp_field.push(string_to_chars(&row[0]));
                rows_in_field += 1;
                cols_in_field = row[0].len();
            }

            //empty row, close one field and start the next
            let mut field_2d = Array2::<char>::default((rows_in_field, cols_in_field));
            for (i, mut row) in field_2d.axis_iter_mut(Axis(0)).enumerate() {
                for (j, col) in row.iter_mut().enumerate() {
                    *col = temp_field[i][j];
                }
            }

            full_field.push(field_2d.clone());
            full_field_pt.push(field_2d.clone());
            temp_field = Vec::new();
            rows_in_field = 0;
            cols_in_field = 0;
        }
        else {
            temp_field.push(string_to_chars(&row[0]));
            rows_in_field += 1;
            cols_in_field = row[0].len();
        }
    }

    // check each field, by row then by column
    let mut total_score : u64 = 0;
    let mut total_score2 = 0;
    let mut original_row = 0;
    let mut original_col = 0;
    // need to keep track of originals so that we can ignore them later!
    for (i, field) in full_field.iter_mut().enumerate() {
        println!("{:?}", field);

        // check symmetry along a row line
        let rows = field.nrows();
        for r_axis in 0..(rows-1) {
            // calculate symmetry range
            let offset = min(r_axis+1, (rows-r_axis-1));
            // println!(" r axis {}, range {}", r_axis, offset);
            let pre_range : Vec<usize> = ((r_axis - (offset -1))..r_axis+1).rev().collect();
            let post_range : Vec<usize> = (r_axis+1..(r_axis + offset+1)).collect();
            // println!("r ranges {:?} {:?}", pre_range, post_range);
            // so we extract the two ranges as rows, and check they are equal
            let mut equal = true;
            for r in 0..offset {
                let r1 = &field.row(pre_range[r]);
                let r2 = &field.row(post_range[r]);
                if r1 != r2 {
                    equal = false;
                    break;
                }
            }
            if equal {
                total_score += (100 * (r_axis as u64 +1));
                original_row = r_axis;
                // println!("r axis {}", r_axis);
            }

        }
        // same for col
        let cols = field.ncols();

        for c_axis in 0..(cols-1) {
            // calculate symmetry range
            let offset = min(c_axis+1, (cols-c_axis-1));
            // println!("c axis {}, range {}", c_axis, offset);
            let pre_range : Vec<usize> = ((c_axis - (offset -1))..c_axis+1).rev().collect();
            let post_range : Vec<usize> = (c_axis+1..(c_axis + offset+1)).collect();
            // println!("c ranges {:?} {:?}", pre_range, post_range);
            // so we extract the two ranges as rows, and check they are equal
            let mut equal = true;
            for c in 0..offset {
                let c1 = &field.column(pre_range[c]);
                let c2 = &field.column(post_range[c]);
                if c1 != c2 {
                    equal = false;
                    break;
                }
            }
            if equal {
                total_score += c_axis as u64 +1;
                original_col = c_axis;
                // println!("c axis {}", c_axis);
            }
        }

        // PART 2
        // all in the same loop so that we don't have issues with borrowing muts
        let mut new_found = false;
        for r_hack in 0..rows {
            for c_hack in 0..cols {
                // replace
                if field[(r_hack, c_hack)] == '.' {
                    field[(r_hack, c_hack)] = '#';
                }
                else {field[(r_hack, c_hack)] = '.';}
                // then run
                for r_axis in 0..(rows-1) {
                    // we skip old symmetry line
                    if r_axis != original_row {

                        // calculate symmetry range
                        let offset = min(r_axis + 1, (rows - r_axis - 1));
                        // println!(" r axis {}, range {}", r_axis, offset);
                        let pre_range: Vec<usize> = ((r_axis - (offset - 1))..r_axis + 1).rev().collect();
                        let post_range: Vec<usize> = (r_axis + 1..(r_axis + offset + 1)).collect();
                        // println!("r ranges {:?} {:?}", pre_range, post_range);
                        // so we extract the two ranges as rows, and check they are equal
                        let mut equal = true;
                        for r in 0..offset {
                            let r1 = field.row(pre_range[r]);
                            let r2 = field.row(post_range[r]);
                            if r1 != r2 {
                                equal = false;
                                break;
                            }
                            // else {
                            //     println!("r1 {:?}", r1);
                            //     println!("r2 {:?}", r2);
                            // }
                        }
                        if equal {
                            total_score2 += (100 * (r_axis as u64 + 1));
                            println!("r hack {}, c hack {}", r_hack, c_hack);
                            println!("r axis {}", r_axis);
                            new_found = true;
                            break;
                        }
                    }
                }
                // replace back
                if field[(r_hack, c_hack)] == '.' {
                    field[(r_hack, c_hack)] = '#';
                }
                else {field[(r_hack, c_hack)] = '.';}
                if new_found {break;}
            }
            if new_found {break;}
        }

        if !new_found {
            new_found = false;
            for r_hack in 0..rows {
                for c_hack in 0..cols {
                    // replace
                    if field[(r_hack, c_hack)] == '.' {
                        field[(r_hack, c_hack)] = '#';
                    } else { field[(r_hack, c_hack)] = '.'; }

                    for c_axis in 0..(cols - 1) {
                        // and again skip old
                        if c_axis != original_col {
                            // calculate symmetry range
                            let offset = min(c_axis + 1, (cols - c_axis - 1));
                            // println!("c axis {}, range {}", c_axis, offset);
                            // println!("range opt1 {}, opt2 {}", c_axis + 1, (cols - c_axis - 1));
                            let pre_range: Vec<usize> = ((c_axis - (offset - 1))..c_axis + 1).rev().collect();
                            let post_range: Vec<usize> = (c_axis + 1..(c_axis + offset + 1)).collect();
                            // println!("c ranges {:?} {:?}", pre_range, post_range);
                            // so we extract the two ranges as rows, and check they are equal
                            let mut equal = true;
                            // println!("f2");
                            for c in 0..offset {
                                // println!("{c}");
                                // println!("{}", pre_range[c]);
                                // println!("{}", post_range[c]);
                                let c1 = field.column(pre_range[c]);
                                let c2 = field.column(post_range[c]);
                                if c1 != c2 {
                                    equal = false;
                                    // println!("c1 {:?}", c1);
                                    // println!("c2 {:?}", c2);
                                    break;
                                }
                                // else {
                                //     println!("c1 {:?}", c1);
                                //     println!("c2 {:?}", c2);
                                // }
                            }
                            if equal {
                                total_score2 += c_axis as u64 + 1;
                                // println!("c1 {:?}", c1);
                                // println!("c2 {:?}", c2);
                                println!("r hack {}, c hack {}", r_hack, c_hack);
                                println!("c axis {}", c_axis);
                                new_found = true;
                                break;
                            }
                        }
                    }
                    // replace back
                    if field[(r_hack, c_hack)] == '.' {
                        field[(r_hack, c_hack)] = '#';
                    } else { field[(r_hack, c_hack)] = '.'; }
                    if new_found { break; }
                }
                if new_found { break; }
            }
        }

    }
    println!("Part 1: {}", total_score);
    // yes! 36015

    println!("Part 2: {}", total_score2);
    // 48113 too high
    // 34508 too low
    // 35308 too low






    Ok(())
}