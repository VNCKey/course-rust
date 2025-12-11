pub fn variables() {
    let x = 5;

    println!("x = {}", x);
}

pub fn variables_mutability() {
    let mut x = 5;
    
    println!("x = {}", x);
    
    x += 1;

    println!("x = {}", x);
}


pub fn variables_shadowing() {
    let x = 5;
    let mut y = 10;

    let x = 15;
    let mut y = 20;

    println!("x = {}", x);
    println!("y = {}", y);
}

const VALOR_CONSTANTE = "Texto constante";

pub static APP_NAME: &str = "MiApp";

pub fn constant_and_static(){
    //se inlinea: cada uso es una copia del valor literal.
    println!("Constante: {}", VALOR_CONSTANTE);
    //tiene dirección única; todos los usos comparten la misma ubicación de memoria.
    println!("Static: {}", APP_NAME);
}

pub fn scope(){
    let x = 10;
    {
        let y = 20;
        println!("x = {}, y = {}", x, y);
    }
    println!("x = {}", x);
}

pub fn scalar_types() {
    println!("------Tipos escalares------");
    println!("Integers: i8, i16, i32, i64, i128, isize");
    let x: i32 = 1_000;
    let x2 = -2_000;    // inferido i32
    let x3 = 42i64;     // sufijo explícito
    println!("x = {}", x);
    println!("Unsigned integers: u8, u16, u32, u64, u128, usize");
    // sin signo
    let y: u32 = 20;
    println!("y = {}", y);
    println!("Floats: f32, f64");
    let z: f32 = 3.14;
    let z1 = 3.14;      // inferido f64
    println!("z = {}", z);
    println!("Boolean: bool");
    let b: bool = true;
    println!("b = {}", b);
    println!("Characters: char");
    let c: char = 'a';
    println!("c = {}", c);
}
