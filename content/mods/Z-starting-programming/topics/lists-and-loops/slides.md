---
layout: section
---

# Lists and Loops

---

# Lists and Loops

* List = **Array**
* List of multiple elements
  * Usernames and Passwords
  * Pixels on the Screen

```rust
let list = [1, 1337, 42];
```

---

# Lists and Loops --- Moving over Elements

```rust
let list = [1, 1337, 42];

for number in list {
    println!("{number}");
}
```

#### Output:

```
1
1337
42
```

---

# Lists and Loops --- Sum of Elements

```rust
let list = [1, 1337, 42];

let mut sum = 0;
for number in list {
    sum = sum + number;
}

println!("Sum of List: {sum}");
```

#### Output:

```
Sum of List: 1390
```

---

# Lists and Loops --- Loops without Lists

```rust
// NOTE: Does not include `1340`
for number in 1337..1340 {
    println!("{number}");
}
```

#### Output:

```
1337
1338
1339
```

---

# Lists and Loops --- Unconditional Loops

```rust
loop {
    println!("Please help! I cannot stop!");
}
```

#### Output:

```
Please help! I cannot stop!
Please help! I cannot stop!
Please help! I cannot stop!
Please help! I cannot stop!
...
```

---

# List and Loops --- Breaking Early

```rust
let mut number = 1337;
loop {
    // If the number is greater than 3, break out of the loop
    if number > 1339 {
        break;
    }

    println!("{number}");
    number = number + 1;
}
```

#### Output:

```
1337
1338
1339
```
