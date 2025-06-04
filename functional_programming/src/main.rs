#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0;
        let mut num_blue = 0;

        for colour in &self.shirts {
            match colour {
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }

        if num_blue > num_red {
            ShirtColour::Blue
        } else {
            ShirtColour::Red
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_preference = Some(ShirtColour::Red);
    let giveaway = store.giveaway(user_preference);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference, giveaway
    );

    let user_preference_1 = None;
    let giveaway_1 = store.giveaway(user_preference_1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference_1, giveaway_1
    );
}
