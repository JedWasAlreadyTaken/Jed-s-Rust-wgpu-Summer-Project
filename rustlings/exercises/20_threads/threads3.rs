use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?
    let sender = tx.clone();
    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending {val:?}");

            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending {val:?}");
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}

/*
What was the problem?

Both thread::spawn closures needed to send values through tx, but each move closure takes
ownership of whatever it captures. The first closure moved tx into itself, so by the time
the second closure tried to move tx too, it had already been moved away and no longer
existed to capture - tx can only have one owner at a time, and a single mpsc::Sender can't
be owned by two threads simultaneously.

This isn't a shared-mutable-state problem like threads2's jobs_done counter, so Arc<Mutex<T>>
wasn't the right tool here - tx isn't a value being mutated that needs locking, it's a
handle to a channel, and mpsc::Sender is specifically designed to support multiple
independent senders without any locking at all.

How does let sender = tx.clone(); fix this?

mpsc::Sender<T> implements Clone for exactly this situation: cloning a Sender doesn't
duplicate the channel, it creates a second, independent handle to the same underlying
channel, and both the original and the clone can send messages that all land in the same
receiver (rx). Calling tx.clone() before the first thread::spawn creates sender as that
second handle, before the first closure's move has a chance to take ownership of the
original tx. That way, the first closure moves and owns tx, and the second closure moves
and owns sender instead - two separate Sender values, each valid to move into its own
thread, both writing into the same channel that rx reads from.
*/
