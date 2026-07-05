# Before you go...

* **A struct groups labeled data.** Instead of juggling loose variables, you bundle related fields under one named type, and each field has its own name so the meaning is clear.
* **Shorthand syntax saves typing.** Field init shorthand lets you write a field once when the variable already has that name, and update syntax copies the rest of the fields from another value so you only spell out what changes.
* **Tuple and unit structs are lighter variants.** Use a tuple struct when the field names would not add much, and a unit struct when you need a type but no data at all.
* **Methods add behavior.** Functions written in an impl block that take self become methods you call with a dot. Most borrow self so they can read the value without taking ownership.
* **Associated functions act like constructors.** A function in the impl block that does not take self, like the common new, builds and returns a value of the type.
* **Derive Debug prints for free.** Adding the derive Debug line lets you print a struct with the debug formatter, and the hash version pretty prints it across lines while you inspect things.