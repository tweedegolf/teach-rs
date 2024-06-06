---
layout: section
---

# Types --- Text

---
layout: three-slots
---

# Types --- Text

::left::

- Character: `c`, `🐵`, Space
- String: `Monkey 🐵`, `A piece of text`

::right::

```rust
let text = "Monkey 🐵";

for character in text.chars() {
    // `print` a `line` (ln) to the screen
    println!("{character}");
}
```

#### Output:

```
M
o
n
k
e
y
 
🐵
```
