// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num:u64) {
    for i in 0..=num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(8);
}

/*
What the problem was
The original signature was `fn call_me(num) { ... }` — the parameter `num` had
no type annotation at all.

Why is this a problem?
Rust never infers function parameter types the way it infers `let` bindings —
type inference only works within a function body, based on how a value is
used there. Signatures are always fully explicit, so a bare `num` with no `:
Type` doesn't compile.

Why does `num: u64` fix this?
It spells out explicitly what `num` is, satisfying the requirement that
signatures never rely on inference. This is deliberate: it means a function's
signature alone tells you everything it accepts and returns, without needing to
read the body.
*/
