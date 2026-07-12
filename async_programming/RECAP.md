# Before you go...

* **An async function hands back a future.** Calling it runs nothing, you get a value that stands for work still to come.
* **Await runs a future and steps aside.** It drives the future to its value, and while that future waits, the thread gets on with other work.
* **Futures need a runtime.** They can't run themselves, so a runtime like Tokio schedules and drives them, which is what that tokio main attribute sets up.
* **Join overlaps the waiting.** It drives several futures together on one thread, and a stream carries the same idea to items that arrive over time.
* **Async waits, threads compute.** Pick async when the work mostly waits, and threads when it mostly does heavy computation.