use AoC2023::InputType;

pub fn day00(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 0 - template");




    let mut data = AoC2023::read_input(0, input_type, manual_name)?;

    for row in data {
        println!("{:?}", row);
    }




    Ok(())

}