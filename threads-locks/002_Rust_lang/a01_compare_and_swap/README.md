# Result

```bash
ret = 1
value = 20
```

- The C code uses **GCC/Clang inline assembly** to implement an atomic compare-and-swap (CAS). In Rust, the closest idiomatic equivalent is `AtomicI32::compare_exchange`.

### Idiomatic Rust

```rust
use std::sync::atomic::{AtomicI32, Ordering};

fn compare_and_swap(ptr: &AtomicI32, old: i32, new: i32) -> u8 {
    match ptr.compare_exchange(
        old,
        new,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

fn main() {
    let value = AtomicI32::new(10);

    let ret = compare_and_swap(&value, 10, 20);

    println!("ret = {ret}");
    println!("value = {}", value.load(Ordering::SeqCst));
}
```

Output:

```text
ret = 1
value = 20
```

If the value isn't what you expected:

```rust
let value = AtomicI32::new(10);

let ret = compare_and_swap(&value, 5, 20);

println!("ret = {ret}");
println!("value = {}", value.load(Ordering::SeqCst));
```

gives:

```text
ret = 0
value = 10
```

### Mapping from C to Rust

| C                  | Rust                            |
| ------------------ | ------------------------------- |
| `int *ptr`         | `&AtomicI32`                    |
| `old`              | expected value                  |
| `new`              | replacement value               |
| `cmpxchgl`         | `compare_exchange`              |
| `sete`             | `Ok(_) => 1`, `Err(_) => 0`     |
| `lock`             | handled by the atomic operation |
| `"memory"` clobber | handled by `Ordering`           |
| `char` return      | `u8`                            |

The important part is:

```rust
ptr.compare_exchange(old, new, Ordering::SeqCst, Ordering::SeqCst)
```

Conceptually, it does:

```text
if *ptr == old {
    *ptr = new;
    return success;
} else {
    return failure;
}
```

But the entire operation is **atomic**, so another thread cannot observe an intermediate state.

### If you specifically want the C `char` behavior

Your C function returns `0` or `1`, so Rust can also use `bool`, which is more idiomatic:

```rust
use std::sync::atomic::{AtomicI32, Ordering};

fn compare_and_swap(ptr: &AtomicI32, old: i32, new: i32) -> bool {
    ptr.compare_exchange(
        old,
        new,
        Ordering::SeqCst,
        Ordering::SeqCst,
    )
    .is_ok()
}
```

Then:

```rust
let value = AtomicI32::new(10);

if compare_and_swap(&value, 10, 20) {
    println!("CAS succeeded");
} else {
    println!("CAS failed");
}
```

This is generally preferable to reproducing the inline assembly in Rust. The Rust standard library's atomic API lets the compiler select the appropriate CPU instruction, such as `cmpxchg` on x86.

