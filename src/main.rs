use std::env;

pub mod day00;
pub mod day01;

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

            match day {
                1 => day01::day01()?,
                _others => day00::day00()?}
        }
        else {
            println!("Not a number, try again")
        }
    }

    Ok(())
}
