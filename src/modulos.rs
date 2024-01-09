/* los módulos permiten segmentar nuestro código en distintos
 * archivos e importar código de ellos, tenemos varias formas
 * de hacerlo, el primero es crear uno en un mismo archivo.
 *
 * Por defecto todos los módulos y funciones son privadas, por
 * lo que hay que hacerlas públicas
 */

pub mod modulo1 {
    fn _fun1() {}
    pub fn _fun2() {
        _fun1();
    }

    pub mod otro_modulo {
        pub fn _otrafn1() {}
    }
}

pub fn hacer_algo1() {
    /* podemos llamar a una funcion de este modulo con una ruta absoluta
     * haciendo uso de la palabra reservada crate::, y como se puede ver
     * tuvimos que especificar este propio archivo, y como fn1 es privada
     * no podemos llamarla de esta forma, pero otras funciones de ese
     * módulo sí
     */
    crate::modulos::modulo1::_fun2();

    //de hecho podemos llamar esta propia funcion
    crate::modulos::hacer_algo1();

    //o podemos usar rutas relativas
    modulo1::_fun2();
}

// Y si no queremos escribir todo eso cada vez que llamamos un módulo?
// podemos importar directamente el módulo en nuestro scope
use modulo1::otro_modulo;

// Pero qué si sólo queremos usar una única cosa del módulo?
use modulo1::otro_modulo::_otrafn1;

use crate::operaciones;
use crate::operaciones::vectores::Vector2;

pub fn hacer_algo2() {
    otro_modulo::_otrafn1();

    _otrafn1();
}

pub fn fun_random() {}

pub mod a {
    fn coso1() {
        // super te mueve un "nivel" arriba
        super::fun_random();
    }

    pub mod b{
        fn coso2(){
            // 1 super para salir de b, y otro para
            // salir de a
            super::super::fun_random();
        }
    }
}

/* Pero esto es algo caótico, así que podemos hacer una carpeta
 * en src/ que contenga nuestro módulo y archivos para poder 
 * agruparlos y manejarlos mejor. Crearemos una carpeta que
 * se llame operaciones, esta debe tener un mod.rs que importe
 * los archivos que crearemos con las funciones a usar
 */ 

fn coso3() {
    operaciones::aritmetica::suma(1, 2);

    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(2.0, 0.0);
    let suma_vect = operaciones::vectores::suma(v1, v2);

    println!("Suma: {}, {}", suma_vect.x, suma_vect.y);
}
