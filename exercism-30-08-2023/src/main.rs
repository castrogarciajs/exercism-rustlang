use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    // #4 Juego de adivinanza
    // que el usuario intente adivinar el numero que la maquina esta pensando
    println!("Please input your guess😀: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    

    
        // Shadowing redefinir el valor de la variable con uno nuevo
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", input);
    
        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
