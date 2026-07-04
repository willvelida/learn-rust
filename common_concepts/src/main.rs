fn main() {

    let x = 5;

    let mut y = 6;
    y += 1;

    const MAX_VALUE: u32 = 100_000;

    let gaps = "   ";
    let gaps = gaps.len();

    let guess: u32 = "21".parse().expect("not a number");

    let tup: (i32, f64, char) = (500, 6.4, 'w');
    let (a, b, c) = tup;
    let first = tup.0;

    let arr = [1, 2, 3, 4, 5];

    let number = if true { 5 } else { 6 };

    for n in 1..=5 {
        println!("{n}");
    }

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

