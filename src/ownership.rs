/* El ownership es una caracteristica propia de Rust la cual se encarga
 * de asignar y liberar automáticamente la memoria sin la necesidad de
 * gestionar manualmente la memoria como en C o C++ y sin tener recolector
 * de basura como en otros lenguajes con el fin de prevenir errores
 *
 * Es fundamental para los programadores rastrear la memoria, ya que no
 * hacerlo puede resultar en una “fuga de memoria” - (Leak).
 */ 

enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

pub fn main() {
    let dull = Light::Dull;

    display_light(dull);
    // display_light(dull); // will not work
}
