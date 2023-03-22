fn main() {
    let s1 = String::from("hola");

    let len = calcula_longitud(&s1);

    println!("La longitud de '{}' és {}.", s1, len);
}

// ANCHOR: here
fn calcula_longitud(s: &String) -> usize { // s és una referència a un String
    s.len()
} // Aquí s surt d'àmbit, però, com que no li pertany el valor que referencia,
  // aquest no serà eliminat.
// ANCHOR_END: here
