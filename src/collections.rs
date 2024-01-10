/* Rust en la librería estandar incluye una serie de
 * estructuras de datos bastante útiles, se les denomina
 * colecciones / collections. Los tipos de datos pueden
 * representar un valor específico, pero las colecciones
 * pueden contener múltiples valores. La información
 * almacenada en estas colecciones están almacenadas en
 * el heap. Lo que significa que el numero de datos o
 * tamaño no debe ser conocido en tiempo de compilación.
 * Cada una de estas estructuras tiene diferentes capacidades
 * y costos, y elegir la estructura adecuada para cada
 * situación dependerá de tu habilidad como programador.
 *
 * Tipos
 * - Vector: te permite almacenar un número indeterminado
 * de valores uno tras de otro
 * - String: Es una colección de caracteres.
 * - Hash Map: te permite asociar un valor particular a una
 * "clave", Es la implementación de una estructura de datos
 * llamada map
 *
 * Para más detalles, la documentación: https://doc.rust-lang.org/std/collections/index.html
 */

pub fn main() {
    /* Vect<T>: Permite almacenar datos de un mismo tipo uno
     * tras de otro y cambiar su tamaño en tiempo de ejecución
     * como no metemos ningún dato debemos espeificar el tipo
     */

    // Inicializar vacío
    let mut _v: Vec<i32> = Vec::new();
    println!("Vector: {:?}", _v);

    _v.push(1);
    _v.push(2);
    _v.push(3);
    println!("Vector: {:?}", _v);

    //inicializar con datos
    let _v = vec![1, 2, 3];

    println!("\nObtener datos de un vector");
    /* Obtener datos de un vector
     * De esta forma es insegura, pues podemos tratar de acceder
     * a un indice no existente, y obtendremos un "main panicjed at"
     */
    let third: &i32 = &_v[2];
    println!("The third element is {third}");

    /* Una forma más apropiada de manejar el caso en el que no exista
     * El método get() devuelve un Option<T> y permite manejar el caso
     * donde el indice buscado sea None
     */
    let third: Option<&i32> = _v.get(3);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    println!("\nFor en un vector");
    let mut vector = vec![12, 23, 1];
    println!("Vector {:?}\n", vector);
    let mut i = 1;
    for element in &vector {
        println!("Elemento {}: {}", i, element);
        i += 1;
    }

    println!("\nCambiar valores en un for: ");
    i = 1;
    for elemento in &mut vector {
        // Elemento es un puntero al elemento en el
        // vector y para cambiarlo accedemos al contenido
        // en el puntero
        *elemento = i;
        println!("Elemento {}: {}", i, elemento);
        i += 1;
    }

    // Podemos almacenar varios datos en un vector si usamos un enum
    println!("\nDiferentes tipos de dato en un vector");
    let _vector: Vec<VariosTipos> = Vec::new();
    let _vector = vec![
        VariosTipos::Int(3),
        VariosTipos::Float(3.1416),
        VariosTipos::Text(String::from("Hola")),
    ];

    for elemento in &_vector {
        elemento.obtener_datos();
    }

    // No se pierde el dato
    for elemento in &_vector {
        elemento.obtener_datos();
    }
    // Al igual que cualquier otra variable, cuando deja de usarse se
    // libera

    // Por si debiera usar un """array""" bidimensional dinámico por alguna wea (seguramente deba
    // hacerse refactorización)
    let mut votos: Vec<Vec<usize>> = Vec::new();
    let tope = 10;

    // Inicializar el arreglo bidimensional con valores predeterminados (por ejemplo, ceros)
    for _ in 0..tope {
        let fila = vec![0; tope];
        votos.push(fila);
    }

    for municipio in &votos {
        for _candidato in municipio {
            //aquí se accede al dato dentro del vector que está dentro del vector xd
        }
    }
}

enum VariosTipos {
    Int(i32),
    Float(f32),
    Text(String),
}

impl VariosTipos {
    fn obtener_datos(&self) {
        match self {
            VariosTipos::Int(dato) => println!("El entero almacenado es: {}", dato),
            VariosTipos::Float(dato) => println!("El flotante almacenado es: {}", dato),
            VariosTipos::Text(dato) => println!("El texto almacenado es: {}", dato),
        }
    }
}
