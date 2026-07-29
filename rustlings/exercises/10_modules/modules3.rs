// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.

// TODO: Bring `SystemTime` and `UNIX_EPOCH` from the `std::time` module into
// your scope. Bonus style points if you can do it with one line!
 use std::time::{SystemTime, UNIX_EPOCH} ;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/*
What the problem was
`main` already used `SystemTime` and `UNIX_EPOCH` directly (no `std::time::`
prefix), but nothing had brought those names into scope yet — the TODO asked
for the `use` statement to be written.

Why is this a problem?
Without a `use` bringing `SystemTime` and `UNIX_EPOCH` into scope, referring to
them by their bare names in `main` doesn't resolve to anything — you'd need
the fully qualified `std::time::SystemTime` etc. everywhere instead.

Why does `use std::time::{SystemTime, UNIX_EPOCH};` fix this?
It brings both items into scope with one statement, using the `{...}` brace
syntax to group multiple imports from the same path — the "bonus style points"
shortcut instead of two separate `use` lines. This is the same `use` mechanism
as modules2, just pulling from the standard library instead of a local module —
there's no special syntax for "internal" vs. "std" imports.
*/
