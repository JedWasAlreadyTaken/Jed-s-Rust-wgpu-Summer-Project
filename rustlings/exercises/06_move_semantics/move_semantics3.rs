// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

/*
What the problem was
Same underlying issue as move_semantics1 — `vec.push(88)` needs a mutable
binding — but this time the TODO forbids adding any new line (so the
`let mut vec = vec;` shadowing trick from move_semantics1 isn't allowed here).

Why is this a problem?
Without either a `mut` parameter or a shadowing `let mut`, there's no mutable
binding for `.push()` to call on, and the constraint of "no new line" rules out
the fix used last time.

Why does `fn fill_vec(mut vec: Vec<i32>)` fix this?
Marking the parameter itself `mut` in the signature achieves the same thing as
move_semantics1's extra line, just without needing it. `mut` on a parameter is
exactly the same mechanism as `mut` on a `let` — it's about whether *this
binding* can be mutated, not about the value's type. Since the function already
owns `vec` (moved in by value), making the binding `mut` is enough to call
`.push()` on it.
*/
