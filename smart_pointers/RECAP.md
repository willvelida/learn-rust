# Before you go...

* **Box puts data on the heap.** It holds one value behind a pointer of known size, which is what lets a type contain itself, like our interpreter's tree.
* **Deref and Drop make smart pointers feel native.** Deref lets you use a smart pointer as if it were the value inside; Drop runs cleanup code automatically when the value goes out of scope.
* **Rc gives shared ownership.** It lets several owners share one value on a single thread, and frees it only when the last owner goes away. Its thread-safe cousin, for sharing across threads, is Arc.
* **RefCell adds interior mutability.** It lets you mutate through a shared reference by moving the borrow check to run time, so breaking the rules panics instead of failing to compile.