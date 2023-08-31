use std::io;

fn main() {
    // - Convertir temperaturas entre Fahrenheit a Celsius.

    println!("Ingresa los grados celsisus😀: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error en la entrada");

    let input: f64 = input.trim().parse().expect("Error no es un numero");

    println!("{}", input);
    println!("Hello, world!");

    let result = fahrenheit_to_celsisu(input);

    println!(
        "grados f: {} son aproximadamente {} grados celsius",
        input, result
    );
}

fn fahrenheit_to_celsisu(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}
