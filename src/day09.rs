use std::thread::current;
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
        // (input.iter().min() == input.iter().max()) & (input.iter().min().unwrap() == 0)
        input.iter().filter(|&n| *n == 0).count() == input.len()
    }


    fn extrapolate_history (history: &Vec<i64>) -> i64 {
        // find the differences
        let mut differences = Vec::new();
        for i in 0..history.len()-1 {
            differences.push(history[i+1] - history[i]);
        }
        // println!("--------------");
        // println!("differences {:?}", &differences);
        let mut output: i64 = 0;
        if all_zeroes(&differences) {
            // ok, this is good!
            // we add one more entry to the differences row
            // differences.push(differences[0]);
            output = history[0];
            // println!("differences {:?}", &differences);
        }
        else {
            // one level down
            output = extrapolate_history(&differences);
            // println!("return!");
            // println!("output0 {:?}", &output.0);
            // println!("differences {:?}", &differences);
            // println!("last value {:?}", &history[history.len()-1]);
            output = history[history.len()-1] + output;
            // println!("output0 post {:?}", &output.0);
            // println!("output1 {:?}", &output.1);
        }
        output
    }

    let mut sum : i64 = 0;
    for row in oasis_history {
        // println!("{:?}", row);
        // let mut my_row = row.clone();
        // let mut count : u32 = 1;
        let new_output = extrapolate_history(&row);
        // println!("{:?}", &new_output.0);
        // println!("{:?}", &new_output.1);
        // println!("==================");
        sum += new_output;
    }

    println!("Part 1: {}", sum);
    //1757008019


    Ok(())
}