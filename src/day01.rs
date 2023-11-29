use AoC2023::InputType;

pub fn day01() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 1");

    let mut data = AoC2023::read_input(1, InputType::Sample)?;

    for row in data {
        println!("{:?}", row);
    }

    Ok(())


}