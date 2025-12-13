
// Variables con let
fn main(){
    // Las variables son inmutables por defecto
    let edad = 25;
    let numero = 50;
    
    // Primeras macros basicos
    // formas de imprimir
    println!("Mi edad es {}", edad);
    print!("Mi edad es {edad}");
    
    
    // Argumentos posicionales
    println!("{0} + {0} = {1}", edad, numero);
    
    // Nombres de argumento
    println!("{nombre} tiene {edad} años", nombre = "Ana", edad = 30);
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
    // println!("y = {}", y);  // ❌ Error: y no existe aquí
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


// Statements
fn main() {
    let x = 5;
    let y = {
        let z = 3;
        z + 1
    };

    println!("x = {}, y = {}", x, y);
}

// Expresión
fn main() {
    // Todo lo siguiente son expresiones
    let a = 5;              // 5 es una expresión
    let b = a + 1;          // a + 1 es una expresión
    let c = if a > 3 { 10 } else { 20 };

    println!("a={}, b={}, c={}", a, b, c);
}
