//! Ejemplos de tipos escalares en Rust.
//!
//! Módulo didáctico que muestra los cuatro grupos de
//! tipos escalares: enteros, flotantes, booleanos y caracteres.

/// Imprime un pequeño repaso de los tipos escalares.
///
/// # Ejemplo
///
/// ```
/// clase01::scalar_type();
/// ```
///
/// # Salida estándar (aprox.)
/// ```text
/// ------Tipos escalares------
/// Integers: i8, i16, i32, i64, i128, isize
/// x = 1000
/// Unsigned integers: u8, u16, u32, u64, u128, usize
/// y = 20
/// Floats: f32, f64
/// z = 3.14
/// Boolean: bool
/// b = true
/// Characters: char
/// c = a
/// ```
pub fn scalar_type() {
    println!("------Tipos escalares------");
    println!("Integers: i8, i16, i32, i64, i128, isize");
    let x: i32 = 1_000;
    let x2 = -2_000; // inferido i32
    let x3 = 42i64; // sufijo explícito
    println!("x = {}", x);
    println!("Unsigned integers: u8, u16, u32, u64, u128, usize" );
    let y: u32 = 20;
    println!("y = {}", y);
    println!("Floats: f32, f64");
    let z: f32 = 3.14;
    let z1 = 3.14; // inferido f64
    println!("z = {}", z);
    println!("Boolean: bool");
    let b: bool = true;
    println!("b = {}", b);
    println!("Characters: char");
    let c: char = 'a';
    println!("c = {}", c);
}
