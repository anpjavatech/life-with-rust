#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn check_if_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
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

    dbg!(&user1);

    // check if the user is an adult or not
    let is_adult = dbg!(user1.check_if_adult());
    println!("Is the user an Adult : {is_adult}");
}
