use std::io;

fn main() {
    println!("Type 0 if you want to convert celsius to fahrenheit, or 1 to convert fahrenheit to celsius: ");

    let conversion_type: f32 = read_terminal();

    if conversion_type == 0.0 {
        println!("Converting from celsius to fahrenheit");
        println!("Please enter number of celsius degrees: ");
        let celsius_temperature = read_terminal();
        let fahrenheit_temperature = celsius_to_fahrenheit(celsius_temperature);
        println!("{} degrees in celsius is the same as {} degrees in fahrenheit", celsius_temperature, fahrenheit_temperature);
    } else if conversion_type == 1.0 {
        println!("Converting from fahrenheit to celsius");
        println!("Please enter number of fahrenheit degrees: ");
        let fahrenheit_temperature = read_terminal();
        let celsius_temperature = fahrenheit_to_celsius(fahrenheit_temperature);
        println!("{} degrees in fahrenheit is the same as {} degrees in celsius", fahrenheit_temperature, celsius_temperature);
    } else {
        println!("You have to choose between zero and one");
    }

}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}


fn read_terminal() -> f32 {
    loop {
        let mut user_guess = String::new();
        println!("Enter a number!: ");

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to Read Line");

        let _user_guess: f32 = match user_guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}
