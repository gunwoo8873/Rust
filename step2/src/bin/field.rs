struct Datatype {
    name : String,
    age : i32,
    email : String,
    address : String,
    skill : String,
    count : u64,
    ative : bool,
}

fn main() {
    user_info();
}

fn user_info() {
    let user = Datatype {
        name : String :: from("choigunwoo"),
        age : 27,
        email : String :: from("gunwoo8873@outlook.kr"),
        address : String :: from("seoul"),
        skill : String :: from("Shell, Powershell, Rust"),
        count : 1,
        ative : true,
    };

    println!("name : {}", user.name);
    println!("age : {}", user.age);
    println!("email : {}", user.email);
    println!("address : {}", user.address);
    println!("skill : {}", user.skill);
    println!("count : {}", user.count);
    println!("ative : {}", user.ative);
}