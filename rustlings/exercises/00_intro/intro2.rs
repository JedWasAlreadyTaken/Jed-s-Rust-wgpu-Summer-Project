fn main() {
    // TODO: Fix the code to print "Hello world!".
    println!("Hello world!");
}

/*
What the problem was
The exercise starts with an invalid call to the `println!` macro (the standard
template has `println!("Hello world!")` written without something required to
compile — commonly the macro's `!` missing, or a format string that doesn't
match its arguments).

Why is this a problem?
`println!` isn't an ordinary function call — it's macro syntax, and the compiler
checks the format string against the arguments it's given at compile time. Get
that wrong and the program refuses to build at all.

Why does fixing the call work?
Writing a syntactically valid `println!("Hello world!");` gives the macro a
literal string with no placeholders and no arguments to check, so there's
nothing left for the compiler to reject. This up-front format-string checking is
also why Rust doesn't have C's `printf` problem, where a mismatched format
specifier is undefined behaviour discovered only at runtime (if ever).
*/
