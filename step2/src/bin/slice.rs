fn main() {
    let str = String :: from("Slice variable str");

    let slice = slice(&str[..]);
    println!("Slice str : {}", slice);
}

fn slice(str : &str) -> &str {
    let bytes = str.as_bytes(); // as_bytes = Change String type to Byte type

    // iter : Collection element String type to Byte type change returen
    // enumerate : Each result of iter is mapped to a tuple and returned.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}