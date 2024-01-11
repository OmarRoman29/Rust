use std::collections::HashMap;

pub fn main() {
    // Defines un conjunto de elementos al cual se le asocia
    // una "clave" de algún tipo, util para un gran volumen
    // de datos
    let mut scores: HashMap<String, u128> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get devuelve un some
    if let Some(dato) = scores.get("Blue") {
        println!("Dato: {dato}");
    } else {
        println!("No existe dato con ese hash");
    }

    // Get devuelve un some
    if let Some(dato) = scores.get("Blue") {
        println!("Dato: {dato}");
    } else {
        println!("No existe dato con ese hash");
    }

    // Pero parce ser que no es necesario
    let team_name = String::from("Blue");
    // Obtener una referencia al valor en el mapa
    let _score = scores.get(&team_name);
    // Obtener una copia de tipo Option<T>
    let _score = scores.get(&team_name).copied();

    // Retirar el Option<T> con unwrap_or(), en caso de None devuelve 0
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    /* Métodos
    * unwrap():
    * unwrap es un método que se puede llamar en un Option. Si
    * el Option es Some(value), devuelve value; de lo contrario,
    * si es None, desencadenará un panic.
    *
    * Es una forma concisa de extraer el valor de un Some cuando
    * estás seguro de que no es None. Sin embargo, se debe usar
    * con precaución, ya que un panic detiene la ejecución del
    * programa.
    *
    * unwrap_or():
    *
    * unwrap_or es un método que también se puede llamar en un
    * Option. Devuelve el valor contenido en Some, o un valor
    * predeterminado proporcionado como argumento si el Option es
    * None.
    *
    * Este método es útil cuando deseas proporcionar un valor de
    * respaldo para el caso en que el Option sea None.
    *
    * unwrap_or_else():

    * Similar a unwrap_or, unwrap_or_else es un método de Option
    * que toma una clausura como argumento en lugar de un valor
    * predeterminado. La clausura se ejecutará (o se calculará
    * dado sea el caso) y devolverá su resultado si el Option es
    * None.
    *
    * Es útil cuando calcular el valor predeterminado puede ser
    * costoso o depende de la lógica.
    */
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // No podemos usar field_name aquí porque se ha movido a la
    // propiedad del hashmap
    // let a = field_name;

    /* Un hash puede estar asociado a un único valor, pero varias
     * claves pueden tener asignado un mismo valor, En caso en el que quieras cambiar
     * un valor de un hashmap puedes sobreescribir el valor anterior accediendo en
     * la clave
     */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Si no existe Yellow, inserta 50
    scores.entry(String::from("Yellow")).or_insert(50);
    // Como ya existe blue, no hace ningún cambio
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    /* Entry devuelve un Entry, que es un enum del módulo hash_map
     * y puede tener valor Ocupied y Vacant
     *
     * or_insert(): Este método inserta un valor predeterminado si
     * la entrada no existe y devuelve una referencia mutable al
     * valor existente o al nuevo valor insertado.
     *
     * or_insert_with(): Similar a or_insert, pero en lugar de
     * proporcionar un valor predeterminado, se proporciona una
     * clausura que se ejecutará para calcular el valor a insertar
     * si el retorno es Vacant.
     *
     * and_modify(): Se ejecuta en una entrada ocupada y permite
     * modificar el valor existente.
     *
     * En todos los anteriores, el retorno es una referencia mutable
     * al valor existente (sea que el que se haya insertado o el que
     * ya existía)
     *
     * key(): Devuelve una referencia a la clave de la entrada.
     *
     * or_default(): Inserta el valor predeterminado del tipo si
     * la entrada no existe y devuelve una referencia mutable al
     * valor existente o al nuevo valor insertado.
     *
     * Posiblemente haya más y estarán detallados en la documentación
     */

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Split withespace retorna un conjunto de slices con base al texto
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(15);
        *count += 1;
    }

    println!("{:?}", map);

    /* Por defecto, HashMap utiliza una función de hashing llamada SipHash
     * que puede proporcionar resistencia a ataques de Denegación de Servicio
     * (DoS) que involucran tablas de hash1. Este no es el algoritmo de
     * hashing más rápido disponible, pero el compromiso entre mejor
     * seguridad y la disminución en el rendimiento vale la pena. Si perfilas
     * tu código y encuentras que la función de hash predeterminada es 
     * demasiado lenta para tus propósitos, puedes cambiar a otra función
     * especificando un hasher diferente. Un hasher es un tipo que implementa
     * el trait BuildHasher. Hablaremos sobre traits y cómo implementarlos en
     * el Capítulo 10. No necesariamente debes implementar tu propio hasher
     * desde cero; crates.io tiene bibliotecas compartidas por otros usuarios
     * de Rust que proporcionan hashers que implementan muchos algoritmos de hashing 
     */ 
}
