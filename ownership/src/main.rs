fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let got = gives_ownership();
    println!("main got: {got}");
}

fn takes_ownership(s: String) {
    println!("takes_ownership got: {s}");
}

fn gives_ownership() -> String {
    String::from("yours")
}
