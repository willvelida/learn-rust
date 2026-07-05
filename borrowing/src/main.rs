fn main() {
    let phrase = String::from("uptown east");
    let area: &str = &phrase[0..6];
    let zone: &str = &phrase[7..11];
    println!("{area} / {zone}");

    let numbers = [1, 2, 3, 4, 5];
    let middle: &[i32] = &numbers[1..4];
    println!("{middle:?}");
}
