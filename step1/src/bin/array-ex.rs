use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("msg"); 

    let index : usize = index
        .trim()
        .parse()
        .expect("msg");

    let element = a[index];
    println!("{} : {}", index, element);
}