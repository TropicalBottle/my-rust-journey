use std::io;

#[allow(dead_code)]
pub fn temperature_converter() -> f32 {
    let mut asked_temperature: String = String::new();
    let mut asked_unit: String = String::new();

    println!("Do you want to convert to Celsius (c) or Fahrenheit (f): ");
    io::stdin()
        .read_line(&mut asked_unit)
        .expect("An error was encoutered");
    println!("What temperature do you want to convert: ");
    io::stdin()
        .read_line(&mut asked_temperature)
        .expect("An error was encoutered");

    //TODO: I can crash the program if I enter a value that is like a dot
    let temperature: f32 = asked_temperature
        .trim()
        .parse()
        .expect("Please enter a number");
    if asked_unit.trim() == "c" {
        (temperature - 32.0) * (5.0 / 9.0)
    } else {
        (temperature * (9.0 / 5.0)) + 32.0
    }
}
