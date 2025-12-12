use std::any::type_name;
use std::mem::size_of;

pub fn stack() {
    let numero: u8 = 5;

    println!("main/");
    println!(" ├── numero");
    println!(" │  ├─ valor: {numero}");
    println!(" │  └─ Memoria stack: {:p}", &numero);

    stack_memory(numero);
}

pub fn stack_memory<T: Copy>(parameter: T) {
    let copia = parameter;
    println!(" │        ");
    println!(" │───────── fn_stack(<Dirección: {:p}>) ", &parameter);
    println!(" │          │");
    println!(" │          │── Tamaño de {}", type_name::<T>());
    println!(" │          │── {} bytes", size_of::<T>());
    println!(" │          │── {} bits", size_of::<T>() * 8);
    println!(" │          │");
    println!(" │          │ let copia = parameter;");
    println!(" │          │ Dirección de la copia: {:p}", &copia);
}
