# Before you go...

* **Read the arguments into a Config.** We take the arguments iterator straight into a Config and check it makes sense before doing any work.
* **Keep main thin, put the logic in the library.** main just wires things up; the real work lives in src/lib.rs, which is exactly what makes it easy to test.
* **Count with an iterator, store in a hash map.** One chain splits, normalizes, filters, and folds every word into a map of counts.
* **Rank with a vector.** We tip the pairs into a vector and sort by count, biggest first, breaking ties alphabetically so the output is stable.
* **Errors return Box dyn Error and go to standard error.** One boxed type covers every kind of failure, and standard error keeps messages out of piped output.
* **Directory walking is recursive.** The tool steps into subfolders by calling itself on each one, counting files and exploring folders all the way down.
* **Tests let you add features safely.** Non-text files get skipped instead of crashing, and because our tests still passed, we added directory walking knowing we hadn't broken the counting.