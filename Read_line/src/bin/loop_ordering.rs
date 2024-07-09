use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //// guess라는 변수를 정의
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // size = 8, align = 0x8
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");


    //// infinity try loop
    loop {
        println!("Please input your guess.");
        
        match guess.cmp(&guess) {
            //// Ordering : 열거형 Less, Greater, Equal의 3가지 배리언트들을 가지고 있다.
            Ordering :: Less => println!("Too small!"), // a < b
            Ordering :: Greater => println!("Too big!"), // a > b
            Ordering :: Equal => println!("You win!"), // a = b
        }
    }
}