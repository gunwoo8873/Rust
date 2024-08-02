fn main() {
    string_type();
}

fn string_type() {
    let mut str = String :: from("STR Variable");
    str.push_str(" + NEW add STR Variable");
    println!("{}", str);
}