---
layout: section
---

# Conditionals --- `if`

---

# Conditionals

* Output changes based on a condition
* *Branches* the execution flow
* Commonly done with `if` and optional `else`

```rust
if (
    // A condition
) {
    // If the condition is true do this.
} else {
    // Else do this
}
```

---

# Conditionals --- Example (1)

```rust {all|1|3|4}
let age = 19;

if age > 17 {
    println!("You are an adult 👩");
} else {
    println!("You are a child 🧒");
}
```

#### Output:

```
You are an adult 👩
```

---

# Conditionals --- Example (2)

#### Code:

```rust {all|1|3|4}
let age = 12;

if age > 17 {
    println!("You are an adult 👩");
} else {
    println!("You are a child 🧒");
}
```

#### Output:

``` 
You are a child 🧒
```

---

# Conditionals --- Example (3)

```rust {1|3|4}
let role = "Student";

// `==` asks whether they are equal
if role == "Administrator" {
    println!("All systems operational! 🔓");
} else {
    println!("You don't have the authority!");
}
```

#### Output:

```
You are an adult 👩
```

---

# Conditionals --- Operations

- Equality: `==`
- Inequality: `!=`
- Comparison: `<`, `>`, `<=`, `>=`
- Invert: `!`
