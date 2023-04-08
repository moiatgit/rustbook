fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let no_existeix = &v[100];
    let no_existeix = v.get(100);
    // ANCHOR_END: here
}
