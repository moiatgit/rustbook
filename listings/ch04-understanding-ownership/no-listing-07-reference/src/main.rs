// ANCHOR: all
fn main() {
    // ANCHOR: here
    let s1 = String::from("hola");

    let len = calcula_longitud(&s1);
    // ANCHOR_END: here

    println!("La longitud de '{}' és {}.", s1, len);
}

fn calcula_longitud(s: &String) -> usize {
    s.len()
}
// ANCHOR_END: all
