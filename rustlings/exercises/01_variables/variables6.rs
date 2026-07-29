// TODO: Change the line below to fix the compiler error.
const NUMBER:i8 = 3;

fn main() {
    println!("Number: {NUMBER}");
}

/*
What the problem was
The original line was `const NUMBER = 3;` — a `const` declared with no type
annotation.

Why is this a problem?
Unlike `let`, `const` can never have its type inferred — it must always be
written out explicitly. `const NUMBER = 3;` is missing that, so it doesn't
compile.

Why does `const NUMBER: i8 = 3;` fix this?
Adding the explicit `: i8` satisfies that requirement. This isn't an arbitrary
extra rule — a `const` isn't a memory location at all; it's inlined at every use
site by the compiler (closer to a C `#define` than a variable), and by
convention its name is SCREAMING_SNAKE_CASE. Because it's baked into the binary
at compile time rather than computed at runtime, there's no value the compiler
could inspect to infer a type from — it has to be told.
*/
