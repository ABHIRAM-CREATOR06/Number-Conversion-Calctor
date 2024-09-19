use std::io;
pub fn meter(){
    println!("enter  the value : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = input.trim().parse().expect("Please type a number");
    let kilometer=input/1000.0;
    println!("{} meter is equal to {} kilometer",input,kilometer);
    let centimeter=input*100.0;
    println!("{} meter is equal to {} centimeter",input,centimeter);
    let  millimeter=input*1000.0;
    println!("{} meter is equal to {} millimeter",input,millimeter);
    let   inch=input*39.37;
    println!("{} meter is equal to {} inch",input,inch);
    let   feet=input*3.2808;
    println!("{} meter is equal to {} feet",input,feet);
    let yard=input*1.0936132983;
    println!("{} meter is equal to {} yard",input,yard);
    let mile =input*0.000621371;
    println!("{} meter is equal to {} mile",input,mile);
}
pub fn kilometer(){
    println!("enter  the value : ");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = input.trim().parse().expect("Please type a number");
    let meter=input*1000.0;
    println!("{} kilometer is equal to {} meter",input,meter);
    let centimeter=input*1000.0*100.0;
    println!("{} kilometer is equal to {} centimeter",input,centimeter);
    let   millimeter=input*1000.0*1000.0;
    println!("{} kilometer is equal to {} millimeter",input,millimeter);
    let    inch=input*39370.0;
    println!("{} kilometer is equal to {} inch",input,inch);
    let     feet=input*3280.8;
    println!("{} kilometer is equal to {} feet",input,feet);
    let yard =input*1093.61;
    println!("{} kilometer is equal to {} yard",input,yard);
    let  mile =input*0.621371;
    println!("{} kilometer is equal to {} mile",input,mile);
}
pub fn centimeter(){
    println!("enter  the value : ");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = input.trim().parse().expect("Please type a number");
    let meter=input/100.0;
    println!("{} centimeter is equal to {} meter",input,meter);
    let kilometer=input/1000000.0;
    println!("{} centimeter is equal to {} kilometer",input,kilometer);
    let millimeter=input*10.0;
    println!("{}centimeter  is equal to {} millimeter",input, millimeter);
    let  inch=input*0.393701;
    println!("{} centimeter is equal to {} inch",input,inch);
    let  feet=input*0.010936133;
    println!("{} centimeter is equal to {} feet",input,feet);
    let yard=input *0.0010936133;
    println!("{} centimeter is equal to {} yard",input,yard);
    let mile =input*0.000010936133;
    println!("{} centimeter is equal to {} mile",input,mile);
}
pub fn millimeter(){
    println!("enter  the value : ");
    let  mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = input.trim().parse().expect("Please type a number");
    let meter=input/1000.0;
    println!("{} millimeter is equal to {} meter",input,meter);
    let kilometer=input/1000000.0;
    println!("{} millimeter is equal to {} kilometer",input,kilometer);
    let centimeter=input/10.0;
    println!("{} millimeter is equal to {} centimeter",input,centimeter);
    let inch=input/25.5;
    println!("{} millimeter is equal to {} centimeter",input,inch);
    let feet= input*0.003281;
    println!("{} millimeter is equal to {} feet",input,feet);
    let yard=input/914.4;
    println!("{} millimeter is equal to {}  yard",input,yard);
    let mile=input/ 1_609_344.0;
    println!("{} millimeter is equal to {} mile",input,mile);
}
pub fn inch(){
    println!("enter  the value : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = input.trim().parse().expect("Please type a number");
    let meter=input/39.37;
    println!("{} inch is equal to {} meter",input,meter);
    let kilometer=input/39370.0;
    println!("{} inch is equal to {} kilometer",input,kilometer);
    let centimeter=input*2.54;
    println!("{} inch is equal to {} centimeter",input,centimeter);
    let millimeter=input*25.4;
    println!("{} inch is equal to {} millimeter",input,millimeter);
    let feet=input/12.0;
    println!("{} inch is equal to {} feet",input,feet);
    let yard=input/36.0;
    println!("{} inch is equal to {} yard",input,yard);
    let mile=input/63360.0;
    println!("{} inch is equal to {} mile",input,mile);
}
pub fn feet(){
    println!("enter  the value : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = input.trim().parse().expect("Please type a number");
    let meter=input/3.2808;
    println!("{} feet is equal to {} meter",input,meter);
    let kilometer=input/3280.8;
    println!("{} feet is equal to {} kilometer",input,kilometer);
    let centimeter=input*30.48;
    println!("{} feet is equal to {} centimeter",input,centimeter);
    let millimeter=input*304.8;
    println!("{} feet is equal to {} millimeter",input,millimeter);
    let inch=input*12.0;
    println!("{} feet is equal to {} inch",input,inch);
    let yard=input/3.0;
    println!("{} feet is equal to {} yard",input,yard);
    let mile=input/5280.0;
    println!("{} feet is equal to {} mile",input,mile);
}

pub fn yard(){
    println!("enter  the value : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = input.trim().parse().expect("Please type a number");
    let meter=input/1.0936132983;
    println!("{} yard is equal to {} meter",input,meter);
    let kilometer=input/1093.6132983;
    println!("{} yard is equal to {} kilometer",input,kilometer);
    let centimeter=input*91.44;
    println!("{} yard is equal to {} centimeter",input,centimeter);
    let millimeter=input*914.4;
    println!("{} yard is equal to {} millimeter",input,millimeter);
    let inch=input*36.0;
    println!("{} yard is equal to {} inch",input,inch);
    let feet=input*3.0;
    println!("{} yard is equal to {} feet",input,feet);
    let mile=input/1760.0;
    println!("{} yard is equal to {} mile",input,mile);
}
pub fn mile(){
    println!("enter  the value : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = input.trim().parse().expect("Please type a number");
    let meter=input/0.000621371;
    println!("{} mile is equal to {} meter",input,meter);
    let kilometer=input/0.621371;
    println!("{} mile is equal to {} kilometer",input,kilometer);
    let centimeter=input/0.000010936133;
    println!("{} mile is equal to {} centimeter",input,centimeter);
    let millimeter=input/0.0000010936133;
    println!("{} mile is equal to {} millimeter",input,millimeter);
    let inch=input*63360.0;
    println!("{} mile is equal to {} inch",input,inch);
    let feet=input*5280.0;
    println!("{} mile is equal to {} feet",input,feet);
    let yard=input*1760.0;
    println!("{} mile is equal to {} yard",input,yard);
}

