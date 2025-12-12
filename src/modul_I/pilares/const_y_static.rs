const VALOR_CONSTANTE: &str = "Texto constante";

pub static APP_NAME: &str = "MiApp";

pub fn constant_and_static() {
    //se inlinea: cada uso es una copia del valor literal.
    println!("Constante: {}", VALOR_CONSTANTE);
    //tiene dirección única; todos los usos comparten la misma ubicación de memoria.
    println!("Static: {}", APP_NAME);
}
