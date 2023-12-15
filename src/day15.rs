use std::collections::HashMap;
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

    fn hash (mut value: u32, ch: &char) -> u32 {
        // add ASCII
        value += *ch as u32;
        //mult by 17
        value *= 17;
        //remainder by 256
        value %= 256;
        value
    }

    let mut total_hash : u32 = 0;
    let mut current_hash : u32 = 0;

    for instr in &instructions {
        current_hash = 0;
        for ch in instr {
            current_hash = hash(current_hash, ch);
        }
        total_hash += current_hash;
    }

    println!("Part 1 {}", total_hash);
    // 497373 oh yes, first time running!

    // ok, part 2 is convoluted but not terrible
    let mut lenses_hashmap: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

    for instr in &instructions {
        // println!("instruction: {:?}", instr);
        let mut box_num = 0;
        let mut i: usize = 0;
        let mut label= String::new();
        for c in 0..instr.len()  {
            // println!("{}", instr[c]);
            if (instr[c] == '=') | (instr[c] == '-') {
                i = c;
                break;
            }
            box_num = hash(box_num, &instr[c]);
            label.push(instr[c]);

        }

        // println!("box number {}", box_num);
        // println!("label {}", label);
        // next instruction is the operation
        // -
        if instr[i] == '-' {
            if lenses_hashmap.contains_key(&box_num) {
                let box_contents= lenses_hashmap.get_mut(&box_num).unwrap();
                let mut index = 0;
                let mut found = false;
                for v in &mut *box_contents {
                    if v.0 == label {
                        found = true;
                        break;
                    }
                    index += 1;
                }
                if found {
                    box_contents.remove(index);
                }

            }
        }
        if instr[i]== '=' {
            // get focal length
            let mut fl = String::new();
            for l in i+1..instr.len() {
                fl.push(instr[l]);
            }
            let focal_length = string_to_u32(&fl);

            if lenses_hashmap.contains_key(&box_num) {
                let box_contents= lenses_hashmap.get_mut(&box_num).unwrap();
                let mut found = false;
                for v in &mut *box_contents {
                    if v.0 == label {
                        // replace focal length
                        v.1 = focal_length;
                        found = true;
                        break;
                    }
                }
                // otherwise add it
                if !found {
                    box_contents.push((label, focal_length));
                }
            }
            else {
                // first time in box, add
                let start_lens = vec![(label, focal_length)];
                lenses_hashmap.insert(box_num, start_lens);
            }
        }
    }

    // add the total power
    let mut total_power = 0;
    for (key, value) in &lenses_hashmap {
        if !value.is_empty() {
            for (i, v) in value.into_iter().enumerate() {
                let mut local_power = 1;
                local_power *= (key+1);

                local_power *= (i as u32 +1);
                local_power *= v.1;
                // println!("{}", local_power);
                total_power += local_power;
            }

        }


    }

    // println!("{:?}", lenses_hashmap);
    println!("Part 2: {}", total_power);
    //259356 surprisingly, that was easy, just a bit convoluted


    Ok(())

}