use std::cmp::{max, min};
use std::collections::HashMap;
use AoC2023::*;
use AoC2023::Direction::{East, North, South, West};

pub fn day18(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 18");


    let data = AoC2023::read_input(18, input_type, manual_name)?;

    struct DigInstr {
        dir: Direction,
        length: u32,
        colour: String,
    }


    let mut dig_plan = Vec::new();

    for row in &data {
        // println!("{:?}", row);
        let mut instruction = DigInstr {dir:North, length:0, colour: String::new()};
        // direction
        let udlr = string_to_chars(&row[0]);
        match udlr[0] {
            'U' => instruction.dir = North,
            'D' => instruction.dir = South,
            'L' => instruction.dir = West,
            _=> instruction.dir = East,
        }
        // number
        instruction.length = string_to_u32(&row[1]);
        // colour, unprocessed for now
        instruction.colour = row[2].clone();
        dig_plan.push(instruction);
    }

    let mut dig_actual = Vec::new();
    let mut max_row = 0;
    let mut min_row = 0;
    let mut max_col = 0;
    let mut min_col = 0;
    let mut current_point = Point{r:0, c:0};
    let mut shoelace_vertices = Vec::new();
    dig_actual.push(current_point);
    shoelace_vertices.push(current_point);

    for (i, instruction) in dig_plan.iter().enumerate() {
        for n in 0..instruction.length {
            current_point = move_point(&current_point, instruction.dir);
            if !dig_actual.contains(&current_point)
            { dig_actual.push(current_point); }
        }
        // add to shoelace list
        shoelace_vertices.push(current_point);
        // check limits
        max_row = max(max_row, current_point.r);
        min_row = min(min_row, current_point.r);
        max_col = max(max_col, current_point.c);
        min_col = min(min_col, current_point.c);
    }
    // add original back to list
    // shoelace_vertices.push(Point{r:0, c:0});
    // because it's a cycle, the last one is automatically added

    // println!("{:?}", shoelace_vertices);
    // for ri in min_row..max_row+1 {
    //     let mut contr = String::new();
    //     for ci in min_col..max_col+1 {
    //         // if shoelace_vertices.contains(&Point{r:ri as i32, c:ci as i32}) {
    //         if dig_actual.contains(&Point{r:ri as i32, c:ci as i32}) {
    //             contr.push('#');
    //         }
    //         else {
    //             contr.push('.');
    //         }
    //     }
    //     println!("{}", contr);
    // }
    // println!();

    // for ri in min_row..max_row+1 {
    //     let mut contr = String::new();
    //     for ci in min_col..max_col+1 {
    //         if shoelace_vertices.contains(&Point{r:ri as i32, c:ci as i32}) {
    //             // if dig_actual.contains(&Point{r:ri as i32, c:ci as i32}) {
    //             contr.push('#');
    //         }
    //         else {
    //             contr.push('.');
    //         }
    //     }
    //     println!("{}", contr);
    // }
    // println!();

    // Shoelace
    let mut area = 0;
    let mut area1 = 0;
    let mut area2 = 0;
    for v in 0..(shoelace_vertices.len()-1) {
        let p1 = shoelace_vertices[v];
        let p2 = shoelace_vertices[v+1];
        // println!("p1 {:?}", p1);
        // println!("p2 {:?}", p2);
        area1 += (p1.r * p2.c);
        area2 += (p1.c * p2.r);
    }
    area = (area1 - area2).abs();
    area = area/2;
    area += (dig_actual.len() as i32/2 as i32)+1;

    println!("Part 1: {}", area);
    // 28115 is too low
    // 69698 still too low
    // 70026 finally

    Ok(())
}

pub fn day182(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 18 - part 2");


    let data = AoC2023::read_input(18, input_type, manual_name)?;

    struct DigInstr {
        dir: Direction,
        length: u32,
        colour: String,
    }

    let mut dig_plan = Vec::new();

    for row in &data {
        // println!("{:?}", row);
        let mut instruction = DigInstr { dir: North, length: 0, colour: String::new() };
        // println!("{}", row[2]);
        let mut hex_as_char = string_to_chars(&row[2]);
        // println!("{:?}", hex_as_char);
        // remove brakets
        hex_as_char.remove(0);
        // println!("{:?}", hex_as_char);
        hex_as_char.remove(hex_as_char.len()-1);
        // println!("{:?}", hex_as_char);

        // direction
        let udlr = hex_as_char.remove(hex_as_char.len()-1);
        // println!("{}", udlr);
        match udlr {
            '0' => instruction.dir = East,
            '1' => instruction.dir = South,
            '2' => instruction.dir = West,
            _ => instruction.dir = North,
        }

        // length
        hex_as_char.remove(0); // remove #
        let distance_string : String = hex_as_char.into_iter().collect();
        // println!("string {}", distance_string);
        let z = u32::from_str_radix(distance_string.as_str(), 16).unwrap();
        instruction.length = z;
        // println!("meters {}", z);

        dig_plan.push(instruction);
    }

    let mut dig_actual = Vec::new();
    let mut max_row = 0;
    let mut min_row = 0;
    let mut max_col = 0;
    let mut min_col = 0;
    let mut current_point = Point { r: 0, c: 0 };
    let mut shoelace_vertices = Vec::new();
    dig_actual.push(current_point);
    shoelace_vertices.push(current_point);
    let mut perimeter: i64 = 0;

    for (i, instruction) in dig_plan.iter().enumerate() {
        current_point = move_point_length(&current_point, instruction.dir, &(instruction.length as i32));
        perimeter += instruction.length as i64;
        // add to shoelace list
        shoelace_vertices.push(current_point);
        // check limits
        max_row = max(max_row, current_point.r);
        min_row = min(min_row, current_point.r);
        max_col = max(max_col, current_point.c);
        min_col = min(min_col, current_point.c);
    }

    // Shoelace
    let mut area: i64 = 0;
    let mut area1: i64 = 0;
    let mut area2: i64 = 0;
    for v in 0..(shoelace_vertices.len() - 1) {
        let p1 = shoelace_vertices[v];
        let p2 = shoelace_vertices[v + 1];
        // println!("p1 {:?}", p1);
        // println!("p2 {:?}", p2);
        area1 += p1.r as i64 * p2.c as i64;
        area2 += p1.c as i64 * p2.r as i64;
    }
    area = (area1 - area2).abs();
    area = area / 2;
    area += (perimeter as i64 / 2) + 1;

    println!("Part 2: {}", area);
    // 68548301037382 refreshingly easy

    Ok(())
}