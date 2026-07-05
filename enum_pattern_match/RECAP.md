# Before you go...

* **An enum is a value with several shapes.** It lets one type be exactly one of a fixed set of variants, and each variant can carry its own data. It is perfect when something is "one of these options."
* **Option replaces null.** Instead of a value that might secretly be nothing, Rust makes the absence visible: Some holds a value, None means there is none. You cannot forget to handle the empty case, because the type makes you.
* **Match forces you to handle every case.** When you match on an enum, the compiler checks that you covered every variant. Forget one and it will not compile, which turns a whole class of bugs into an error you see immediately.
* **If let and let else are the lightweight tools.** When you only care about one pattern, if let handles it in a couple of lines, and let else pulls the value out or bails early, keeping your main path flat and readable.