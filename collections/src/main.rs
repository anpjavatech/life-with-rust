fn main() {
    println!("Collections!");

    // Vector
    let mut default_vector_with_type: Vec<i32> = Vec::new();
    default_vector_with_type.push(1);

    let implicit_vector_values = vec!["anoop", "manasa", "ishaan"];
    // implicit_vector_values.push("whoisnext"); // to update a vector

    // to access n element using index
    let third_value: &str = &implicit_vector_values[2];
    println!("third value : {third_value}");

    // to access n element using get method
    let second_element: Option<&&str> = implicit_vector_values.get(1); // Option<&&str> : becaus eof string reference.. if it was an integer it will be Option<&str>
    match second_element {
        Some(second_element) => println!("second value : {second_element}"),
        None => println!("second value : None"),
    }
}
