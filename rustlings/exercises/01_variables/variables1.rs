fn main() {
    // TODO: Add the missing keyword.
    let x = 5;

    println!("x has the value {x}");
}

/*
What the problem was
The original line was just `x = 5;` — a bare assignment with no `let`, which
isn't a valid variable declaration in Rust.

Why is this a problem?
Rust doesn't have implicit variable declaration the way Python or JavaScript
do. You can't just assign to a name and have it spring into existence — the
compiler rejects `x = 5;` because `x` was never introduced as a binding.

Why does adding `let` fix this?
`let x = 5;` is what actually introduces `x` as a new binding in scope. This
catches typos early too — an undeclared name is a compile error, not a silent
global — and it's the same keyword every later feature in this section builds
on: mutability, shadowing, and type annotations all start from `let`.
*/
