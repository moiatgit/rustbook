fn main() {
    // ANCHOR: here
    {                      // s no és vàlida aquí ja que encara no ha estat declarada
        let s = "hola";    // s és vàlida a partir d'aquest moment

        // ometem codi que fa servir s
    }                      // aquest àmbit ha finalitzat i a partir d'aquí s ja no és vàlida
    // ANCHOR_END: here
}
