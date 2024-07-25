fn main() {
    ragular_fn();
    return_fn();
}

fn ragular_fn() {
    let x = {
        let a = 10;
        a * 15 // a의 값이 * 15 만큼 값을 계산후 반환
    };
    
    println!("Value of a : {x}"); // x가 a의 값을 바인딩 후 인수받아 값이 변경되며 반환
}

fn value_fn() -> i32 {
    10
}

fn return_fn() {
    let x = value_fn();
    println!("Function Arrow value : {x}");
}