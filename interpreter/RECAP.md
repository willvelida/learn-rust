# Before you go....


* **The lexer turned text into tokens.** It read through the characters and grouped them into the pieces that matter: numbers, operators, and brackets.
* **The AST gave those tokens a shape.** Every kind of expression was an enum variant, and we used Box so an expression could hold other expressions inside it.
* **The parser put things in the right order.** It built the tree so multiply and divide happen before plus and minus, just like in normal maths.
* **The evaluator worked out the answer.** It walked the tree, handled each kind of node with match, and kept our variables in a hash map.
* **Result caught the mistakes.** Whenever something went wrong, a bad character, an unknown variable, a divide by zero, we got a clean error back instead of a crash.