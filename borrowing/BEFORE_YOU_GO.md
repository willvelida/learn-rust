# Before you go

* **A reference borrows instead of moves.** Writing an ampersand in front of a value lends it out, so the original owner keeps the value and can still use it after the call.

* **Ampersand mut borrows for writing.** A plain reference can only read. When a function needs to change your value in place, you hand it an ampersand mut, and three things must agree: the variable is mut, the parameter is ampersand mut, and the call site passes ampersand mut.

* **Many readers or one writer, never both.** At any moment you can have any number of shared references, or exactly one mutable reference. That rule is what makes data races impossible, and the compiler checks it for you before the program ever runs.

* **A slice is a borrowed window.** A slice points at part of a string or array without copying it, so it is cheap. Prefer ampersand str for function parameters, because it accepts both a String and a plain literal.