fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let tercer: &i32 = &v[2];
    println!("El tercer element és {tercer}");

    let tercer: Option<&i32> = v.get(2);
    match tercer {
        Some(tercer) => println!("El tercer element és {tercer}"),
        None => println!("No hi ha tercer element."),
    }
    // ANCHOR_END: here
}
