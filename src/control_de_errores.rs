/* Los erroes son un hecho en el tiempo de vida del software, por lo que Rust
 * tiene múltiples funciones para manejar situaciones en las que algo sale mal.
 * En muchos casos Rust requiere de un acuse de recibido para alguna posibilidad
 * de error, y tomar acciones antes de que el código compile. Este requerimiento
 * hace a tu programa más robusto al asegurarte que descubrirás errores y los
 * manejarás de forma correcta antes de desplegar a producción.
 *
 * Rust agrupa erroes en dos principales categorías. recoverable y unrecoverable
 * errors, Para un error recuperable / recoverable como un archivo no encontrado
 * lo más probable es que sólo queramos informar del problema al usuario y volver
 * a intentar la operación. Errores no recuperables / unrecoverable son siempre
 * un síntoma de bugs, como acceder a una referencia después del fin de un array,
 * por lo que querremos finalizar inmediatamente el prorgama.
 *
 * Muchos lenguajes no diferencian entre estos tipos de errores, y los manejan de
 * la misma manera, usando exceptions. Rust no tiene exceptions. En su lugar
 * tiene el tipo Result<T, E> para erroes recuperables y la macro panic! que
 * detiene la ejecución cuando el programa encuentra un error no recuperable
 *
 * Errores no recuperables con panic!:
 * Hay veces en las que cosas malas pasan en nuestro código, y no hay nada que
 * podamos hacer para remediarlo. En este caso, Rust tiene la macro panic!. Hay
 * dos formas de causar panic! en la práctica: Causando un panic! en el código
 * como accediendo a un índice no válido en un array o llamando directamente a
 * la macro panic!. Por defecto un panic imprimirá un mensaje de error, hace
 * unwind, limpia el stack y finaliza. Con una variable de entorno puedes hacer
 * que Rust imprima la pila de llamadas cuando un panic ocurre para hacer más
 * sencillo para encontrar de forma más sencilla la fuente del panic.
 *
 * Unwinding the stack or aborting in response to a panic:
 * Por defecto cuando un panic ocurre el programa empieza un proceso de unwinding.
 * Lo que significa que Rust vuelve en el stack y limpia la informacipon de la
 * función que encuentra. Sin embargo, esta acción es bastante costosa, por lo
 * que Rust te permite elegir entre finalizar inmediatamente el programa sin esta
 * limpieza
 *
 * La memoria que el programa estaba usando tendrá que ser limpiada por el SO en
 * consecuencia. Si necesitas que el binario de tu programa sea lo más pequeño
 * posible, puedes cambiar del modo unwinding a aborting después de un panic
 * agregando "panic = 'abort'" en la seccion [profile] correspondiente en el
 * Cargo.toml. (Ya lo he cambiado en el toml)
 */

fn _func_panic() {
    /* Causamos un error con titulo del texto escrito en la macro
     * el código src/control_de_errores.rs:50:5: al ocurrir el panic indica qué
     * archivo, qué linea
     */
    panic!("Crash and burn");
}

fn _func_panic2() {
    /* Si estuvieramos en C, obtendríamos el resultado de lo que fuera que
     * estuviera en la memoria, así no le corresponda al array, o vector
     * pero Rust previene esto de forma normal
     */
    let vector = vec![1, 2, 3];
    vector[10];
}

/* Si compilamos con: RUST_BACKTRACE=1 cargo run
 * obtendremos un poco más de información de qué pasó exactamente. Backtrace es
 * una lista de todas las funciones que se llamaron para llegar a este punto.
 * Backtraces en rust funcionan como en otros lenguajes: la clave para leer el
 * backtrace es empezar desde arriba y leer hasta que vea los archivos que
 * escribimos. Este es el punto donde se originó el problema. Y las lineas
 * descritas debajo son el punto exacto en el archivo que causaron el error.
 *
 * La salida de esto será diferente dependiendo del sistema operativo y la
 * versión de Rust. Para obtener todos estos mensajes debemos tener los
 * "debug symbols" activados. Por defecto estos están activados cuando
 * compilamos usando cargo build y cargo run. Son omitidos cuando compilamos
 * con la flag --release
 */

use core::panicking::panic;
/* Errores recuperables con Result
 * La mayoría de los errores no necesitan que el programe finalice en su
 * totalidad. Muchas veces cuando una función falla, es por una razón que
 * puedes entender y responder en contra. Por ejemplo, si quieres abrir un
 * archivo pero la tarea falla porque el archivo no existe. Puede que quieras
 * crear el archivo en lugar de finalizar el programa. El enum result se define
 * de la siguiente forma:
 * enum Result<T, E> {
 *   Ok(T),
 *   Err(E),
 * }
 *
 * Donde T y E son tipos genpericos como parámetros. Se verá más a detalle los
 * tipos genéricos en un futuro. Lo que se necesita saber es que T representa
 * un dato que será retornado en caso de una tarea exitosa en la variante Ok,
 * E representa el tipo del error que será retornado en caso de la variante Err.
 * Debido a que Result tiene estos tipos de error como parámetros, podemos usar
 * el tipo Result y sus funciones implementadas en muchas situaciones diferentes
 * donde el valor de una operación exitosa y una de error deban ser diferentes
 *
 * Veamos un ejemplo
 */
use std::fs::File;
fn _crear_un_archivo() {
    // En esta parte mencionan mucho un "File handle", y la unica traducción
    // que econtré es identificador de archivo, pero se refiere a una referencia
    // en una variable de la cual podemos manipular el archivo una vez abierto

    /* El resultado de File::open es un Result<T,E>,
     * El parámetro genérico T ha sido cubierto por la implementación de
     * File::open con el tipo del valor de éxito, std::fs::File, que es un
     * una librería que puede manejar archivos. El tipo de E utilizado en el
     * valor de error es std::io::Error. Este tipo de retorno significa que la
     * llamada a File::open podría tener éxito y devolver un medio para manipular
     * el archivo al cual escribir. La llamada a la función también podría
     * fallar por razones como: el archivo podría no existir o es posible que no
     * tengamos permisos para acceder al archivo. La función File::open necesita
     * tener una forma de indicarnos si tuvo éxito o falló y, al mismo tiempo,
     * proporcionarnos tanto el identificador de archivo como la información de
     * error. Esta información es exactamente lo que transmite el enum Result
     *
     * En caso que File::open sea exitoso, el valor de la variable será una
     * instancia del valor en Ok, un identificador de archivo, de caso contrario
     * nuestra variable será una instancia de Err que contendrá más información
     * del error
     */
    let archivo = File::open("hola.txt");

    // Por ownership match liberaría la memoria de la variable
    let archivo = match archivo {
        // Obtenemos el T en Ok<T>
        Ok(archivo) => archivo,
        // Manejamos el error
        Err(error) => panic!("Error abriendo el archivo: {:?}", error),
    };
}

//use std::fs::File;
use std::io::ErrorKind;
fn _crear_un_archivo2() {
    // Si queremos manejar distintos errores
    let archivo = File::open("hola.txt");

    // Por ownership match liberaría la memoria de la variable
    let archivo = match archivo {
        // Obtenemos el T en Ok<T>
        Ok(archivo) => archivo,
        // Manejamos el error obteniendo el tipo
        Err(error) => match error.kind() {
            // Hacemos match
            ErrorKind::NotFound => match File::create("hola.txt") {
                //Creamos el archivo y de ser exitoso retornamos el identificador
                Ok(archivo_creado) => archivo_creado,
                Err(error) => panic!("Problema creando archivo: {:?}", error),
            },
            other_error => {
                panic!("Problema abriendo el archivo: {:?}", other_error);
            }
        }, // Por esta coma lo de arriba sigue siendo un retorno
    };
}

pub fn main() {
    // _func_panic();
    // _func_panic2();
    _crear_un_archivo();
}
