// Characters (`char`)

fn main() {
    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: Analogous to the example before, declare a variable called `your_character`
    // below with your favorite character.
    // Try a letter, try a digit (in single quotes), try a special character, try a character
    // from a different language than your own, try an emoji 😉
    // let your_character = '';
    let your_character = '😉';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

/*
What the problem was
`your_character` was left as an empty literal, `let your_character = '';`, which
isn't a valid `char`.

Why is this a problem?
A `char` literal must hold exactly one value — an empty `''` has nothing in it,
so it can't be interpreted as a single character and fails to compile.

Why does `let your_character = '😉';` fix this?
Any single Unicode scalar value works here, including an emoji — a `char` in
Rust always represents one full Unicode scalar value (4 bytes), not one byte,
which is why an emoji fits in a single `char` even though it wouldn't fit in a
byte. This is a real difference from languages like C, where `char` is just a
byte, and it's also why Rust's `String`/`&str` are UTF-8 byte sequences rather
than arrays of `char` — indexing a string by byte position could land in the
middle of a multi-byte character, which is part of why Rust doesn't let you
index strings with `s[i]` directly (see the strings section later).
*/
