fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);
        let second = numbers.1;
        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

/*
What the problem was
`second` needed to be the tuple's 2nd element (value `2`), but the TODO comment
left only a placeholder (`let second = ???;`) instead of real code.

Why is this a problem?
`assert_eq!(second, 2, ...)` needs `second` to be bound to an actual value
pulled out of `numbers`, and `???` isn't valid Rust.

Why does `numbers.1` fix this?
Tuple fields can be accessed directly by 0-indexed position with dot syntax —
`.1` is the second element. This is an alternative to destructuring the whole
tuple with `let (a, b, c) = numbers;` like in primitive_types5; it's handy when
you only need one field and don't want to name the others, but it's less
self-documenting — `numbers.1` doesn't tell a reader what that field means the
way a destructured name like `second` does. Structs (section 07) solve this
properly with named fields.
*/
