use std::mem::{size_of, size_of_val};

pub fn heap() {
    let texto: String = String::from("Rûst");

    println!("main/");
    println!(" ├── let texto: String = String::from(\"Rûst\");");
    println!(" ├── texto es propietario del buffer 'texto'");
    println!(" ├── String esta dividido en dos partes");
    println!(" │  └─ Stack");
    println!(" │    ├─ Memoria Stack: {:p}", &texto);
    println!(" │    ├─ Len: {} bytes", texto.len());
    println!(" │    ├─ Caracteres Unicode: {}", texto.chars().count());
    println!(" │    ├─ Capacity: {:?} bytes reservados", texto.capacity());
    println!(
        " │    └─ ptr: {:?} puntero al buffer interno que vive en el heap",
        texto.as_ptr()
    );
    println!(" │  └─ Heap");
    println!(" │    ├─ Contenido: {}", format_string(&texto));
    println!(" │    ├─ Bytes: {:?}", texto.as_bytes());
    println!(" │    └─ Memoria heap: {:p}", texto.as_ptr());

    fn_heap_memory(texto);
}

pub fn fn_heap_memory(parametro: String) {
    println!(" │ ");
    println!(" └─ fn_heap_memory(parametro: String)/");
    println!("   ├─ parametro es dueño del buffer 'texto' contiene ahora el:");
    println!("   ├─ len: {} bytes", parametro.len());
    println!("   ├─ capacity: {} bytes reservados", parametro.capacity());
    println!("   └─ buffer: {:p}", parametro.as_ptr());
    println!("      ├─ Contenido: {}", format_string(&parametro));
    println!("      └─ Bytes: {:?}", parametro.as_bytes());
    //println!("   ├─ x bits");
    //println!("   │");
    //println!("   │ let copia = parameter;");
    //println!("   │ Dirección de la copia: {:p}", &copia);
}

fn format_string(parameter: &String) -> String {
    let mut aux = String::from("[");

    for char in parameter.chars() {
        aux.push_str(&format!("'{}',", char));
    }
    aux.pop();
    aux.push(']');
    aux
}
