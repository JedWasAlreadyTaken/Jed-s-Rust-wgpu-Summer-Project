// Type casting in Rust is done via the usage of the `as` operator.
// Note that the `as` operator is not only used when type casting. It also helps
// with renaming imports.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Make a conversion before dividing.
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}

/*
What was the problem?

The original code had `total / values.len();` as its own statement (note the trailing
semicolon), which discards the division's result instead of returning it. The function
then fell through to `total as f64` as its final expression, a no-op cast since `total`
is already an `f64`, so `average` returned the raw sum instead of the average.

There was also a type mismatch hiding underneath: `values.len()` returns a `usize`, and
Rust doesn't allow dividing an `f64` by a `usize` directly - the two operands of `/` must
be the same type.

How does the fix address this?

`total / values.len() as f64` casts `values.len()` to `f64` with the `as` operator so the
division type-checks, and removing the semicolon makes the division the function's
returned expression instead of a discarded statement. The leftover `total as f64` line is
also gone, since it was never doing anything useful.
*/
