fn main() {
    move_stack();
    copy_stack();
    push_stack();

    let take_str = String :: from("Take variable str");
    take_stack(take_str);

    let give_str = String :: from("Give variable str");
    give_stack(give_str);

    reference_stack();
    mut_reference_stack();

    dangling_stack();
}

fn move_stack() {
    let str = String :: from("Move Memory variable str");
    let move_str = str;
    println!("Move Result : {}", move_str);
}

fn copy_stack() {
    let str = String :: from("Clone Memory variable str");
    let copy_str = str.clone();
    println!("Copy Result : {}", copy_str);
}

fn push_stack() {
    let mut str = String :: from("Push Memory variable str");
    str.push_str("vaiable str");
    println!("Push Result : {}", str);
}

fn take_stack(take_str : String) {
    println!("{}", take_str);
}

fn give_stack(give_str : String) -> String {
    give_str
}

fn reference_stack() {
    let str = String :: from("Reference Memory variable str");
    let length = string_length(&str);
    println!("Reference Result : '{}' , length = '{}'", str, length);
}

fn string_length(reference_str : &String) -> usize {
    reference_str.len()
}

fn mut_reference_stack() {
    let mut str = String :: from("mut Reference Memory variable str");
    push_stack2(&mut str);
}

fn push_stack2(psuh_str : &mut String) {
    psuh_str.push_str(", Push Stack");
}

fn dangling_stack() -> String {
    let str = String :: from("Dangling Memory variable str");
    str
}