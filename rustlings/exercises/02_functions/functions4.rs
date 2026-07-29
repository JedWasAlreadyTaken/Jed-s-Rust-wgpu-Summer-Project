// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
fn sale_price(price: i64) -> i64{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

/*
What the problem was
`sale_price`'s signature was originally missing a return type (`fn
sale_price(price: i64) { ... }`), even though its body clearly produces a value
via the `if`/`else` tail expressions.

Why is this a problem?
Same rule as functions2 — signatures are never inferred. A function whose body
returns something but whose signature omits `->` entirely (implying `-> ()`) is
a type mismatch: the compiler won't guess that you meant to return `i64` just
because the body looks like it does.

Why does adding `-> i64` fix this?
It makes the declared return type match what the body actually produces,
which the compiler requires to be stated explicitly rather than inferred.
*/
