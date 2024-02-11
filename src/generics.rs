/* Supongamos que queremos hacer una funcion donde se
 * reciban dos valores y se operen de una cierta manera,
 * ¿qué pasa cuando cambiamos los tipos? pues debemos
 * reescribir la funcion para el tipo adecuado
 */ 
fn iterar_algo_u32(x: &[u32]){
    for element in x{
        println!("{}", element);
    }
    println!("");
}

fn iterar_algo_f32(x: &[f32]){
    for element in x{
        println!("{}", element);
    }
    println!("");
}

/* Como vemos es bastante repetitivo, de ahí que existan
 * los genéricos, los genéricos establecen un método de
 * indicar que se recibe un tipo de dato, y que en tiempo
 * de compilación rust va a determinar el tipo de dato que
 * corresponde. usamos <T> para determinar que este tipo 
 * está en el scope de la función, pero con la diferencia
 * que tenemos el : Display, este es un trait y en un momento
 * lo explicamos. Posteriormente definimos nuestro array de
 * tipo T, el cual va a ser iterado e impreso, sin embargo
 * podemos tener un tipo de dato que no sea capas de ser impreso
 * como los numeros que usamos anteriormente, por ejemplo un
 * struct, o un enum, por lo que debemos especificarle al 
 * compilador que estos elementos deben cumplir esa "propiedad",
 * rasgo o trait.
 */ 
fn iterar_algo<T: std::fmt::Display>(x: &[T]){
    for element in x{
        println!("{}", element);
    }
    println!("");
}

struct _Coso{
    _b: u32,
    _a: f32,
}

impl _Coso {
    fn new(_b: u32, _a: f32)->Self{
        Self{_b, _a}
    }
}

/* Cuando definimos un tipo genérico, si uno o más
 * variables usan este tipo, obviamente no pueden 
 * variar en tipo, es redundante y obvio pero igual
 * lo anoto por si se me olvida.
 *
 * La convención es iniciar en T y de ahí seguir
 * con las letras del abecedario
 */ 

fn hacer_algo<T, U>(_x: &T, _y: &T, _z: &U){
    println!("Haciendo algo\n");
}

// Podemos definir tipos genericos en nuestros structs
// o enums
struct Vector3<T>{
    x: T,
    y: T,
    z: T,
}

/* Al implementar sus funciones debemos especificar 
 * T para que esté en el scope. No comprendo del todo
 * el trait de add, pero debe verse así para poder
 * funcionar correctamente jsjs, supongo que especifica
 * el tipo de dato que debe resultar la suma. También
 * tenemos el trait copy para poder pasar por referencia
 * y hacer el copy de los valores sin desplazar el valor
 * de los vectores pasados como suma
 */
impl <T: std::ops::Add<Output = T> + Copy>  Vector3<T> {
    // Forma 1 de usar esto
    fn new(x: T, y: T,z: T) -> Vector3<T>{
        Vector3{x,y,z}
    }

    //Forma dos
    fn add_vector(self: &Self, vector2: &Self ) -> Self{
        Self::new(self.x + vector2.x, self.y + vector2.y, self.z + vector2.z)
    }
}

impl Vector3<f32> {
    /* Por alguna razon el &self es obligatorio en este
     * tipo de implementaciones, si no colocamos el self
     * debemos llamarla por el nombre largo:
     * Vector3::hacer_algo();
     */

    fn hacer_algo(&self){
        println!("Haciendo algo");
    }
}

// no necesitamos escribir como tal T al parecer, la notacion
// de <algo> representa por sí mismo el tipo(?)
struct _Algo<TipoGenerico>{
    _y: TipoGenerico,
}

pub fn _main() {
    let arr_u32: [u32;3]  = [1, 3, 4]; 
    iterar_algo_u32(&arr_u32);

    let arr_f32: [f32;3]  = [1.2, 7.5, 6.2]; 
    iterar_algo_f32(&arr_f32);
    
    let _arr_coso: [_Coso; 3] = [_Coso::new(1, 2.3), _Coso::new(2, 3.2), _Coso::new(3,4.5)];

    iterar_algo(&arr_u32);
    iterar_algo(&arr_f32);
    // iterar_algo(&_arr_coso); // Err: _Coso doesn't implement std::fmt::Display
    
    hacer_algo(&1, &1, &2.3);
    hacer_algo(&"Hola", &"Adios", &_Coso::new(1, 2.3));

    // Crear 2 vectores
    let vector1 = Vector3::new(1, 2, 3);
    let vector2 = Vector3::new(2, -1, 3);

    let _vector_resultante = Vector3::add_vector(&vector1, &vector2);

    //vector2.hacer_algo() // err, no existe en Vector3<{integer}>
    let vector = Vector3::new(1.0, 1.0, 1.0);
    vector.hacer_algo();
}
