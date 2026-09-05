// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex}, 
    thread, 
    time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0}));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    let status = status.lock().unwrap();
    println!("Jobs done: {}", status.jobs_done);
}

/*
What was the problem?

Arc<T> only gives shared ownership across threads - it doesn't allow mutation, since
multiple threads could otherwise write to jobs_done at the same time and corrupt it (a
data race). Wrapping JobStatus directly in Arc<JobStatus> would let every thread read
jobs_done, but none of them could actually increment it, since Arc only hands out shared
(&T) references, never a mutable one. Each thread's closure also needed to actually do
something to update the shared jobs_done value before incrementing it - just writing
status_shared.jobs_done += 1 directly wouldn't compile for the same reason. Finally,
main needed to read jobs_done back out after all threads finished, to print the final
count.


How does let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0})); fix this?

We need firstly a way to lock the shared data to one thread at a time, therefore a Mutex
is needed, since it can hold the lock for one thread at a time, forcing every other thread
that wants access to wait until that lock is released. Mutex<T> wraps the JobStatus and
provides interior mutability with that safety guarantee built in, so wrapping it inside
Arc::new(Mutex::new(...)) gives every thread shared ownership (via Arc) of a value that can
still be safely mutated (via Mutex), which Arc<JobStatus> alone couldn't provide.

Inside each thread's closure, status_shared.lock() requests that lock, blocking the thread
until it's free, and returns a Result - .unwrap() extracts the actual MutexGuard from that
Result, panicking only if another thread had already poisoned the lock by panicking while
holding it. The returned MutexGuard derefs to the underlying JobStatus, so status.jobs_done
+= 1 can mutate it directly. The lock is automatically released once status goes out of
scope at the end of the closure, letting the next waiting thread acquire it in turn - this
is what makes status_shared.jobs_done += 1 on its own invalid but status.jobs_done += 1
(through the locked guard) valid: the mutation is only ever allowed while holding the lock.

Finally, let status = status.lock().unwrap(); in main, after all handles have been
joined, acquires the lock one last time to read the final jobs_done value once every
thread has finished incrementing it, so println! prints the completed count rather than
whatever value happened to be there mid-run.
*/

