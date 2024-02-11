use std::fmt::Display;

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
fn iterar_algo<T: Display>(x: &[T]){
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
}
