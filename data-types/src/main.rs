// Classical Structs. Includes the field name and it's data type
struct Student {
    name: String,
    level: u8,
    remote: bool
}

// Tuple structs. Just the data types in this one
struct Grades(char,char,char, f32);

// Start of our main function. 'fn' is a keyword in Rust that denotes that this is a function.
// Functions have parameters that we can pass in, and we can also return values from our function. More on this later.
fn main() {

    // the println! marco is a built-in function from Rust. This prints output to the console.
    // We can use value substitution for {} arguments
    println!("Hello, {} {}!", "Will", "Velida");

    // Variable bindings are immutable by default, meaning once we bind a value to variables, we can't change them.
    // We can use the 'mut' keyword to make a variable mutuable (meaning we can change its value AFTER we declare and bind a value to it).
    let mut age = 33;
    let birth_year = 1991;

    println!("I am {} years old", age);

    // We can also declare a new variable that uses the name of an existing variable.
    // This is called 'variable shadowing' as it 'shadows' the previous variable.
    // It still exists, but you can't refer to it in this scope anymore.
    let birth_year = birth_year - 1;
    
    age = 34;

    println!("I am now {} years old", age);
    println!("I was born in {}", birth_year);

    // Rust is a statically typed language. The complier must know the exact data type for all variables in your code to compile and run.
    // The compiler can usually infer the data type for a variable based on the bound value.
    // Here, we're telling the compiler to create a nephew_age variable as a 32-bit integer.
    // We specify the data type u32 after the variable name
    let nephew_age: u32 = 14;
    println!("My nephew is {} years old", nephew_age);

    // Rust comes with some built in data types.

    // Float
    let float: f32 = 4.0;
    
    println!("1 x 2 = {}", 1*2);

    // Boolen: true or false values.
    let is_bigger_num = 2 < 4;
    println!("Is 2 < 4: {}", is_bigger_num);

    // Strings
    // Character types. A single character.
    let first_char: char = 'W';
    let last_char: char = 'l';

    let second_char = 'i';

    // Strings which are series of characters.
    // Most of the time, String literals are of type &str
    let my_name = "Will";

    println!("{} is the first character, {} is the last character, {} is the second character of my name {}", first_char, last_char, second_char, my_name);

    // Tuples are a grouping of values of different types collected into one compound value.
    // The individual values inside a tuple are called elements.
    // Tuples have a fixed length. Once declared, it can't grow or shrink in size, elements can't be added or removed.
    let my_dog = ("Toby", 15, false);

    println!("My dog's name was {}, he was {} years old, is he alive? {}", my_dog.0, my_dog.1, my_dog.2);
    
    // After we define a struct type, we can use it by creating an instance of the type and specifying the values for each field.
    let student_1 = Student{
        name: String::from("Will Velida"),
        remote: true,
        level: 5
    };

    let grades = Grades('A','A','B',3.5);

    // For classical structs, we can get the value by referring to the name of the property
    println!("{}, is a level {} programmer. Does he work remotely: {}",
        student_1.name, student_1.level, student_1.remote);

    // For tuple structs, we get the value by referring to its position in the index.
    println!("{},{},{},GPA = {}", grades.0, grades.1, grades.2, grades.3);
}