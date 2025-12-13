fn main() {
    let mensaje: &'static str = "Listo";
    let mensaje2: String = String::from("Rust");
    let mensaje3: &str = "Hola";

    println!("{mensaje}");
    println!("{mensaje2}");
    println!("{mensaje3}");
}
