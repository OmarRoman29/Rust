/* El ownership es una caracteristica propia de Rust la cual se encarga
 * de asignar y liberar automáticamente la memoria sin la necesidad de
 * gestionar manualmente la memoria como en C o C++ y sin tener recolector
 * de basura como en otros lenguajes con el fin de prevenir errores
 *
 * Es fundamental para los programadores rastrear la memoria, ya que no
 * hacerlo puede resultar en una “fuga de memoria” - (Leak).
 *
 * Las reglas son:
 * Each value in Rust has an owner.
 * There can only be one owner at a time.
 * When the owner goes out of scope, the value will be dropped.
 */

pub fn main() {
    // hola aquí no existe
    let hola = "hola"; // Se crea hola y al ser inmutable se almacena en el stack
    println!("{hola}"); // Se usa hola
                        // Como de aquí en adelante hola no es usado, se libera automáticamente

    // Move
    let var1 = "var1";
    let var2 = var1;
    //Se crea una copia del valor de var1 en var2, y a su vez puedo llamar a ambos
    //datos sin problemas
    println!("{var1} {var2}");

    //Pero con un dato en el heap es diferente
    let var1 = String::from("Hola");
    // let _var2 = var1;
    // println!("{var1}"); // obtenemos error borrow of moved value: `var1` borrowed here after move

    /* Supongamos que tienes tu cadena en el heap, Rust maneja la memoria y se ve algo así:
     * | H | o | l | a | Tenemos los caracteres y sus posiciones, pero esto no es lo que tiene
     * | 0 | 1 | 2 | 3 | como tal var1. var1 se ve algo así:
     *
     * | ptr | [Puntero a la cadena anteriormente mostrada]
     * | len | 4
     * | cap | 4  <- esta es la capacidad
     *
     * Se tiene una serie de punteros a la estructura anteriormente creada con la cadena, entonces
     * cuando creamos var2, hacemos que ambos apunten a la misma estructura en vez de copiar el
     * valor como sucedía antes. Pero esto puede causar que Rust decida liberar var1, creo que
     * debido a la regla de tener un único dueño a la vez
     *
     * Si queremos mantener el valor de ambos, debemos "clonar" el valor, de forma que creas una
     * nueva estructura de datos con el mismo valor que se tiene, de forma que ambos son el único
     * dueño de su propio dato, en variables numéricas se implementa automáticamente el "copy
     * traid", pero en strings no, por lo que debemos llamar a un método
     */
    let _var2 = var1.clone();
    println!("{var1}");

    /* ¿Cómo afecta el ownership a las funciones? Pues cuando no pasamos por referencia los
     * valores es como si estuvieramos haciendo el var2 = var1 por lo que el owner ahora es
     * la variable de la función. Pero, cuando la función termine esa función es el encargado
     * de liberar la memoria de esa variable, y ya no podremos acceder al dato fuera de la
     * función
     */

    //_hacer_algo_sin_paso_por_referencia(var1);

    // Obtenemos error aquí porque el valor se "movió" en la función anterior
    // _hacer_algo_sin_paso_por_referencia(var1);

    /* Para esto rust tiene pasos de referencia mutables e inmutables, podemos pasar por referencia
     * el valor para evitar este comportamiento, pero en esta función unicamente queremos hacer una
     * consulta al valor y no modificarlo, por lo que no requerimos que sea mutable
     */

    // Esto se conoce como borrowing, porque "prestas" temporalmente el valor pero al final, var1
    // vuelve a ser el dueño de var1
    hacer_algo_con_paso_por_referencia(&var1);
    hacer_algo_con_paso_por_referencia(&var1);
    hacer_algo_con_paso_por_referencia(&var1);

    // Y seguimos teniendo acceso al valor de var1
}

fn _hacer_algo_sin_paso_por_referencia(x: String) {
    println!("{x}");
    //x se libera y no existe más
}

fn hacer_algo_con_paso_por_referencia(x: &String) {
    println!("{x}");
    // Se libera x también, pero tiene un puntero
    // a var1, no a la estructura que tiene la cadena
}

fn _retorno(x: String) -> String {
    x
    /* Aquí cuando se retorne el dueño será la variable que asigne el retorno y la liberación 
     * depende del scope de la variable a la que se le asigne el valor, si no se asinga a 
     * ninguna variable, se libera por sí sola cuando termine el scope de la función
     */ 
}
