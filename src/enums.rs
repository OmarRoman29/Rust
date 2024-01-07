/* "Las enumeraciones o enums te permiten definir un tipo
 * enumerando sus posibles variantes. Primero definiremos
 * y usaremos un enum para mosrtar como se puede codificar
 * 'significado junto con datos'. Después exploraremos un
 * particular enum llamado ''Option', que representa como
 * un dato puede ser 'algo' o 'nada'"
 *
 * Imaginemos un caso donde un enum es más apropiado que
 * una estructura. Supongamos que debemos trabajar con
 * direcciones IP. Por ahora los estándares más usados
 * en la actualidad son IPv4 e IPv6. Debido a que son los
 * unicos tipos de IP que nuestro programa debe admitir
 * podemos enumerar estas variantes
 *
 * Una ip puede ser v4 o v6 pero no ambas a la vez, enum
 * permite elegir uno entre sus variantes, pero estas ip
 * deben ser tratadas como el mismo tipo de dato
 */

enum TipoDeIp {
    V4,
    V6,
}

pub fn main() {
    // Se crea un nuevo tipo de dato
    let four = TipoDeIp::V4;
    let six = TipoDeIp::V6;

    // Incluso podemos crear funciones que reciban este
    // tipo
    route(four);
    route(six);

    // O hacer estructuras
    let _loopback_v4 = Ip {
        _tipo_ip: TipoDeIp::V4,
        _ip: String::from("127.0.0.1"),
    };

    let _loopback_v6 = Ip{
        _tipo_ip: TipoDeIp::V6,
        _ip: String::from("::1"),
    };

    /* De esta forma usando la estructura se agrupa mejor
     * el tipo de ip y la dirección, pero si se hiciera todo
     * en el mismo enum posiblemente sería más claro
     */ 
    let _usr1_dir = DirIpV1::V4(String::from("192.168.1.1"));
    let _usr2_dir = DirIpV1::V6(String::from("::5"));

    /* Ahora agregamos la información directamente a la variante
     * del enum, por lo que no necesitamos un struct extra además
     * no necesitamos de un constructor. Además pueden almacenar
     * un tipo diferente de información cada enum. Por ejemplo, 
     * una Ipv4 solo tendrá 4 numeros entre 0 y 255, mientras que
     * Ipv6 puede llegar a contener caracteres. Incluso puedes
     * agregar enums en enums
     */ 
    let _usr1_dir = DirIpV2::V4(192, 168, 1, 0);
    let _usr2_dir = DirIpV2::V6(String::from("::5"));
}

fn route(_tipo_ip: TipoDeIp) {
    println!("Hacer algo");
}

struct Ip{
    _tipo_ip: TipoDeIp,
    _ip: String,
}

enum DirIpV1{
    V4(String),
    V6(String),
}

enum DirIpV2{
    V4(u8, u8, u8, u8),
    V6(String),
}
