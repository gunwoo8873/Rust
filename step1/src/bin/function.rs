fn main() {
    println!("Main test Function");
    sub_fn();

    // Parameter Value x
    parameter_fn(5);

    // 10은 i32 정수을 저장
    // a는 char로 단일 문자열만 저장
    measurement_fn(10, 'a');
}

/** Rust 관례로 Snake case를 기용 */
fn sub_fn() {
    println!("Sub test Function");
}

// function(parameter) {} => parameter = 인수(argument) or 매개변수
// parameter에는 반드시 정의의 타입 명시를 선언해야 한다.
fn parameter_fn(x : i32) {
    println!("Value of x is : {x}");
}

fn measurement_fn(x : i32, y : char) {
    println!("measurement : {x} {y}");
}