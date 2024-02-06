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

// use core::panicking::panic;
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
        // Manejamos el error obteniendo el tipo con el método kind
        // dado en la librería estandar
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

/* Existe una variante enum io::NotFound que es dada por la librería
 * estandar que indicaría que el archivo que buscamos abrir no fue
 * encontrado. Al igual que con los otros enum.Al error ser un enum
 * podemos manejarlo con tipo de estructura de control
 */
fn _crear_un_archivo3() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

/* También existen métodos que permiten manejar errores sin
 * necesidad de utilizar un match que el enum Result<T,E>
 * nos provee.
 *
 * unwrap(): Si el valor de Result es Ok, unwrap retornará el valor
 * contenido en el Ok, si este es Err unwrap va a llamar a a la
 * macro panic!
 */
fn _crear_un_archivo4() {
    let _greeting_file = File::open("hello.txt").unwrap();
}

/* Usamos el método expect en la misma forma que unwrap con la
 * diferencia que nosotros establecemos un mensaje de error
 * diferente al habitual
 */

fn _crear_un_archivo5() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

/* Existe otro método llamado unwrap_or_else() este al igual que
 * unwrap y expect manejan el contenido de un Option<T> y proporciona
 * una forma de manejar el caso en caso que el resultado sea Err o None
 *
 * Además con este se agregan los "closures", cláusulas de cierre, funciones
 * anónimas o funciones lambda. En programacion un closure es un bloque de
 * código que puede ser almacenado en una variable o pasarse como argumento
 * a una función. "o más distintivo de los closures es su capacidad para
 * capturar y recordar el entorno en el que fueron creados, lo que les permite
 * acceder a variables y valores desde ese entorno incluso después de que la
 * función que los creó haya terminado de ejecutarse." La verdad no entiendo
 * cómo funciona del todo, porque reciben argumentos y luego parecen comportarse
 * como apuntadores a funciones en C. Pero se tocan los cloures a detalle en
 * el siguiente cap después del manejo de errores. Por ahora nos quedamos con
 * que permiten pasar un bloque de código como argumento y en caso de
 * unwrap_or_else nos permiten manejar el error e incluso retornar un valor
 */
fn _unwrap_or_else_prueba() {
    let maybe_value: Option<i32> = None;

    // unwrap_or_else se usa para proporcionar un valor predeterminado en caso de None
    let result = maybe_value.unwrap_or_else(|| {
        println!("No hay valor, proporcionando uno predeterminado");
        32
    });

    println!("Resultado: {}", result);
    println!("Maybe_Value: {:?}", maybe_value);
}

/* Propagando errores
 * Cuando la implementación de una función llama a algo que pueda fallar. En lugar
 * de manejar directamente el error en la función misma, puedes retornar un error
 * que retorna tu llamado. [Aquí existe más descipción en el libro pero no entendí
 * y no tengo wifi para consultar una traducción]
 */

// use std::fs::File;
use std::io::{self, Read};

// Esta función puede refactorizarse pero se deja así por fines de la enseñanza
fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/* Forma recortada de propagar errores con el operador ?
 * implementación de read_username pero usando el operador ?
 */
//use std::fs::File;
//use std::io::{self, Read};

fn _read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
/* El simbolo ? puesto después de un Result va a tener el casi el
 * mismo funcionamiento que la expresión match. Si el retorno del
 * result es el enum Ok, el valor contenido en este enum será
 * retornado y el programa continuará, de lo contrario el error
 * será retornado en la función como si explicitamente hubieramos
 * usado la palabra reservada return.
 *
 * Hay una diferencia bastante marcada en diferencia con la expresión
 * match: Cuando se utiliza el operador ?, los valores de error que
 * pasan a través de él invocan la función from, la cual está definida
 * en el trait From de la biblioteca estándar de Rust. El trait From es
 * utilizado para convertir un tipo en otro.
 *
 * Esta función permite convertir un tipo de error en otro. La idea es que
 * puedas tener diferentes tipos de errores en tu código, pero cuando
 * devuelves un Result desde una función, puedes especificar un único tipo de
 * error que representará todos los posibles errores que esa función pueda
 * encontrar.
 *
 * Por ejemplo, imagina que una función puede fallar por diversas razones,
 * cada una representada por un tipo de error diferente. Al usar el operador
 * ?, puedes convertir automáticamente esos errores específicos en un tipo de
 * error más general que refleje todas las posibles formas en que la función
 * podría fallar.
 *
 * En resumen, el operador ? en Rust no solo maneja el desempaquetado de
 * resultados y errores, sino que también permite la conversión de tipos de
 * error específicos a un tipo de error más general, facilitando así el
 * manejo consistente de errores en toda tu aplicación. Esto se logra a través
 * de la implementación de la función from en el trait From.
 * - Chatgpt 2024
 */

//use std::io;

// Definición del tipo de error personalizado
#[derive(Debug)]
struct OurError {
    _message: String,
}

// Implementación de From para convertir io::Error a OurError
impl From<io::Error> for OurError {
    fn from(_error: io::Error) -> Self {
        OurError {
            _message: format!("Ocurrió un error personalizado"),
        }
    }
}

fn _read_username_from_file3() -> Result<String, OurError> {
    let mut username = String::new();

    // Uso del operador ? para manejar errores y convertir de io::Error a OurError
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

/* Ten en cuenta que puedes usar el operador ? en un Result en una función
 * que devuelve Result, y puedes usar el operador ? en un Option en una
 * función que devuelve Option, pero no puedes mezclar y combinar. El
 * operador ? no convertirá automáticamente un Result en un Option o
 * viceversa; en esos casos, puedes utilizar métodos como el método ok en
 * Result o el método ok_or en Option para realizar la conversión
 * explícitamente.
 *
 * Hasta ahora, todas las funciones principales que hemos utilizado devuelven
 * (). La función principal es especial porque es el punto de entrada y salida
 * de los programas ejecutables, y hay restricciones sobre qué tipo de retorno
 * puede tener para que los programas se comporten como se espera.
 *
 *
 * Afortunadamente, la función principal también puede devolver un Result<(), E>.
 * El Listado 9-12 tiene el código del Listado 9-10, pero hemos cambiado el tipo
 * de retorno de la función principal a Result<(), Box<dyn Error>> y hemos agregado
 * un valor de retorno Ok(()) al final. Este código ahora se compilará:
 */

use std::error::Error;
//use std::fs::File;

fn _funcion() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}

/* Listado 9-12: Cambiar la función principal para devolver Result<(), E> permite el uso
 * del operador ? en valores de Result
 *
 * El tipo Box<dyn Error> es un objeto de trait, del cual hablaremos en la sección "Uso
 * de objetos de trait que permiten valores de diferentes tipos" en el Capítulo 17. Por
 * ahora, puedes entender Box<dyn Error> como "cualquier tipo de error". El uso de ? en
 * un valor de Result en una función principal con el tipo de error Box<dyn Error> está
 * permitido, porque permite que cualquier valor Err se devuelva temprano. Aunque el
 * cuerpo de esta función principal solo devolverá errores del tipo std::io::Error, al
 * especificar Box<dyn Error>, esta firma seguirá siendo correcta incluso si se agrega
 * más código que devuelve otros errores al cuerpo de la función principal.
 *
 *
 * Cuando una función principal devuelve un Result<(), E>, el ejecutable saldrá con un
 * valor de 0 si la función principal devuelve Ok(()) y saldrá con un valor distinto de
 * cero si la función principal devuelve un valor Err. Los ejecutables escritos en C
 * devuelven enteros cuando salen: los programas que salen con éxito devuelven el entero
 * 0, y los programas que tienen errores devuelven algún entero distinto de 0. Rust
 * también devuelve enteros en ejecutables para ser compatible con esta convención.
 *
 * La función principal puede devolver cualquier tipo que implemente el trait
 * std::process::Termination, que contiene una función report que devuelve un ExitCode.
 * Consulta la documentación de la biblioteca estándar para obtener más información sobre
 * la implementación del trait Termination para tus propios tipos.
 *
 * Ahora que hemos discutido los detalles de llamar a panic! o devolver Result, volvamos
 * al tema de cómo decidir cuál es apropiado usar en qué casos.
 */

/* Esta función si bien se ve bastante confusa y pa los más expertos en Rust bastante
 * Ineficiente, está hecha con el fin de explicar la función siguiente que es la que
 * viene en el ejemplo del libro. Es esta función tomamos una cadena como argumento,
 * le aplicamos el método .lines() que según cuantos saltos de linea encuentre los
 * convierte en un iterable, por ejemplo de tener "hola\ncomo\nestas" pasas a tener
 * {"hola", "como", "estas"}. El método .next() hace que avancemos una elemento en
 * un iterable, por lo que nos colocamos en el primer elemento con esto, pues parece
 * ser que con unicamente lines() no nos colocamos en la primer posición, pero debemos
 * manejar el Option en caso de que no existan cadenas que lines pueda obtener. De ahí
 * sobre nuestro primer elemento usamos el método chars() que devuelve un iterador de
 * caracteres que componen una cadena y pues con last() nos colocamos en el último elemento,
 * sin embargo puesto que last ya maneja el si se le pasa como argumento un None retornará
 * también un None para este caso permitiendo que nosotros manejemos este error como
 * necesitemos, pero en este caso unicamente devolveremos el caracter o None según sea el
 * caso. Ahora podemos omitir este if let haciendo uso del operador ?
 */
fn _last_char_of_first_line(text: &str) -> Option<char> {
    if let Some(first_line) = text.lines().next() {
        return first_line.chars().last();
    }
    None
}

fn _last_char_of_first_line2(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Caso error en main, significa, retorna "unit" () o el error
// Chatgpt:
/* El tipo () se utiliza principalmente para representar la ausencia de valor o la falta de
 * información. Es similar al concepto de void en algunos otros lenguajes de programación,
 * pero a diferencia de void, que generalmente indica que una función no devuelve ningún
 * valor, () no indica que no haya valor, sino que el único valor que tiene es la unidad
 * misma.
 *
 * Por ejemplo, una función en Rust que no devuelve ningún valor (similar a una función de
 * tipo void en otros lenguajes) tendría una firma de esta forma:
 
    fn mi_funcion() -> () {
        // Código de la función aquí
    }

 * O simplemente

    fn mi_funcion() {
        // Código de la función aquí
    }
 * En ambos casos, la función mi_funcion no devuelve nada útil en términos de valor, pero
 * técnicamente devuelve una unidad (). Esto es útil para indicar que la función se ejecutó
 * correctamente sin producir ningún resultado significativo.
 * - Chatgpt 2024
 * Lo otro que vamos a poner es un "trait" (se hablará a detalle más adelante) que significa
 * cualquier error
 */ 
pub fn main() -> Result<(), Box<dyn Error>>{
// En caso de manejar un Option
//pub fn main() -> Option<()>{
    // _func_panic();
    // _func_panic2();
    // _crear_un_archivo();
    // _crear_un_archivo2();
    // _crear_un_archivo3();
    // _crear_un_archivo4()
    // _crear_un_archivo5();
    // _unwrap_or_else_prueba();
    // _read_username_from_file();
    // _read_username_from_file2();
    // _read_username_from_file3();
    // _funcion();
    // _last_char_of_first_line("text").unwrap();
    // _last_char_of_first_line2("").unwrap();

    //En caso de manejar un option
    //"hola".lines().next()?;
    //Some(())

    //En caso de manejar un error
    let _f = File::open("a.txt")?;
    Ok(())
}
