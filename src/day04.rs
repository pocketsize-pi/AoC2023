use std::collections::HashMap;
use AoC2023::InputType;

pub fn day04(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 4");




    let data = AoC2023::read_input(4, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }

    // --- Day 4: Scratchcards ---
    // The gondola takes you up. Strangely, though, the ground doesn't seem to be coming with you;
    // you're not climbing a mountain. As the circle of Snow Island recedes below you, an entire new
    // landmass suddenly appears above you! The gondola carries you to the surface of the new island and lurches into the station.

    // As you exit the gondola, the first thing you notice is that the air here is much warmer than
    // it was on Snow Island. It's also quite humid. Is this where the water source is?

    // The next thing you notice is an Elf sitting on the floor across the station in what seems to be a pile of colorful square cards.

    // "Oh! Hello!" The Elf excitedly runs over to you. "How may I be of service?" You ask about water sources.

    // "I'm not sure; I just operate the gondola lift. That does sound like something we'd have,
    // though - this is Island Island, after all! I bet the gardener would know. He's on a different
    // island, though - er, the small kind surrounded by water, not the floating kind. We really need
    // to come up with a better naming scheme. Tell you what: if you can help me with something quick,
    // I'll let you borrow my boat and you can go visit the gardener. I got all these scratchcards as a gift,
    // but I can't figure out what I've won."

    // The Elf leads you over to the pile of colorful cards. There, you discover dozens of scratchcards,
    // all with their opaque covering already scratched off. Picking one up, it looks like each card has
    // two lists of numbers separated by a vertical bar (|): a list of winning numbers and then a list
    // of numbers you have. You organize the information into a table (your puzzle input).

    // As far as the Elf has been able to figure out, you have to figure out which of the numbers
    // you have appear in the list of winning numbers. The first match makes the card worth one point
    // and each match after the first doubles the point value of that card.

    // For example:

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    // In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and eight
    // numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have, four of them
    // (48, 83, 17, and 86) are winning numbers! That means card 1 is worth 8 points (1 for the first
    // match, then doubled three times for each of the three matches after the first).

    // Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
    // Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
    // Card 4 has one winning number (84), so it is worth 1 point.
    // Card 5 has no winning numbers, so it is worth no points.
    // Card 6 has no winning numbers, so it is worth no points.
    // So, in this example, the Elf's pile of scratchcards is worth 13 points.

    // Take a seat in the large pile of colorful cards. How many points are they worth in total?

     //    let mut data = Vec::new();
     // data.push(vec! ["Card", "1:", "41", "48", "83", "86", "17", "|", "83", "86", "6", "31", "17", "9", "48", "53"]);
     // data.push(vec! ["Card", "2:", "13", "32", "20", "16", "61", "|", "61", "30", "68", "82", "17", "32", "24", "19"]);
     // data.push(vec! ["Card", "3:", "1", "21", "53", "59", "44", "|", "69", "82", "63", "72", "16", "21", "14", "1"]);
     // data.push(vec! ["Card", "4:", "41", "92", "73", "84", "69", "|", "59", "84", "76", "51", "58", "5", "54", "83"]);
     // data.push(vec! ["Card", "5:", "87", "83", "26", "28", "32", "|", "88", "30", "70", "12", "93", "22", "82", "36"]);
     // data.push(vec! ["Card", "6:", "31", "18", "13", "56", "72", "|", "74", "77", "10", "23", "35", "67", "36", "11"]);



    let mut new_data : Vec<(Vec<u32>,Vec<u32>)> = Vec::new();
    let mut final_cards = HashMap::new();
    let mut game_id : usize = 0;
    for row in data {
        let mut winning_numbers : Vec<u32> = Vec::new();
        let mut my_numbers : Vec<u32> = Vec::new();
        let mut process_winning = true;
        for j in 2..row.len(){
            if process_winning {
                // winning numbers until we find |
                if row[j] != "|" {
                    // get the number
                    winning_numbers.push(row[j].parse::<u32>().unwrap());
                }
                else {
                    // pipe, so we move to our numbers
                    process_winning = false;
                }
            }
            else {
                my_numbers.push(row[j].parse::<u32>().unwrap());

            }
        }

        new_data.push((winning_numbers, my_numbers));
        final_cards.insert(game_id,1);
        game_id += 1
    }

    // now we loop
    // we could have done it at the same time, but this feels neater
    let mut total_score = 0;

    for i in 0..new_data.len() {
        // println!{"{:?}", final_cards};
        let row = &new_data[i];
        let winning_numbers = &row.0;
        let my_numbers = &row.1;
        let mut winning_entries = 0;

        for number in winning_numbers {
            if my_numbers.contains(&number) {
                winning_entries += 1;
            }
        }
        if winning_entries > 0 {
            total_score += 2_i32.pow(winning_entries - 1);
            for j in (i+1)..(i+1+winning_entries as usize) {
                if j < new_data.len() {
                    final_cards.insert(j, final_cards[&j] + final_cards[&i]);
                }
            }
        }

    }

    println!("Part 1: {}", total_score);
    // 28750 is the correct answer! Thank you rust playground
    let mut final_sum : u32 = 0;
    for val in final_cards.values() {
        final_sum += val;
    }
    println!("Part 2: {}", final_sum);
    // 10212704 yes!! answer! Thank you rust playground


    Ok(())

}