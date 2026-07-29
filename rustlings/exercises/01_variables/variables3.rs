fn main() {
    // TODO: Change the line below to fix the compiler error.
    let mut x:i32 = 5;

    println!("Number {x}");
}

/*
What the problem was
The original line was `let x: i32 = 5;` with no `mut`, in an exercise that
expects `x` to be usable as a mutable value.

Why is this a problem?
Bindings in Rust are immutable by default. `let x = 5;` means `x` can never be
reassigned or mutated — the compiler enforces that at compile time, not just by
convention.

Why does adding `mut` fix this?
`let mut x: i32 = 5;` explicitly opts `x` into mutability. This is a deliberate
design choice, not a limitation: defaulting to immutable makes it obvious, just
by reading a `let`, which values can change under you — something that matters a
lot once borrowing and concurrency show up later in the course.
*/
