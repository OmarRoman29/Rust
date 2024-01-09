/* Rust tiene una estructura de control bastante poderosa llamada
 * match que te permite comparar un valor con una serie de patrones
 * y después ejecuta código según el patrón que coincida
 *
 * Se hablará más a futuro de los tipos de patrones y qué hacen. El
 * poder de match reside en qué tan expresivos sean los patrones y
 * que el compilador comprueba que todos los casos son llamados
 *
 * Una metáfora a esto puede ser una máquina que ordena monedas.
 * todas las monedas entran por la misma entrada y es en el filtrado
 * donde cada una va a donde corresponde
 */

enum Monedas {
    _Cobre,
    _Plata,
    _Oro,
}

/* Esto puede escribirse de forma más apropiada retornando directo
 * del match pero es más para no olvidarme que hay un retorno.
 * Cuando se usa código pequeño no se suelen usar llaves pero match
 * admite la ejecuciónd de todo un bloque de código que retorne algo
 * al final. Lo unico que no entiendo es por qué dejan que se ponga
 * o no al final de las llaves una coma, pero no hay pp
 *
 * A cada opción del match le llaman "arms"
 */
fn convertir_en_pesos(moneda: Monedas) -> u8 {
    let a = match moneda {
        Monedas::_Cobre => {
            println!(":C");
            1
        }
        Monedas::_Plata => 10,
        Monedas::_Oro => 100,
    };

    return a;
}

/* "Patrones que unen valores"
 * Otra función interesante de los arms de un match es que pueden
 * extraer valores contenidos en un enum
 */

enum Mascotas {
    Ninguna,
    Gato(String),
}

fn mostrar_mascota(mascota: Mascotas) {
    // Si omitimos algun enumerable el compilador nos va a dar error
    match mascota {
        Mascotas::Ninguna => println!("Te conseguiremos un gato"),
        Mascotas::Gato(color) => println!("Tienes un gato {}", color),
    }
}

// De esta forma podemos obtener los datos genéricos de algun Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/* Pero qué pasa si tenemos muchas opciones y la funcionalidad que debemos
 * agregar solo afecta a 2? no podemos poner None en cada uno. Para eso existe
 * la palabra reservada other o _
 */
fn eres_millonario(moneda: Monedas){
    match moneda {
        Monedas::_Oro => println!("Eres millonario"),
        // other => println!("No eres millonario"),
        _ => println!("No eres millonario"),
    }
}


pub fn main() {
    let moneda = Monedas::_Cobre;
    println!("Conversión: {}", convertir_en_pesos(moneda));

    let mascota = Mascotas::Gato(String::from("Amarillo"));
    mostrar_mascota(mascota);

    let _five = Some(5);
    let _six = plus_one(_five);
    let _none = plus_one(None);

    let moneda = Monedas::_Cobre; 
    eres_millonario(moneda);

    let var = "Hola";
    //Match también admite otros tipos, pero no pude hacer que
    //buscara match con String
    let var = match var {
        "Hola" => true,
        other => false,
    };
}
