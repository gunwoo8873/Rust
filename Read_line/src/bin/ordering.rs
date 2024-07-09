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
    
    /*
    * Shadowing : value.trim().parse()를 사용하여 한개의 타입을 다른 타입으로 변경
    * => guess라는 변수 타입을 다른 타입의 변수로 변경
    * u32 : 숫자형 데이터만 저장 가능 (32bit 정수)
    * trim : NN와 같은 공백 문자 제거
    * parse : 문자열을 다른 타입으로 변경 ex) String -> Number
    * => 주의 : 에러가 발생하기 쉽다. ex) A%같은 특수 문자열이 포함되어 있다면 정수로 바꿀 방법이 없다.
    */
    let guess: u32 = guess.trim().parse().expect("Type a Number");

    //// cmp : 두 값을 비교하여 비교가 가능한 모든 것들을 대해 호출한다.
    match guess.cmp(&guess) {
        //// Ordering : 열거형 Less, Greater, Equal의 3가지 배리언트들을 가지고 있다.
        Ordering :: Less => println!("Too small!"),
        Ordering :: Greater => println!("Too big!"),
        Ordering :: Equal => println!("You win!"),
    }
}