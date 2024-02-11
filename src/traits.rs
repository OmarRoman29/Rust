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

// dyn indica que admite "cualquier implementacion de ave"
fn mostrar_datos(ave: &dyn Ave){
    println!("Nombre: {}", ave.get_nombre());
    println!("Nombre cientifico: {}", ave.get_nombre_cientifico());
    println!("Peso: {}, Tamaño alas: {}", ave.get_peso(), ave.get_tamanio_alas());
}

pub fn _main(){
    let cody_maverick = Pinguino::new(12.3, 0.30);

    mostrar_datos(&cody_maverick);
}
