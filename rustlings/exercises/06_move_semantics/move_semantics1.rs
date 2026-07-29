// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

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
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}

/*
What the problem was
`fill_vec` took its parameter as `vec: Vec<i32>` (not `mut`), and its body tried
to call `vec.push(88)` directly on it.

Why is this a problem?
`.push()` needs a mutable binding to call on, and a plain `vec: Vec<i32>`
parameter isn't mutable by default — same immutable-by-default rule as the
variables section, just applied to a function parameter instead of a `let`.

Why does `let mut vec = vec;` fix this?
It shadows the incoming parameter with a new, mutable binding of the same name,
so `.push(88)` is now allowed. This is the first real encounter with Rust's move
semantics too: `vec: Vec<i32>` takes the `Vec` by value, which moves ownership
of it into the function — `vec0` in the test is no longer valid after
`fill_vec(vec0)` runs, since the function is now the sole owner and is free to
mutate and return it. Compare with move_semantics2, where the caller needs the
original vector to still be usable afterward.
*/
