Several ways exist to solve this data race, depending on the specific requirements:

**1. Using Interior Mutability:**

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);

    let y = x.borrow_mut();
    *y = 6;

    let z = x.borrow_mut();
    *z = 7; // this will not compile, because of the borrow

    println!("x = {}", x.borrow()); //Prints 6
}
```

Using `RefCell` provides interior mutability, preventing simultaneous mutable access to the same value.

**2. Using Mutex (for concurrent access):**

```rust
use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));

    let y = x.clone();
    let z = x.clone();

    let _ = std::thread::spawn(move || {
       let mut y_guard = y.lock().unwrap();
       *y_guard = 6;
    });

    let mut z_guard = z.lock().unwrap();
    *z_guard = 7; 

    println!("x = {}", *x.lock().unwrap()); //Prints 7
}
```

This utilizes a `Mutex` to safeguard against race conditions, ensuring that only one thread can modify `x` at any given time.