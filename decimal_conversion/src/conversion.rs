pub fn convert_base(number: &str, from: u32, to: u32) -> String {
    let (integer_part, fractional_part) = match number.split_once('.') {
        Some((int, frac)) => (int, frac),
        None => (number, ""),
    };

    let integer_value = u64::from_str_radix(integer_part, from).expect("Invalid integer part");
    let mut integer_result = String::new();
    let mut intp = integer_value;
    if intp == 0 {
        integer_result.push('0');
    } else {
        while intp > 0 {
            let remainder = intp % to as u64;
            integer_result = format!("{}{}", digit_to_char(remainder), integer_result);
            intp /= to as u64;
        }
    }

    let mut fraction_result = String::new();
    let mut fracp = fractional_part.chars().fold(0.0, |acc, c| {
        acc * from as f64 + char_to_digit(c) as f64
    }) / from.pow(fractional_part.len() as u32) as f64;

    for _ in 0..10 {
        fracp *= to as f64;
        let digit = fracp.trunc() as u64;
        fraction_result.push(digit_to_char(digit));
        fracp -= digit as f64;
    }

    format!("{}.{}", integer_result, fraction_result)
}

fn digit_to_char(digit: u64) -> char {
    if digit < 10 {
        (digit as u8 + b'0') as char
    } else {
        (digit as u8 - 10 + b'A') as char
    }
}

fn char_to_digit(c: char) -> u32 {
    if c.is_digit(10) {
        c.to_digit(10).unwrap()
    } else {
        c.to_digit(36).unwrap() - 10
    }
}
