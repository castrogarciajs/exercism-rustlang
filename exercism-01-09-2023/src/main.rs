fn main() {

    // fibonnaci
    let mut table_multiplicacion = 1;

    loop {

        table_multiplicacion += 1;

        println!("{} * 2 = {} ", table_multiplicacion, table_multiplicacion * 2);
        if table_multiplicacion == 10 {
            let table_multiplicacion = table_multiplicacion * 2;
            println!("{}", table_multiplicacion);
            break;
        }
        
    }
    for i in 0..=10 {
        println!("Fibonacci({}) = {}", i, fibonnaci_recursive(i));
    }
    println!("Hello, world!");
}


fn fibonnaci_recursive(n: u64) -> u64 {

    if n <= 1 {
        return n;
    }

    return fibonnaci_recursive(n - 1) + fibonnaci_recursive(n - 2);
}