fn main() {
    literal_push();
    memory_move();
}

fn literal_push() {
    let mut stack = String :: from("Value");
    stack.push_str(", Add Value"); // push_str = add String Literal

    println!("{}", stack);
}

fn memory_move() {
    let owner_1 = String :: from("Rust");
    let owner_2 = owner_1.clone(); // The owner 1 stack copy

    println!("owner_1 = {}, owner_2 = {}", owner_1, owner_2);
}