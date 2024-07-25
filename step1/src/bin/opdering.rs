use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn loop_opdering() {
    let gen_num = rand :: thread_rng().gen_range(0..=1000);
    
    //// infinity try loop
    loop {
        // select_num라는 변수를 정의
        let mut select_num = String :: new();

        io::stdin()
            .read_line(&mut select_num) // size = 8, align = 0x8
            .expect("Failed to read line");

        // let select_num: u32 = select_num.trim().parse().expect("Please type a Num");
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

fn opdering() {
    let gen_num = rand :: thread_rng().gen_range(0..=100);
    let mut select_num = String::new(); // select_num라는 변수를 정의
    
    io::stdin()
        .read_line(&mut select_num) // size = 8, align = 0x8
        .expect("Failed to read line");
    println!("secret Number : {gen_num}");

    // parse : 문자열을 다른 타입으로 변경 ex) String -> Number
    // 주의 : 에러가 발생하기 쉽다. ex) A%같은 특수 문자열이 포함되어 있다면 정수로 바꿀 방법이 없다.
    let select_num: u32 = select_num.trim().parse().expect("Please type a Num");

    println!("You select_num: {select_num}");

    // cmp : 두 값을 비교하여 비교가 가능한 모든 것들을 대해 호출한다.
    match select_num.cmp(&gen_num) {

        // Ordering : 열거형 Less, Greater, Equal의 3가지 배리언트들을 가지고 있다.
        Ordering :: Less => println!("Too small!"), // a < b
        Ordering :: Greater => println!("Too big!"), // a > b
        Ordering :: Equal => println!("You win!"), // a = b
    }
}

fn main() {
    loop_opdering();
    opdering();
}