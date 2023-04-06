fn primera_paraula(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: here
fn main() {
    let mut s = String::from("Hola Món");

    let paraula = primera_paraula(&s);

    s.clear(); // error!

    println!("La primera paraula és: {}", paraula);
}
// ANCHOR_END: here
