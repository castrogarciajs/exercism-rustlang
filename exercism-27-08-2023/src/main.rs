use std::io;

fn main() {
    // # 01 - Escribe un programa en Rust que tome dos números enteros como entrada y permita al usuario seleccionar una operación (suma, resta, multiplicación o división). 
    // Luego, realiza la operación seleccionada y muestra el resultado.

    println!("Hello!, Welcome to calculator rust 🤓\n");

    println!("what operator do you want to perform? 🤔: ");

    
    let mut operator = String::new();
    const MESSAGE_ERROR: &str = "Error operator";

    io::stdin().read_line(&mut operator).expect(&MESSAGE_ERROR);

    // todo lo que ingrese el cliente sea escrito de la misma forma
    let response_operator_client: String = operator.trim().to_lowercase();

    let mut first_number = String::new();
    let mut second_number = String::new();

    println!("Insert the first number 😀: ");

    io::stdin()
        .read_line(&mut first_number)
        .expect(&MESSAGE_ERROR);

    println!("Insert the second number 😀: ");

    io::stdin()
        .read_line(&mut second_number)
        .expect(&MESSAGE_ERROR);

        
    let first_convert_to_number: Result<i32, _> = first_number.trim().parse();
    let second_convert_to_number: Result<i32, _> = second_number.trim().parse();

    // unwrap_or_default maneja resultados exitosos quiere decir que cuando exista una convercion o algun cambio de dato rust devuelve un Result y si necesitamos manejar el valor de un Resultado utilizaremos la funcion: unwrap_or_default
    if response_operator_client == "suma" {
        let result = first_convert_to_number.unwrap_or_default()
            + second_convert_to_number.unwrap_or_default();

        println!("Result is: {}", result);
    } else if response_operator_client == "resta" {
        let result = first_convert_to_number.unwrap_or_default()
            - second_convert_to_number.unwrap_or_default();

        println!("El resultado de tu resta es: {}", result);
    } else if response_operator_client == "multiplicacion" {
        let result = first_convert_to_number.unwrap_or_default()
            * second_convert_to_number.unwrap_or_default();

        println!("El resultado de tu multiplicacion es: {}", result);
    } else if response_operator_client == "division" {
        let result = first_convert_to_number.unwrap_or_default()
            / second_convert_to_number.unwrap_or_default();
        println!("El resultado de tu division es: {}", result);
    } else {
        println!("{}", MESSAGE_ERROR);
    }
}
