use std::io;

fn main() {
    /*
        Ejercicio: Calculadora de Potencias

    Escribe un programa en Rust que solicite al usuario ingresar una base y un exponente, y calcule la potencia resultante. Implementa esto utilizando una función.

    Requisitos:

    Crea una función llamada power que tome dos parámetros: la base y el exponente.
    Dentro de la función, calcula y devuelve la potencia de la base elevada al exponente.
    Solicita al usuario ingresar la base y el exponente.
    Utiliza la función power para calcular y mostrar el resultado.
         */
    println!("Welcom Rust power 😀\n");

    let mut base = String::new();
    let mut exp = String::new();
    const ERROR_STDIN: &str = "ERROR STD IN";

    println!("Escribe la base 😁");
    io::stdin().read_line(&mut base).expect(&ERROR_STDIN);

    println!("Escribe el exponente 😁");
    io::stdin().read_line(&mut exp).expect(&ERROR_STDIN);

    let convert_base_to_number: f64 = base.trim().parse().expect("No es un numero");
    let convert_exp_to_number: i32 = exp.trim().parse().expect("No es un numero");

    let result = power(convert_base_to_number, convert_exp_to_number);
    println!("resultado: {}", result);
    println!("Hello, world!");
}

fn power(base: f64, exp: i32) -> f64 {
    return base.powi(exp);
}
