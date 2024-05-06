fn main() {

    // Arrays: A collection of objects of the same type stored sequentially in memory
    // You can declare an array, initialize all values, and the compiler will infer the length
    let working_days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    // You can also declare the array, initialize all values, and specify the length
    let working_days_num = [0; 5];

    // We can also index into the array, using the position of the element.
    println!("{}", working_days[0]);

    // Vectors also store multiple values that have the same data type
    // We can declare the vector, initialize all the values
    let nephews_age = vec![14, 9, 0];
    println!("Nephews age: {:?}", nephews_age);

    // We can also declare the vector, intialize all values, and specify the length
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // We can add or remove values to a vector using the push or pop methods.
    let mut names = Vec::new();

    names.push("Will");
    names.push("Isaac");
    names.push("Sam");

    println!("Names: {:?}", names);

    names.pop();
    println!("Names: {:?}", names);

    // We can also access an element in the vector by its position in the vector
    let mut fruit = vec!["Apple", "Melon", "Orange"];
    let orange = fruit[2];
    fruit[0] = "Strawberry";
    println!("Fruits: {:?}, Orange = {}", fruit, orange);
    
}
