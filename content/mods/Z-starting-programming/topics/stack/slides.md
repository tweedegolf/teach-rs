---
layout: section
---

# Stack

---

# Stack

- *Stack*: Variables that belong to a scope
- Scope ends $\implies$ Variable gone

```rust
// `a` get pushed on the stack
let a = 3;
{
    // `b` get pushed on the stack
    let b = 10;

    // `x` get pushed on the stack
    let x = a * b;
} // `b` and `x` get removed from the stack
```
