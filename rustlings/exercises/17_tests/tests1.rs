// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::*;
    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(!is_even(21));
        assert!(is_even(20));
    }
}

/*
What was the problem?

The test called `some_func(is_even(21))` and `some_func(is_even(20))`, but `some_func`
was never defined anywhere in the file — it doesn't exist, so this wouldn't compile.
Even ignoring that, the values were wrong for what `assert!` needs: `assert!(expr)` only
passes when `expr` is `true`, but `is_even(21)` evaluates to `false` (21 is odd), so
passing it straight into `assert!` would fail the assertion even with the wrapper removed.

How does removing `some_func` and adding `!` fix this?

`assert!` doesn't need a wrapper function at all — it just takes a boolean expression
directly and panics if that expression is `false`. Removing `some_func` leaves
`is_even(21)` and `is_even(20)` as plain boolean expressions, which is all `assert!`
needs. `is_even(20)` is already `true` (20 is even), so `assert!(is_even(20))` passes
as-is. `is_even(21)` is `false` (21 is odd), so proving "21 is not even" means asserting
the negation: `assert!(!is_even(21))`, which is `true`.
*/
