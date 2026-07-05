# Borrowing Rules

1. At any given moment, you can have either any number of shared references, which can only read, or exactly one mutable reference, which can write, but never both at once.

2. A reference must always be valid. You can never hold a reference to data that has already been dropped, so dangling references simply cannot exist.