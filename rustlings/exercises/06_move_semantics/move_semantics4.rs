fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

/*
What the problem was
The original ordering created `z = &mut x` while `y` (also `&mut x`) was still
considered "alive" — for instance, having both borrows created before either
was used, or `y` being referenced again after `z` existed.

Why is this a problem?
Rust allows only one mutable borrow of a value at a time (or many immutable
borrows, but never a mutable one alongside any other). This isn't just a
scoping rule — it's what prevents data races and iterator-invalidation bugs at
compile time, before the program ever runs.

Why does reordering the lines fix this?
As written above, `y.push(42)` is `y`'s last use — its borrow's lifetime ends
right there — so `z = &mut x` can start cleanly afterward with no overlap.
Thanks to "non-lexical lifetimes" (NLL), a borrow's scope ends at its last use,
not at the end of the enclosing block, which is exactly why reordering (so `y`
is fully finished before `z` is born) is enough to satisfy the borrow checker
without adding, changing, or removing any line.
*/
