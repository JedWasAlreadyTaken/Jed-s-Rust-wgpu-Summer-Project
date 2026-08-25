struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Returning a `Result` would be better here. But we want to learn
            // how to test functions that can panic.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: This test should check if the rectangle has the size that we
        // pass to its constructor.
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Check width
        assert_eq!(rect.height, 20); // Check height
    }

    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative width.
    #[should_panic]
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
        
    }

    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative height.
    #[should_panic]
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);

    }
}

/*
What was the problem?

`correct_width_and_height` originally tried to call `rect(10, 20)` as if `rect` were a
function, and wrapped it in `todo!(...)`, a placeholder macro that always panics when
reached rather than something to fill an expression into. But `rect` is a `Rectangle`
instance (a variable, not a function), and `todo!()` was meant to be deleted and replaced
entirely, not built around.

`negative_width` and `negative_height` had a different kind of problem: they used
`assert_eq!` to compare field values after calling `Rectangle::new` with invalid input. But
`Rectangle::new` calls `panic!` on non-positive dimensions before it ever constructs and
returns a `Rectangle`, so there was no instance to check fields on, and by default a test
that panics is reported as a *failing* test, not a passing one.

How did the fixes address this?

For `correct_width_and_height`, `rect.width` and `rect.height` access the actual fields of
the `Rectangle` instance created on line 31, which is what the test needed to compare
against the constructor's arguments — no function call or `todo!()` involved.

For the panic tests, adding `#[should_panic]` above `#[test]` tells the test runner that a
panic inside this specific test is the expected, passing outcome, rather than a crash.
Since `panic!` stops execution immediately and never returns a value, the `assert_eq!`
calls that used to check `_rect`'s fields were removed — there was nothing left to check
after the panic, and the test's only job is to trigger it.
*/
