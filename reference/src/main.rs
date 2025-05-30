fn main() {
    // use of reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // use of mutable reference
    let mut s2 = String::from("hello");
    change(&mut s2);

    // cannot have two mutable reference at the same scope. so added a separate scope using {}
    let mut s3 = String::from("hello");
    {
        let r1 = &mut s3;
        println!("{}", r1);
    }
    let r2 = &mut s3;
    println!("{}", r2);

    //cannot have a mutable reference while we have an immutable one to the same value.
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
