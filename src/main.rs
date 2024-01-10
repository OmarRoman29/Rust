mod enums;
mod estructuras_de_control;
mod leer_de_consola;
mod match_cf;
mod modulos;
mod option;
mod ownership;
mod structs;
mod tipos_datos;
mod operaciones;
mod collections;

fn main() {
    //En este orden se fueron realizando los archivos y es orden
    //de las "clases"

    // tipos_datos::main();
    // leer_de_consola::main();
    // estructuras_de_control::main();
    // ownership::main();
    // structs::main();
    // enums::main();
    // option::main();
    // match_cf::main();
    // aquí va lo de modulos.rs
    collections::main();
}
