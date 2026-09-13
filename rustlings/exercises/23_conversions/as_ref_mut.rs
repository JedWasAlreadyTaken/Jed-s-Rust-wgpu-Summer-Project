// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// Obtain the number of bytes (not characters) in the given argument
// (`.len()` returns the number of bytes in a string).
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using `as_mut()`.
// TODO: Add the appropriate trait bound.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    let x = arg.as_mut();
    *x *= *x;
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}

/*
What was the problem?

Three functions needed the right `AsRef`/`AsMut` trait bounds so they could work generically
over multiple types that can cheaply produce a reference of the needed kind, rather than
being locked to one concrete type like `&str` or `String`.

How does the implementation address this?

`byte_counter<T: AsRef<str>>` and `char_counter<T: AsRef<str>>` accept any `T` that can be
borrowed as a `&str` (both `&str` and `String` implement `AsRef<str>`), then call
`arg.as_ref()` to get that `&str` view before measuring it with `.len()` (byte count) or
`.chars().count()` (character count) - the two differ on non-ASCII input like "Café" because
some characters take more than one byte in UTF-8 but still count as a single `char`.

`num_sq<T: AsMut<u32>>(arg: &mut T)` accepts a mutable reference to any `T` that can produce
a `&mut u32` (a `Box<u32>` implements `AsMut<u32>` by exposing the boxed value). Calling
`arg.as_mut()` gets that `&mut u32`, and `*x *= *x` squares the value in place through the
reference, which is why the test can see the change reflected in the original `Box<u32>`
after the call.
*/
