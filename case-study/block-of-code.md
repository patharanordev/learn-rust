# Block of Code

A `{ ... }` block only creates a temporary scope. You ***normally use it when you want a variable to drop early***, like:

```rs
{
    let guard = lock.write();
    // use lock
} // guard dropped here
```

When would you need the block?

Only if:

- The function returns something that ***holds a borrow of object***
- And you need that ***borrow to end before later*** code:

    ```rs
    {
        let guard = object.lock();
        // use guard
    } // borrow ends here before next line
    ```
