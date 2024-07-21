---
layout: section
---

# Types --- Booleans

---

# Types --- Booleans (1)

* Boolean answer a *yes/no question* (`true` or `false`)
* Examples:
  * Is $x$ even? $\to$ $x$ is even
  * Is $x$ larger than 42? $\to$ $x$ is larger than 42
  * Does text start with "Wow"? $\to$ text starts with "Wow"
  * Is the left mouse button pressed? $\to$ Left mouse button is pressed

```rust
let is_x_larger_than_42 = x > 42;

let does_text_start_with_wow = text.starts_with("Wow");
```

---

# Types --- Booleans (2)

* Booleans are also the conditions for `if`

```rust
let age = 19;
let is_adult = age > 17;

if is_adult {
    println!("You are an adult 👩");
} else {
    println!("You are a child 🧒");
}
```

---

# Types --- Booleans (3)

* Booleans can be combined
  * This or that?
  * This and that?

```rust
if number > 10 && number < 20 {
    println!("{number} is somewhere between 11 and 19");
}

if number < 10 || number > 20 {
    println!("{number} is lower than 10 or higher than 20");
}
```
