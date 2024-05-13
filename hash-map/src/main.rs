fn main() {
    
    // The 'use' command brings in the HashMap definition from the 'collections' part of the Rust standard library.
    use std::collections::HashMap;
    // We create an empty hash map using the HashMap::new() method.
    let mut items: HashMap<String, String> = HashMap::new();

    // We add elements to the hash map by using the insert(<key>, <value>) method
    items.insert(String::from("One"), String::from("Book"));
    items.insert(String::from("Two"), String::from("Keyboard"));
    items.insert(String::from("Three"), String::from("Sunglasses"));

    // After we add data to our hash map, we can retrieve a specific value for a key with the get(<key>) method.
    let keyboard = items.get("Two");
    println!("{:?}", keyboard);

    // We can also remove entries from a hash map by using the .remove(<key>) method.
    // If you try to retrieve a removed value from the hash map, 'None' will be returned.
    items.remove("Three");
    
    println!("{:?}", items.get("Three"));
}
