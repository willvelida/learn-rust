# Before you go...

* **Modules organize a growing codebase.** We split our code into modules, pub controls what's visible from outside, and use pulls names in so we're not writing full paths everywhere.
* **Vectors, strings, and hash maps are our everyday containers.** A vector is a growable list, a String is growable text, and a hash map maps keys to values. Most of the data we touch fits one of the three.
* **panic versus Result.** Panic is for the bugs that should be impossible, and it stops the program. Result is for the failures we expect, like a missing file, and it makes the caller respond. And when you're not sure, it's almost always Result.
* **The question mark cleans up error handling.** It means: if this failed, return the error; otherwise give me the value. No big stacks of nested matches, as long as our function returns a Result.