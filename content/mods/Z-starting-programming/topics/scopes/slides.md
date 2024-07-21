---
layout: section
---

# Scopes

---

# Scopes (1)

- A *scope* is an isolated variable and function environment
- Scope is indicated with `{ ... }`

```rust
let x = 10;

{
	let y = 20;
}

// Will give an error: "cannot find value `y` in this scope"
let z = x + y;
```

---

# Scopes (2)

- Many earlier concepts will open a new scope

#### If Statements

```rust
if x == y {
	let z = 10; // `z` defined in this scope
}
// `z` inaccessible in this scope
```

#### Loops

```rust
for character in text.chars() {
	// `character` defined in this scope
}
// `character` inaccessible in this scope
```

#### Functions
```rust
fn double(number: i32) -> i32 {
	// `number` defined in this scope
}
// `number` inaccessible in this scope
```
