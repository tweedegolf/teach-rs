---
layout: section
---

# Heap

---

# Heap

- *Heap*: Variables that belong to the entire program. You only get the receipt.
- Receipt is gone $\implies$ variable data is gone
- Useful for:
  - Unknown time of use
  - Unknown size

```rust
{
    // `numbers` is only a receipt to the data
    let mut numbers = vec![];

    // numbers = []

    list_of_numbers.push(1); // numbers = [1]
    list_of_numbers.push(3); // numbers = [1, 3]
    list_of_numbers.push(3); // numbers = [1, 3, 3]
    list_of_numbers.push(7); // numbers = [1, 3, 3, 7]
} // `numbers` gets removed from the stack, so data is also gone
```
