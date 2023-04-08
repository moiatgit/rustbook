fn main() {
    // ANCHOR: here
    let mut v = vec![1, 2, 3, 4, 5];

    let primer = &v[0];

    v.push(6);

    println!("El primer element és: {primer}");
    // ANCHOR_END: here
}
