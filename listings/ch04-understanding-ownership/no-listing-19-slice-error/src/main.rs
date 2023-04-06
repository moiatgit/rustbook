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
    let mut s = String::from("Hola Mon");   // Món sense accents perquè sigui ASCII

    let paraula = primera_paraula(&s);

    s.clear(); // error!

    println!("La primera paraula és: {}", paraula);
}
// ANCHOR_END: here
