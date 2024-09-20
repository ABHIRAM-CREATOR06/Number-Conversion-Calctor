use std::f64::consts::PI;
use std::io;

pub fn angle() {
    println!("Enter the first value:");
    let mut value1 = String::new();
    io::stdin().read_line(&mut value1).expect("Failed to read line");
    let value1: f64 = value1.trim().parse().expect("Please type a number");

    println!("Is the first value in degrees or radians? (d/r):");
    let mut unit1 = String::new();
    io::stdin().read_line(&mut unit1).expect("Failed to read line");
    let unit1 = unit1.trim();

    let value1_in_radians = if unit1 == "d" {
        value1 * PI / 180.0 // Convert degrees to radians
    } else {
        value1 // Already in radians
    };

    println!("Enter the second value:");
    let mut value2 = String::new();
    io::stdin().read_line(&mut value2).expect("Failed to read line");
    let value2: f64 = value2.trim().parse().expect("Please type a number");

    println!("Is the second value in degrees or radians? (d/r):");
    let mut unit2 = String::new();
    io::stdin().read_line(&mut unit2).expect("Failed to read line");
    let unit2 = unit2.trim();

    let value2_in_radians = if unit2 == "d" {
        value2 * PI / 180.0 // Convert degrees to radians
    } else {
        value2 // Already in radians
    };

    println!("Choose an arithmetic operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();

    println!("Select trigonometric function for the first value (sin, cos, tan, cot, sec, cosec):");
    let mut func1 = String::new();
    io::stdin().read_line(&mut func1).expect("Failed to read line");
    let func1 = func1.trim();

    println!("Select trigonometric function for the second value (sin, cos, tan, cot, sec, cosec):");
    let mut func2 = String::new();
    io::stdin().read_line(&mut func2).expect("Failed to read line");
    let func2 = func2.trim();

    let trig_value1 = match func1 {
        "sin" => value1_in_radians.sin(),
        "cos" => value1_in_radians.cos(),
        "tan" => value1_in_radians.tan(),
        "cot" => 1.0 / value1_in_radians.tan(),
        "sec" => 1.0 / value1_in_radians.cos(),
        "cosec" => 1.0 / value1_in_radians.sin(),
        _ => {
            println!("Invalid function for the first value");
            return;
        }
    };

    let trig_value2 = match func2 {
        "sin" => value2_in_radians.sin(),
        "cos" => value2_in_radians.cos(),
        "tan" => value2_in_radians.tan(),
        "cot" => 1.0 / value2_in_radians.tan(),
        "sec" => 1.0 / value2_in_radians.cos(),
        "cosec" => 1.0 / value2_in_radians.sin(),
        _ => {
            println!("Invalid function for the second value");
            return;
        }
    };

    let result = match operation {
        "+" => trig_value1 + trig_value2,
        "-" => trig_value1 - trig_value2,
        "*" => trig_value1 * trig_value2,
        "/" => trig_value1 / trig_value2,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("Result of {}({}) {} {}({}): {:.4}", func1, value1, operation, func2, value2, result);
}

pub fn datasp(){
    println!("coming soon");
}
pub fn fuel(){
    println!("coming soon");
}
