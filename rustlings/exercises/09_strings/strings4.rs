// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

/*
What the problem was
Every call in `main` originally used the placeholder `placeholder(...)`, which
doesn't distinguish between values that are `&str` and values that are `String`.

Why is this a problem?
`placeholder` takes no meaningful argument type, so it can't actually verify
anything — the exercise's real goal is routing each value to the function
matching its actual type: `string_slice` (takes `&str`) or `string` (takes
owned `String`). Calling the wrong one is a type mismatch.

Why does routing each call to `string_slice` or `string` fix this?
It matches each expression to what it actually produces: literals like `"blue"`
are `&str`; `.to_string()`, `.to_owned()`, `.into()`, `format!(...)`, and
`.replace(...)`/`.to_lowercase()` all produce an owned `String`; and
`&String::from("abc")[0..1]` / `"...".trim()` produce `&str` slices. This is a
drill in recognizing, from a method or literal alone, whether you're holding a
borrowed view or an owned allocation. The byte-indexing warning on
`&String::from("abc")[0..1]` also flags a real footgun: string indexing is by
byte offset, and slicing into the middle of a multi-byte UTF-8 character panics
at runtime, which is why Rust doesn't support `s[i]` for single characters at
all (only ranges), forcing you to think in bytes when you slice.
*/
