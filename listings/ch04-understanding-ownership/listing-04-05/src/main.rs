fn main() {
    let s1 = String::from("hola");

    let (s2, len) = calcula_longitud(s1);

    println!("La longitud de '{}' és {}.", s2, len);
}

fn calcula_longitud(s: String) -> (String, usize) {
    let longitud = s.len(); // len() retorna la longitud del String

    (s, longitud)
}
