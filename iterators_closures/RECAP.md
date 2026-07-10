# Before you go...

* **A closure is an inline function that remembers its surroundings.** You write a small unnamed function right where you need it, and it can grab variables from the code around it.
* **There are three kinds of closure.** Fn just reads what it captured, FnMut changes it, and FnOnce uses it up. Rust picks the right one for you.
* **Iterators are lazy.** They hand you one value at a time through next, and do no work until something asks. That's what makes long chains cheap.
* **Adapters transform, consumers finish.** map and filter add a step and hand back a new iterator; collect or sum are what actually run the chain. Nothing happens until a consumer runs.
* **Chains are as fast as a loop.** The compiler turns your iterator chain into the same machine code as a hand-written loop, so it's readable and fast at the same time. That's a zero-cost abstraction.