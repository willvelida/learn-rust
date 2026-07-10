use std::{borrow::Cow::Owned, path::Component::Normal};

fn main() {
    /*
    let multiplier = 3;
    let triple = |x| x * multiplier;
    println!("{}", triple(10));
    

    // Fn: only reads its captures, so it can be called many times
    let label = String::from("count");
    let show = || println!("label is {label}");
    show();
    show();

    // FnMut: changes its captured state
    let mut total = 0;
    let mut add = |n| total += n;
    add(3);
    add(4);
    println!("total is {total}");

    // FnOnce: moves a captured value out, so it can run only once
    let owned = String::from("done");
    let consume = move || owned;
    println!("consumed: {}", consume());
    

    let scores = vec![10, 20, 30];
    let mut it = scores.iter();

    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    */

    let total: i32 = (1..=10)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("total = {total}");

    let names: Vec<String> = vec!["a", "b"]
        .iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("{names:?}");
}
