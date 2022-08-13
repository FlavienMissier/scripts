use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut unit: char = 'n';
    let number = match args.get(1) {
        Some(arg1) => match arg1.as_str() {
            "ctf" => match args.get(2) {
                Some(arg2) => match arg2.trim().parse() {
                    Ok(res) => {
                        unit = 'F';
                        Some(celcius_to_fahrenheit(res))
                    },
                    Err(_) => {
                        println!("This command only accepts numbers");
                        None
                    }
                },
                None => {
                    println!("Incorrect syntax");
                    None
                },
            },
            "ftc" => match args.get(2) {
                Some(arg2) => match arg2.trim().parse() {
                    Ok(res) => {
                        unit = 'C';
                        Some(fahrenheit_to_celsius(res))
                    },
                    Err(_) => {
                        println!("This command only accepts numbers");
                        None
                    }
                },
                None => {
                    println!("Incorrect syntax");
                    None
                },
            },
            _ => {
                    println!("Incorrect argument '{}'", arg1);
                    None
                },
        },
        None => {
            println!("Incorrect syntax");
            None
        },
    };
    match number {
        Some(number) => println!("{}{}", number, unit),
        None => println!(),
    }
}
fn celcius_to_fahrenheit(number: f32) -> f32 {
    number * 9.0/5.0 + 32.0
}
fn fahrenheit_to_celsius(number: f32) -> f32 {
    (number - 32.0) * 5.0/9.0
}