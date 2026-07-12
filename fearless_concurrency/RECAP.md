# Before you go...

* **Spawn a thread, join its handle.** spawn starts code running on another thread, and join waits for it to finish before you carry on.
* **Move data in with move.** Since a thread can outlive the code that started it, move hands ownership of what it captures into the thread.
* **Channels pass messages.** One thread sends values down the pipe, another picks them up, and ownership travels with each one, so there's no shared memory.
* **Arc and Mutex share state.** Arc lets threads co-own one value, and Mutex makes sure only one touches it at a time.
* **Send and Sync are the gatekeepers.** They're the marker traits that tell the compiler what's safe to move or share, which is how it blocks things like Rc before your program ever runs.