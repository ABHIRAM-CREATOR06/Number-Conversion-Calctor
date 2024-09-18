use std::io;

pub fn rectangle_area() {
    println!("Enter the length of the rectangle:");
    let length = read_number();
    println!("Enter the breadth of the rectangle:");
    let breadth = read_number();
    let area = length * breadth;
    println!("Area of Rectangle: {} square meters", area);
    unit_conversion(area);
}

pub fn circle_area() {
    println!("Enter the radius of the circle:");
    let radius = read_number();
    let area = std::f64::consts::PI * radius * radius;
    println!("Area of Circle: {} square meters", area);
    unit_conversion(area);
}

pub fn triangle_area() {
    println!("Enter the base of the triangle:");
    let base = read_number();
    println!("Enter the height of the triangle:");
    let height = read_number();
    let area = 0.5 * base * height;
    println!("Area of Triangle: {} square meters", area);
    unit_conversion(area);
}

pub fn square_area() {
    println!("Enter the side length of the square:");
    let side = read_number();
    let area = side * side;
    println!("Area of Square: {} square meters", area);
    unit_conversion(area);
}

pub fn polygon_area() {
    println!("Enter the number of sides of the regular polygon:");
    let sides = read_number();
    println!("Enter the length of a side:");
    let side_length = read_number();
    let area = (sides * side_length.powf(2.0)) / (4.0 * (std::f64::consts::PI / sides).tan());
    println!("Area of Regular Polygon: {} square meters", area);
    unit_conversion(area);
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}

fn unit_conversion(area: f64) {
    println!("Choose the unit for conversion:");
    println!("1. Acres (acre)");
    println!("2. Hectares (hectare)");
    println!("3. Square Feet (ft²)");
    println!("4. Square Inches (in²)");
    println!("5. Square Kilometers (km²)");
    println!("6. Square Miles (mi²)");
    println!("7. Square Yards (yd²)");

    let mut unit_choice = String::new();
    io::stdin().read_line(&mut unit_choice).expect("Failed to read line");
    let unit_choice: u8 = unit_choice.trim().parse().expect("Invalid choice");

    convert_unit(area, unit_choice);
}

fn convert_unit(area: f64, unit_choice: u8) {
    match unit_choice {
        1 => println!("Area in Acres: {}", area * 0.000247105),
        2 => println!("Area in Hectares: {}", area * 0.0001),
        3 => println!("Area in Square Feet: {}", area * 10.7639),
        4 => println!("Area in Square Inches: {}", area * 1550.0031),
        5 => println!("Area in Square Kilometers: {}", area * 0.000001),
        6 => println!("Area in Square Miles: {}", area * 0.000000386102),
        7 => println!("Area in Square Yards: {}", area * 1.19599),
        _ => println!("Invalid unit choice."),
    }
}
