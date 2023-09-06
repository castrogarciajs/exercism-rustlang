fn main() {
    /*

        Ejercicio: Escribe un programa en Rust que calcule la suma de todos los números pares del 1 al 100 y la imprima en la pantalla.

    Puedes usar un bucle for para iterar sobre los números del 1 al 100 y una variable acumuladora para llevar un seguimiento de la suma de los números pares. */

    let mut suma = 0;

    for number in 1..=100 {
        if number % 2 == 0 {
            suma += number;
        }
    }
    println!("La suma de los numeros pares es: {}", suma);
    println!("Hello, world!");
}
