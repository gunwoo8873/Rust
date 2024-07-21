fn let_variable() {
    let mut x = 10;
    println!("Value not update Result : {x}"); // Variable not update Result = 10

    x = 6;
    println!("Value update Result : {x}"); // Variable update Result = 6
}

fn let_variable_err() {
    let x = 10;
    println!("x : {}", x);

    x = 15; // Error
    println!("x : {}", x);
}

fn const_variable() {
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}"); // Result = 10800
}

fn shaodwing() {
    let a = 150;
    let a = a * 10;
    
    {
        let a = a / 5;
        println!("Scope Result : {a}");
    }
    
    println!("Value Result : {a}");
}

fn main() {
    let_variable();
    const_variable();
    shaodwing();
}