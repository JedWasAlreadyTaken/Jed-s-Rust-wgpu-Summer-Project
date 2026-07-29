fn main() {
    // TODO: Change the line below to fix the compiler error.
    let x: i32 = -128;

    if x >= 10 {
        println!("x is higher than ten!");
    } else {
        println!("x is lower than ten!");
    }
}

/*
What the problem was
The original `let x` line didn't give the compiler a value it could compare
against `10` with `>=` — either `x` was declared without a valid initializer, or
its type didn't line up with the numeric comparison below it.

Why is this a problem?
`x >= 10` requires `x` to actually be a number the compiler can compare, and
every `let` binding has to be definitely initialized before it's read. Rust also
never silently coerces between numeric types, so anything that isn't cleanly an
integer of a matching type fails to compile rather than getting quietly
converted.

Why does `let x: i32 = -128;` fix this?
It gives `x` a concrete, correctly-typed value up front — an `i32` literal that
`>=` can compare directly against the `10` literal, with no ambiguity for the
compiler to reject.
*/
