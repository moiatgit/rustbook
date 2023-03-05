fn main() {
    let s = String::from("hola");   // s entra en àmbit

    mou(s);                         // el valor de s es mou a la funció i, ...
                                    // ... per tant, s deixa de ser vàlida aquí

    let x = 5;                      // x entra en àmbit

    copia(x);                       // el valor de x es mou a la funció, però
                                    // com que i32 implementa Copy, x segueix
                                    // sent vàlid

} // Aquí x surt d'àmbit i després s. Com que el valor de s va ser mogut,
  // no passa res especial.

fn mou(un_string: String) {         // un_string entra en àmbit
    println!("{}", un_string);
} // Aquí un_string surt d'àmbit i es crida el mètode `drop` amb el que la
  // memòria és alliberada.

fn copia(un_enter: i32) {           // un_enter entra en àmbit
    println!("{}", un_enter);
} // Aquí, un_enter surt d'àmbit. No passa res especial.
