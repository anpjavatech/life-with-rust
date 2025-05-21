fn main() {
    let my_string = String::from("Anoop is the best!");
    let first_word = get_first_word(&my_string);
    println!("The first word : {first_word}");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = get_first_word(&my_string[0..6]);
    println!("The first word 0..6 : {word}");
    let word = get_first_word(&my_string[..]);
    println!("The first word .. : {word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = get_first_word(&my_string);
    println!("The first word & : {word}");

    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = get_first_word(&my_string_literal[0..6]);
    println!("The first word 0..6 : {word}");
    let word = get_first_word(&my_string_literal[..]);
    println!("The first word .. : {word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = get_first_word(my_string_literal);
    println!("The first word : {word}");
}

fn get_first_word(s: &str) -> &str {
    let str_bytes = s.as_bytes();
    for (i, &byt_pntr) in str_bytes.iter().enumerate() {
        if byt_pntr == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
