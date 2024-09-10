pub fn convert_base(number: f64, from: u32, to: u32) -> String {
    let integer = number.trunc() as u64;
    let fraction = number.fract();

    // Convert the integer part
    let mut integerb = String::new();
    let mut intp = integer;
    while intp > 0 {
        let remainder = intp % to as u64;
        integerb = format!("{}{}", remainder, integerb);
        intp /= to as u64;
    }

    // Convert the fractional part
    let mut fractionb = String::new();
    let mut fracp = fraction;
    for _ in 0..10 {
        fracp *= to as f64;
        let digit = fracp.trunc() as u64;
        fractionb.push_str(&digit.to_string());
        fracp -= digit as f64;
    }

    format!("{}.{}", integerb, fractionb)
}
