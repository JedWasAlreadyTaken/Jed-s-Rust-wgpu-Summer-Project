fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
   let number = 3;
    println!("Number plus two is: {}", number + 2);
}

/*
What the problem was
The line was originally something like `number = 3;` — a plain reassignment of
the existing `number` (a `&str` holding `"T-H-R-E-E"`) to an integer, without
`let`, without renaming the variable.

Why is this a problem?
A plain reassignment can't change a binding's type, and it can't happen at all
without `mut`. Even with `mut`, you still can't reassign a `&str` binding to an
`i32` value — a variable's type is fixed once declared.

Why does `let number = 3;` fix this?
Adding `let` turns it into shadowing rather than reassignment: a brand new
binding named `number`, of a different type, that hides the previous one for
the rest of this scope — the original string `number` isn't mutated, it's
just no longer reachable by that name. Shadowing is exactly for this: reusing a
good variable name across a chain of transformations (parse a string, trim it,
convert it) without needing `number_str`, `number_trimmed`, `number_int`, etc.
*/
