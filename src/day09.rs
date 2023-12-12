use AoC2023::InputType;
use AoC2023::string_to_i64;

pub fn day09(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 9");


    let data = AoC2023::read_input(9, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }

    let mut oasis_history = Vec::new();
    for row in &data {
        let mut hist = Vec::new();
        for r in row {hist.push(string_to_i64(r));}
        oasis_history.push(hist);
    }

    fn all_zeroes (input: &Vec<i64>) -> bool {
        input.iter().filter(|&n| *n == 0).count() == input.len()
    }


    fn extrapolate_history (history: &Vec<i64>) -> i64 {
        // find the differences
        let mut differences = Vec::new();
        for i in 0..history.len()-1 {
            differences.push(history[i+1] - history[i]);
        }

        let mut output: i64 = 0;
        if all_zeroes(&differences) {
            // ok, this is good!
            output = history[0];
        }
        else {
            // one level down
            output = extrapolate_history(&differences);
            output = history[history.len()-1] + output;
        }
        output
    }

    let mut sum : i64 = 0;
    for row in &oasis_history {
        let new_output = extrapolate_history(&row);
        sum += new_output;
    }

    println!("Part 1: {}", sum);
    //1757008019

    sum = 0;
    for i in 0..oasis_history.len(){
        let mut row = oasis_history[i].clone();
        row.reverse();
        let new_output = extrapolate_history(&row);
        sum += new_output;
    }
    println!("Part 2: {}", sum);
    // 995 - well, that was disturbingly easy!

    Ok(())
}