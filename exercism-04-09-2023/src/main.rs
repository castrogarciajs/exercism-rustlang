fn main() {
    // fizz buzz

    /*
        Claro, el ejercicio Fizz Buzz es un problema común de programación que implica imprimir una serie de números y aplicar ciertas reglas a cada número. Aquí está la descripción del problema:

    Para cada número del 1 al 100, debes imprimir lo siguiente:

    Si el número es divisible por 3, imprime "Fizz" en lugar del número.
    Si el número es divisible por 5, imprime "Buzz" en lugar del número.
    Si el número es divisible tanto por 3 como por 5, imprime "FizzBuzz".
    De lo contrario, imprime el número tal como está.
         */

        for number in 1..=100 {
            if number % 3 == 0 {
                println!("Fizz");
            } else if number % 5 == 0{
                println!("Buzz");
            } else {
                println!("fizzBuzz");
            }
        }
    println!("Hello, world!");
}
