fn main() {
    println!("Main function!!..");
    another_function(25);
    statements_expressions();
    let val = get_val(20);
    println!("val is {val}")
}

fn another_function(num: u64){
    println!("Another function is called!!!..");
    println!("Parameter : {num}");
}


// statment wont return anything and will end with semicolon and expression is just opposite(dont end with semicolon and it returns a value)
fn statements_expressions(){
    let x = {
        let y = 10;    // is a statement
        y+10            // is an expression
    };

    println!("The value of x : {x}");
}

// function with return (last line will be returned by default..)
// a line ends with semicoln will not return any value (its a statement)
// line without semicolon is considered as an expression and its returns the value.
fn get_val(val: u64) -> u64{
    let num = 10 + val;
    num
}
