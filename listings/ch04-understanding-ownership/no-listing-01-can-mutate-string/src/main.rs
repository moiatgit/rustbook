fn main() {
    // ANCHOR: here
    let mut s = String::from("Hola");

    s.push_str("Món!"); // push_str() afegeix un literal a un String

    println!("{}", s); // Escriurà `Hola Món!`
                       // ANCHOR_END: here
}
