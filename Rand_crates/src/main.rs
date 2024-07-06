// Rng : 난수 생성을 위한 메서드들을 정의한 트레이트
use rand::Rng;

fn main() {
    println!("Hello, world!");

    /*
    rand::thread_rng() : 특정 난수 생성 함수를 호출하고 seed를 정한 이후에 현재 스레드에서만 사용가능한 난수 생성기능이다.
    gen_range(value) : 지정한 범위의 값중 임의의 값을 저장 (value)는 start..=end의 로직으로 Min과 Max라고 생각하면 된다.
     */
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {secret_num}"); // 1 ~ 100
    println!("Guess the number!");
}
