// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}

/*
What the problem was
The body originally had a semicolon: `num * num;`.

Why is this a problem?
With the semicolon, `num * num;` is a statement, which evaluates to `()` (unit)
— it doesn't match the function's declared `-> i32` return type, so the
function's implicit final-expression return is `()` instead of an `i32`.

Why does removing the semicolon fix this?
Without it, `num * num` is a tail expression, and the value of the last
expression in a block (with no trailing semicolon) becomes that block's value —
here, the function's return value. This is one of the most Rust-specific habits
to build: a semicolon isn't just punctuation, it's the difference between "an
expression that produces a value" and "a statement that discards it".
*/

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
