fn main() {
    break_loop();
}

fn break_loop() {
    let mut current_count = 0;
    
    /* 'counting_up = loop label no reverse counting. */
    'counting_up : loop {
        println!("Start count : {current_count}");
        let mut result_count = 10;

        loop {
            println!("result_count : {result_count}");
            if result_count == 1 {
                break;
            }
            if current_count == 10 {
                break 'counting_up;
            }
            result_count -= 1;
        }
        current_count += 1;
    }
    println!("End count = {current_count}");
}

fn for_fn() {
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    for number in arr {
        println!("Number {number}");
    };
}