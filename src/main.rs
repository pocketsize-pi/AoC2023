use std::env;

pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
// pub mod day10;
pub mod day11;
pub mod day13;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, Advent of Code!");

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        // No choice
        println!("Try again with a day")
    }
    else {

        if args[0].parse::<u8>().is_ok() {
            let day = args[0].parse::<u8>().unwrap();
            let mut input_type = AoC2023::InputType::Sample;
            let mut manual_name = "not_used.txt";

            if args.len() > 1 {
                if args[1] == "s" { input_type = AoC2023::InputType::Sample; }
                else if args[1] == "d" { input_type = AoC2023::InputType::Data; }
                else if args[1].contains(".") {
                    input_type = AoC2023::InputType::Manual;
                    manual_name = args[1].as_str();
                }
            }


            match day {
                1 => day01::day01()?,
                2 => day02::day02(input_type, manual_name)?,
                3 => day03::day03(input_type, manual_name)?,
                4 => day04::day04(input_type, manual_name)?,
                5 => day05::day05(input_type, manual_name)?,
                6 => day06::day06(input_type, manual_name)?,
                7 => day07::day07(input_type, manual_name)?,
                72=> day07::day72(input_type, manual_name)?,
                8 => day08::day08(input_type, manual_name)?,
                9 => day09::day09(input_type, manual_name)?,
                // 10=> day10::day10(input_type, manual_name)?,
                11=> day11::day11(input_type, manual_name)?,
                112=> day11::day112(input_type, manual_name)?,
                13 => day13::day13(input_type, manual_name)?,
                _others => day00::day00(input_type, manual_name)?}
        }
        else {
            println!("Not a number, try again")
        }
    }

    Ok(())
}
