use std::io;
pub fn acceleration(){
   println!("please enter the avilable dats: ");
   println!(" 1::if available datas are time,distance");
   println!("2::if force and mass are available data");
   println!("3::if the centripital force and radius given ");
   println!("4::if the displacement is known");
   let mut choice=String::new();
   std::io::stdin().read_line(&mut choice).expect("failed to read line");
   let choice:u32=choice.trim().parse().expect("please type a number");
   match choice {
    1 => td(),
    2 => fm(),
    3 => cf(),
    4 => dis(),
    _ => println!("invalid choice"),
   }
}
pub fn velocity() {
    println!("Please enter an option: ");
    println!("1:: if the time and distance are known");
    println!("2:: if the force and mass are known");
    println!("3:: if you want to calculate average velocity");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("please type a valid number");

    match choice {
        1 => time_distance_velocity(),
        2 => force_mass_velocity(),
        3 => average_velocity(),
        _ => println!("Invalid choice"),
    }
}

fn time_distance_velocity() {
    println!("Enter time and distance:");
    let mut time = String::new();
    std::io::stdin().read_line(&mut time).expect("failed to read line");
    let time: f32 = time.trim().parse().expect("please type a valid number");
    let mut distance = String::new();
    std::io::stdin().read_line(&mut distance).expect("failed to read line");
    let distance: f32 = distance.trim().parse().expect("please type a valid number");
    let velocity = distance / time;
    println!("The velocity is: {}", velocity);
}
fn force_mass_velocity() {
    println!("Enter force and mass and time :");  
    let mut force = String::new();
    std::io::stdin().read_line(&mut force).expect("failed to read line");
    let force: f32 = force.trim().parse().expect("please type a valid number");
    let mut mass = String::new();
    std::io::stdin().read_line(&mut mass).expect("failed to read line");
    let mass: f32 = mass.trim().parse().expect("please type a valid number");
    let mut time=String::new();
    std::io::stdin().read_line(&mut time).expect("failed to read  line");
    let time:f32=time.trim().parse().expect("please type a valid number");
    let velocity = (force / mass)*time;
    println!("The velocity is: {}", velocity);
}
fn average_velocity() {
    println!("Enter initial and final velocity:");
    let mut initial_velocity = String::new();
    std::io::stdin().read_line(&mut initial_velocity).expect("failed to read line");
    let initial_velocity: f32 = initial_velocity.trim().parse().expect("please type a valid number");
    let mut final_velocity = String::new();
    std::io::stdin().read_line(&mut final_velocity).expect("failed to read line");
    let final_velocity: f32 = final_velocity.trim().parse().expect("please type a valid number");
    let average_velocity = (initial_velocity + final_velocity) / 2.0;
    println!("The average velocity is: {}", average_velocity);
}
pub fn force() {
    println!("Please enter an option: ");
    println!("1:: if mass and acceleration are known");
    println!("2:: if pressure and area are known (to calculate force via pressure)");
    println!("3:: if momentum and time are known");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("please type a valid number");

    match choice {
        1 => mass_acceleration_force(),
        2 => pressure_area_force(),
        3 => momentum_time_force(),
        _ => println!("Invalid choice"),
    }
}

fn mass_acceleration_force() {
    println!("Enter mass and acceleration:");

    let mut mass = String::new();
    std::io::stdin().read_line(&mut mass).expect("failed to read line");
    let mass: f32 = mass.trim().parse().expect("please type a valid number");

    let mut acceleration = String::new();
    std::io::stdin().read_line(&mut acceleration).expect("failed to read line");
    let acceleration: f32 = acceleration.trim().parse().expect("please type a valid number");

    let force = mass * acceleration;
    println!("The force is: {}", force);
}

fn pressure_area_force() {
    println!("Enter pressure and area:");

    let mut pressure = String::new();
    std::io::stdin().read_line(&mut pressure).expect("failed to read line");
    let pressure: f32 = pressure.trim().parse().expect("please type a valid number");

    let mut area = String::new();
    std::io::stdin().read_line(&mut area).expect("failed to read line");
    let area: f32 = area.trim().parse().expect("please type a valid number");

    let force = pressure * area;
    println!("The force is: {}", force);
}
fn momentum_time_force() {
    println!("Enter momentum and time:");

    let mut momentum = String::new();
    std::io::stdin().read_line(&mut momentum).expect("failed to read line");
    let momentum: f32 = momentum.trim().parse().expect("please type a valid number");

    let mut time = String::new();
    std::io::stdin().read_line(&mut time).expect("failed to read line");
    let time: f32 = time.trim().parse().expect("please type a valid number");

    let force = momentum / time;
    println!("The force is: {}", force);
}
pub fn mass() {
    println!("Please enter an option: ");
    println!("1:: if force and acceleration are known");
    println!("2:: if density and volume are known");
    println!("3:: if weight and gravitational acceleration are known");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("please type a valid number");

    match choice {
        1 => force_acceleration_mass(),
        2 => density_volume_mass(),
        3 => weight_gravity_mass(),
        _ => println!("Invalid choice"),
    }
}

fn force_acceleration_mass() {
    println!("Enter force and acceleration:");

    let mut force = String::new();
    std::io::stdin().read_line(&mut force).expect("failed to read line");
    let force: f32 = force.trim().parse().expect("please type a valid number");

    let mut acceleration = String::new();
    std::io::stdin().read_line(&mut acceleration).expect("failed to read line");
    let acceleration: f32 = acceleration.trim().parse().expect("please type a valid number");

    let mass = force / acceleration;
    println!("The mass is: {}", mass);
}

fn density_volume_mass() {
    println!("Enter density and volume:");

    let mut density = String::new();
    std::io::stdin().read_line(&mut density).expect("failed to read line");
    let density: f32 = density.trim().parse().expect("please type a valid number");

    let mut volume = String::new();
    std::io::stdin().read_line(&mut volume).expect("failed to read line");
    let volume: f32 = volume.trim().parse().expect("please type a valid number");

    let mass = density * volume;
    println!("The mass is: {}", mass);
}

fn weight_gravity_mass() {
    println!("Enter weight and gravitational acceleration:");

    let mut weight = String::new();
    std::io::stdin().read_line(&mut weight).expect("failed to read line");
    let weight: f32 = weight.trim().parse().expect("please type a valid number");

    let mut gravity = String::new();
    std::io::stdin().read_line(&mut gravity).expect("failed to read line");
    let gravity: f32 = gravity.trim().parse().expect("please type a valid number");

    let mass = weight / gravity;
    println!("The mass is: {}", mass);
}
pub fn energy() {
    println!("Please enter an option: ");
    println!("1:: if mass and velocity are known (Kinetic Energy)");
    println!("2:: if mass, gravity, and height are known (Potential Energy)");
    println!("3:: if power and time are known");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("please type a valid number");

    match choice {
        1 => kinetic_energy(),
        2 => potential_energy(),
        3 => power_time_energy(),
        _ => println!("Invalid choice"),
    }
}

fn kinetic_energy() {
    println!("Enter mass and velocity:");

    let mut mass = String::new();
    std::io::stdin().read_line(&mut mass).expect("failed to read line");
    let mass: f32 = mass.trim().parse().expect("please type a valid number");

    let mut velocity = String::new();
    std::io::stdin().read_line(&mut velocity).expect("failed to read line");
    let velocity: f32 = velocity.trim().parse().expect("please type a valid number");

    let energy = 0.5 * mass * velocity.powi(2);
    println!("The kinetic energy is: {}", energy);
}

fn potential_energy() {
    println!("Enter mass, gravity, and height:");

    let mut mass = String::new();
    std::io::stdin().read_line(&mut mass).expect("failed to read line");
    let mass: f32 = mass.trim().parse().expect("please type a valid number");

    let mut gravity = String::new();
    std::io::stdin().read_line(&mut gravity).expect("failed to read line");
    let gravity: f32 = gravity.trim().parse().expect("please type a valid number");

    let mut height = String::new();
    std::io::stdin().read_line(&mut height).expect("failed to read line");
    let height: f32 = height.trim().parse().expect("please type a valid number");

    let energy = mass * gravity * height;
    println!("The potential energy is: {}", energy);
}

fn power_time_energy() {
    println!("Enter power and time:");

    let mut power = String::new();
    std::io::stdin().read_line(&mut power).expect("failed to read line");
    let power: f32 = power.trim().parse().expect("please type a valid number");

    let mut time = String::new();
    std::io::stdin().read_line(&mut time).expect("failed to read line");
    let time: f32 = time.trim().parse().expect("please type a valid number");

    let energy = power * time;
    println!("The energy is: {}", energy);
}
pub fn power() {
    println!("Please enter an option: ");
    println!("1:: if work and time are known");
    println!("2:: if force, velocity, and angle are known");
    println!("3:: if energy and time are known");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("please type a valid number");

    match choice {
        1 => work_time_power(),
        2 => force_velocity_angle_power(),
        3 => energy_time_power(),
        _ => println!("Invalid choice"),
    }
}

fn work_time_power() {
    println!("Enter work and time:");

    let mut work = String::new();
    std::io::stdin().read_line(&mut work).expect("failed to read line");
    let work: f32 = work.trim().parse().expect("please type a valid number");

    let mut time = String::new();
    std::io::stdin().read_line(&mut time).expect("failed to read line");
    let time: f32 = time.trim().parse().expect("please type a valid number");

    let power = work / time;
    println!("The power is: {}", power);
}

fn force_velocity_angle_power() {
    println!("Enter force, velocity, and angle (in degrees):");

    let mut force = String::new();
    std::io::stdin().read_line(&mut force).expect("failed to read line");
    let force: f32 = force.trim().parse().expect("please type a valid number");

    let mut velocity = String::new();
    std::io::stdin().read_line(&mut velocity).expect("failed to read line");
    let velocity: f32 = velocity.trim().parse().expect("please type a valid number");

    let mut angle = String::new();
    std::io::stdin().read_line(&mut angle).expect("failed to read line");
    let angle: f32 = angle.trim().parse().expect("please type a valid number");

    let angle_radians = angle.to_radians(); // Convert degrees to radians for cos function
    let power = force * velocity * angle_radians.cos();
    println!("The power is: {}", power);
}
fn energy_time_power() {
    println!("Enter energy and time:");

    let mut energy = String::new();
    std::io::stdin().read_line(&mut energy).expect("failed to read line");
    let energy: f32 = energy.trim().parse().expect("please type a valid number");

    let mut time = String::new();
    std::io::stdin().read_line(&mut time).expect("failed to read line");
    let time: f32 = time.trim().parse().expect("please type a valid number");

    let power = energy / time;
    println!("The power is: {}", power);
}
pub fn work() {
    println!("Please enter an option: ");
    println!("1:: if force and displacement are known");
    println!("2:: if power and time are known");
    println!("3:: if pressure and volume are known");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("please type a valid number");

    match choice {
        1 => force_displacement_work(),
        2 => power_time_work(),
        3 => pressure_volume_work(),
        _ => println!("Invalid choice"),
    }
}

fn force_displacement_work() {
    println!("Enter force and displacement:");

    let mut force = String::new();
    std::io::stdin().read_line(&mut force).expect("failed to read line");
    let force: f32 = force.trim().parse().expect("please type a valid number");

    let mut displacement = String::new();
    std::io::stdin().read_line(&mut displacement).expect("failed to read line");
    let displacement: f32 = displacement.trim().parse().expect("please type a valid number");

    let work = force * displacement;
    println!("The work is: {}", work);
}
fn power_time_work() {
    println!("Enter power and time:");

    let mut power = String::new();
    std::io::stdin().read_line(&mut power).expect("failed to read line");
    let power: f32 = power.trim().parse().expect("please type a valid number");

    let mut time = String::new();
    std::io::stdin().read_line(&mut time).expect("failed to read line");
    let time: f32 = time.trim().parse().expect("please type a valid number");

    let work = power * time;
    println!("The work is: {}", work);
}
fn pressure_volume_work() {
    println!("Enter pressure and volume:");

    let mut pressure = String::new();
    std::io::stdin().read_line(&mut pressure).expect("failed to read line");
    let pressure: f32 = pressure.trim().parse().expect("please type a valid number");

    let mut volume = String::new();
    std::io::stdin().read_line(&mut volume).expect("failed to read line");
    let volume: f32 = volume.trim().parse().expect("please type a valid number");

    let work = pressure * volume;
    println!("The work is: {}", work);
}
pub fn torque() {
    println!("Please enter an option: ");
    println!("1:: if force and radius are known");
    println!("2:: if power and angular velocity are known");
    println!("3:: if moment of inertia and angular acceleration are known");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("please type a valid number");

    match choice {
        1 => force_radius_torque(),
        2 => power_angular_velocity_torque(),
        3 => inertia_angular_acceleration_torque(),
        _ => println!("Invalid choice"),
    }
}

fn force_radius_torque() {
    println!("Enter force and radius:");

    let mut force = String::new();
    std::io::stdin().read_line(&mut force).expect("failed to read line");
    let force: f32 = force.trim().parse().expect("please type a valid number");

    let mut radius = String::new();
    std::io::stdin().read_line(&mut radius).expect("failed to read line");
    let radius: f32 = radius.trim().parse().expect("please type a valid number");

    let torque = force * radius;
    println!("The torque is: {}", torque);
}

fn power_angular_velocity_torque() {
    println!("Enter power and angular velocity:");

    let mut power = String::new();
    std::io::stdin().read_line(&mut power).expect("failed to read line");
    let power: f32 = power.trim().parse().expect("please type a valid number");

    let mut angular_velocity = String::new();
    std::io::stdin().read_line(&mut angular_velocity).expect("failed to read line");
    let angular_velocity: f32 = angular_velocity.trim().parse().expect("please type a valid number");

    let torque = power / angular_velocity;
    println!("The torque is: {}", torque);
}

fn inertia_angular_acceleration_torque() {
    println!("Enter moment of inertia and angular acceleration:");

    let mut inertia = String::new();
    std::io::stdin().read_line(&mut inertia).expect("failed to read line");
    let inertia: f32 = inertia.trim().parse().expect("please type a valid number");

    let mut angular_acceleration = String::new();
    std::io::stdin().read_line(&mut angular_acceleration).expect("failed to read line");
    let angular_acceleration: f32 = angular_acceleration.trim().parse().expect("please type a valid number");

    let torque = inertia * angular_acceleration;
    println!("The torque is: {}", torque);
}
pub fn pressure() {
    println!("Please enter an option: ");
    println!("1:: if force and area are known");
    println!("2:: if density, gravity, and height are known (for fluid pressure)");
    println!("3:: if volume, temperature, and moles of gas are known (Ideal Gas Law)");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("please type a valid number");

    match choice {
        1 => force_area_pressure(),
        2 => fluid_pressure(),
        3 => ideal_gas_law_pressure(),
        _ => println!("Invalid choice"),
    }
}

fn force_area_pressure() {
    println!("Enter force and area:");

    let mut force = String::new();
    std::io::stdin().read_line(&mut force).expect("failed to read line");
    let force: f32 = force.trim().parse().expect("please type a valid number");

    let mut area = String::new();
    std::io::stdin().read_line(&mut area).expect("failed to read line");
    let area: f32 = area.trim().parse().expect("please type a valid number");

    let pressure = force / area;
    println!("The pressure is: {} Pascals", pressure);
}

fn fluid_pressure() {
    println!("Enter density (kg/m³), gravity (m/s²), and height (m):");

    let mut density = String::new();
    std::io::stdin().read_line(&mut density).expect("failed to read line");
    let density: f32 = density.trim().parse().expect("please type a valid number");

    let mut gravity = String::new();
    std::io::stdin().read_line(&mut gravity).expect("failed to read line");
    let gravity: f32 = gravity.trim().parse().expect("please type a valid number");

    let mut height = String::new();
    std::io::stdin().read_line(&mut height).expect("failed to read line");
    let height: f32 = height.trim().parse().expect("please type a valid number");

    let pressure = density * gravity * height;
    println!("The fluid pressure is: {} Pascals", pressure);
}

fn ideal_gas_law_pressure() {
    println!("Enter volume (m³), temperature (Kelvin), and number of moles:");

    let mut volume = String::new();
    std::io::stdin().read_line(&mut volume).expect("failed to read line");
    let volume: f32 = volume.trim().parse().expect("please type a valid number");

    let mut temperature = String::new();
    std::io::stdin().read_line(&mut temperature).expect("failed to read line");
    let temperature: f32 = temperature.trim().parse().expect("please type a valid number");

    let mut moles = String::new();
    std::io::stdin().read_line(&mut moles).expect("failed to read line");
    let moles: f32 = moles.trim().parse().expect("please type a valid number");

    const GAS_CONSTANT: f32 = 8.314; // Ideal gas constant in J/(mol*K)

    let pressure = (moles * GAS_CONSTANT * temperature) / volume;
    println!("The pressure using Ideal Gas Law is: {} Pascals", pressure);
}
fn td(){
    println!("enter time,distance in order");
    let mut time=String::new();
    std::io::stdin().read_line(&mut time).expect("failed to read line");
    let time:f32=time.trim().parse().expect("please type a number");
    let mut distance=String::new();
    std::io::stdin().read_line(&mut  distance).expect("failed to read line");
    let distance:f32=distance.trim().parse().expect("please type a number");
    let acceleration = (2.0*distance)/time.powf(2.0);
    println!("the acceleration is:{}",acceleration);
}
fn fm(){
    println!("enter force and mass in order");
    let mut force=String::new();
    std::io::stdin().read_line(&mut force).expect("failed to read line");
    let force:f32=force.trim().parse().expect("print error");
    let mut mass=String::new();
    std::io::stdin().read_line(&mut mass).expect("failed to read line");
    let mass:f32=mass.trim().parse().expect("print error");
    let acceleration = force/mass;
    println!("the acceleration is:{}",acceleration);
}
fn cf() {
    println!("enter velocity and radius details ");
    let mut velocity=String::new();
    std::io::stdin().read_line(&mut velocity).expect("failed to read line");
    let velocity:f32=velocity.trim().parse().expect("please type a number");
    let mut radius=String::new();
    std::io::stdin().read_line(&mut radius).expect("failed to read line");
    let radius:f32=radius.trim().parse().expect("please type a number");
    let acceleration= velocity.powf(2.0)/radius;
    println!("the acceleration is {}",acceleration);
}
fn dis(){
    println!("1:: when velocity [initial and final] and  time is given");
    println!("2:: when displacement,time and initial velocity");
    println!("3:: when velocity[initial anf final] and displacement  is given");
    println!("enter choice: ");
    let mut choice =String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice:u8=choice.trim().parse().expect("please type a number");
    if choice==1{
        println!("enter final velcity,time,initial velocity in order");
        let mut final_velocity=String::new();
        std::io::stdin().read_line(&mut final_velocity).expect("failed to read line");
        let final_velocity:f32=final_velocity.trim().parse().expect("please type a number");
        let mut time=String::new();
        std::io::stdin().read_line(&mut time).expect("failed to read line");
        let time:f32=time.trim().parse().expect("please type a number");
        let mut initial_velocity=String::new();
        std::io::stdin().read_line(&mut initial_velocity).expect("failed to read line");
        let initial_velocity:f32=initial_velocity.trim().parse().expect("please type a number");
        let acceleration=(final_velocity-initial_velocity)/time;
        println!("Acceleration: {}",acceleration);   
    }
    else if choice==2{
        println!("enter  displacement,time,initial velocity in order");
        let mut s= String::new();
        std::io::stdin().read_line(&mut s).expect("failed to read number ");
        let s:f32=s.trim().parse().expect("please type a number");
        let mut time=String::new();
        std::io::stdin().read_line(&mut time).expect("failed to read line");
        let time:f32=time.trim().parse().expect("please type a number");
        let mut initial_velocity=String::new();
        std::io::stdin().read_line(&mut initial_velocity).expect("failed to read line");
        let initial_velocity:f32=initial_velocity.trim().parse().expect("please type a number");
        let acceleration=(s-time*initial_velocity)/time.powf(2.0);
        println!("Acceleration: {}",acceleration);
    }
    else if choice==3{
        println!("enter initial velocity,final velocity,displacement in order");
        let mut initial_velocity=String::new();
        std::io::stdin().read_line(&mut initial_velocity).expect("failed to read line");
        let initial_velocity:f32=initial_velocity.trim().parse().expect("please type a number");
        let mut final_velocity=String::new();
        std::io::stdin().read_line(&mut final_velocity).expect("failed to read line");
        let final_velocity:f32=final_velocity.trim().parse().expect("please type a number");
        let mut s= String::new();
        std::io::stdin().read_line(&mut s).expect("failed to read line");
        let s:f32=s.trim().parse().expect("please type a number");
        let acceleration=(final_velocity.powf(2.0)-initial_velocity.powf(2.0))/(2.0*s);
        println!("Acceleration: {}",acceleration);
    }
    else{
        println!("invalid choice");
    }
}


