fn main() {
    move_stack();
    copy_stack();
    push_stack();

    let take_str = String :: from("Take variable str");
    take_stack(take_str);

    let give_str = String :: from("Give variable str");
    give_stack(give_str);
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
    let mut str = String :: from("Push Memory");
    str.push_str("vaiable str");
    println!("Push Result : {}", str);
}

fn take_stack(take_str : String) {
    println!("{}", take_str);
}

fn give_stack(give_str : String) -> String {
    give_str
}