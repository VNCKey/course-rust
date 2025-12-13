fn main() {
    // La variable coche es propietaria del texto "Honda"
    // Cada valor tiene un unico propietario "owner"
    let coche: String = String::from("Honda");
    println!("Mi carro es {}", coche);
} // Final del scope y la variable coche llama a drop() 
  //y por lo tanto se libera la memoria y se destruye


// Copy
use std::mem::size_of;

fn main() {
    let numero: u8 = 5;
    let numero2 = numero;

    println!(" ╔ numero");
    println!(" ║  ├─ valor: {numero}");
    println!(" ║  └─ Memoria stack: {:p}", &numero);
    println!(" ╚ numero2");
    println!("    ├─ valor: {numero2}");
    println!("    └─ Memoria stack: {:p}", &numero2);

}

// Move
use std::mem::{size_of, size_of_val};

fn main() {
    let texto: String = String::from("Rûst");
    
    println!("let texto: String = String::from(\"Rûst\");");
    println!(" ╔ texto");
    println!(" ║    └─ Stack");
    println!(" ║        ├─ Memoria: {:p}", &texto);
    println!(" ║        ├─ Len: {} bytes", texto.len());
    println!(" ║        ├─ Capacity: {:?} bytes reservados", texto.capacity());
    println!(" ║        └─ ptr: {:?} puntero al buffer interno que vive en el heap",texto.as_ptr());
    println!(" ╚    └─ Heap");
    println!("          ├─ Contenido: [R,û,s,t]");
    println!("          ├─ Bytes: {:?}", texto.as_bytes());
    println!("          └─ Memoria heap: {:p}", texto.as_ptr());
    
    let texto2: String = texto;
    println!("let texto2: String = texto;");
    println!("-----------------");
    println!(" ╔ texto");
    println!(" ║    └─ Stack");
    println!(" ║        ├─ Memoria: ");
    println!(" ║        ├─ Len: ");
    println!(" ║        ├─ Capacity:");
    println!(" ║        └─ ptr:");
    println!(" ╚    └─ Heap");
    println!("          ├─ Contenido: ");
    println!("          ├─ Bytes: ");
    println!("          └─ Memoria heap:");
    
    println!(" ╔ texto2");
    println!(" ║    └─ Stack");
    println!(" ║        ├─ Memoria: {:p}", &texto2);
    println!(" ║        ├─ Len: {} bytes", texto2.len());
    println!(" ║        ├─ Capacity: {:?} bytes reservados", texto2.capacity());
    println!(" ║        └─ ptr: {:?} puntero al buffer interno que vive en el heap",texto2.as_ptr());
    println!(" ╚    └─ Heap");
    println!("          ├─ Contenido: [R,û,s,t]");
    println!("          ├─ Bytes: {:?}", texto2.as_bytes());
    println!("          └─ Memoria heap: {:p}", texto2.as_ptr());
    // println!("{}",texto); ERROR!!!!
    println!("{}",texto2);
}


// Clone
fn main(){
    // Transferencia de Ownership

    let coche = String::from("Honda");
    
    // coche se movio a coche2
    let coche2 = coche.clone();
    
    println!("Mi carro es {}", coche2);
    println!("Mi carro es {}", coche);
}

// Borrowing
