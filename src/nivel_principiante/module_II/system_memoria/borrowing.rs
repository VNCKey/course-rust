use std::mem::{size_of, size_of_val};

fn main() {
    let texto: String = String::from("Rûst");

    println!("let texto: String = String::from(\"Rûst\");");
    println!(" ╔ texto");
    println!(" ║    └─ Stack");
    println!(" ║        ├─ Memoria: {:p}", &texto);
    println!(" ║        ├─ Len: {} bytes", texto.len());
    println!(
        " ║        ├─ Capacity: {:?} bytes reservados",
        texto.capacity()
    );
    println!(
        " ║        └─ ptr: {:?} puntero al buffer interno que vive en el heap",
        texto.as_ptr()
    );
    println!(" ╚    └─ Heap");
    println!("          ├─ Contenido: [R,û,s,t]");
    println!("          ├─ Bytes: {:?}", texto.as_bytes());
    println!("          └─ Memoria heap: {:p}", texto.as_ptr());

    let texto2: &String = &texto;
    println!("let texto2: String = texto.clone();");
    println!("-----------------");
    println!(" ╔ texto");
    println!(" ║    └─ Stack");
    println!(" ║        ├─ Memoria: {:p}", &texto);
    println!(" ║        ├─ Len: {} bytes", texto.len());
    println!(
        " ║        ├─ Capacity: {:?} bytes reservados",
        texto.capacity()
    );
    println!(
        " ║        └─ ptr: {:?} puntero al buffer interno que vive en el heap",
        texto.as_ptr()
    );
    println!(" ╚    └─ Heap");
    println!("          ├─ Contenido: [R,û,s,t]");
    println!("          ├─ Bytes: {:?}", texto.as_bytes());
    println!("          └─ Memoria heap: {:p}", texto.as_ptr());

    println!(" ╔ texto2");
    println!(" ║    └─ Stack");
    println!(" ║        ├─ Memoria: {:p}", &texto2);
    println!(" ║        ├─ Len: {} bytes", texto2.len());
    println!(
        " ║        ├─ Capacity: {:?} bytes reservados",
        texto2.capacity()
    );
    println!(
        " ║        └─ ptr: {:?} puntero al buffer interno que vive en el heap",
        texto2.as_ptr()
    );
    println!(" ╚    └─ Heap");
    println!("          ├─ Contenido: [R,û,s,t]");
    println!("          ├─ Bytes: {:?}", texto2.as_bytes());
    println!("          └─ Memoria heap: {:p}", texto2.as_ptr());
    // println!("{}",texto); ERROR!!!!
    println!("{}", texto2);
}
