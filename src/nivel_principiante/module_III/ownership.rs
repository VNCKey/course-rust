// ownership + scope ejemplos
// === Scope y Ownership

// [source,rust,linenums]
// ----
// fn main() {
//     let mensaje = String::from("Hola");  // <1>

//     {
//         let saludo = mensaje;  // <2>
//         println!("Dentro: {saludo}");
//     }  // <3>

//     // println!("Fuera: {mensaje}");  // ❌ Error: valor movido
// }
// ----
// <1> `mensaje` es dueño del String
// <2> Ownership se mueve a `saludo`
// <3> `saludo` se destruye aquí, liberando la memoria

// [IMPORTANT]
// ====
// Este ejemplo muestra **ownership**: cuando `mensaje` se mueve a `saludo`, `mensaje` ya no es válido.
// ====
