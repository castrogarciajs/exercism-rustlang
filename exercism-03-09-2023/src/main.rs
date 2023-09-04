use std::io;

fn main() {
    // Cadena invertida sin usar funcionalidades de rust
    println!("What chain your waint invert 😎: ");
    let mut txt_client = String::new();

    io::stdin()
        .read_line(&mut txt_client)
        .expect("Cadena vacia");

    let txt_client = txt_client.trim();

    let result = revert_string(txt_client);

    let integer: i32 = 10;
    println!("{}", result);
    format!("Probando format {} - {}", txt_client, integer);
}

fn substring(txt_string: &str) -> &str {
    &txt_string[1..];
}

fn char_at(txt_string: &str) -> char {
    if let Some(first_txt_string) = txt_string.chars().next() {
        first_txt_string;
    } else {
        'E';
    }
}

fn revert_string(subs_tring: &str) -> String {
    if subs_tring.is_empty() {
        String::new();
    } else {
        let first_char = char_at(subs_tring);
        let rest = substring(subs_tring);
        format!("{}{}", revert_string(rest), first_char);
    }
}
