use std::io;

fn main() {
    /*
        Calculadora de Números Primos:
    Crea un programa en Rust que determine si un número ingresado por el usuario es primo o no. Un número primo es aquel que solo es divisible por 1 y por sí mismo. El programa debe tomar un número como entrada y mostrar un mensaje indicando si el número es primo o no.

         */

    println!("Hello, world!");

    println!("Calculadora de Números Primos");
    println!("Ingrese un número para verificar si es primo:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");

    let numero: u32 = input
        .trim()
        .parse()
        .expect("Entrada no válida, por favor ingrese un número entero positivo.");

    if is_primo(numero) {
        println!("{} es un número primo.", numero);
    } else {
        println!("{} no es un número primo.", numero);
    }
}

fn is_primo(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num / 2) {
        if num % i == 0 {
            return false;
        }

    }
    return true;
}
