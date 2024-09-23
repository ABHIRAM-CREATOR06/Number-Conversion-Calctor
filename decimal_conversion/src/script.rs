 pub fn what_language(text: &str) {
    let info = detect(text).unwrap();
    println!("Language type: {:?}", info.lang());
    println!("Language Script: {:?}", info.script());
    println!("System confidence: {}", info.confidence());
    println!("System reliability: {}", info.is_reliable());

    if info.is_reliable() {
        println!("The detected language is reliable: True");
    } else {
        println!("The detected language is reliable: False");
    }
}