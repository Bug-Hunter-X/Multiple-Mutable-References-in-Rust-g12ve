# Multiple Mutable References in Rust

This repository demonstrates a potential issue in Rust when dealing with mutable references.  The `bug.rs` file shows code that compiles and runs but highlights a situation to avoid for better code clarity and safety.  The `bugSolution.rs` file shows safer alternatives.

**Key Concept:** Rust's borrow checker prevents data races by disallowing multiple mutable references to the same data within the same scope.  While the example in `bug.rs` appears to work, it's generally bad practice and could lead to problems in more complex scenarios. 