use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
}

struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("cleaning up");
    }
}

fn main() {

    let shared = Rc::new(RefCell::new(vec![1,2,3]));
    shared.borrow_mut().push(4);
    println!("{:?}", shared.borrow());
    /*
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a); // cheap: bumps the count, no deep copy
    println!("count = {}", Rc::strong_count(&a));
    println!("{a} and {b}");

    
    let _r = Resource;
    println!("using the resource");

    
    let tree = Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)));
    println!("{tree:?}");
    */
}
