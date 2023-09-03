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

    println!("{}", result);
}

fn substring(txt_string: &str) -> &str {
    return &txt_string[1..];
}

fn char_at(txt_string: &str) -> char {
    if let Some(first_txt_string) = txt_string.chars().next() {
        return first_txt_string;
    } else {
        return 'E';
    }
}

fn revert_string(subs_tring: &str) -> String {
    if subs_tring.is_empty() {
        return String::new();
    } else {
        let first_char = char_at(subs_tring);
        let rest = substring(subs_tring);
        return format!("{}{}", revert_string(rest), first_char);
    }
}
