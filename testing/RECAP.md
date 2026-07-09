# Before you go...

* **A test is just a function with a tag.** Mark it with the test attribute, and cargo test finds it and runs it. Nothing to install, it's built right in.
* **Assertion macros check the results.** You say what you expect, and macros like assert_eq compare it against what you actually got. If they don't match, the test fails and shows you both values.
* **You can test failure on purpose.** should_panic makes a test pass only when the code panics, so you can prove your error cases really fire. And a test can return a Result, so the question mark works inside it too.
* **Unit and integration tests do different jobs.** Unit tests sit next to your code and check the small pieces from the inside. Integration tests live in their own folder and use your library the way a real user would.