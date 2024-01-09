/* Enum Option
 * Option es un enum definido en la librería estandar. Este tipo
 * codifica el escenario donde un valor es "algo" o "nada"
 *
 * Por ejemplo si consultas el primer elemento de una lista no
 * vacía obtendrás algo, si haces lo mismo en una lista con datos
 * obtendrás algo. Esta implementación permite al compilador
 * comprobar si ha manejado todos los cassos que se deberían
 * manejar. Esto permite manejar errores comunes en otros
 * lenguajes
 *
 * Rust no tiene NULL como en otros lenguajes, NULL representa
 * si existe o no valor.El problema que tiene NULL es que si
 * tratas de usarlo como si se tratara de un valor NOT-NULL se
 * obtendrá un error. En su lugar Rust tiene en enum Option<T>
 */

/* Está implementada de la siguiente forma
 * enum Option<T> {
 * None,
 * Some(T),
 * }
 */

/* Option es tan util que se incluye en el prelude, por lo que
 * no es necesario usar el prefijo Option::
 *
 * <T> es una función en Rust que representa un tipo genérico.
 * se tratará a detalle más adelante. <T> Implica que Some puede
 * tomar información de cualquier tipo.
 */

pub fn main() {
    let _some_number = Some(5); // Tipo Option<i32>
    let _some_char = Some('e'); // Tipo Option<char>

    // Aquí necesitamos especificar más info sobre su tipo
    let _absent_number: Option<i32> = None;

    /* Sin embargo algo no válido es hacer, porque no son del mismo tipo
     * pero tampoco podríamos sumar un Option<i32> + Option<i32> 
     * let result = _some_number + 2;
     *
     * Para esto, necesitamos convertir Option<T> a T
     */ 
}

// Quedó bastante vacío pero esto era más teoría
