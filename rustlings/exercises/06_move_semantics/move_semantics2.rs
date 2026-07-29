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

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

/*
What the problem was
The test originally called `fill_vec(vec0)` directly, moving `vec0` into the
function, and then tried to use `vec0` again afterward in `assert_eq!(vec0,
[22, 44, 66])`.

Why is this a problem?
`fill_vec` takes ownership of its argument (`vec: Vec<i32>` by value), so once
`vec0` is moved into it, `vec0` is no longer valid in the test — using it again
after the move is a compile error, not just a runtime bug.

Why does `fill_vec(vec0.clone())` fix this?
`.clone()` passes a deep copy into `fill_vec` instead of moving `vec0` itself,
leaving the original `vec0` intact so the test can check both `vec0` and `vec1`
afterward. `.clone()` is the escape hatch for exactly this situation — needing
both the original and a moved/owned copy — but it has a real cost (a fresh heap
allocation and copy of every element), so it's a deliberate trade-off, not
something to reach for by default. Later exercises favour passing `&vec0` (a
borrow) instead when the callee doesn't actually need ownership — see
move_semantics5.
*/
