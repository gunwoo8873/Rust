use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let gen_num = rand :: thread_rng().gen_range(0..=1000);
    
    //// infinity try loop
    loop {
        //// select_num라는 변수를 정의
        let mut select_num = String :: new();

        io::stdin()
            .read_line(&mut select_num) // size = 8, align = 0x8
            .expect("Failed to read line");

        //// let select_num: u32 = select_num.trim().parse().expect("Please type a Num");
        let select_num: u32 = match select_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        match select_num.cmp(&gen_num) {
            // Ordering : 열거형 Less, Greater, Equal의 3가지 배리언트들을 가지고 있다.
            Ordering :: Less => println!("Too small!"), // a < b
            Ordering :: Greater => println!("Too big!"), // a > b
            //Ordering :: Equal => println!("You win!"), // a = b
            Ordering :: Equal => {
                println!("You win!");
                break;
            }
        }
    }
}