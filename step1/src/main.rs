/** std::io : Input and Output import lib */
// I/O 라이브러리를 Scope로 가져와야 하며, 표준 라이브러리에 정의된 집합을 프렐루드(prelude)라고 한다.
use std::io;

/** main이라는 함수를 정의를 하지만 특별한 함수로서 최우선 순위로 실행된는 함수 */
fn main() {
    // println! : println은 자체적으론 함수 코드이지만 !라는 코드가 붙으면 Macro 호출 코드로 변환다.
    println!("Test Number");
    println!("Please input your guess");

    /*
    * let : 변수의 값을 바인딩(bind) 한다.
    * mut : 변경 가능한 변수 및 참조 또는 포인터
    * :: : new가 String 타입의 연관 함수(associated function)임을 나타냄.
    */
    let mut guess = String::new();

    /*
    * & : 데이터를 메모리로 여러 번 복사하지 않고 접근하기 위한 참조자(Reference)이다.
    * &mut guess처럼 가변으로 만들 필요가 있다.
    * io::stdin : 입력값을 받아 처리하는 과정 
    * => stdin 함수는 Terminal의 handle을 타나내는 타입의 인스턴스를 돌려준다.
    */
    io::stdin()
        .read_line(&mut guess) // size = 8, align = 0x8
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}