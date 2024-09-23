use std::{io, num::ParseIntError};
use whatlang::{detect, Lang, Script};
use std::io::{Write};


mod conversion;
mod area;
mod tempe;
mod lenght;
mod physics;
mod physicc;
mod afd;
mod time;
mod data;
mod fuel;

fn main() {
    loop {
        println!("Enter your choice:");
        println!("1: convertion of number to different base");
        println!("2: calculate area in different units");
        println!("3: Temperature converter [Kelvin, Celcius, Fahrenheit, Rankine]");
        println!("4: length converter");
        println!("5: Physics calculator [mass, density, power, pressure]");
        println!("6::miscellaneous calulations [angle,data transfer unit,fuel economy");
        println!("7: Time operations");
        println!("8: Data operations");
        println!("9: Fuel operations");
        println!("10: Find language and Script");
        let mut ch = String::new();
        io::stdin().read_line(&mut ch).expect("Failed to read line");
        let choice: u8 = ch.trim().parse().expect("Invalid input");
        match choice {
            1 => convert(),
            2 => area(),
            3 => tempe(),
            4 => lenght(),
            5 => physics(),
            6 => afd(),
            7=> time(),
            8=> data(),
            9=> fuel(),
            10=>script(),
            _ => println!("Invalid choice"),
        }

        println!("Do you want to continue? (yes/no)");
        let mut cont = String::new();
        io::stdin().read_line(&mut cont).expect("Failed to read line");
        let cont = cont.trim().to_lowercase();
        if cont != "yes" {
            break;
        }
    }
}

fn convert() {
    println!("Enter a number in the original base: ");
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Failed to read the number");
    let number = ip.trim();

    println!("Enter the original base: ");
    let mut ipb = String::new();
    io::stdin().read_line(&mut ipb).expect("Failed to read the line");
    let from_base: u32 = ipb.trim().parse().expect("Invalid base");

    println!("Enter the target base: ");
    let mut ipd = String::new();
    io::stdin().read_line(&mut ipd).expect("Failed to read the line");
    let to_base: u32 = ipd.trim().parse().expect("Invalid target base");

    let result = conversion::convert_base(number, from_base, to_base);  // Assuming the function exists in conversion.rs
    println!("Converted number: {}", result);
}

fn area() {
    println!("WELCOME TO THE AREA CALCULATOR");
    println!("Please select an option to calculate the area:");
    println!("1. Area of Rectangle");
    println!("2. Area of Circle");
    println!("3. Area of Triangle");
    println!("4. Area of Square");
    println!("5. Area of Regular Polygon");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("Invalid choice");

    match choice {
        1 => area::rectangle_area(),
        2 => area::circle_area(),
        3 => area::triangle_area(),
        4 => area::square_area(),
        5 => area::polygon_area(),
        _ => println!("Invalid option. Please choose a number from 1 to 5."),
    }
}

fn tempe() {
    println!("welcome to temperature converter");
    println!("Enter original temperature unit");
    println!("1. Celcius");
    println!("2. Farenheit");
    println!("3. Kelvin");
    println!("4. Rankine");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("Invalid choice");
    match choice {
        1 => tempe::celcius(),
        2 => tempe::fahrenheit(),
        3 => tempe::kelvin(),
        4 => tempe::rankine(),
        _ => println!("Invalid option. Please choose a number from 1 to 4"),
    }
}

fn lenght() {
    println!("welcome to length converter");
    println!("enter original units: ");
    println!("1. meter");
    println!("2. centimeter");
    println!("3. millimeter");
    println!("4. kilometer");
    println!("5. inch");
    println!("6. feet");
    println!("7. yard");
    println!("8. mile");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read the line");
    let choice: u8 = choice.trim().parse().expect("Invalid choice");
    match choice {
        1 => lenght::meter(),
        2 => lenght::centimeter(),
        3 => lenght::millimeter(),
        4 => lenght::kilometer(),
        5 => lenght::inch(),
        6 => lenght::feet(),
        7 => lenght::yard(),
        8 => lenght::mile(),
        _ => println!("Invalid option. Please choose a number from 1 to 8"),
    }
}

fn physics() {
    println!("welcome to physics calculator");
    println!("note that all units are in SI units:");
    println!("1. acceleration");
    println!("2. velocity");
    println!("3. force");
    println!("4. mass");
    println!("5. energy");
    println!("6. power");
    println!("7. work");
    println!("8. torque");
    println!("9. pressure");
    println!("10.convert the units: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read the line");
    let choice: u16 = choice.trim().parse().expect("failed to read the message");
    match choice {
        1 => physics::acceleration(),
        2 => physics::velocity(),
        3 => physics::force(),
        4 => physics::mass(),
        5 => physics::energy(),
        6 => physics::power(),
        7 => physics::work(),
        8 => physics::torque(),
        9 => physics::pressure(),
        10 =>physicc::convert(),
        _ => println!("Invalid option. Please choose a number from 1 to 9"),
    }
}
fn afd(){
    println!("welcome to afd calculator");
    println!("1:: Angle operations ");
    println!("2:: Fuel econmy operations: ");
    println!("3:: Speed operations ");
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read the line");
    let choice:u8=choice.trim().parse().expect("Invalid choice");
    match choice {
        1=>afd::angle(),
        2=>afd::fuel(),
        3=>afd::datasp(),
        _=>println!("Invalid option. Please choose a number from 1 to 3"),
    }
}
fn time(){
    println!("welcome to time calculator");
    println!("1. time conversions");
    println!("2. time calculations");
    println!("Enter choice");
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read the line");
    let choice:u8=choice.trim().parse().expect("Invalid choice");
    match choice {
        1=>time::convert(),
        2=>time::calculate(),
        _=>println!("Invalid option. Please choose a number from 1 to 3"),
    }

}
fn fuel(){
    println!("welcome to fuel calculator");
    println!("Press 1 for Fuel operation:");
    let mut  choice=String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read the line");
    let choice:u8=choice.trim().parse().expect("Invalid choice");
    match choice {
        1=>fuel::fuel(),
        _=>println!("Invalid option. Please choose a number from 1"),
    }
}
fn data(){
    println!("welcome to data calculator");
    println!("1. data conversions");
    println!("2. data calculations");
    println!("3.  data statistics");
    println!("4. download/update speed");
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read the line");
    let choice:u8=choice.trim().parse().expect("Invalid choice");
    match choice {
        1=>data::convert(),
        2=>data::calculate(),
        3=>data::stat(),
        4=>data::speed(),
        _=>println!("Invalid option. Please choose a number from 1 to 4"),
    }    

}
fn what_language(text: &str) {
    let info = detect(text).unwrap();
    println!("Language type: {:?}", info.lang());
    println!("Language Script: {:?}", info.script());
    println!("System confidence: {}", info.confidence());
    println!("System reliability: {}", info.is_reliable());

    if info.is_reliable() {
        println!("The detected language is reliable: True");
    } else {
        println!("The detected language is reliable: False");
    }
}

fn script() {
    print!("Enter some text: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    what_language(input);
}