use AoC2023::*;

pub fn day15(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 15");


    let data = AoC2023::read_input(15, input_type, manual_name)?;

    let mut instructions: Vec<Vec<char>> = Vec::new();
    for row in &data {
        // println!("{:?}", row);

        let inst: Vec<String> = row[0].split(",")
            .map(|s| s.to_string())
            .collect();
        for i in inst {
            instructions.push(string_to_chars(&i));
        }

    }

    // println!("{:?}", instructions);

    fn hash (mut value: u32, ch: char) -> u32 {
        // add ASCII
        value += ch as u32;
        //mult by 17
        value *= 17;
        //remainder by 256
        value %= 256;
        value
    }

    let mut total_hash : u32 = 0;
    let mut current_hash : u32 = 0;

    for instr in instructions {
        current_hash = 0;
        for ch in instr {
            current_hash = hash(current_hash, ch);
        }
        total_hash += current_hash;
    }

    println!("Part 1 {}", total_hash);
    // 497373 oh yes, first time running!
    Ok(())
}