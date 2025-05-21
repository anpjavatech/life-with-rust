fn main() {
    struct User {
        name: String,
        age: u32,
        username: String,
        email: String,
        active: bool,
    }

    let user1 = User {
        name: String::from("Anoop"),
        age: 37,
        username: String::from("anpks"),
        email: String::from("anpks@gmail.com"),
        active: true,
    };

    println!("Name of the created user : {0}", user1.name);
    println!("age of the created user : {0}", user1.age);
    println!("username of the created user : {0}", user1.username);
    println!("email of the created user : {0}", user1.email);
    println!("active of the created user : {0}", user1.active);
}
