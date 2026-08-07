// This exercise is an altered version of the `errors4` exercise. It uses some
// concepts that we won't get to until later in the course, like `Box` and the
// `From` trait. It's not important to understand them in detail right now, but
// you can read ahead if you like. For now, think of the `Box<dyn ???>` type as
// an "I want anything that does ???" type.
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, the `Box` is declared as of type `Box<dyn Trait>` where
// `Trait` is the trait the compiler looks for on any value used in that
// context. For this exercise, that context is the potential errors which
// can be returned in a `Result`.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: Add the correct return type `Result<(), Box<dyn ???>>`. What can we
// use to describe both errors? Is there a trait which both errors implement?
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

/*
What is the problem?

There are two `?`s in `main`, and each one can fail with a *different* concrete error
type: `pretend_user_input.parse()?` can fail with `ParseIntError`, and
`PositiveNonzeroInteger::new(x)?` can fail with `CreationError`. `main`'s original
signature had a syntax error (`Result<(), <Box<dyn Error>>`, a stray `<`), and even once
that's fixed, a plain `Result<(), CreationError>` or `Result<(), ParseIntError>` wouldn't
work, because a single `Result` can only name one concrete error type in its `E` slot, and
here two different ones need to fit through it.

Why does `Result<(), Box<dyn Error>>` fix it?

`dyn Error` is a trait object: instead of naming one specific error type, it says "any
type that implements the `Error` trait". `Box<dyn Error>` is a pointer to a value of some
unknown-at-compile-time type on the heap, as long as that type implements `Error`. Both
`ParseIntError` and `CreationError` implement `Error` (the standard library provides it for
`ParseIntError`, and this file implements it for `CreationError` at line 34), so `?` can
convert either one into a `Box<dyn Error>` automatically and return it through the same
slot. That's the difference from errors3: there `Box<dyn Error>` was mostly about giving
`?` somewhere to put its error at all; here it's actually needed because of polymorphism
over two distinct error types sharing one return path.
*/