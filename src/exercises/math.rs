use std::io;

pub fn fibonacci_sequence() {
    let mut asked_limit: String = String::new();
    println!("How many numbers into the Fibonacci Sequence do you want:");
    io::stdin()
        .read_line(&mut asked_limit)
        .expect("An error was encoutered");
    let asked_limit: u128 = asked_limit.trim().parse().expect("Please enter a number");

    let mut index: u128 = 0;
    let mut calculation: Vec<u128> = vec![1, 0];
    let mut result_string: String = String::new();

    while index < asked_limit {
        let step: u128 = calculation[0] + calculation[1];
        calculation[1] = calculation[0];
        calculation[0] = step;
        index += 1;
        result_string.push_str(&(step.to_string() + ", "));
    }

    println!("Fibonacci Sequence : {}", result_string);
}
