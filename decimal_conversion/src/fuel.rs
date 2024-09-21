use std::io;

pub fn fuel() {
    let fuel_used = get_input("Enter the amount of fuel used (in liters):");
    let distance = get_input("Enter the distance traveled (in kilometers):");
    let fuel_price = get_input("Enter the fuel price (per liter):");
    let daily_distance = get_input("Enter the daily distance traveled (in kilometers):");
    println!();
    println!("An emission factor is a coefficient that quantifies the amount of a pollutant released into the atmosphere as a result of a specific activity.");
    println!("Emission factor for gasoline/petrol Vechicle: 2.31");
    println!("Emission factor for Diesel Vechicle: 2.64");
    println!("Emission factor for CNG Vechicle: 1.68");
    println!("Emission factor for LPG Vechicle: 1.64");
    println!("Emission factor for Electric Vechicle: 0");
    println!("Emission factor for Hybrid vehicle : around 0.3");
    let emission_factor = get_input("Enter the emission factor (kg CO2 per liter):");

    let fuel_consumption = calculate_fuel_consumption(fuel_used, distance);
    let fuel_efficiency = calculate_fuel_efficiency(distance, fuel_used);
    let trip_fuel_cost = calculate_trip_fuel_cost(distance, fuel_efficiency, fuel_price);
    let annual_fuel_cost = calculate_annual_fuel_cost(daily_distance, fuel_price, fuel_efficiency);
    let carbon_footprint = calculate_carbon_footprint(fuel_used, emission_factor);

    println!("Fuel Consumption: {:.2} L/100km", fuel_consumption);
    println!("Fuel Efficiency: {:.2} km/L", fuel_efficiency);
    println!("Trip Fuel Cost: {:.2} ₹", trip_fuel_cost);
    println!("Annual Fuel Cost: {:.2} ₹", annual_fuel_cost);
    println!("Carbon Footprint: {:.2} kg CO2", carbon_footprint);
    println!("planning to change into new car!!! Lets check for monthly savings by Pressing 1");
    let mut s=String::new();
    io::stdin().read_line(&mut s).expect("failed to read line");
    let s:u8=s.trim().parse().expect("failed to read number");
    if s==1{
        let new_fuel_efficiency = get_input("Enter the fuel efficiency of the new vehicle (in km/L):");
        let fuel_savings = calculate_fuel_savings(daily_distance, fuel_price, fuel_efficiency, new_fuel_efficiency);
        println!("Monthly Fuel Savings: {:.2} ₹", fuel_savings);
    }
    else{
        println!("Have a pleasant journey");
    }

}
fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}

fn calculate_fuel_consumption(fuel_used: f64, distance: f64) -> f64 {
    (fuel_used / distance) * 100.0
}

fn calculate_fuel_efficiency(distance: f64, fuel_used: f64) -> f64 {
    distance / fuel_used
}

fn calculate_trip_fuel_cost(distance: f64, fuel_efficiency: f64, fuel_price: f64) -> f64 {
    (distance / fuel_efficiency) * fuel_price
}

fn calculate_annual_fuel_cost(daily_distance: f64, fuel_price: f64, fuel_efficiency: f64) -> f64 {
    daily_distance * 365.0 * (fuel_price / fuel_efficiency)
}

fn calculate_carbon_footprint(fuel_used: f64, emission_factor: f64) -> f64 {
    fuel_used * emission_factor
}

fn calculate_fuel_savings(daily_distance: f64, fuel_price: f64, current_efficiency: f64, new_efficiency: f64) -> f64 {
    let current_monthly_cost = daily_distance * 30.0 * (fuel_price / current_efficiency);
    let new_monthly_cost = daily_distance * 30.0 * (fuel_price / new_efficiency);
    current_monthly_cost - new_monthly_cost
}
