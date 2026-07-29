// This function returns how much ice cream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no ice cream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete the function body.
    if hour_of_day > 23{
         None
    }
    else if hour_of_day < 22{
         Some(5)
    }
    else {
         Some(0)
    }

}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        let ice_creams = maybe_ice_cream(5).unwrap();
        // .unwrap() pulls the value out of Some(value), or panics if it's None

        assert_eq!(ice_creams, 5); // Don't change this line.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}

/*
What the problem was
`maybe_ice_cream` had to return `Option<u16>` instead of a plain `u16`, because
there's a real "no answer" case (any hour past 23 is invalid) alongside two valid
numeric answers (5 scoops, or 0 after closing).

Why is this a problem?
If the function just returned `u16`, there'd be no honest way to signal "this
hour doesn't exist" — you'd have to pick some fake sentinel number (like 999) and
hope every caller remembers to check for it. Nothing forces that check.

Why does Option fix this?
`Option<u16>` has exactly two shapes: `Some(u16)` when there's a real scoop count,
`None` when the hour is invalid. The test then has to explicitly unwrap it:

let ice_creams = maybe_ice_cream(5).unwrap();

`.unwrap()` extracts the value inside `Some`, or panics immediately if it's
`None`. Because you can't accidentally treat an `Option<u16>` as a plain `u16` —
the compiler won't let you use it in arithmetic or comparisons without first
unwrapping or matching on it — the "did this actually have a value" check can't
be silently skipped the way a sentinel value could be.
*/
