use std::env;

pub mod day01;

fn main() {
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
                1 => day01::day01(),
                _others => println!("This day doesn't exist yet")}
        }
        else {
            println!("Not a number, try again")
        }
    }

}
