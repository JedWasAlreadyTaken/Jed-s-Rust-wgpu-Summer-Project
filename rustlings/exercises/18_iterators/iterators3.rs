#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    match (a, b){
        (_, 0) =>Err(DivisionError::DivideByZero),
        (a,b) if  a==i64::MIN && b==-1 =>Err(DivisionError::IntegerOverflow),
        (a,b) if a % b == 0 => Ok(a/b),
        _ => Err(DivisionError:: NotDivisible),
    }
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()

}

// TODO: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> Vec<Result<i64, DivisionError>>{
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}

/*
What the problem was
divide had an empty body and needed to check three failure conditions before dividing: b
being 0 (DivideByZero), a == i64::MIN && b == -1 specifically (IntegerOverflow, since that
division mathematically overflows i64's range), and a not being evenly divisible by b
(NotDivisible). Along the way I made several mistakes: using = instead of == for the
remainder check, misspelling NotDivisable, and initially writing the overflow check as an
unconditional match arm whose body was a bare bool (a==i64::MIN && b==-1) rather than a
guarded arm returning Err(DivisionError::IntegerOverflow) - which both failed to type-check
against the function's Result<i64, DivisionError> return type and would have swallowed
every later arm as dead code, since it had no guard.

result_with_list and list_of_results were both missing return types and complete bodies.
Both needed to apply divide to every number via .map(), but collect the resulting iterator
of individual Result<i64, DivisionError> values differently depending on the desired
output shape.

How do the fixes work?
For divide, each match arm now returns an actual Result<i64, DivisionError>, and the
IntegerOverflow check uses a match guard (if a == i64::MIN && b == -1) placed before the
general division arm, so it's checked as a condition rather than replacing the arm's
return value, and doesn't swallow the arms below it.

For result_with_list, the desired output Ok([1, 11, 1426, 3]) is a single Result wrapping
all the successful values, so the return type is Result<Vec<i64>, DivisionError> and the
final expression numbers.into_iter().map(|n| divide(n, 27)).collect() relies on Result's
special collect() behavior: collecting an iterator of Result<T, E> into a Result<Vec<T>, E>
produces Ok(vec_of_values) if every item succeeded, or short-circuits to the first Err if
any item failed.

For list_of_results, the desired output [Ok(1), Ok(11), Ok(1426), Ok(3)] keeps every
individual Result separately rather than combining them, so the return type is
Vec<Result<i64, DivisionError>> instead - the same .map() chain, but collected into a plain
Vec of Results rather than a Result of Vec. In both functions, the final expression also
needed no trailing semicolon, since collect()'s result had to be the function's actual
return value rather than a discarded statement.
*/
