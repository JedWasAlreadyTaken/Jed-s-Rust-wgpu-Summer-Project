// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

use std::num::ParseIntError;
use std::error::Error;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing the signature and body of the
// `main` function.
fn main()->Result<(),Box<dyn Error>> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
    Ok(())
}

/*
What is the Problem?
Main did not have any return type so it is implicitly returns (), the unit type aka no meaningful value. But there is a ? on line 24 which means that there is a early return with the error if an Err() exists. For ? to be valid inside main, a return type is needed so the error can be held in something, and fn main, doesnt have a valid return type, as () cant hold anything, so ?'s early return path doesnt have a slot to fit into

Why is this a problem?
? on a Result<T,E> either unwraps to T and continues, or returns Err(e) immediately, but that only can compile when the function's return type is Result<_, E>, main returning () doesnt give anything for ? to send the Err to.

Why does ->Result<(), Box<dyn Error>> fix this?

Firstly dyn Error is a check for all errors, no matter which specific type, so the behaviour of error is captured, and Box<> is a pointer of unknown size found at runtime, so the pair of the 2 leads to a pointer of some error, which type isnt known until runtime, but it only contains Error, this is for the fail case of the Result and thus can be use for any generic error handling 

the () type is just passing in that the result passed successfully and no data is returned

*/