---
layout: section
---

# Mutation

---

# Mutation (1)

- How to modify a variable's value?

```rust
// Get the sum over the items
let items = [1, 3, 3, 7];
let sum = 0;

for item in items {
    let sum = sum + item;
}

// Output: `0`
println!("{sum}");
```

---

# Mutation (2)

![Compiler Warning](images/A0-mutation-sum-warning.png)

---

# Mutation (3)

- Solution: *Mutation*

```rust
// Get the sum over the items
let items = [1, 3, 3, 7];
let mut sum = 0;

for item in items {
    sum = sum + item;
}

// Will output `14`
println!("{sum}");
```
