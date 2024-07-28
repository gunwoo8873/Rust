fn data_type() {
    let base_string = " ";
    let change_string = base_string.len(); // String => int Type change
    println!("{}", change_string); // Type change Result = 5
}

fn float_type() {
    let x : f32 = 2.0; // 1배수 정밀도
    let y : f64 = 3.0; // 2배수 정밀도
    println!("Float Output 1 : {} {}", x, y);
    println!("Float Output 2 : {x} {y}");
}

fn operator_type() {
    let a : i32 = 10;
    let a : f64 = a as f64; // a의 Int타입을 f64타입으로 변경
    let b : f64 = 3.14;

    let sum = a + b;
    println!("sum : {sum}");

    let difference = a - b;
    println!("difference : {:.2}", difference);
    
    let product = a * b;
    println!("product : {:.2}", product);

    let quotient = a / b;
    println!("quotient : {:.2}", quotient);

    let truncated = -a / b;
    println!("truncated : {:.2}", truncated);

    // let remainder = a % b;
    let remainder = a.rem_euclid(b); /* rem_euclid = % */
    println!("remainder : {:.2}", remainder);
}

fn boolean_type() {
    let t = true;
    let f = false;

    if t == f {
        println!("{f}");
    }
    else {
        println!("{t}");
    }
}

fn string_type() {
    let texta = 'z';
    let textb : char = 'Z'; // char = 4byte
    println!("{texta} {textb}"); // z Z
}

fn compound_type() {
    // Type = Array
    let arr : (i32, f64, u8) = (500, 6.4, 1);

    // destructuring
    let (x, y, z) = arr;
    println!("tup : {x} {y} {z}");

    let x = arr.0;
    let y = arr.1;
    let z = arr.2;
    println!("{x} {y} {z}");

    let arr2 = [10; 10];
    println!("{:?}", arr2);
}

fn array_type() {
    // [Bit; Length] = [Index]
    let arr : [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a = arr[0];
    let b = arr[5];
    println!("a : {a}, b : {b}");
}

fn main() {
    data_type();
    // statically();
    float_type();
    operator_type();
    boolean_type(); // true + false
    compound_type(); // tuple + destructuring
    string_type();
    array_type();
}