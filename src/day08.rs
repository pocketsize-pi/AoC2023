use std::collections::HashMap;
use AoC2023::{InputType, string_to_chars};

pub fn day08(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 8");


    let data = AoC2023::read_input(8, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }

    let movements = string_to_chars(&data[0][0]);
    let length_movements = movements.len();
    let mut map = HashMap::new();

    for i in 2..data.len() {
        let key = data[i][0].clone();
        let mut val1 = data[i][2].clone();
        val1.pop();
        val1.remove(0);
        let mut val2 = data[i][3].clone();
        val2.pop();
        map.insert(key, (val1, val2));

    }
    // println!("{:?}", movements);
    // println!("{:?}", map);

    let mut current_node = String::new();
    let mut step : u32 = 0;

    current_node = "AAA".parse().unwrap();
    while current_node != "ZZZ" {
        let instructions = map.get(&current_node).unwrap();
        let step_as_index = step % length_movements as u32;
        if movements[step_as_index as usize] == 'L' {
            current_node = instructions.clone().0;
        }
        else { current_node = instructions.clone().1 }

        step += 1;
    }

    println!("Part 1: {}", step);
    // 14257, was easy!

    Ok(())
}