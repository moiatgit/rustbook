fn main() {
    // ANCHOR: here
    enum CellaFullCalcul {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let fila = vec![
        CellaFullCalcul::Int(3),
        CellaFullCalcul::Text(String::from("blau")),
        CellaFullCalcul::Float(10.12),
    ];
    // ANCHOR_END: here
}
