// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}

/*
What the problem was
The body originally returned `"blue"` directly, against a signature declaring
`-> String`.

Why is this a problem?
`"blue"` on its own is a `&str` (a string literal / slice), not a `String` — the
function's return type requires an owned, heap-allocated string, and the
compiler won't silently convert one to the other.

Why does `.to_string()` fix this?
It converts the `&str` literal into an owned `String` so the return type
matches. This is the string-specific version of move_semantics5's `&T` vs `T`
distinction — `&str` is a borrowed view (here, into the binary's static data),
while `String` owns its buffer on the heap and can be grown/mutated/returned
from a function without worrying about who else is borrowing it.
*/
