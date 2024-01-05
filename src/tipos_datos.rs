pub fn main() {
    const _CONSTANTE: u8 = 3;
    // Por defecto las variables son inmutables
    let mut _var_mutable;
    _var_mutable = 2;

    // Enteros
    // Entero de 8 bits, rango de -(2^7) a 2^7-1
    let _int_var: i8;
    // Entero de 16 bits, rango de -(2^15) a 2^15-1
    let _int_var: i16;
    // Entero de 32 bits, rango de -(2^31) a 2^31-1
    let _int_var: i32;
    // Entero de 64 bits, rango de -(2^63) a 2^63-1
    let _int_var: i64;
    // Entero de 128 bits, rango de -(2^127) a 2^127-1
    let _int_var: i128;
    // Entero de 64 o 32 bits segun la arquitectura de tu pc
    let _int_var: isize;

    //Sin signo
    // Entero de 8 bits, de 0 a 2^8-1
    let _int_var: u8;
    // Entero de 16 bits, rango de 0 a 2^16-1
    let _int_var: u16;
    // Entero de 32 bits, rango de 0 a 2^32-1
    let _int_var: u32;
    // Entero de 64 bits, rango de 0 a 2^64-1
    let _int_var: u64;
    // Entero de 128 bits, rango de 0 a 2^128-1
    let _int_var: u128;
    // Entero de 64 o 32 bits segun la arquitectura de tu pc
    let _int_var: usize;

    // Nums de coma flotante
    // flotante de 32 bits
    let _float: f32 = 3.0;
    // flotante de 64 bits
    let _float: f64 = 3.0;

    // Booleanos
    let _bool: bool = true;

    // Caracteres en unicode
    // ASCII contiene caracteres que fueron pensados para el idioma ingles,
    // Unicode contiene los caracteres de casi todos los alfabetos del mundo
    let _char: char = 'a';

    // Convertir de cadena a algun num(?)
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Strings: No son tipos de dato primitivos y es más bien una estructura
    // de datos. En rust existen dos tipos, String y str.

    // Str: puede volverse mutable y reemplazar su valor pero nunca a un num
    // de caracteres mayor al establecido al inicio, se almacena en el Stack
    let mut _str: &str = "Hola";
    _str = "Adio";

    //String: Es una cadena en el heap, puede variar en tamaño
    let mut _string = String::new(); //Una nueva cadena en el heap, vacía
    println!("Cadena: {_string}");

    // Para agregar caracteres uno por uno con el método push
    _string.push('H');
    println!("push: {_string}");

    // O agregamos cadenas
    _string.push_str("ola mundo");
    println!("Push str: {_string}");

    // Podemos tmb eliminar caracteres con
    _string.pop();
    println!("eliminar caracteres con pop: {_string}");

    //Crear una cadena e inicializarla
    let mut _string = String::from("Adiós mundo");
    println!("Crear una cadena e inicializarla: {_string}");

    //Cadena String de un str
    let mut _string = _str.to_string();
    println!("Cadena String de un str: {_string}");

    // Tambien podemos hacer un str de un String
    let _str = _string;
}
