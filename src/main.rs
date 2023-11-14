fn main() {
    println!("Cifra de César!");
    let text = "Este é um texto de teste.";
    let shift = 3;
    let result = cesar_cipher(text, shift);
    println!("{} -> {}", text, result);
}

fn cesar_cipher(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        let mut new_char = c as u8 + shift;
        if new_char > 122 {
            new_char = new_char - 26;
        }
        result.push(new_char as char);
    }
    result
}
