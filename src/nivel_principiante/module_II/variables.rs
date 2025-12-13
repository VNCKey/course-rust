
// Variables con let
fn main(){
    // Las variables son inmutables por defecto
    let edad = 25;
    
    // formas de imprimir
    println!("Mi edad es {}", edad);
    println!("Mi edad es {edad}");
}

// Variables con let mut
fn main(){
    // Las variables mutables
    let mut carro = "Toyota";
    
    println!("Mi carro es {}", carro);
    
    carro = "Honda";
    
    println!("Mi nuevo carro es {}", carro);
}

// Sombrear variables
fn main(){
    let mut edad = 25;
    
    println!("Mi edad es {}", edad);
    
    let edad = 35;
    
    println!("Mi edad sombreada es {}", edad);
}

// Scope
fn main() {
    let x = 10;
    {
        let y = 20;
        println!("x = {}, y = {}", x, y);
    }
    println!("x = {}", x);
}



// const y static

const VALOR_CONSTANTE: &str = "Texto constante";

pub static APP_NAME: &str = "MiApp";

pub fn main() {
    //se inlinea: cada uso es una copia del valor literal.
    println!("Constante: {}", VALOR_CONSTANTE);
    //tiene dirección única; todos los usos comparten la misma ubicación de memoria.
    println!("Static: {}", APP_NAME);
}
