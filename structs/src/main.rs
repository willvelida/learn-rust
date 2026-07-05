struct Account {
    handle: String,
    email: String,
    verified: bool,
}

struct Point(i32, i32);
struct Marker;

#[derive(Debug)]
struct Room {
    length_m: u32,
    width_m: u32,
}

impl Room {
    fn floor_area(&self) -> u32 {
        self.length_m * self.width_m
    }

    fn square(side: u32) -> Self {
        Self { length_m: side, width_m: side}
    }
}

fn main() {
    let account = Account {
        handle: String::from("willvelida"),
        email: String::from("will@example.com"),
        verified: false,
    };

    let account2 = Account {
        email: String::from("willvelida@example.com"),
    ..account
    };

    println!("{} <{}> verified: {}", account2.handle, account2.email, account2.verified);

    let room = Room { length_m: 4, width_m: 6};
    println!("area = {}", room.floor_area());

    let sq = Room::square(5);
    println!("square area = {}", sq.floor_area());

    println!("{room:?}");
    println!("{room:#?}");
}

fn make_account(handle: String, email: String) -> Account {
    Account { handle, email, verified: false }
}