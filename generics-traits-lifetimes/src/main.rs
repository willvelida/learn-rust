struct Pair<T> {
    first: T,
    second: T,
}

struct Book {
    title: String
}

trait Preview {
    fn preview(&self) -> String {
        String::from("[no preview]")
    }
}

struct Podcast {
    title: String,
}

impl Preview for Book {
    fn preview(&self) -> String {
        format!("preivew of {}", self.title)
    }
}

impl Preview for Podcast {
    fn preview(&self) -> String {
        format!("podcast: {}", self.title)
    }
}

fn announce<T: Preview>(item: &T) {
    println!("New: {}", item.preview());
}

fn announce_shorthand(item: &impl Preview) {
    println!("New: {}", item.preview());
}

fn first_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    println!("{}", first_longer("ferris", "corro"));
    /*
    let temps = vec![7, -3, 21, 4, 18];
    println!("largest: {}", largest(&temps));

    let words = vec!["apple", "pear", "fig"];
    println!("largest: {}", largest(&words));

    let p = Pair { first: 1.5, second: 4.0 };
    println!("pair: ({}, {})", p.first, p.second);
    

    let book = Book {
        title: String::from("The Art of War"),
    };
    println!("{}", book.preview());
    
    let show = Podcast {
        title: String::from("The Will Velida Hour"),
    };


    announce(&show);
    announce_shorthand(&show);
    */
}


fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in list {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}
