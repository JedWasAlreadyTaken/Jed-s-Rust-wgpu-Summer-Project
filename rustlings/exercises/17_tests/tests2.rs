// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        assert_eq!(power_of_2(1),2);
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(3), 8);
        assert_eq!(power_of_2(4), 16);
    }
}

/*
What was the problem?

Each `assert_eq!` call originally had only one argument, e.g. `assert_eq!(power_of_2(20))`.
`assert_eq!` always needs two arguments — a left and a right value to compare — and panics
if they aren't equal, unlike `assert!`, which just takes a single boolean. One line also
passed `-1` into `power_of_2`, but the function takes `n: u8`, an unsigned type that can't
hold negative numbers, so that was a separate type error on top of the missing argument.

How does adding the expected values fix this?

Adding a second argument to each `assert_eq!` gives it something to compare the function's
result against, satisfying the macro's required two-argument shape. The values themselves
also needed to be correct: `power_of_2(n)` computes `1 << n`, which is 2 raised to the
power of `n`, not `n` squared. The first attempt at expected values mixed those two up
(e.g. treating `power_of_2(3)` as `3*3=9` instead of `2^3=8`), so the assertions still
failed even once the argument count was fixed, until the expected values were recalculated
using `2^n`.
*/
