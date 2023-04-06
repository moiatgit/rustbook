fn main() {
    // ANCHOR: here
    let s = String::from("Hola Mon");   // Món sense accent perquè sigui ASCII

    let hola = &s[0..4];
    let mon = &s[5..8];
    // ANCHOR_END: here
}
