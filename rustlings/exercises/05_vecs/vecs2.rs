fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Multiply each element in the `input` slice by 2 and push it to
        // the `output` vector.
        output.push(element*2)
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // An example of collecting a vector after mapping.
    // We map each element of the `input` slice to its value plus 1.
    // If the input is `[1, 2, 3]`, the output is `[2, 3, 4]`.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: Here, we also want to multiply each element in the `input` slice
    // by 2, but with iterator mapping instead of manually pushing into an empty
    // vector.
    // See the example in the function `vec_map_example` above.
    input
        .iter()
        .map(|element| {
            // ???
            element * 2
        })
        .collect()
}

fn main() {
    // You can optionally experiment here.
}

/*
What the problem was
Both `vec_loop` and `vec_map` were missing the actual doubling logic — `vec_loop`
had a TODO where the push value should be, and `vec_map`'s closure had a `// ???`
placeholder instead of the multiplication.

Why is this a problem?
The tests expect each input element doubled (`[2, 4, 6, 8, 10]` -> `[4, 8, 12,
16, 20]`); without the multiplication actually written, neither function
produces that output.

Why does `element * 2` in both places fix this?
In `vec_loop`, `output.push(element * 2)` inside the `for` loop builds the new
`Vec` by hand, one push at a time. In `vec_map`, the same `element * 2` inside
`.iter().map(...)` does the same transformation without a mutable accumulator or
manual `push` — modeled directly on `vec_map_example`'s `element + 1`. Note
iterating `input` (a `&[i32]`) with `for element in input` or `.iter()` yields
`&i32` references, not owned `i32`s — that's why `element * 2` works via
auto-deref without needing `*element * 2` written out. The `.map().collect()`
style is the idiomatic Rust way to transform a collection: it reads as "what"
(map each element) rather than "how" (loop, allocate, push).
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
