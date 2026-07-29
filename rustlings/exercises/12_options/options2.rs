fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
       if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

/*
What the problem was
`simple_option` needed an `if let Some(word) = optional_target` to pull `word` out
of the `Option`, and `layered_option` needed a `while let Some(Some(integer)) =
optional_integers.pop()` to keep popping until the vector ran dry.

Why is this a problem?
`optional_target` is `Option<&str>`, not `&str` — you can't compare it directly to
`target` in `assert_eq!` without unwrapping first, similar to options1. The second
test is layered because `optional_integers` is a `Vec<Option<i8>>` — popping it
gives `Option<Option<i8>>` (outer `Option` from `.pop()` itself possibly being
empty, inner `Option` because the vector's elements are `Option<i8>`).

Why does if-let / while-let fix this?
`if let Some(word) = optional_target { ... }` only runs the block when the value
matches `Some(...)`, binding the inner value to `word` — a shorthand for a `match`
that only cares about one pattern and ignores the rest.

`while let Some(Some(integer)) = optional_integers.pop() { ... }` loops for as
long as `.pop()` keeps returning `Some(Some(_))` — i.e. the vector isn't empty
*and* the popped element itself isn't `None`. The loop stops the moment either
layer comes back empty (the vector's real `None` sentinel at the front, pushed
before the loop that fills it), which is exactly why the pushed-first `None`
works as a natural stopping point.
*/
