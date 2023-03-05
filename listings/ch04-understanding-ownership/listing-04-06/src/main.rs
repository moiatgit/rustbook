fn main() {
    let s = String::from("hola");

    change(&s);
}

fn change(un_string: &String) {
    un_string.push_str("món");
}
