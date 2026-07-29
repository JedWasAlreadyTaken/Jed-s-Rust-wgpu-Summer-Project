// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

/*
What the problem was
`main` called `is_a_color_word(word)` directly, passing the `String` by value
(or otherwise not as a `&str`), against a function that takes `attempt: &str`.

Why is this a problem?
`is_a_color_word` can't be changed (the TODO says fix `main` instead), and it
expects `&str`, not an owned `String` — passing `word` itself doesn't match
the parameter type, and it would also move `word` out of `main`.

Why does `is_a_color_word(&word)` fix this?
`&word` borrows the `String` instead of moving it, and deref coercion
automatically turns that `&String` into `&str` — no need for `&word[..]` or
`word.as_str()`. This is why so many Rust APIs take `&str` as a parameter type
instead of `&String`: a function taking `&str` can accept both string literals
and borrowed `String`s, making it strictly more flexible. `&String` in a
signature is almost always a sign the function could be loosened to `&str`.
*/
