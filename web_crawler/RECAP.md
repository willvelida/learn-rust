# Before you go:

* **Fetch pages asynchronously.** Each request uses reqwest and await, so while one page downloads the program can start others.
* **Visit each URL once.** A shared set records the pages we've seen, so we never fetch the same one twice or loop forever.
* **Stay on one site.** extract_links keeps only same-site links and skips assets, so the crawl follows your pages instead of wandering off across the web.
* **Share safely with Arc and Mutex.** That set lives behind Arc and Mutex, the same pair we used for shared state, so every task can co-own it and only one edits it at a time.
* **Bound the work with a semaphore.** A semaphore caps how many requests run at once.