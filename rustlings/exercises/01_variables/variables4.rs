// TODO: Fix the compiler error.
fn main() {
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}

/*
What the problem was
The original declaration was `let x = 3;` (no `mut`), but the line below it,
`x = 5;`, reassigns `x` — and that line isn't allowed to change.

Why is this a problem?
Same immutable-by-default rule as variables3: an `x` declared without `mut`
can never be reassigned, so `x = 5;` is a compile error against a plain `let x`.

Why does adding `mut` to the declaration fix this?
`let mut x = 3;` marks the binding mutable from the start, so the later
`x = 5;` becomes legal without touching the reassignment line itself. It's the
same rule as variables3, just approached from the other direction — here the
reassignment already exists in the code and it's the declaration that has to
accommodate it.
*/
