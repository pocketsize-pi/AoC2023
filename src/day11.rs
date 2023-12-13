use std::collections::HashMap;
use std::thread::current;
use AoC2023::*;
use combinatorial::Combinations;

pub fn day11(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 11");


    let data = AoC2023::read_input(11, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }


    let mut star_map_vec = Vec::new();
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

pub fn day112(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 11 part 2");


    let data = AoC2023::read_input(11, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }


    let mut star_map_vec = Vec::new();

    for i  in 0..data.len() {
        // let row = &data[i];
        let my_row = string_to_chars(&data[i][0]);

        star_map_vec.push(my_row.clone()); // cloning here to keep it afterwards

    }
    // println!();
    let height = star_map_vec.len();
    let width = star_map_vec[0].len();

    // ok, now I have two star maps (one right way up, the other flipped)
    // let original_height = data.len();
    // let mut aged_star_map1 = HashMap::new();
    let mut aged_star_vec_by_col : HashMap <i32,Vec<Point>>= HashMap ::new();
    // insert rows
    let mut local_row_id = 0;
    let current_age = 1000000;
    let mut current_galaxy = 0;
    // let mut map_galaxies = Vec::new();
    for r_i in 0..star_map_vec.len() {
        let row = &star_map_vec[r_i];
        if row.iter().filter(|&n| *n == '.').count() == row.len() {
            // row is all equal, let's add another one
            local_row_id += current_age;
        }
        else {
            for c_i in 0..row.len() {
                let col = row[c_i];
                let c32 = c_i as i32;
                // lets only store the actual galaxies
                if col == '#' {
                    // aged_star_vec_by_col.insert(Point{r:local_row_id as i32, c:c_i as i32}, c);
                    if !aged_star_vec_by_col.contains_key(&c32){
                        aged_star_vec_by_col.insert(c32, vec![Point{r:local_row_id as i32, c:c_i as i32}]);
                        current_galaxy += 1;

                    }
                    else {
                        let mut temp = aged_star_vec_by_col.get(&c32).unwrap().clone();
                        temp.push(Point{r:local_row_id as i32, c:c_i as i32});
                        // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut
                        let new_value = aged_star_vec_by_col.get_mut(&c32).unwrap();
                        *new_value = temp;
                        current_galaxy += 1;
                    }
                }
            }
            local_row_id +=1;
        }
    }

    // ok, now we need to insert the cols
    let mut actual_galaxy_list = Vec::new();
    let mut local_col_id = 0;

    for c_i in 0..width as i32 {
        if !aged_star_vec_by_col.contains_key(&c_i) {
            local_col_id += current_age;
        }
        else {
            for values in aged_star_vec_by_col.get(&c_i) {
                for value in values {
                    actual_galaxy_list.push(Point {r: value.r, c:local_col_id});
                    // galaxy_map.insert(galaxy_num, Point {r: value.r, c:local_col_id});
                }

            }
            local_col_id += 1;
        }
    }

    let galaxy_pairs_num = Combinations::of_size(0..actual_galaxy_list.len(), 2).count();;
    let mut total_distance : i64 = 0;
    let galaxy_pairs = Combinations::of_size(0..actual_galaxy_list.len(), 2);
    for pair_id in galaxy_pairs {
        total_distance += manhattan_distance(&actual_galaxy_list[pair_id[0]],&actual_galaxy_list[pair_id[1]]) as i64;

    }
    println!("{:?}", total_distance);
    // 678728808158
    // blimey, easier than expected also

    Ok(())
}