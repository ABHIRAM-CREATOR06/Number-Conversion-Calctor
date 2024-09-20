use std::io;
pub fn convert(){
    println!("weclome to time conversion");
    println!("INPUT TYPE");
    println!("1:: Hours entered ");
    println!("2:: Minutes entered");
    println!("3:: Seconds entered");
    let mut choice=String::new();
    io::stdin().read_line(& mut choice).expect("failed to readline");
    let choice:u8=choice.trim().parse().expect("input a integer");
    match choice {
        1=>hour(),
        2=>minutes(),
        3=>seconds(),
        _ =>println!(":::Invalid selection:::"),
    }
}
fn hour(){
    println!("enter hour value:: ");
    let mut hour=String::new();
    io::stdin().read_line(&mut hour).expect("failed to readline:");
    let hour:f64=hour.trim().parse().expect("Failed to read the input");
    let minutes=hour*60.0;
    let seconds=minutes*60.0;
    println!("{} hours is equal to {} minutes and {} seconds",hour,minutes,seconds);
}
fn  minutes(){
    println!("enter minutes value:: ");
    let mut minutes=String::new();
    io::stdin().read_line(&mut minutes).expect("failed to readline:");
    let minutes:f64=minutes.trim().parse().expect("Failed to read the input");
    let hours=minutes/60.0;
    let seconds=minutes*60.0;
    println!("{} minutes is equal to {} hours and {} seconds",minutes,hours,seconds);
}

fn seconds(){
    println!("enter seconds value:: ");
    let  mut seconds=String::new();
    io::stdin().read_line(&mut seconds).expect("failed to readline:");
    let seconds:f64=seconds.trim().parse().expect("Failed to read the input");
    let minutes=seconds/60.0;
    let hours=seconds/3600.0;
    println!("{} seconds is equalt to {} minutes and {} hours",seconds,minutes,hours);
}
pub fn calculate(){
    println!("welcome to time conversion");
    println!(":::Here we would be dealing on the concept of:- ");
    println!("1:: Time difference");
    println!("2:: Efficiency");
    let mut  choice=String::new();
    io::stdin().read_line(&mut choice).expect("failed to readline");
    let choice:u8=choice.trim().parse().expect("failed to read  the input");
    match choice {
        1=>time_difference(),
        2=>efficiency(),
        _=>println!(":::Invalid selection:::"),
    }
}
fn time_difference(){
    let mut input = String::new();

    println!("Enter the first time (hours, minutes, seconds):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    let hours1 = parts.next().unwrap().parse::<f64>().expect("Please type numbers!");
    let minutes1 = parts.next().unwrap().parse::<f64>().expect("Please type numbers!");
    let seconds1 = parts.next().unwrap().parse::<f64>().expect("Please type numbers!");
    input.clear();
    println!("Enter the second time (hours, minutes, seconds):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    let hours2 = parts.next().unwrap().parse::<f64>().expect("Please type numbers!");
    let minutes2 = parts.next().unwrap().parse::<f64>().expect("Please type numbers!");
    let seconds2 = parts.next().unwrap().parse::<f64>().expect("Please type numbers!");
    let total_seconds1 = hours1 * 3600.0 + minutes1 * 60.0 + seconds1;
    let total_seconds2 = hours2 * 3600.0 + minutes2 * 60.0 + seconds2;
    let time_difference_seconds = (total_seconds2 - total_seconds1).abs();
    let hours = time_difference_seconds / 3600.0;
    let minutes = (time_difference_seconds % 3600.0) / 60.0;
    let seconds = time_difference_seconds % 60.0;
    println!("The time difference is {} hours, {} minutes, and {} seconds", hours, minutes, seconds);
}

fn efficiency(){
    println!("Note:::::::: Workdone/minute as efficiency::");
    let mut work_done_input = String::new();
    let mut time_taken_input = String::new();

    println!("Enter the amount of work done [out of 100]:");
    io::stdin().read_line(&mut work_done_input).expect("Failed to read line");
    let work_done: f64 = work_done_input.trim().parse().expect("Please enter a valid number");
    println!("Enter the time taken :");
    io::stdin().read_line(&mut time_taken_input).expect("Failed to read line");
    let time_taken: f64 = time_taken_input.trim().parse().expect("Please enter a valid number");
    println!(":::::checking that entered time is in minutes:::::::: if yes... type 0.....else type 1");
    let mut ch = String::new();
    io::stdin().read_line(&mut ch).expect("Failed to read line");
    let ch:u8=ch.trim().parse().expect("failed to read  the input");
    if ch==1{
        println!(":::::enter base ::::::::");
        println!("1::Hours");
        println!("2::Seconds");
        let mut  base = String::new();
        io::stdin().read_line(&mut base).expect("Failed to read line");
        let  base:u8=base.trim().parse().expect("failed to read  the input");
        if base==1{
            println!(":::::entered the base value==Hours::::::::");
            if time_taken==0.0{
                println!("time taken cannot be zero");
            }
            else{
                let work_done_efficiency = work_done / (time_taken * 60.0);
                println!("The efficiency is {} %", work_done_efficiency*100.0);
            }
        }
        else if base==2{
            println!(":::::entered the base value==Seconds::::::::");
            if time_taken==0.0{
                println!("time taken cannot be zero");
            } else {
                let work_done_efficiency = work_done / (time_taken/60.0);
                println!("The efficiency is {} %", work_done_efficiency*100.0);
            }
            
        }
    }
    else if ch==0 {
        if time_taken == 0.0 {
            println!("Time taken cannot be zero!");
        } 
        else {
            let efficiency = work_done / time_taken;
            println!("Efficiency: {} %", efficiency*100.0);
        }
    
    }
    else{
        println!("Invalid input");
    }
}

 

