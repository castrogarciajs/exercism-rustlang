fn main() {
    /*
        Instrucciones
    Invertir una cadena

    Por ejemplo: entrada: "cool" salida: "looc"

    Pruebe su función en esta cadena: uüu y vea qué sucede. Intente escribir una función que invierta correctamente esta cadena. Pista: grupos de grafemas

    Para ejecutar la prueba adicional, elimine el indicador de ignorar (#[ignorar]) de la última prueba y ejecute las pruebas con:
         */

    println!("Hello, world!");

    let result = reverse_string("cool");

    println!("{}", result);
}

fn reverse_string(input: &str) -> String {
    // chars -> ['c', 'o', 'o', 'l']
    let reverse_string = input.chars().rev().collect();

    return reverse_string;
}
