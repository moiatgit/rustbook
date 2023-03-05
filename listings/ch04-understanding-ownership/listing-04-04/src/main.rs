fn main() {
    let s1 = dona_pertinenca();         // dona_pertinenca mou el seu
                                        // valor de retorn a s1

    let s2 = String::from("hola");      // s2 entra en àmbit

    let s3 = pren_i_retorna(s2);        // s2 es mogut a pren_i_retorna(),
                                        // que a la seva vegada mou
                                        // el seu valor de retorn a s3
} // Aquí, s3 surt d'àmbit i és alliberat. s2 va ser mogut, així que no
  // se'n fa res. s1 surt d'àmbit i és alliberat.

fn dona_pertinenca() -> String {        // dona_pertinenca() mourà el seu
                                        // valor de retorn a la funció
                                        // que fa la crida

    let un_string = String::from("tot teu"); // un_string entra en àmbit

    un_string                           // un_string és retornat i mogut a
                                        // la funció que fa la crida
}

// Aquesta funció obté un String i el retorna
fn pren_i_retorna(altre_string: String) -> String { // altre_string entra en àmbit
    altre_string  // altre_string és retornat i es mou a la funció que fa la crida
}
