fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        a
    } else {
        b
    }
}

/*
What the problem was
The original arms were `a;` and `b;` (with trailing semicolons), inside an
`if`/`else` that's meant to be the function's return value.

Why is this a problem?
Same lesson as functions5: `a;` and `b;` are statements that evaluate to `()`,
so neither arm actually produces an `i32`, and the function's declared `-> i32`
return type isn't satisfied.

Why does removing the semicolons fix this?
Dropping them turns `a` and `b` into tail expressions. Since both arms now
evaluate to `i32`, the whole `if`/`else` becomes a valid `i32` expression in its
own right, which becomes the function's return value with no `return` keyword
needed. In Rust, `if`/`else` is an expression, not just a control-flow statement
— but only when every branch's tail evaluates to the same type. This is what
makes patterns like `let x = if cond { 1 } else { 2 };` possible.
*/

fn main() {
    // You can optionally experiment here.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
