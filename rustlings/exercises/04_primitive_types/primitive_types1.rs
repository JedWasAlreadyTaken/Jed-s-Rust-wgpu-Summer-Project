// Booleans (`bool`)

fn main() {
    let is_evening = true;
    let is_morning = false;
    if is_morning {
        println!("Good morning!");
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    // let …
    if is_evening {
        println!("Good evening!");
    }
}

/*
What the problem was
`is_evening` didn't exist yet — the `if is_evening { ... }` block below already
referenced it, but nothing declared it.

Why is this a problem?
`if` needs a real `bool` to test, and `is_evening` had no binding at all, so the
compiler has no idea what to check.

Why does `let is_evening = !is_morning;` fix this?
`!` is logical negation, so this declares `is_evening` as the opposite of
`is_morning`. It's worth noting `bool` is its own type here, not an alias for
an integer — there's no implicit `0`/`1` <-> `bool` conversion like in C, so
`if` conditions must be a genuine `bool`, not a "truthy" value like `0` or `""`.
That's also why a typo like `if is_morning = false` (assignment instead of
comparison) would be a compile error rather than a silent logic bug.
*/
