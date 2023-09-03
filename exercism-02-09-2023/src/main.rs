fn main() {
    let regalos = [
        "un Patridge en un peral",
        "dos tórtolas",
        "tres gallinas francesas",
        "cuatro pájaros llamados Llameantes",
        "cinco anillos dorados",
        "seis ocas a-laying",
        "siete cisnes a-nadar",
        "ocho criadas de leche",
        "nueve damas bailando",
        "diez señores que saltan",
        "once pipers que flautan",
        "doce bateristas a tamborilear",
    ];

    let dias = [
        "primer",
        "segundo",
        "tercer",
        "cuarto",
        "quinto",
        "sexto",
        "séptimo",
        "octavo",
        "noveno",
        "décimo",
        "undécimo",
        "duodécimo",
    ];

    println!("Los Doce Días de Navidad:\n");

    for i in 0..12 {
        println!("En el {} día de Navidad, mi verdadero amor me dio:", dias[i]);
        for j in (0..=i).rev() {
            println!("{}", regalos[j]);
        }
        println!();
    }
}
