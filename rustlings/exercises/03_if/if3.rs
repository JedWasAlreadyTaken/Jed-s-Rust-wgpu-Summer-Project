fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
}

/*
What the problem was
The `let identifier = if ... else ...` statement had a compiler error — one of
its arms likely produced a different type than the others (mixing, say, a
string in one branch with an integer in another).

Why is this a problem?
When an `if`/`else if`/`else` chain is used to produce a value for a `let`
binding, every arm's tail expression must evaluate to the same type. If even
one arm disagreed, the compiler couldn't decide what type `identifier` should
be, and the whole `let` would fail to compile.

Why does making every arm return a plain integer (`1`, `2`, `3`, `0`) fix this?
With every branch producing the same type, the compiler can infer a single
concrete type for `identifier` and accept the binding — the branches of a
value-producing `if` chain all have to agree, and here they finally do.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
