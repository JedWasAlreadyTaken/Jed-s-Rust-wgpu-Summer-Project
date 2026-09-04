// In this exercise, we are given a `Vec` of `u32` called `numbers` with values
// ranging from 0 to 99. We would like to use this set of numbers within 8
// different threads simultaneously. Each thread is going to get the sum of
// every eighth value with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, …
// The second thread (offset 1), will sum 1, 9, 17, …
// The third thread (offset 2), will sum 2, 10, 18, …
// …
// The eighth thread (offset 7), will sum 7, 15, 23, …
//
// Each thread should own a reference-counting pointer to the vector of
// numbers. But `Rc` isn't thread-safe. Therefore, we need to use `Arc`.
//
// Don't get distracted by how threads are spawned and joined. We will practice
// that later in the exercises about threads.

// Don't change the lines below.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Define `shared_numbers` by using `Arc`.
     let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
         let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}

/*
What was the problem?

shared_numbers and child_numbers both needed defining. numbers is a plain Vec<u32>, and
the exercise wants 8 separate threads to each own a pointer to the same underlying vector
at once, rather than each thread getting its own copy. Rc<T> would normally do that kind
of shared ownership, but Rc isn't thread-safe - its reference count isn't updated
atomically, so two threads cloning or dropping an Rc at the same time could corrupt the
count. Since these 8 threads run concurrently, a thread-safe alternative was needed.

How do Arc::new and Arc::clone fix this?

Arc<T> ("atomically reference counted") works like Rc<T>, but its reference count uses
atomic operations, so it's safe to clone and drop from multiple threads at once without
risking a corrupted count. Arc::new(numbers) moves numbers onto the heap and wraps it in
an Arc, giving shared_numbers a thread-safe shared pointer to it. Inside the loop,
Arc::clone(&shared_numbers) creates a new Arc pointing at that same underlying vector for
each thread, incrementing the shared reference count rather than copying the data itself.
Each child_numbers is then moved into its own thread via the move closure, so every thread
gets its own Arc handle to the one shared Vec<u32>, which is what let each of the 8
threads read from the same numbers without needing to duplicate it eight times.
*/
