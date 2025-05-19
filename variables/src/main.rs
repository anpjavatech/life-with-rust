fn main() {

    // Default variables are immutable but with the help of mut we could update the variable value.
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Shadowing a variable value is possible until the scope of the shadow ends.
    let x = 10;
    let x = x+1;
    {
        let x = x*2;
        println!("The value of x in the inner scope is : {}", x);
    }
    println!("The value of x is : {}", x);

    // every value in Rust is of a certain data type. Rust is statically typed language.
    let x: u32 = "1000".parse().expect("Invalid type");
    println!("Value of {}", x);

    // compound variable : tuple
    let tup = (100, 200, 300);
    let (x, y, z) = tup;
    println!("x:{x}, y:{y}, z:{z}");

    // could also use dot notation to access the tuple value
    let tup: (u32, &str, f64) = (100, "200", 300.22);
    let x = tup.0;
    let y = tup.1;
    let z= tup.2;
    println!("x:{x}, y:{y}, z:{z}");

    // Array is for fixed length and with same types.
    let arr = [2, 2, 3, 3, 4, 4, 5, 5];
    let first = arr[0];
    let mid = arr[4];
    println!("Fiest value : {first} and Mid value : {mid}")

}
