# Threads in Tokio

Tokio uses two different thread pools to handle work efficiently:

1. Async worker threads - These are used to run non-blocking async tasks (tasks that use await).

    - Designed for fast, lightweight operations
    - Should never block (no sleep, no sync I/O, no heavy CPU work)
    - Examples:
    
      - HTTP requests with reqwest
      - Async database queries
      - Timers with tokio::time::sleep

2. Blocking thread pool - This pool is for blocking or CPU-heavy tasks. Examples of blocking work:

    - std::thread::sleep
    - Synchronous file I/O
    - zmq::poll
    - Heavy cryptography
    - Large data processing
    - Compression / encryption

## Task spawn

### tokio::spawn

`tokio::spawn` runs a non-blocking async task on Tokio’s async worker threads.

### tokio::task::spawn_blocking

Run this blocking task outside the async runtime.

What it does:

- Sends the task to the blocking thread pool
- Allows the code to block safely
- Prevents async worker threads from freezing
- Still lets you await the result

```rs
let result = tokio::task::spawn_blocking(|| {
  heavy_crypto_work()
}).await?;
```

#### What NOT to do

If you run blocking code like this:

```rs
tokio::spawn(async {
  std::thread::sleep(std::time::Duration::from_secs(5));
})
```

Then:

- A Tokio worker thread is blocked for 5 seconds
- Other async tasks have to wait
- Under high load, your app becomes slow or unresponsive

#### Why this matters

Async runtimes like Tokio are optimized for:

- Thousands of concurrent tasks
- Fast context switching
- Non-blocking I/O

Blocking code breaks this model and wastes worker threads.
