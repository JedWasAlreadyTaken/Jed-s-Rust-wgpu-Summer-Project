// This program spawns multiple threads that each runs for at least 250ms, and
// each thread returns how much time it took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.
        let el_time = handle.join().expect("A thread failed");
        results.push(el_time)
    }


    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}

/*
What was the problem?

thread::spawn returns a JoinHandle for each spawned thread, but the main thread doesn't
automatically wait for those threads to finish or get their return values back - the
threads run independently in the background. The loop over handles had a TODO where
nothing was done with each handle, so results stayed empty and the program would panic
at the results.len() != 10 check, since no thread's return value had actually been
collected yet.

How does handle.join() fix this?

.join() blocks the calling thread (main, here) until that specific spawned thread finishes
running, and returns a Result containing whatever value the thread's closure returned -
in this case, start.elapsed().as_millis() from inside the closure. .expect("A thread
failed") unwraps that Result, panicking with that message if the thread itself panicked
instead of completing normally. Pushing the unwrapped value onto results means that after
the loop has gone through every handle, results contains exactly one entry per thread -
its elapsed time - and only once every thread has actually finished, since the loop can't
move past a given handle.join() until that thread returns.
*/
