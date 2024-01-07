/* Una estructura o struct es un tipo de dato abstracto que permite
 * agrupar distintos tipos de datos primitvos bajo un nombre con
 * el fin de representar una entidad. En rust además de tener estas
 * variables que pueden almacenar datos pueden incluirse métodos
 *

 * Definición de una estructura llamada Usuario. dentro de los
 * dentro de las llaves se establecen los miembros que tendrá y
 * su tipo además del nombre con el que haremos referencia
 *
 * Cuando se hace una estructura tenemos un problema al usar &str.
 * Este tipo de dato puede darse el caso en el que se libere la 
 * memoria de la cadena antes de que se libere la memoria de 
 * la estructura completa, por lo que en determinado caso podríamos
 * acceder a un espacio de memoria en el cual no exista dado, que
 * es algo que no deseamos. Por lo que se pueden hacer dos cosas.
 * Uno usar String en su lugar, o configurar un "time life". Algo
 * que se hablará más adelante (y que al momento de escribir esto
 * aún no entiendo)
 */

struct Usuario {
    active: bool,
    username: String,
    age: u8,
}

// Crear estructuras usando tuplas sin nombre para sus miembros
struct RgbColor(i32, i32, i32);

// Métodos: En POO un método es una función propia de una clase, en
// Rust las estructuras también pueden contener métodos

// Definimos propiedades
struct AnimalVolador {
    nombre: String,
    edad: u8,
}

// Definimos métodos
impl AnimalVolador {
    /* Crear un método constructor, En lugar de AnimalVolador se puede
     * usar la palabra reservada Self (con mayuscula) para referrir al
     * mismo tipo de estructura. Toda función donde no se incluya como
     * argumento &self es llamada función asociada
     */
    pub fn new(nombre: String, edad: u8) -> AnimalVolador {
        AnimalVolador { nombre, edad }
    }

    /* Si vamos a usar dentro de la función los valores de los miembros
     * de nuestra variable usamos la referencia &self para poder acceder
     * a estos valores. self también debe ser mutable o inmutable según
     * necesites
     */
    pub fn volar(&self, velocidad: f32) {
        println!("{} vuela a {} km/h", self.nombre, velocidad);
    }
}

pub fn main() {
    // Una forma de inicializarlo es:
    let _usuario1 = Usuario {
        active: true,
        username: String::from("someusername123"),
        age: 18,
    };

    // Podemos crear también una función que retorne los valores
    let mut usuario1 = crear_usuario1(String::from("Juan"), 18);

    // Podemos acceder a estos miembros usando el .
    if usuario1.active {
        println!(
            "Hola soy {} y tengo {} años",
            usuario1.username, usuario1.age
        );
        usuario1.active = false;
    }

    // Y si quiero instanciar un nuevo usuario con base al usuario1?
    // Con esto, agrego el nombre como "José" y todos los demás miembros
    // se toman de lo que tenga usuario1
    let _usuario2 = Usuario {
        username: String::from("José"),
        ..usuario1
    };

    //Estructura sin miembro con nombres
    let color_negro = RgbColor(0, 0, 0);
    println!(
        "Color rgb: r: {}, g: {}, b: {}",
        color_negro.0, color_negro.1, color_negro.2
    );

    /* Creamos un nuevo animal volador usando el constructor
     * para llamar a una función asociada, la cual no necesita una
     * instancia de una estructura para llamarse se usa Tipo::función()
     */
    let pajaro = AnimalVolador::new(String::from("Murcielago"), 1);

    //llamamos a su método
    pajaro.volar(12.0);
}

//forma cutre que se puede refactorizar, incluso rust-analyzer lo marca

/*
fn crear_usuario1(username: String) -> Usuario {
    Usuario {
        active: true,
        username: username,
    }
}
*/

//Si el nombre del argumento es exactamente igual al del miembro, podemos
//omitir cosas

fn crear_usuario1(username: String, age: u8) -> Usuario {
    Usuario {
        active: true,
        username,
        age,
    }
}
