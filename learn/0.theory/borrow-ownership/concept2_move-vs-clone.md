# Move vs .clone() (Shallow vs Deep Copy)

When you assign a variable `let y = x`, what happens to the memory?

## 1. Shallow Copy (The "Move") — Default for Heap Types

- Copies *only* the **Stack** data (the pointer/length/capacity).
- **Fast**: It's just copying 3 integers (192 bits).
- **Problem**: Now two keys point to the same house. Who cleans it up?
- **Rust's Solution**: The first variable (`x`) is immediately invalidated ("Moved").

## 2. Deep Copy (`.clone()`) — Explicit Opt-in

- Copies the **Stack** data AND recursively copies the **Heap** data.
- **Slow**: If you have 1GB string, it copies 1GB of memory.
- **Explicit**: Rust forces you to write `.clone()` so you generally *see* where expensive operations happen. If it was automatic, your code could be slow without you noticing.
