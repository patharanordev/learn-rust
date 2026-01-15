# Ownership vs Borrow

Ownership is a core concept in Rust that helps manage memory safety without a garbage collector. It's based on the idea that each value has a single owner, and when that owner goes out of scope, the value is automatically cleaned up.

Before start, please read and understand 2-concepts below:

1. [Stack and Heap](concept1_stack-vs-heap.md)
2. [Move and Clone](concept2_move-vs-clone.md)

***ownership → borrow → move (each statement)*** sequence, but rewritten to also include the `.clone()` case (so you can clearly see MOVE vs CLONE side-by-side).
Assume 64-bit, example addresses:

```txt
Stack: 0x7FFF_....
Heap: 0x6000_....
String stack header = ptr/len/cap = 3 × 64b
&String = 1 × 64b pointing to the stack header
```

## 1. Ownership

```rs
let s1 = String::from("hello");
```
Memory

```txt
STACK (left)                             HEAP (right)
┌────────────────────────────┐          ┌──────────────────────────┐
│ 0x7FFF_1010  s1.cap = 5     │ [64b]    │ 0x6000_0100..0104: hello │
├────────────────────────────┤          └──────────────────────────┘
│ 0x7FFF_1008  s1.len = 5     │ [64b]
├────────────────────────────┤
│ 0x7FFF_1000  s1.ptr = 0x6000_0100 ───────────────► heap bytes
└────────────────────────────┘ [64b]
```

STATE: s1 owns heap buffer @0x6000_0100

## 2. Immutable borrow (&) — same as before

```rs
let r = &s1;
```

Memory

```txt
STACK                                   HEAP
┌────────────────────────────┐          ┌──────────────────────────┐
│ 0x7FFF_1040  r = 0x7FFF_1000 ─────►   │ 0x6000_0100..: hello     │
├────────────────────────────┤ [64b]    └──────────────────────────┘
│ 0x7FFF_1010  s1.cap = 5     │ [64b]
├────────────────────────────┤
│ 0x7FFF_1008  s1.len = 5     │ [64b]
├────────────────────────────┤
│ 0x7FFF_1000  s1.ptr = 0x6000_0100 ───────────────► heap bytes
└────────────────────────────┘
```

STATE (shared borrow):
- ✅ many & allowed
- ❌ no &mut
- ❌ cannot move s1 while r alive

Important: r points to s1’s stack header, not directly to heap.

## 3. Borrow ends (scope end)

```rs
{ let r = &s1; }
```

After the block ends, r is dropped:

STATE: no borrows active → now move/clone/&mut are possible

## 4A. MOVE case: let s2 = s1; (ownership transfer)

```rs
let s2 = s1; // MOVE
```

Memory

```txt
STACK                                   HEAP
┌────────────────────────────┐          ┌──────────────────────────┐
│ 0x7FFF_1030  s2.cap = 5     │ [64b]    │ 0x6000_0100..: hello     │
├────────────────────────────┤          └──────────────────────────┘
│ 0x7FFF_1028  s2.len = 5     │ [64b]
├────────────────────────────┤
│ 0x7FFF_1020  s2.ptr = 0x6000_0100 ───────────────► heap bytes
├────────────────────────────┤
│ 0x7FFF_1010  s1.cap = <moved out>     │ [64b]
├────────────────────────────┤
│ 0x7FFF_1008  s1.len = <moved out>     │ [64b]
├────────────────────────────┤
│ 0x7FFF_1000  s1.ptr = <moved out> ❌  │ [64b]
└────────────────────────────┘
```

STATE:
- s2 is the only owner
- s1 is invalid (cannot be used)
✅ Heap buffer did not get copied; only ownership changed.

## 4B. CLONE case: let s2 = s1.clone(); (deep copy)

```rs
let s2 = s1.clone(); // CLONE (deep copy heap)
```

Memory

```txt
Now there are two separate heap buffers.

STACK                                   HEAP
┌────────────────────────────┐          ┌──────────────────────────┐
│ 0x7FFF_1030  s2.cap = 5     │ [64b]    │ 0x6000_0200..: hello     │
├────────────────────────────┤          ├──────────────────────────┤
│ 0x7FFF_1028  s2.len = 5     │ [64b]    │ 0x6000_0100..: hello     │
├────────────────────────────┤          └──────────────────────────┘
│ 0x7FFF_1020  s2.ptr = 0x6000_0200 ───────────────► heap bytes #2
├────────────────────────────┤
│ 0x7FFF_1010  s1.cap = 5     │ [64b]
├────────────────────────────┤
│ 0x7FFF_1008  s1.len = 5     │ [64b]
├────────────────────────────┤
│ 0x7FFF_1000  s1.ptr = 0x6000_0100 ───────────────► heap bytes #1
└────────────────────────────┘
```

STATE:
- s1 owns heap buffer #1 @0x6000_0100
- s2 owns heap buffer #2 @0x6000_0200
- both are valid and independent


After `.clone()`, you can modify s2 without affecting s1, because they point to different heap allocations.

### Ref. MOVE vs CLONE

|Operation      |  Heap allocation?  |  Owners after?     |     Cost  |
|---------------|--------------------|------------------------|--------|
|let s2 = s1;   |  NO (reuse same)   |   s2 only (s1 invalid) | cheap  |
|let s2 = s1.clone(); | YES (new heap)  |  s1 and s2 both own  |   more expensive |

If you want, next I can extend the same per-statement stack/heap columns with:
`.clone()` + borrowing together (e.g., `let r = &s1; let s2 = s1.clone();` ✅ allowed)
`.clone()` while `&mut` exists (spoiler: not allowed because `&mut` is exclusive)

---

## 5. Mutable References (&mut) — The "Writer's Lock"
Rust allows you to modify data via a reference, but with a catch: **Review the "Mutable Borrowing" scenario in the visualization.**

### Rule: One distinct writer OR many readers.
You cannot have a mutable reference (`&mut`) if *any* other reference (immutable `&` or mutable `&mut`) is active.
**Code**

```rs
let mut s = String::from("world");
let r1 = &s;      // ✅ OK: immutable borrow
// let r2 = &mut s; // ❌ ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable
drop(r1);         // r1 is gone
let r2 = &mut s;  // ✅ OK: now we can have a mutable borrow
```

**Memory Visualization**

- **&mut Pointer**: Often visualized as a "lock" on the original data. No other pointers can exist while this lock is active.

## 6. Copy Types (Stack-Only) — No Ownership Transfer
Fixed-size types (like integers, chars, bools) live *entirely* on the stack. They don't have a "pointer" part and a "heap" part.

**Code**

```rs
let x = 5;
let y = x; // COPY, not move
```

**Memory**

```
STACK                                   
┌────────────────────────────┐          
│ 0x7FFF_3008  y = 5          │ [64b]   
├────────────────────────────┤         
│ 0x7FFF_3000  x = 5          │ [64b]   
└────────────────────────────┘          
```

**STATE:**
- `x` is still valid.
- `y` is a completely independent copy.
- No heap allocation involved.
---

## Visualizing the "Stack Tower" vs "Heap"

> ---
> Please see [rust-ownership-viz.html](rust-ownership-viz.html) for an interactive visualization of the Stack Tower and Heap.
>
> ---

To understand *Pass by Reference* vs *Pass by Value*, imagine the memory as two distinct regions:

### The Stack Tower (Ordered, Fast)

*   **Structure**: A vertical stack of frames. Each function call pushes a new frame.
*   **Contents**: Local variables, pointers to heap, fixed-size data (integers).
*   **Addresses**: High numbers (e.g., `0x7FFF...`), growing downwards.
*   **Pass by Value**: Copies the *bits* inside the stack slot to a new slot (e.g., copying `x=5` to `y=5`, or copying the `ptr/len/cap` struct of a String).

### The Heap (Unordered, Slow)

*   **Structure**: A vast ocean of memory for dynamic data.
*   **Contents**: The actual string text `"hello"`, vectors, large structs.
*   **Addresses**: Lower numbers (e.g., `0x6000...`).
*   **Access**: You can only reach here via a **Pointer** stored in the Stack Tower.
