fn main() {
    println!("Hello, world!");
    control_func();
    let result = execute_loop();
    println!("loop result : {result}");
    let multi_result = multi_loop_using_labels();
    println!("multi result : {multi_result}");
    while_loop();
    for_loop();
    for_loop_with_range();
}

fn control_func(){
    let x = 10;
    if x < 10 {
        println!("less than 10");
    }else if x > 10{
        println!("greater than 10");
    }else{
        println!("equals 10");
    }
}

fn execute_loop() -> u64{

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    result
}

fn multi_loop_using_labels() -> u64 {

    let mut upper_count = 0;
    'loop_label : loop {
        println!("upper count : {upper_count}");
        let mut remaining_count = 10;
        loop {
            println!("remaining count : {remaining_count}");
            if remaining_count == 9 {
                break;
            }
            if upper_count == 2{
                break 'loop_label;
            }
            remaining_count -= 1;
        }
        upper_count += 1;
    }
    upper_count
}


fn while_loop(){
    let mut num = 0;
    while num <= 10 {
        println!("num : {num}");
        num +=1;
    }
}


fn for_loop(){
    let arr = [10, 20, 30, 40, 50, 60, 70];
    for ele in arr{
        println!("value : {ele}");
    }
}


fn for_loop_with_range(){
    // .rev() to reverse the numbers.
    for num in (1..10).rev() {
        println!("Range num : {num}");
    }
}
