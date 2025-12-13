// Variables con let
fn main() {
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
fn main() {
    // Las variables mutables
    let mut carro = "Toyota";

    println!("Mi carro es {}", carro);

    carro = "Honda";

    println!("Mi nuevo carro es {}", carro);
}

// Sombrear variables
fn main() {
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
    //tiene dirección única; todos los usos comparten
    // la misma ubicación de memoria.
    println!("Static: {}", APP_NAME);
}

// Statements
fn main() {
    // Los statements son instrucciones que realizan acciones
    // pero no producen un valor.
    // Terminan en ;
    let x = 5; // let nunca devuelve nada.
    // let = statement
    // No pueden ser anidadas dentro de expresiones
    //  let y = (let x = 5); ERROR!!!!

    // En python y = (x := 5)  es igual a  y = 5
    // En JavaScript y = (x = 5)  es igual a  y = 5
    // let y = (x = 1, x++);  // valor devuelto: 1, x vale 2
    // Rust evita ese side-effect oculto.
    // En Kotlin val y = (x = 5)  es igual a  y = 5
    // En Go y := (x = 5)  es igual a  y = 5
    // Rust: let → statement → no valor → no se puede envolver.
    // Rust prohíbe que let sea una expresión para eliminar una
    // clase entera de errores que sí existen en lenguajes donde
    // la asignación devuelve valor.
    //
    // side-effects ocultos
    // JAVASCRIPT------------
    // let x = 1;
    // let y = (x = 2, x++);   // y = 2; x = 3
    // console.log(x, y);      // 3 2 → side-effect dentro de la expresión
    //
    //  Truthy / falsy → más fácil el error
    //
    // if (count = 0) { }   // 0 es falsy → nunca entra, pero *asigna*
    // JAVASCRIPT------------
    //
    // PYTHON--------------
    // Múltiples walrus en una línea
    //
    // a = (b := 1) + (b := 2)   # a = 3, b = 2 (orden izq→der)
    // # legibilidad muy baja
    // PYTHON--------------
    //
    // JavaScript y Python permiten asignaciones dentro de expresiones,
    // por lo que sí sufren los side-effects
    // ocultos que Rust elimina por diseño.
    // Rust prohíbe que let devuelva valor → evita bugs clásicos como if (x = 5)
    // y expresiones con mutaciones sorpresa.
    // if x = 5 { } ERROR!!!

    if x == 5 {
        println!("x es igual a 5");
    } else {
        println!("x no es igual a 5");
    }

    // La expresión `x + y` se evalúa, pero su resultado se descarta
    x + y;

    // `println!` devuelve `()`, y la llamada como declaración
    // descarta ese valor
    println!("Hola");
    
    
    // Expresiones
    // Una expresión es cualquier construcción sintáctica 
    // que produce un valor y tiene un tipo inferible. 
    // Las expresiones no llevan punto y coma.
    let y = {
        let x = 3;
        x + 1 // no termina en ;  por lo que devuelve el valor
    };

    println!("x = {}, y = {}", x, y);
}

// Expresión
fn main() {
    // Todo lo siguiente son expresiones
    let a = 5; // 5 es una expresión
    let b = a + 1; // a + 1 es una expresión
    let c = if a > 3 { 10 } else { 20 };

    println!("a={}, b={}, c={}", a, b, c);
}
