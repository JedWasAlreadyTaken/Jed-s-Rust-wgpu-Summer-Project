// TODO: Fix the compiler error on this function.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } 
    else if food == "potato"{
        "I guess I can eat that."
    }
    else {
        "No thanks!"
    }
}

fn main() {
    // You can optionally experiment here.
}

/*
What the problem was
This function is the one with the compiler error to fix, per the file's own
TODO comment — the arms of the `if`/`else if`/`else` chain didn't consistently
produce the `&str` the signature promises (likely one arm using `return` with a
semicolon, or a mismatched type in one branch).

Why is this a problem?
An `if`/`else if`/`else` chain used as an expression needs every arm's tail to
evaluate to the same type. If one arm doesn't line up — wrong type, or a
semicolon making it a `()`-producing statement — the whole chain fails to
type-check against the function's `-> &str` return type.

Why does making every arm a plain `&str` tail expression fix this?
With `"Yummy!"`, `"I guess I can eat that."`, and `"No thanks!"` as bare tail
expressions in each arm, the whole chain type-checks as a single `&str`
expression that becomes the function's return value — no `return` keyword
needed. This reinforces if1's point: `if`/`else` chains are expressions, and
it's a preview of why `&str` matters later — these are all `'static` literals
baked into the binary, so there's no lifetime puzzle yet even without an
explicit lifetime annotation on the signature.
*/
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
