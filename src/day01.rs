use AoC2023::InputType;

pub fn day01() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 1");

    let mut data = AoC2023::read_input(1, InputType::Data)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }


    // --- Day 1: Trebuchet?! ---
    //     Something is wrong with global snow production, and you've been selected to take a look.
    // The Elves have even given you a map; on it, they've used stars to mark the top fifty locations
    // that are likely to be having problems.
    //
    //     You've been doing this long enough to know that to restore snow operations, you need to
    // check all fifty stars by December 25th.
    //
    // Collect stars by solving puzzles. Two puzzles will be made available on each day in the
    // Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
    //
    // You try to ask why they can't just use a weather machine ("not powerful enough") and
    // where they're even sending you ("the sky") and why your map looks mostly blank ("you sure
    // ask a lot of questions") and hang on did you just say the sky ("of course, where do you
    // think snow comes from") when you realize that the Elves are already loading you into a
    // trebuchet ("please hold still, we need to strap you in").
    //
    //     As they're making the final adjustments, they discover that their calibration document
    // (your puzzle input) has been amended by a very young Elf who was apparently just excited to
    // show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
    //
    //     The newly-improved calibration document consists of lines of text; each line originally
    // contained a specific calibration value that the Elves now need to recover. On each line,
    // the calibration value can be found by combining the first digit and the last digit (in that
    // order) to form a single two-digit number.
    //
    //     For example:
    //
    // 1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet
    // In this example, the calibration values of these four lines are 12, 38, 15, and 77.
    // Adding these together produces 142.
    //
    // Consider your entire calibration document. What is the sum of all of the calibration values?

    let mut total : u32 = 0;
    for row in &data {
        let mut first_found =false;
        let mut first: char = '0';
        let mut last: char = '0';
        let mut num_string = String::new();
        for i in row[0].chars() {
            // println!("{}",i);
            if i.is_digit(10){
                // always set last: makes the compiler happy, and deals with the treb7uchet case
                last =  i;
                if !first_found {
                    first = i;

                    first_found = true;
                }
            }
        }
        // println!("{} {}", &first, &last);
        num_string.push(first);
        num_string.push(last);
        // println!("{}", &num_string);
        total += num_string.parse::<u32>().unwrap();
        // println!("{}", &total);
    }
    println!("{}", &total);
    // Part 1: 55090

    // --- Part Two ---
    //     Your calculation isn't quite right. It looks like some of the digits are actually spelled
    // out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
    //
    //     Equipped with this new information, you now need to find the real first and last digit on each line. For example:
    //
    //     two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen
    // In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
    //
    // What is the sum of all of the calibration values?
    let text_numbers = vec!("one", "two", "three", "four", "five", "six", "seven", "eight", "nine");
    total = 0;
    for row in &data {
        // println!("{:?}", row);
        let mut first_found =false;
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        let mut build_string = String::new();

        // ah, you dumdum: eightwo = 82, and we are not taking that into account!
        // ok, so we go forwards, then backwards

        // fowards
        // println!("forwards");
        for i in row[0].chars() {
            // println!("{}",i);
            // println!("{}", &build_string);
            if i.is_digit(10){
                first = i.to_digit(10).unwrap();
                break;
            }
            else {
                // ok, not a number, so let's build a string
                build_string.push(i);
                // yay, brute force
                let mut j: u32 = 0;
                for n in text_numbers.iter() {
                    j += 1;
                    if build_string.contains(n) {
                        first  = j;
                        first_found = true;
                        // println!("found text");
                        break;
                    }
                }
                if first_found { break;}; // exit outer loop too
            }
        }
        // println!("first found: {}", first);

        // backwards
        // println!("backwards");
        build_string = String::default();
        first_found = false;
        for i in row[0].chars().rev() {
            // println!("{}",i);
            // println!("{}", &build_string);
            if i.is_digit(10){
                last = i.to_digit(10).unwrap();
                break;
            }
            else {
                // ok, not a number, so let's build a string
                build_string.insert(0,i);
                // yay, brute force
                let mut j: u32 = 0;
                for n in text_numbers.iter() {
                    j += 1;
                    if build_string.contains(n) {
                        last  = j;
                        // println!("found text");
                        first_found = true;
                        break;
                    }
                }
                if first_found { break;}; // exit outer loop too
            }
        }
        // println!("last found: {}", last);


        // println!("{} {}", &first, &last);
        // num_string.push(first);
        // num_string.push(last);
        total += (first * 10) + last;
        // println!("{}", &total);
    }
    println!("{}", &total);

    // 55214 is too high


    Ok(())


}