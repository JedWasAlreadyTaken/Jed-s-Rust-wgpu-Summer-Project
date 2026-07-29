// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn calculate_price_of_apples(quantity:u32)->u32{
    if quantity >40
    {quantity}
    else{
        quantity*2
    }
}

fn main() {
    // You can optionally experiment here.
}

/*
What the problem was
The file started with only a signature comment (`fn calculate_price_of_apples(???)
-> ??? { ??? }`) and no real implementation — both the signature and the pricing
logic had to be written from scratch.

Why is this a problem?
Without a real function, there's nothing for the tests (`calculate_price_of_apples(35)`,
etc.) to call. And even once a signature exists, getting the parameter/return
types or the branch logic wrong (e.g. an `if`/`else` that doesn't type-check, or
using the wrong threshold/multiplier) would fail the assertions or fail to
compile.

Why does this implementation fix it?
`fn calculate_price_of_apples(quantity: u32) -> u32` picks types that match how
the tests call it and what they expect back. The body follows the same rule as
if1/if3: no semicolons on the tail expressions, so the `if`/`else` itself is a
`u32` expression returned directly. When `quantity > 40`, the price per apple
has dropped to 1, so the total is just `quantity`; otherwise it's `quantity *
2`. This pulls together variables, functions, and if/else from the last three
sections into one exercise.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
