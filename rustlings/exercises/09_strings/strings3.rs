fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    input.to_string()+ " world!"

}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}

/*
What the problem was
All three functions had TODOs with no implementation: `trim_me` needed to
strip whitespace, `compose_me` needed to append `" world!"`, and `replace_me`
needed to swap `"cars"` for `"balloons"`.

Why is this a problem?
Without real bodies, none of the three functions produce the strings the tests
expect (`trim_me("Hello!     ")` should give `"Hello!"`, etc.) — there's no
default behavior to fall back on.

Why do `.trim()`, `.to_string() + " world!"`, and `.replace(...)` fix this?
`.trim()` returns a `&str` slice with leading/trailing whitespace excluded — no
new allocation, it just narrows the borrowed range, which is why `trim_me` can
return `&str` matching its signature. `.to_string() + " world!"` allocates a
new owned `String` by concatenating. `.replace(from, to)` builds a new `String`
with every occurrence of `from` swapped for `to`. Notice `trim_me` returns
`&str` (borrowing from its input) while `compose_me` and `replace_me` return
`String` (owned) — the return type tells you whether a method allocates new
data or just gives you a view into what's already there.
*/
