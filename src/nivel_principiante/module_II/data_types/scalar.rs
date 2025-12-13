fn main() {
    println!("------Tipos escalares------");

    println!("Integers: i8, i16, i32, i64, i128, isize");
    let x: i32 = 1_000;
    let x2 = -2_000; // inferido i32
    let x3 = 42i64; // sufijo explícito
    println!("x = {}", x);

    println!("Unsigned integers: u8, u16, u32, u64, u128, usize");
    // sin signo
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
