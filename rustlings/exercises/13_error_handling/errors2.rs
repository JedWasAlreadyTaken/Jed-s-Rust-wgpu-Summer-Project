// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the items. Since
// the player typed in the quantity, we get it as a string. They might have
// typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all. What we want
// to do is: If we call the `total_cost` function on a string that is not a
// number, that function will return a `ParseIntError`. In that case, we want to
// immediately return that error from our function and not try to multiply and
// add.
//
// There are at least two ways to implement this that are both correct. But one
// is a lot shorter!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Handle the error case as described above.
    let qty = item_quantity.parse::<i32>();
    
    match qty{
    Ok(n) =>Ok(n * cost_per_item + processing_fee),
    Err(e) => Err(e),
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}

/*
What the problem was
The function needs to unrap the result from .parse(), so we need to do a maths equation on the success case, as it is as i32 type, in the failure case we need to return the error and not unrap the numeric path 

    // TODO: Handle the error case as described above.
    let qty = item_quantity.parse::<i32>();
    
    Ok(qty * cost_per_item + processing_fee)

Why is this a problem?
.parse::<i32>() returns Result<i32, ParseIntError>, not i32. qty is a wrapper of Result not the destructured number. so We cant to qty * cost_per_item because the * operation doesnt exist between a Result<i32,_> and i32 therefore destructuring is needed to get the i32 out os Result, and a safe way to do that is a match case for both cases 

Why does match qty{} solve this?

match lets us destructure the Result, in the Ok(n) arm the i32 is now bound to n, so now the multipllication can now occur, and in the Err(e) arm i can rewrap e in Err() so i can satisfy the return type conditions, so i can return before any math multiplications 

*/
