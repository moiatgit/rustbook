fn primera_paraula(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("Hola Món");

    let paraula = primera_paraula(&s); // paraula rebrà el valor 4

    s.clear(); // això buida el String,fent-lo igual a ""

    // paraula encara té el valor 4, però ara aquest valor ja no té sentit
    // per s. El valor de paraula és completament invàlid!
}
// ANCHOR_END: here
