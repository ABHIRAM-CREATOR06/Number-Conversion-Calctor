use std::io;

mod conversion;

fn main() {
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

    let result = conversion::convert_base(number, from_base, to_base);
    println!("Converted number: {}", result);
}
