fn main() {
    // ANCHOR: here
    {
        let s = String::from("hola");  // s és vàlida a partir d'aquest moment
        // ometem codi que fa servir s
    }                                  // aquest àmbit ha finalitzat i a partir
                                       // d'aquí s ja no és vàlida
    // ANCHOR_END: here
}
