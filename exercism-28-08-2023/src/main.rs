use std::io;

fn main() {
    // #02 Ejercicio: Contador de Números Pares

    // Escribe un programa en Rust que solicite al usuario ingresar un  número entero.
    // El programa debe contar decir si el número es par y mostrar el resultado al final.


    println!("Bienvenido a rust par");

    println!("Quieres comprobar si tu numero es par?: 😁");

    let mut is_par = String::new();
    const ERROR_MESSAGE_LOG: &str = "Lo que ingresaste no es un numero";

    io::stdin().read_line(&mut is_par).expect(&ERROR_MESSAGE_LOG);

    let convert_to_number: Result<i64, _> = is_par.trim().parse();

    let result = convert_to_number.unwrap_or_default();

    if result % 2 == 0 {

        println!("El numero {} es un numero par", result);
    } else {
        println!("El numero {} no es un numero par", result);
    }
    
}

