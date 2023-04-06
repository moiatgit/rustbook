// ANCHOR: here
fn primera_paraula(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let meu_string = String::from("Hola Mon");

    // `primera_paraula` works on slices of `String`s, whether partial or whole
    let paraula = primera_paraula(&meu_string[0..5]);
    let paraula = primera_paraula(&meu_string[..]);
    // `primera_paraula` també funciona amb referències a `String`s, 
    // que són equivalents a seccions completes de `String`s
    let paraula = primera_paraula(&meu_string);

    let meu_string_literal = "hola mon";

    // `primera_paraula` funciona amb seccions de literals de strings, ja siguin parcials o totals
    let paraula = primera_paraula(&meu_string_literal[0..5]);
    let paraula = primera_paraula(&meu_string_literal[..]);

    // Donat que els literals de string ja *són* seccions de string,
    // això també funciona, sense necessitat de la notació de secció!
    let paraula = primera_paraula(meu_string_literal);
}
// ANCHOR_END: usage
