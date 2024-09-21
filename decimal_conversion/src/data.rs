use std::io;

pub fn convert(){
    println!("Enter the value you want to convert:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let value: f64 = input.trim().parse().expect("Please enter a valid number");
    println!("Enter the unit of the value (e.g., byte, kilobyte, megabyte, etc.):");
    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Failed to read line");
    let unit = unit.trim().to_lowercase();
    let base = match unit.as_str() {
        "byte" => 1.0,
        "kilobyte" => 1024.0,
        "megabyte" => 1024.0 * 1024.0,
        "gigabyte" => 1024.0 * 1024.0 * 1024.0,
        "terabyte" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        "petabyte" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        "exabyte" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        "zettabyte" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        "yottabyte" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => {
            println!("Unknown unit");
            return;
        }
    };
    let mut size = value * base;
    let units = ["byte", "kilobyte", "megabyte", "gigabyte", "terabyte", "petabyte", "exabyte", "zettabyte", "yottabyte"];
    for unit in units.iter() {
        println!("{:.6} {}", size, unit); //accruacy upto 6 places
        size /= 1024.0;
    }
}

pub fn speed(){
    println!("Enter the amount of data transferred (in bytes):");
    let mut data_input = String::new();
    io::stdin().read_line(&mut data_input).expect("Failed to read line");
    let data: f64 = data_input.trim().parse().expect("Please enter a valid number");

    println!("Enter the time taken for the transfer (in seconds):");
    let mut time_input = String::new();
    io::stdin().read_line(&mut time_input).expect("Failed to read line");
    let time: f64 = time_input.trim().parse().expect("Please enter a valid number");

    let speed = data / time;
    println!("Data transfer speed: {:.2} bytes per second", speed);

    let speed_kbps = speed / 1024.0;
    let speed_mbps = speed_kbps / 1024.0;
    let speed_gbps = speed_mbps / 1024.0;

    println!("Data transfer speed: {:.2} KBps", speed_kbps);
    println!("Data transfer speed: {:.2} MBps", speed_mbps);
    println!("Data transfer speed: {:.2} GBps", speed_gbps);
}
pub fn calculate(){
    println!("Enter the number of data transfer events:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_events: usize = input.trim().parse().expect("Please enter a valid number");

    let mut data_transfers = Vec::new();
    let mut total_data = 0.0;
    for i in 1..=num_events {
        println!("Enter the amount of data transferred in event {} (in bytes):", i);
        let mut data_input = String::new();
        io::stdin().read_line(&mut data_input).expect("Failed to read line");
        let data: f64 = data_input.trim().parse().expect("Please enter a valid number");

        data_transfers.push(data);
        total_data += data;
    }
    println!("Total data transferred: {:.2} bytes", total_data);

    if num_events > 1 {
        println!("Differences between consecutive data transfers:");
        for i in 1..num_events {
            let difference = data_transfers[i] - data_transfers[i - 1];
            println!("Difference between event {} and event {}: {:.2} bytes", i, i + 1, difference);
        }
    }
    let units = ["bytes", "kilobytes", "megabytes", "gigabytes", "terabytes"];
    let mut size = total_data;

    for unit in units.iter() {
        println!("Total data transferred: {:.6} {}", size, unit);
        size /= 1024.0;
    }
}
pub fn stat(){
    println!("Enter the number of data transfer events:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_events: usize = input.trim().parse().expect("Please enter a valid number");

    let mut total_data = 0.0;
    let mut total_time = 0.0;
    let mut peak_speed = 0.0;

    for i in 1..=num_events {
        println!("Enter the amount of data transferred in event {} (in bytes):", i);
        let mut data_input = String::new();
        io::stdin().read_line(&mut data_input).expect("Failed to read line");
        let data: f64 = data_input.trim().parse().expect("Please enter a valid number");

        println!("Enter the time taken for event {} (in seconds):", i);
        let mut time_input = String::new();
        io::stdin().read_line(&mut time_input).expect("Failed to read line");
        let time: f64 = time_input.trim().parse().expect("Please enter a valid number");

        total_data += data;
        total_time += time;

        let speed = data / time;
        if speed > peak_speed {
            peak_speed = speed;
        }
    }

    let average_speed = total_data / total_time;

    println!("Total data transferred: {:.2} bytes", total_data);
    println!("Average speed: {:.2} bytes per second", average_speed);
    println!("Peak speed: {:.2} bytes per second", peak_speed);
}

