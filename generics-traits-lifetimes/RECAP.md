# Before you go....

* **Generics let you write code once for many types.** Instead of copying a function out for numbers and again for words, you write it once with a placeholder, and Rust fills in the real type.
* **Traits are shared behavior.** A trait is a promise that a type can do something, and different types can keep that promise in their own way.
* **Trait bounds connect the two.** A bound says a type must implement a trait, and that's what lets a generic function call its methods.
* **Lifetimes say how long a reference is valid.** They don't change anything. They just let the compiler check that a reference never points at data that's already gone.