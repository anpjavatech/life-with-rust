#[derive(Debug)]
enum UsState {
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year >= 1959,
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    match describe_state_quarter(coin) {
        Some(i) => println!("{:?}", Some(i)),
        None => println!("None!!!"),
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin;
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
