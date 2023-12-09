use std::collections::HashMap;
use std::thread::current;
use itertools::min;
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

    let mut list_of_as = Vec::new();

    for i in 2..data.len() {
        let key = data[i][0].clone();
        let mut val1 = data[i][2].clone();
        val1.pop();
        val1.remove(0);
        let mut val2 = data[i][3].clone();
        val2.pop();
        map.insert(key.clone(), (val1, val2));

        // create the list of vectors that end in A
        // this is a case for paralelisation if I ever saw one
        if string_to_chars(&key)[2] == 'A' {
            list_of_as.push(key);
        }

    }
    println!("{:?}", list_of_as.len());
    println!("{:?}", list_of_as);
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
    //14257, was easy!

    step = 0;

    // aha! maths!
    let mut min_common = [0; 6];

    for i in 0..list_of_as.len() {
        current_node = list_of_as[i].clone();
        step = 0;
        while string_to_chars(&current_node)[2] != 'Z' {
            let instructions = map.get(&current_node).unwrap();
            let step_as_index = step % length_movements as u32;
            if movements[step_as_index as usize] == 'L' {
                current_node = instructions.clone().0;
            }
            else { current_node = instructions.clone().1 }

            step += 1;
        }
        min_common[i] = step as u64;
    }

    fn lcm(nums: &[u64]) -> u64 {
        if nums.len() == 1 {
            return nums[0];
        }
        let a = nums[0];
        let b = lcm(&nums[1..]);
        a * b / gcd_of_two_numbers(a, b)
    }

    fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
        if b == 0 {
            return a;
        }
        gcd_of_two_numbers(b, a % b)
    }
    println!("Part 2: {:?}", &min_common);
    println!("Part 2: {}", lcm(&min_common));
    // 16187743689077 oh yeah baby!

    Ok(())
}