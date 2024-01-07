// Crear una función que recibe dos argumentos
fn funcion(argumento1: u16, argumento2: u16) {
    println!("Esta es una función custom");

    let variable = argumento2 + argumento1;
    println!("Suma de arg1 y arg2: {variable}");
}

// Función con retorno tipo i32, rust automaticamente retorna
// el valor del último bloque de código escrito en la función
// siempre y cuando no se agregue el ; al final de este.
fn func_retorno() -> i32 {
    println!("Hola");
    // Si colocamos 5; recibiremos un error
    5
}

// también podemos hacer uso de la palabra reservada return
fn multiplicar(arg1: i32, arg2: i32) -> i32 {
    return arg1 * arg2;
}

pub fn main() {
    println!("\nFUNCIONES\n");
    funcion(2, 3);

    let var = func_retorno();
    println!("Valor de var: {var}");

    multiplicar(3, 9);

    // Expresión if. Si la expresión seguida de la palabra reservada if
    // es verdadera, se ejecuratá el bloque de código dentro de esta
    // sentencia. existe una setencia else para ejecutar otro bloque
    // de codigo en caso de ser esta expresión falsa
    println!("\nEXPRESIÓN IF\n");
    if var == 5 {
        println!("Si");
    } else {
        println!("No");
    }

    // Se pueden concatenar varias sentencias if y else de esta forma
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Debido a que if es una expresión, se puede usarse para asignar valores
    // pero en ambos bloques de código se debe tener el mismo tipo de dato
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Bucles. Existen 3 tipos de bucles en rust, loop, while y for

    // Si no hay una sentencia break en el ciclo, este se repetirá
    // indefinidamente hasta que se cierre manualmente el programa
    // también existe la palabra continue para que al llegar a esta
    // sentencia se pase automaticamente a la siguiente iteración
    println!("\nBUCLE LOOP\n");
    let mut var = 0;

    let var2 = loop {
        if var == 5 {
            // Break por sí mismo rompe el ciclo pero además puede retornar
            // un valor
            break "Hola";
        }

        var += 1;

        // Si vamos a asignar mediante un ciclo el valor de algo, debemos anexar
        // un ; al final del bloque del loop, de lo contrario lo imitimos
    };

    println!("{}", var2);

    // While. Es un bucle que se repetirá siempre que la condición de su
    // interior sea verdadera. Las sentencias break y continue también afectan
    // a este ciclo
    
    println!("\nBUCLE WHILE\n");
    var = 0;
    let arr = [50, 40, 30, 20, 10];
    while var < 5 {
        println!("Iteracion {var}: {}", arr[var]);
        var += 1;
    }
    println!("Fuera del while");

    println!("\nBUCLE FOR\n");
    // for. Este es más similar a un for each más que a un for como en C
    // Se define una variable elemento, que a cada itercación tendrá el
    // valor de cada dato en el arreglo arr
    for elemento in arr {
        println!("{elemento}");
    }

    // "Rust nos proporciona un operador de rango que se denota con dos
    // puntos como (..). Este operador nos permite definir un rango de
    // valores. Usando el operador de rango, podemos especificar los 
    // valores inicial y final del bloque de rango (inclusivo y exclusivo,
    // respectivamente)." El rango no incluye el valor final
    
    // El metodo rev invierte el rango
    for number in (1..4).rev() {
        println!("{number}");
    }
}
