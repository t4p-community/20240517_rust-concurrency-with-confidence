# Rust Concurrency Primitives

In Rust, concurrency primitives are provided to help manage concurrent programming safely and efficiently. The main concurrency primitives in Rust are:

### 1. **Threads**

Rust provides native support for threads, allowing you to spawn new threads of execution.

- **`std::thread`**: The `thread` module provides functions for creating and managing threads.

  ```rust
  use std::thread;

  fn main() {
      let handle = thread::spawn(|| {
          println!("Hello from a new thread!");
      });

      handle.join().unwrap();
  }
  ```

### 2. **Mutexes**

Mutexes (mutual exclusions) provide a mechanism to ensure that only one thread accesses a piece of data at a time.

- **`std::sync::Mutex`**: The `Mutex` type ensures that only one thread can access the data at a time.

  ```rust
  use std::sync::{Mutex, Arc};
  use std::thread;

  fn main() {
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];

      for _ in 0..10 {
          let counter = Arc::clone(&counter);
          let handle = thread::spawn(move || {
              let mut num = counter.lock().unwrap();
              *num += 1;
          });
          handles.push(handle);
      }

      for handle in handles {
          handle.join().unwrap();
      }

      println!("Result: {}", *counter.lock().unwrap());
  }
  ```

### 3. **Atomic Types**

Atomic types provide primitive operations on variables that are atomic (indivisible).

- **`std::sync::atomic`**: This module provides atomic operations on integers and booleans.

  ```rust
  use std::sync::atomic::{AtomicUsize, Ordering};
  use std::thread;

  fn main() {
      let counter = AtomicUsize::new(0);

      let handles: Vec<_> = (0..10).map(|_| {
          thread::spawn(|| {
              counter.fetch_add(1, Ordering::SeqCst);
          })
      }).collect();

      for handle in handles {
          handle.join().unwrap();
      }

      println!("Result: {}", counter.load(Ordering::SeqCst));
  }
  ```

### 4. **Channels**

Channels provide a way for threads to communicate with each other.

- **`std::sync::mpsc`**: This module provides multiple-producer, single-consumer channels for message passing.

  ```rust
  use std::sync::mpsc;
  use std::thread;
  use std::time::Duration;

  fn main() {
      let (tx, rx) = mpsc::channel();

      thread::spawn(move || {
          let val = String::from("hi");
          tx.send(val).unwrap();
          thread::sleep(Duration::from_secs(1));
      });

      let received = rx.recv().unwrap();
      println!("Got: {}", received);
  }
  ```

### 5. **RwLock**

A reader-writer lock allows multiple readers or one writer at a time.

- **`std::sync::RwLock`**: This type allows multiple readers or one writer.

  ```rust
  use std::sync::{Arc, RwLock};
  use std::thread;

  fn main() {
      let lock = Arc::new(RwLock::new(5));
      let lock1 = Arc::clone(&lock);
      let lock2 = Arc::clone(&lock);

      let reader = thread::spawn(move || {
          let r = lock1.read().unwrap();
          println!("Reader got: {}", *r);
      });

      let writer = thread::spawn(move || {
          let mut w = lock2.write().unwrap();
          *w += 1;
          println!("Writer incremented value to: {}", *w);
      });

      reader.join().unwrap();
      writer.join().unwrap();
  }
  ```

### 6. **Arc**

Arc (Atomic Reference Counted) is a thread-safe reference-counting pointer.

- **`std::sync::Arc`**: Arc allows you to share ownership of a value across multiple threads.

  ```rust
  use std::sync::Arc;
  use std::thread;

  fn main() {
      let data = Arc::new(5);
      let mut handles = vec![];

      for _ in 0..10 {
          let data = Arc::clone(&data);
          let handle = thread::spawn(move || {
              println!("Value: {}", data);
          });
          handles.push(handle);
      }

      for handle in handles {
          handle.join().unwrap();
      }
  }
  ```

These primitives allow Rust to manage concurrency in a way that is both safe (thanks to the ownership system) and efficient.
