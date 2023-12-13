use std::cmp::min;
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
    // let mut full_field : Vec<Array2D<char>> = Vec::new();
    let mut full_field : Vec<Array2<char>> = Vec::new();
    let mut temp_field : Vec<Vec<char>> = Vec::new();
    let mut rows_in_field : usize = 0;
    let mut cols_in_field : usize = 0;
    for i in 0..data.len() {
        // println!("{:?}", row);
        let row: &Vec<String> = &data[i];
        if row.is_empty() | (i==(data.len()-1)) {
            let n_cols = data.len();
            //empty row, close one field and start the next
            // let field_2d = Array2D::from_rows(&temp_field)?;
            let mut field_2d = Array2::<char>::default((rows_in_field, cols_in_field));
                // Array2::from_shape_vec((rows_in_field, cols_in_field), temp_field)?;
            for (i, mut row) in field_2d.axis_iter_mut(Axis(0)).enumerate() {
                for (j, col) in row.iter_mut().enumerate() {
                    *col = temp_field[i][j];
                }
            }

            full_field.push(field_2d);
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
    for field in full_field {
        // println!("{:?}", field);

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
                let r1 = field.row(pre_range[r]);
                let r2 = field.row(post_range[r]);
                if r1 != r2 {
                    equal = false;
                    // break;
                }
            }
            if equal {
                total_score += (100 * (r_axis as u64 +1));
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
                let c1 = field.column(pre_range[c]);
                let c2 = field.column(post_range[c]);
                if c1 != c2 {
                    equal = false;
                    // break;
                }
            }
            if equal {
                total_score += c_axis as u64 +1;
                // println!("c axis {}", c_axis);
            }
            // println!("Part 1: {}", total_score);
        }
    }
    println!("Part 1: {}", total_score);
    // yes! 36015

    Ok(())
}