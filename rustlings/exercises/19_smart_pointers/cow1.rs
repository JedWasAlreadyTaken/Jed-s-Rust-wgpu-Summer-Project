// This exercise explores the `Cow` (Clone-On-Write) smart pointer. It can
// enclose and provide immutable access to borrowed data and clone the data
// lazily when mutation or ownership is required. The type is designed to work
// with general borrowed data via the `Borrow` trait.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Clone occurs because `input` needs to be mutated.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // No clone occurs because `input` doesn't need to be mutated.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Borrowed(_)));
    }

    #[test]
    fn owned_no_mutation() {
        // We can also pass `vec` without `&` so `Cow` owns it directly. In this
        // case, no mutation occurs (all numbers are already absolute) and thus
        // also no clone. But the result is still owned because it was never
        // borrowed or mutated.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        // Of course this is also the case if a mutation does occur (not all
        // numbers are absolute). In this case, the call to `to_mut()` in the
        // `abs_all` function returns a reference to the same data as before.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }
}

/*
What was the problem?

Three tests had a todo!() in place of the expected Cow variant after calling abs_all.
Cow<T> (Clone-On-Write) can hold either Cow::Borrowed (pointing at existing data without
copying it) or Cow::Owned (holding its own cloned copy). abs_all only clones into an owned
vector via input.to_mut() when it actually needs to mutate a value (i.e. when it finds a
negative number) - if every value is already non-negative, no mutation happens and no
clone is triggered by that function alone. Each test needed the correct variant filled in
based on whether abs_all would have mutated its input and whether the Cow started out
borrowed or already owned.

How do the fixes work?

reference_no_mutation starts from Cow::from(&vec) (borrowed) with all-non-negative values
([0, 1, 2]), so abs_all never calls to_mut() and input stays Cow::Borrowed(_) - no clone
was ever needed. owned_no_mutation starts from Cow::from(vec) (owned outright, no &), so
even though abs_all still doesn't mutate anything (values are already non-negative), the
Cow was never borrowed in the first place - it stays Cow::Owned(_) simply because it
started that way, independent of whether to_mut() gets called. owned_mutation also starts
from Cow::from(vec) (owned) but with a negative value present ([-1, 0, 1]), so to_mut() is
called - but since the Cow was already Cow::Owned, to_mut() just returns a mutable
reference to the existing owned data rather than cloning anything new, and it remains
Cow::Owned(_) either way. So the three tests each test a different one of the two
independent factors - whether the Cow started borrowed or owned, and whether abs_all
actually needed to mutate it - and the correct variant follows directly from those two
facts rather than always requiring an explicit clone to happen.
*/
