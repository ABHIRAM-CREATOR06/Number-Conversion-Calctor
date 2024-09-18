use std::io;

pub fn celcius() {
    println!("Enter temperature in Celsius:");
    let mut celcius = String::new();
    io::stdin().read_line(&mut celcius).expect("Failed to read line");
    
    let temp: f32 = celcius.trim().parse().expect("Error in parsing the floating number");

    // Convert Celsius to Fahrenheit
    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("{}°C is equal to {}°F", temp, fahrenheit);

    // Convert Celsius to Kelvin
    let kelvin = 273.15 + temp;
    println!("{}°C is equal to {}°K", temp, kelvin);

    // Convert Celsius to Rankine
    let rankine = temp * (9.0 / 5.0) + 491.67;
    println!("{}°C is equal to {}°R", temp, rankine);
}

pub fn fahrenheit() {
    println!("Enter temperature in Fahrenheit:");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

    let temp: f32 = fahrenheit.trim().parse().expect("Error in parsing the floating number");

    // Convert Fahrenheit to Celsius
    let celcius = (temp - 32.0) / 1.8;
    println!("{}°F is equal to {}°C", temp, celcius);

    // Convert Fahrenheit to Kelvin
    let kelvin = (temp - 32.0) / 1.8 + 273.15;
    println!("{}°F is equal to {}°K", temp, kelvin);

    // Convert Fahrenheit to Rankine
    let rankine = temp + 459.67;
    println!("{}°F is equal to {}°R", temp, rankine);
}

pub fn kelvin() {
    println!("Enter temperature in Kelvin:");
    let mut kelvin = String::new();
    io::stdin().read_line(&mut kelvin).expect("Failed to read line");

    let temp: f32 = kelvin.trim().parse().expect("Error in parsing the floating number");

    // Convert Kelvin to Celsius
    let celcius = temp - 273.15;
    println!("{}°K is equal to {}°C", temp, celcius);

    // Convert Kelvin to Fahrenheit
    let fahrenheit = (temp - 273.15) * 9.0 / 5.0 + 32.0;
    println!("{}°K is equal to {}°F", temp, fahrenheit);

    // Convert Kelvin to Rankine
    let rankine = temp * 1.8;
    println!("{}°K is equal to {}°R", temp, rankine);
}

pub fn rankine() {
    println!("Enter temperature in Rankine:");
    let mut rankine = String::new();
    io::stdin().read_line(&mut rankine).expect("Failed to read line");

    let temp: f32 = rankine.trim().parse().expect("Error in parsing the floating number");

    // Convert Rankine to Celsius
    let celcius = (temp - 491.67) * 5.0 / 9.0;
    println!("{}°R is equal to {}°C", temp, celcius);

    // Convert Rankine to Fahrenheit
    let fahrenheit = temp - 459.67;
    println!("{}°R is equal to {}°F", temp, fahrenheit);

    // Convert Rankine to Kelvin
    let kelvin = temp * 5.0 / 9.0;
    println!("{}°R is equal to {}°K", temp, kelvin);
}
