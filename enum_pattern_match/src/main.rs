#[derive(Debug)]
enum Event {
    Close,
    Resize { width: u32, height: u32 },
    Log(String),
    Recolor(u8, u8, u8),
}

fn main() {
    let events = [
        Event::Close,
        Event::Resize { width: 1920, height: 1080 },
        Event::Log(String::from("file saved")),
        Event::Recolor(255, 128, 0),
    ];

    for event in events {
        println!("{event:?}");
    }

    let maybe_id: Option<i32> = Some(42);
    let missing: Option<i32> = None;

    println!("{maybe_id:?}");
    println!("{missing:?}");

    println!("{:?}", double(Some(5)));
    println!("{:?}", double(None));

    let user_quota: Option<u32> = Some(50);

    if let Some(limit) = user_quota {
        println!("the quota is {limit}");
    }

    greet(Some("Will"));
    greet(None);
}

fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n * 2),
    }
}

fn greet(name: Option<&str>) {
    let Some(name) = name else {
        println!("no name given");
        return;
    };

    println!("hello, {name}");
}
