use std::cmp::min;
use std::mem::needs_drop;
use AoC2023::InputType;

pub fn day05(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 5");


    let data = AoC2023::read_input(5, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }

    // --- Day 5: If You Give A Seed A Fertilizer ---
    // You take the boat and find the gardener right where you were told he would be: managing a giant
    // "garden" that looks more to you like a farm.

    // "A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.

    // "Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow
    // with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water
    // a few days... weeks... oh no." His face sinks into a look of horrified realization.

    // "I've been so busy making sure everyone here has food that I completely forgot to check why
    // we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction
    // - it's much faster than your boat. Could you please go check it out?"

    // You barely have time to agree to this request when he brings up another. "While you wait for
    // the ferry, maybe you can help us with our food production problem. The latest Island Island
    // Almanac just arrived and we're having trouble making sense of it."

    // The almanac (your puzzle input) lists all of the seeds that need to be planted. It also
    // lists what type of soil to use with each kind of seed, what type of fertilizer to use with each
    // kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of
    // seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category -
    // that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

    // For example:

    // seeds: 79 14 55 13

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15

    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4

    // water-to-light map:
    // 88 18 7
    // 18 25 70

    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13

    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69

    // humidity-to-location map:
    // 60 56 37
    // 56 93 4
    // The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

    // The rest of the almanac contains a list of maps which describe how to convert numbers from a
    // source category into numbers in a destination category. That is, the section that starts with
    // seed-to-soil map: describes how to convert a seed number (the source) to a soil number
    // (the destination). This lets the gardener and his team know which soil to use with which seeds,
    // which water to use with which fertilizer, and so on.

    // Rather than list every source number and its corresponding destination number one by one,
    // the maps describe entire ranges of numbers that can be converted. Each line within a map
    // contains three numbers: the destination range start, the source range start, and the range length.

    // Consider again the example seed-to-soil map:

    // 50 98 2
    // 52 50 48
    // The first line has a destination range start of 50, a source range start of 98, and a range
    // length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99.
    // The destination range is the same length, but it starts at 50, so its two values are 50 and 51.
    // With this information, you know that seed number 98 corresponds to soil number 50 and that seed
    // number 99 corresponds to soil number 51.

    // The second line means that the source range starts at 50 and contains 48 values: 50, 51, ...,
    // 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52,
    // 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

    // Any source numbers that aren't mapped correspond to the same destination number. So, seed
    // number 10 corresponds to soil number 10.

    // So, the entire list of seed numbers and their corresponding soil numbers looks like this:

    // seed  soil
    // 0     0
    // 1     1
    // ...   ...
    // 48    48
    // 49    49
    // 50    52
    // 51    53
    // ...   ...
    // 96    98
    // 97    99
    // 98    50
    // 99    51
    // With this map, you can look up the soil number required for each initial seed number:

    // Seed number 79 corresponds to soil number 81.
    // Seed number 14 corresponds to soil number 14.
    // Seed number 55 corresponds to soil number 57.
    // Seed number 13 corresponds to soil number 13.
    // The gardener and his team want to get started as soon as possible, so they'd like to know the
    // closest location that needs a seed. Using these maps, find the lowest location number that
    // corresponds to any of the initial seeds. To do this, you'll need to convert each seed number
    // through other categories until you can find its corresponding location number. In this example,
    // the corresponding types are:

    // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
    // Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
    // Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
    // Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
    // So, the lowest location number in this example is 35.

    // What is the lowest location number that corresponds to any of the initial seed numbers?

    fn get_almanac_values (input_data: &Vec<String>) -> ((u64, u64),(u64, u64)) {
        let dest = AoC2023::string_to_u64(&input_data[0]);
        let source = AoC2023::string_to_u64(&input_data[1]);
        let range = AoC2023::string_to_u64(&input_data[2]);
        ((source, source + range), (dest, dest + range))
    }


    let mut seed_numbers : Vec<u64> = Vec::new();
    // array of tuple of tuple (first tuple: source, second tuple: destination)
    let mut seed_to_soil : Vec<((u64, u64),(u64, u64))> = Vec::new();
    let mut soil_to_fert : Vec<((u64, u64),(u64, u64))> = Vec::new();
    let mut fert_to_water : Vec<((u64, u64),(u64, u64))> = Vec::new();
    let mut water_to_light : Vec<((u64, u64),(u64, u64))> = Vec::new();
    let mut light_to_temp : Vec<((u64, u64),(u64, u64))> = Vec::new();
    let mut temp_to_humid : Vec<((u64, u64),(u64, u64))> = Vec::new();
    let mut humid_to_loc : Vec<((u64, u64),(u64, u64))> = Vec::new();

    // function to transform input values to actual infos
    let max_row = data.len();
    let mut i : usize = 0;

    while i<max_row {
        let row = &data[i];

        if row.len() == 0 {
            i += 1;
            continue;
        }
        if row[0].contains("seeds") {
            for j in 1..row.len() {
                seed_numbers.push(AoC2023::string_to_u64(&row[j]));
            }
            i += 1;
            continue;
        }
        if row[0].contains("seed-to-soil") {
            i += 1;
            while data[i].len() != 0 {

                // process seed to soil
                // let source = AoC2023::string_to_u64(&data[i][0]);
                // let dest = AoC2023::string_to_u64(&data[i][1]);
                // let range = AoC2023::string_to_u64(&data[i][2]);
                seed_to_soil.push(get_almanac_values(&data[i]));
                i += 1;
            }
        }
        if row[0].contains("soil-to-fertilizer") {
            i += 1;
            while data[i].len() != 0 {
                soil_to_fert.push(get_almanac_values(&data[i]));
                i += 1;
            }
        }
        if row[0].contains("fertilizer-to-water") {
            i += 1;
            while data[i].len() != 0 {
                fert_to_water.push(get_almanac_values(&data[i]));
                i += 1;
            }
        }
        if row[0].contains("water-to-light") {
            i += 1;
            while data[i].len() != 0 {
                water_to_light.push(get_almanac_values(&data[i]));
                i += 1;
            }
        }
        if row[0].contains("light-to-temperature") {
            i += 1;
            while data[i].len() != 0 {
                light_to_temp.push(get_almanac_values(&data[i]));
                i += 1;
            }
        }
        if row[0].contains("temperature-to-humidity") {
            i += 1;
            while data[i].len() != 0 {
                temp_to_humid.push(get_almanac_values(&data[i]));
                i += 1;
            }
        }
        if row[0].contains("humidity-to-location") {
            i += 1;
            while i < max_row {
                if data[i].len() != 0 {
                    humid_to_loc.push(get_almanac_values(&data[i]));
                    i += 1;
                }
            }
        }

    }
    // println!("{:?}", seed_numbers);
    // println!("{:?}", seed_to_soil);
    // println!("{:?}", soil_to_fert);
    // println!("{:?}", fert_to_water);
    // println!("{:?}", water_to_light);
    // println!("{:?}", light_to_temp);
    // println!("{:?}", temp_to_humid);
    // println!("{:?}", humid_to_loc);


    fn get_next_level_down (input: &Vec<u64>, ranges: &Vec<((u64, u64),(u64, u64))>) -> Vec<u64> {
        let mut output_list: Vec<u64> = Vec::new();
        for seed in input {
            // soil location
            let mut found = false;
            // println!("{:?}", seed);
            for soil in ranges {
                let source = soil.0;
                let dest = soil.1;

                if (source.0..source.1).contains(&seed) {
                    // println!("{:?}", source);
                    // println!("{:?}", dest);
                    // calculate value
                    let diff = seed - source.0;
                    // println!("{:?}", diff);
                    // println!("{:?}", dest.0 + diff);
                    output_list.push(dest.0 + diff);
                    found = true;
                    break;
                }
            }
            if !found {
                output_list.push(*seed);
            }
        }
        output_list
    }

    // we could go recursive, but let's spell it out
    let soil_nums = get_next_level_down(&seed_numbers, &seed_to_soil);
    let fert_nums = get_next_level_down(&soil_nums, &soil_to_fert);
    let water_nums = get_next_level_down(&fert_nums, &fert_to_water);
    let light_nums = get_next_level_down(&water_nums, &water_to_light);
    let temp_nums = get_next_level_down(&light_nums, &light_to_temp);
    let humid_nums = get_next_level_down(&temp_nums, &temp_to_humid);
    let loc_nums = get_next_level_down(&humid_nums, &humid_to_loc);

    println!("Part 1: {:?}", loc_nums.iter().min().unwrap());
    // 199602917

    // --- Part Two ---
    // Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac,
    // it looks like the seeds: line actually describes ranges of seed numbers.

    // The values on the initial seeds: line come in pairs. Within each pair, the first value is
    // the start of the range and the second value is the length of the range. So, in the first
    // line of the example above:

    // seeds: 79 14 55 13
    // This line describes two ranges of seed numbers to be planted in the garden. The first range
    // starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range
    // starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

    // Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

    // In the above example, the lowest location number can be obtained from seed number 82,
    // which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45,
    // humidity 46, and location 46. So, the lowest location number is 46.

    // Consider all of the initial seed numbers listed in the ranges on the first line of the almanac.
    // What is the lowest location number that corresponds to any of the initial seed numbers?

    // Brute force ftw
    // expand seeds

    let mut min_value : u64 = u64::MAX;
    let mut new_seeds : Vec<u64> = Vec::new();

    let min_pain : usize= usize::MAX;

    for i in (0..seed_numbers.len()).step_by(2) {
        let source = seed_numbers[i];
        let range = seed_numbers[i+1];
        new_seeds.push(source);
        new_seeds.push(source + range);

        // for j in 0..range {
        //     let mut new_seeds : Vec<u64> = Vec::new();
        //     new_seeds.push(source + j);
        //
        //     // min_value = min(loc_nums[0],min_value);
        // }
    }
    new_seeds.sort();
    let soil_nums2 = get_next_level_down(&new_seeds, &seed_to_soil);
    let fert_nums2 = get_next_level_down(&soil_nums2, &soil_to_fert);
    let water_nums2 = get_next_level_down(&fert_nums2, &fert_to_water);
    let light_nums2 = get_next_level_down(&water_nums2, &water_to_light);
    let temp_nums2 = get_next_level_down(&light_nums2, &light_to_temp);
    let humid_nums2 = get_next_level_down(&temp_nums2, &temp_to_humid);
    let loc_nums2 = get_next_level_down(&humid_nums2, &humid_to_loc);

    println!("{:?}", new_seeds);
    println!("{:?}", loc_nums2);
    let min_loc = loc_nums2.iter().min().unwrap();
    println!("Part 2: {:?}", min_loc);
    let min_index = loc_nums2.iter().position(|&r| r == *min_loc).unwrap();
    println!("Part 2: {:?}", min_index);

    // we sort them in order, and do two ranges around the seed values that give the smallest edge

    // ok, so I know it's 14, I'm just going to hardcode this rather than do it with code
    let mut less_seeds : Vec<u64> = Vec::new();
    for i in new_seeds[12]..new_seeds[13] {
        less_seeds.push(i);
    }
    for i in new_seeds[14]..new_seeds[15] {
        less_seeds.push(i);
    }
    println!("{:?}", less_seeds.len());

    for seed in less_seeds {
        let soil_nums3 = get_next_level_down(&vec![seed], &seed_to_soil);
        let fert_nums3 = get_next_level_down(&soil_nums3, &soil_to_fert);
        let water_nums3 = get_next_level_down(&fert_nums3, &fert_to_water);
        let light_nums3 = get_next_level_down(&water_nums3, &water_to_light);
        let temp_nums3 = get_next_level_down(&light_nums3, &light_to_temp);
        let humid_nums3 = get_next_level_down(&temp_nums3, &temp_to_humid);
        let loc_nums3 = get_next_level_down(&humid_nums3, &humid_to_loc);
        min_value = min(loc_nums3[0],min_value);
    }



    // println!("{:?}", new_seeds);
    // println!("{:?}", loc_nums2);
    // let min_loc = loc_nums3.iter().min().unwrap();
    // println!("Part 2: {:?}", min_loc);
    // let min_index = loc_nums3.iter().position(|&r| r == *min_loc).unwrap();
    // println!("Part 2: {:?}", min_index);
    println!("Part 2: {:?}", min_value);
    // yes!!!!! 2254686 Brute force saves the day!

    Ok(())
}