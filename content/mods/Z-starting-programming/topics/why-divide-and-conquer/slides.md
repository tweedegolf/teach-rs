---
layout: section
---

# Divide-and-Conquer

---

# Divide-and-Conquer --- Functions (1)

* Functions
    * Take in input = *Parameters*
    * Give back output = *Return Value*


```rust
// The `i32` is an integer type.
//
// `x: i32` gives the Parameters
// `-> i32` gives the Return Value
fn double(x: i32) -> i32 {
    2 * x
}

// y = 2 * x
let y = double(x);

// y = 2 * x + 6
let y = double(x) + 6;
```

---

# Divide-and-Conquer --- Functions (2)

```rust
fn brush_teeth() {
    let toothbrush = find_toothbrush();
    let toothpaste = find_toothpaste();
    put_on(toothbrush, toothpaste);

    // ...
}

fn get_into_pyjamas() {
    // ...
}

fn get_into_bed() {
    // ...
}

fn go_to_bed() {
    brush_teeth();
    get_into_pyjamas();
    get_into_bed();
}
```
