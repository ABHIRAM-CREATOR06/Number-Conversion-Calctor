use std::io;

mod conversion;
mod area;

fn main() {
    println!("Enter your choice:");
    println!("1: convertion of number to different  base");
    println!("2: calculate area in  different units");
    let mut ch = String::new(); 
    io::stdin().read_line(&mut ch).expect("Failed to read line");
    let choice: u8 = ch.trim().parse().expect("Invalid input");

    match choice {
        1 => convert(),  
        2 => area(),     
        _ => println!("Invalid choice"),
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
        1 => area::rectangle_area(),  // Assuming the function exists in area.rs
        2 => area::circle_area(),     // Assuming the function exists in area.rs
        3 => area::triangle_area(),   // Assuming the function exists in area.rs
        4 => area::square_area(),     // Assuming the function exists in area.rs
        5 => area::polygon_area(),    // Assuming the function exists in area.rs
        _ => println!("Invalid option. Please choose a number from 1 to 5."),
    }
}