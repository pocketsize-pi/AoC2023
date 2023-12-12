use std::collections::HashMap;
// use itertools::Combinations;
use AoC2023::*;
use combinatorial::Combinations;

pub fn day11(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 11");


    let data = AoC2023::read_input(11, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }


    let mut star_map_vec = Vec::new();
    let mut start_coords = Point { c: 0, r: 0}; // as row, column, so j, i
    for i  in 0..data.len() {
        // let row = &data[i];
        let my_row = string_to_chars(&data[i][0]);

        star_map_vec.push(my_row.clone()); // cloning here to keep it afterwards
        if my_row.iter().filter(|&n| *n == '.').count() == my_row.len() {
            // row is all equal, let's add another one
            star_map_vec.push(my_row);
        }
    }
    // println!();
    // for row in &star_map_vec {
    //     let s: String = row.into_iter().collect();
    //     println!("{:?}", s);
    // }
    // flip
    let height = star_map_vec.len();
    let width = star_map_vec[0].len();
    let mut flipped_map_vec = Vec::new();
    for c in 0..width {
        let mut flipped = Vec::new();
        for r in 0..height {
            flipped.push(star_map_vec[r][c]);
        }
        flipped_map_vec.push(flipped.clone());

        // if flipped.iter().filter(|&n| *n == '.').count() == flipped.len() {
        //     flipped_map_vec.push(flipped);
        // }
    }

    // println!();
    // for row in &flipped_map_vec {
    //     let s: String = row.into_iter().collect();
    //     println!("{:?}", s);
    // }


    // add
    let mut flipped_map_vec_expand = Vec::new();
    for i  in 0..flipped_map_vec.len() {
        // let row = &data[i];
        let my_row = flipped_map_vec[i].clone();

        flipped_map_vec_expand.push(my_row.clone()); // cloning here to keep it afterwards
        if my_row.iter().filter(|&n| *n == '.').count() == my_row.len() {
            // row is all equal, let's add another one
            flipped_map_vec_expand.push(my_row);
        }
    }


    // println!();
    // for row in &flipped_map_vec_expand {
    //     let s: String = row.into_iter().collect();
    //     println!("{:?}", s);
    // }

    let mut starmap = HashMap::new();
    for c in 0..flipped_map_vec_expand.len() {
        for r in 0..flipped_map_vec_expand[0].len() {
            // println!("{} {}", c, r);
            starmap.insert(Point{r: r as i32, c: c as i32}, flipped_map_vec_expand[c][r]);
        }
    }

    // println!();
    // println!("{:?}", starmap);

    let height = flipped_map_vec_expand[0].len();
    let width = flipped_map_vec_expand.len();
    for r in 0..height {
        let mut display_row = String::new();
        for c in 0..width {
            // println!("{} {}", c, r);
            display_row.push(starmap.get(&Point { r: r as i32, c: c as i32 }).unwrap().clone());
            // println!("{:?}", &display_row);
        }
        // println!("{:?}", display_row);
    }

    let mut galaxies_by_point = HashMap::new();
    let mut galaxies_by_num = HashMap::new();
    let mut num_galaxies = 0;

    for (key, value) in starmap {
        if value == '#' {
            num_galaxies += 1;
            galaxies_by_point.insert(key, num_galaxies);
            galaxies_by_num.insert(num_galaxies, key);
        }
    }

    let galaxy_pairs_num = Combinations::of_size(1..num_galaxies+1, 2).count();
    // println!("{:?}", galaxy_pairs.count());
    let mut total_distance = 0;
    let galaxy_pairs = Combinations::of_size(1..num_galaxies+1, 2);
    for pair in galaxy_pairs {
        // println!("{:?}", pair);
        total_distance += manhattan_distance(&galaxies_by_num.get(&pair[0]).unwrap(),&galaxies_by_num.get( &pair[1]).unwrap());

    }
    println!("{:?}", total_distance);
    // 10165598
    // blimey, easier than expected

    Ok(())
}