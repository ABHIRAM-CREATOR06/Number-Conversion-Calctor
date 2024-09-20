use std::io;

pub fn convert() {
    loop {
        println!("Choose a conversion:");
        println!("1. Force");
        println!("2. Pressure");
        println!("3. Velocity");
        println!("4. Energy");
        println!("5. Mass");
        println!("6. Torque");
        println!("7. Work");
        println!("8. Acceleration");
        println!("9. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number");

        match choice {
            1 => force(),
            2 => pressure(),
            3 => velocity(),
            4 => energy(),
            5 => mass(),
            6 => torque(),
            7 => work(),
            8 => acceleration(),
            9 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn force() {
    println!("Force in N: ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Please type a number");
    println!("Force in dyne: {}", value / 1e-5);
    println!("Force in lbf: {}", value * 0.224808943);
    println!("Force in kgf: {}", value * 0.101971621);
    println!("Force in ozf: {}", value * 0.070931742);
    println!("Force in gf: {}", value * 101.971621);
    println!("Force in pdl: {}", value * 7.233014);
    println!("Force in kip: {}", value * 0.000224809);
    println!("Force in ton-force: {}", value * 0.000101972);
}

fn pressure() {
    println!("Pressure in Pascal (Pa): ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Please type a number");
    println!("Pressure in bar: {}", value / 1e5);
    println!("Pressure in atm: {}", value / 101325.0);
    println!("Pressure in psi: {}", value * 0.000145038);
    println!("Pressure in torr: {}", value * 0.00750062);
    println!("Pressure in mmHg: {}", value * 0.00750062);
    println!("Pressure in inHg: {}", value * 0.0002953);
}

fn velocity() {
    println!("Velocity in m/s: ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Please type a number");
    println!("Velocity in km/h: {}", value * 3.6);
    println!("Velocity in mph: {}", value * 2.23694);
    println!("Velocity in ft/s: {}", value * 3.28084);
    println!("Velocity in knots: {}", value * 1.94384);
    println!("Velocity in cm/s: {}", value * 100.0);
}

fn energy() {
    println!("Energy in J: ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Please type a number");
    println!("Energy in kWh: {}", value / 3.6e6);
    println!("Energy in cal: {}", value / 4.184);
    println!("Energy in BTU: {}", value / 1055.06);
    println!("Energy in eV: {}", value * 6.242e18);
    println!("Energy in erg: {}", value * 1e7);
}

fn mass() {
    println!("Mass in kg: ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Please type a number");
    println!("Mass in g: {}", value * 1000.0);
    println!("Mass in lb: {}", value * 2.20462);
    println!("Mass in oz: {}", value * 35.274);
    println!("Mass in mg: {}", value * 1e6);
    println!("Mass in ton: {}", value * 0.001);
}

fn torque() {
    println!("Torque in Nm: ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Please type a number");
    println!("Torque in lb-ft: {}", value * 0.737562);
    println!("Torque in oz-in: {}", value * 141.611);
    println!("Torque in dyne-cm: {}", value * 1e7);
    println!("Torque in kgf-m: {}", value * 0.101971621);
}

fn work() {
    println!("Work in J: ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Please type a number");
    println!("Work in kWh: {}", value / 3.6e6);
    println!("Work in cal: {}", value / 4.184);
    println!("Work in BTU: {}", value / 1055.06);
    println!("Work in erg: {}", value * 1e7);
    println!("Work in eV: {}", value * 6.242e18);
}

fn acceleration() {
    println!("Acceleration in m/s^2: ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Please type a number"); 
    let acceleration_kmph2 = value* 3.6; 
    let acceleration_g = value * 100.0; 
    let acceleration_ftps2 = value* 3.281; 
    let acceleration_mph2 = acceleration_ftps2 * 3600.0; 
    let acceleration_inps2 = acceleration_ftps2 * 12.0;
    println!("Acceleration (km/h²): {:.2}", acceleration_kmph2);
    println!("Acceleration (g): {:.2}", acceleration_g);
    println!("Acceleration (ft/s²): {:.2}", acceleration_ftps2);
    println!("Acceleration (mph²): {:.2}", acceleration_mph2);
    println!("Acceleration (in/s²): {:.2}", acceleration_inps2);
}
