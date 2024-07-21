---
layout: section
---

# What is Programming

---
layout: three-slots
---

# **Goal**: let machine turn *Input* into *Output*

::left::

* Examples
  * $x \mapsto 2x$
  * "ThIs Is SoMe TeXt" $\mapsto$ "this is some text"
  * [1, 1337, 42] $\mapsto$ [uneven, uneven, even]
  * Game Assets $\mapsto$ Pixels of the screen

* Important Questions:
  * How to **unambiguously** *express our intentions*?
  * How to *revise* our intentions?

::right::

<img src="images/A0-abstract-program.svg" alt="Abstract Program" class="mx-auto" />

---
layout: three-slots
---

# Idea 1: Use Numbers

::left::

* Numbers are simple for computers to understand

```rust
// x --> 2x
2,x,01

// x --> 2x + 6 - 10
2,x,01,6,02,10,03

// x --> 1 + x + x^2 + x^3
1,x,02,x,x,01,02,x,x,01,x,01,02
```

* **Problem**: Difficult for Humans to reason about

::right::

| Operation Number | Behavior                      |
|------------------|-------------------------------|
| 01               | Multiply Previous Two Numbers |
| 02               | Add Previous Two Numbers      |
| 03               | Subtract Previous Two Numbers |

---

# Idea 2: Use normal language

* Normal language is understandable people and is known almost everyone

```
// x --> 2x
Given an x, multiply 2 with that x.

// x --> 2x + 6 - 10
Given an number, multiply two with that number. Add six to that answer and substract ten from the answer.
```

* **Problems**
  * Ambiguous: "Subtract from that answer". Which answer?
  * Difficult to computers to reason about
  * Verbose

---

# Idea 3: Programming Language

* A well-defined language understandable to machines

```rust
// x --> 1 + x + x^2 + x^3
let answer = 1 + x + x*x + x*x*x;
```

* Properties
    * Concise
    * Unambiguous
    * Understandable to machines

---

# What is Programming (1)

* Programming Language: *Human-Readable* format to express machine instructions
* This course discusses the *Rust Programming Language*

```rust
// Multiply `x` by 2
2 * x

// Convert some text to lowercase
"ThIs Is SoMe TeXt".to_lowercase()
```

---

# What is Programming (2)

* Solve problems by a **divide-and-conquer** approach

```
Going to Bed:
    1. Brush Teeth
       1. Find Toothbrush
       2. Find Toothpaste
       3. Put Toothpaste on Toothbrush
       4. ...
    2. Get into Pyjamas
       1. ...
    3. Get into bed
```

* $\implies$ Programs consists of smaller *isolated* parts
