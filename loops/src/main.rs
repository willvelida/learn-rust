fn main() {
    // loops, run forever without a break statement
    /*
    loop {
        println!("Hello!");
    }
    */

    // Using a break condition to stop a loop expression.
    // By using break, you can stop repeating the actions in the expression body
    // and also return a value at the break point
    let mut counter = 1;
    let loop_stop = loop {
        counter *= 4;
        if counter > 100 {
            break counter;
        }
    };

    println!("Break the loop at counter = {}", loop_stop);

    // While loops use conditional expressions. The loop repeats as long as the conditional expression is true.
    let mut num = 0;
    while num < 10 {
        println!("Hello there!");
        num = num + 1;
    }

    // For loops use an iterator to process a collection of items.
    let shopping_list = ["milk", "cheese", "bread", "apples"];

    // The vale of the iterator is bound to the result of the iter() method.
    for item in shopping_list.iter() {
        println!("The next item on my shopping is {}", item);
    }

    // Another easy way to create an iterator is to use the range notation a..b
    // Iterator starts at the 'a' value and continues one by one to the value of b, without using the b value.
    for number in 0..10 {
        println!("Number {}", number);
    }
}
