/* Un trait es similar a las interfaces en 
 * otros lenguajes con algunas diferencias
 * En java tendriamos tal vez una clase llamada
 * ave, que tenga sus atributos básicos, tal vez 
 * nombre, tamaño, etc. Después tendríamos 
 * interfaces que nos darían comportamientos
 * según el ave, pero como no tenemos herencia
 * en rust, para ahorita usaremos solo traits,
 * getters y setters.
 */

use std::ops::Add; 

trait Ave {
    fn get_nombre(&self) -> String;
    fn get_nombre_cientifico(&self) -> String;
    fn get_peso(&self) -> f32;
    fn get_tamanio_alas(&self) -> f32;

    // Podemos definir funcionalidad básica para
    // no necesitar implementacion(?)
    fn hacer_sonido(&self){
        println!("{} Hace sonido", self.get_nombre());
    }
}

trait Volador{
    fn volar(&self);
    fn aterrizar(&self);
}

trait Nadador{
    fn nadar(&self);
    fn secarse(&self);
}

trait Corredor{
    fn correr_rapidamente(&self);
}

/* Parece ser que es necesario pasar por referencia
 * para que el compilador sepa el tamaño, todo dato
 * "prestado" (pasado por referencia) tiene tamanio
 * conocido, por lo cual debemos pasar por referencia
 * con esto decimos que acepta cualquier objeto que 
 * implemente Ave. Tenemoa una forma de polimorfismo
 */ 

struct Pinguino {
    nombre: String,
    nombre_cientifico: String,
    peso: f32,
    tamanio_alas: f32,
}

impl Pinguino {
    fn new(peso: f32, tamanio_alas: f32) -> Self{
        Self{
            nombre: String::from("Pingüino"),
            nombre_cientifico: String::from("Spheniscidae"),
            peso,
            tamanio_alas,
        }
    }
}

impl Ave for Pinguino{
    // Podemos brindar implementacion a pesar de
    // estar definido ya en el trait
    fn hacer_sonido(&self) {
        println!("Sonido Pinguino");
    }

    fn get_tamanio_alas(&self) -> f32 {
        self.tamanio_alas
    }
    fn get_peso(&self) -> f32 {
        self.peso
    }
    fn get_nombre_cientifico(&self) -> String {
        self.nombre_cientifico.clone()
    }
    fn get_nombre(&self) -> String {
        self.nombre.clone()
    }
}

impl Nadador for Pinguino{
    fn secarse(&self) {
        println!("{} se está secando", self.nombre);
    }

    fn nadar(&self) {
        println!("{} está nadando", self.nombre);
    }
}

struct Avestruz{
    nombre: String,
    nombre_cientifico: String,
    peso: f32,
    tamanio_alas: f32,
}

impl Avestruz{
    fn new(peso: f32, tamanio_alas: f32) -> Self{
        Self{
            nombre: String::from("Avestruz"),
            nombre_cientifico: String::from("Struthio camelus"),
            peso,
            tamanio_alas
        }
    }
}

impl Ave for Avestruz{
    fn get_nombre(&self) -> String {
        self.nombre.clone()
    }

    fn get_nombre_cientifico(&self) -> String {
        self.nombre_cientifico.clone()
    }

    fn get_peso(&self) -> f32 {
        self.peso
    }

    fn get_tamanio_alas(&self) -> f32 {
        self.tamanio_alas
    }
}

impl Corredor for Avestruz{
    fn correr_rapidamente(&self) {
        println!("{} corre rápidamente", self.nombre);
    }
}

// impl indica que admite "cualquier implementacion de ave"
fn mostrar_datos(ave: &impl Ave){
    println!("Nombre: {}", ave.get_nombre());
    println!("Nombre cientifico: {}", ave.get_nombre_cientifico());
    println!("Peso: {}, Tamaño alas: {}", ave.get_peso(), ave.get_tamanio_alas());
}

// Impl es una avrebiacion de lo siguiente:
// Pero esta notacion es más corta cuando tienes varios datos
// de este tipo
fn carrera<T: Corredor>(corredor1: &T, corredor2: &T) {
    corredor1.correr_rapidamente();
    corredor2.correr_rapidamente();
}

/* Notación where
 * Podemos definidir los traits de mis tipos genéricos de forma más 
 * ordenada utilizando la palabra reservada where
 */ 
fn _some_function<T, U, V>(_t: &T, _u: &U, _v: &V) -> i32
    where
    T: std::fmt::Display + Clone,
    U: Clone,
    V: Clone + Add + std::fmt::Display
{
    //TODO implementacion
    1
}

// Podemos retornar una impl de ave
fn _func1() -> impl Ave {
    Pinguino::new(12.3, 4.0)
}

/* No es válido retornar dos impl diferentes de ave, o al menos
 * no de esta forma:
 *
 * fn _func2(algo: bool) -> impl Ave {
 *     if algo {
 *         Pinguino::new(12.3, 4.0)
 *     }
 *     else {
 *         Avestruz::new(12.3, 4.0)
 *     }
 * }
 */ 

// De esta forma sí, pero utiliza otros conceptos que se tocarán 
// más adelante

fn _func(switch: bool) -> Box<dyn Ave> {
    if switch {
        Box::new(Pinguino::new(12.3, 2.0))
            
    }
    else {
        Box::new(Avestruz::new(12.3, 2.0))
    }
}

struct Dato<T>{
    dato: T,
}

// Pdemos implementar diferentes cosas según los traits
// de nuestro generico
impl <T: Add<Output = T> + Copy> Dato<T> {
    fn sumar(&mut self, dato: &T){
        self.dato = self.dato + *dato;
    }
}

impl <T: std::fmt::Display> Dato<T> {
    fn print(&self){
        println!("Dato: {}", self.dato);
    }
}

pub fn _main(){
    let cody_maverick = Pinguino::new(12.3, 0.30);

    mostrar_datos(&cody_maverick);

    let avesota = Avestruz::new(40.2, 0.20);

    mostrar_datos(&avesota);

    carrera(&avesota, &avesota);

    let mut var: Dato<u32> = Dato{dato:2};
    let dato: Dato<String> = Dato{dato: String::from("Hola")};

    var.sumar(&12);
    var.print();
    dato.print();
    // dato.sumar(&String::from("a")); //No válido porque string no es sumable
}
