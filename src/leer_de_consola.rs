//use std::io; por defecto
use std::io::{self, Write}; // Extra para la limpieza del buffer

// Print no agrega el salto de linea por defecto pero posiblemente
// para que se imprima de inmediato lo que se quiere mostrar se
// deba limpiar el buffer
fn print(input: String){
    print!("{input}");
    std::io::stdout().flush().expect("Error limpiando el buffer (?)");
}

pub fn main() {
    // Leer dato por consola
    // Creamos una cadena de texto en el heap
    let mut index = String::new();
    println!("Ingresa un número: ");

    // Lees como cadena, y pasas por referencia la cadena
    // los pasos por referencia tmb son inmutables por defecto
    // y manejas un error en caso de no poder leer el dato
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // Conviertes la cadena a un dato usize en una variable del mismo nombre
    let index: usize = index
        // El método trim elimina espacios al inicio y al final
        .trim()
        // Parse convierte la cadena al tipo de dato numérico dado
        .parse()
        // Se maneja un error en caso de no ingresarse un número
        .expect("ERROR: El dato ingresado no es un número");

    println!("Entrada: {index}");

    let cadena = "   Holaaks djsd  ";
    // El método trim elimina espacios al inicio y al final
    println!("{}", cadena.trim());

    let mut input = String::new();
    print("Ingresa un numero: ".to_string());
    io::stdin().read_line(&mut input).expect("Error no se pudo leer dato");
    print("Dato ingresado: ".to_string());
    print(input);
}
