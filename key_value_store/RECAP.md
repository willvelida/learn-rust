# Before you go...

* **Accept TCP connections.** The server listens on a port and answers a small text protocol over each client connection, real network programming with no framework.
* **Share the map with Arc and Mutex.** One hash map lives behind Arc and Mutex, so every worker co-owns it and only one touches it at a time.
* **Serve many clients with a thread pool.** A fixed pool of worker threads pulls jobs off a queue, instead of spawning a fresh thread per connection forever.
* **Shut down gracefully with Drop.** When the pool is dropped, its Drop code stops the workers and waits for them to finish, so no thread is abandoned mid-job.