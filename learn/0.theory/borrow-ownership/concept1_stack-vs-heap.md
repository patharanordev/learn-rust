# Stack vs Heap

Before diving into ownership, it's crucial to understand the two memory regions Rust uses.

## Stack

- **What is it?** A LIFO (Last In, First Out) queue. Think of it as a stack of plates.
- **Characteristics**:
  - **Fast**: Creating variables is just moving a pointer locally.
  - **Fixed Size**: All data stored here must have a known, fixed size at compile time (e.g., `i32`, `bool`, `&str` pointer).
  - **Automatic Cleanup**: When a function returns, its "frame" is popped, and all variables are instantly dropped.

## Heap

- **What is it?** A large pool of memory for dynamic data.
- **Characteristics**:
  - **Slower**: Allocating requires finding a big enough empty spot. Accessing requires following a pointer (pointer chasing).
  - **Dynamic Size**: Can grow or shrink (e.g., a `String` receiving user input).
  - **Explicit Cleanup**: Historically manual (C/C++), but in Rust, **Ownership** handles this for you.

### Why use the Heap?

You might ask, "If the stack is faster, why use the heap?"
1.  **Unknown Size**: You don't know how much text a user will type (`String`).
2.  **Variable Size**: You need a list that grows (`Vec<T>`).
3.  **Large Data**: The stack is small (often few MBs). Putting massive arrays there causes a "Stack Overflow".

*Rust's system of "Ownership" exists primarily to manage this Heap data safely and efficiently without a Garbage Collector.*
