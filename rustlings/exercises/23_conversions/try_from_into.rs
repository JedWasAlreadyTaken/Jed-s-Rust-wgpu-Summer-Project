// `TryFrom` is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as `From`. The main
// difference is that this should return a `Result` type instead of the target
// type itself. You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html

#![allow(clippy::useless_vec)]
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for the `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// TODO: Tuple implementation.
// Correct RGB color values must be integers in the 0..=255 range.
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (r, g, b) = tuple;
        let red = r.try_into().map_err(|_| IntoColorError::IntConversion)?;
        let green = g.try_into().map_err(|_| IntoColorError::IntConversion)?;
        let blue = b.try_into().map_err(|_| IntoColorError::IntConversion)?;
        Ok(Color { red, green, blue })
    }
}

// TODO: Array implementation.
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let [r, g, b] = arr;
        let red: u8 = r.try_into().map_err(|_| IntoColorError::IntConversion)?;
        let green: u8 = g.try_into().map_err(|_| IntoColorError::IntConversion)?;
        let blue: u8 = b.try_into().map_err(|_| IntoColorError::IntConversion)?;
        Ok(Self { red, green, blue })
    }
}

// TODO: Slice implementation.
// This implementation needs to check the slice length.
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        let red = slice[0].try_into().map_err(|_| IntoColorError::IntConversion)?;
        let green = slice[1].try_into().map_err(|_| IntoColorError::IntConversion)?;
        let blue = slice[2].try_into().map_err(|_| IntoColorError::IntConversion)?;
        Ok(Color { red, green, blue })
    }
}

fn main() {
    // Using the `try_from` function.
    let c1 = Color::try_from((183, 65, 14));
    println!("{c1:?}");

    // Since `TryFrom` is implemented for `Color`, we can use `TryInto`.
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{c2:?}");

    let v = vec![183, 65, 14];
    // With slice we should use the `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{c3:?}");
    // or put the slice within round brackets and use `try_into`.
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{c4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use IntoColorError::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(Color::try_from((256, 1000, 10000)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(Color::try_from((-1, -10, -256)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(Color::try_from((-1, 255, 255)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }

    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }
}

/*
What was the problem?

The task was to implement `TryFrom` for three different input shapes - a tuple, a fixed
array, and a slice - each converting `i16` components into a `Color`'s `u8` fields, where
any value outside `0..=255` should produce `IntoColorError::IntConversion`, and a slice of
the wrong length should produce `IntoColorError::BadLen`.

How does the implementation address this?

For the tuple and array versions, the input is destructured into its three components
(`let (r, g, b) = tuple` / `let [r, g, b] = arr`) since both are always exactly 3 elements
- no length check is needed. Each component is then converted from `i16` to `u8` with
`.try_into()`, which fails whenever the value doesn't fit in a `u8` (negative, or greater
than 255). `.map_err(|_| IntoColorError::IntConversion)?` turns that failure into the
exercise's own error type and immediately returns it via `?`, short-circuiting the
function before a `Color` is ever built.

The slice version can't rely on its length at compile time, so it checks
`slice.len() != 3` up front and returns `Err(IntoColorError::BadLen)` if it doesn't match
- this is what makes `test_slice_excess_length` and `test_slice_insufficient_length` pass.
After that check, the same per-element `try_into()` + `map_err` pattern converts
`slice[0]`, `slice[1]`, and `slice[2]` into the `red`, `green`, and `blue` fields.

Because `TryFrom` is implemented for all three types, `TryInto` is available for free,
which is why `main` can use both `Color::try_from(...)` and `.try_into()` interchangeably.
*/
