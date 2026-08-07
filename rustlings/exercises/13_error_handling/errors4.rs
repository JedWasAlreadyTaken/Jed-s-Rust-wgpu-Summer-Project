#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Read the tests below to clarify what should be returned.
       match value{
        ..0 => Err(CreationError::Negative),
        0 => Err(CreationError::Zero), 
       1.. => Ok(Self(value as u64)),
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
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}

/*
What is the problem?

`new` always returned `Ok`, so it never actually communicated failure back to the caller
even though the return type promised a `Result`. The tests call `new` with negative and
zero values and expect `Err(CreationError::Negative)` / `Err(CreationError::Zero)` back,
but the original body just wrapped every input in `Ok`, so those assertions would fail.

Why does the match fix it?

Matching on `value` lets each range map to the outcome the tests expect: any value less
than 0 (`..0`) is negative, exactly `0` is the zero case, and anything from 1 upward
(`1..`) is a valid positive value. Only that last arm returns `Ok`, the other two return
the matching `CreationError` variant, so the function's actual behaviour now matches what
its `Result<Self, CreationError>` signature claims it can do.
*/
