// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises, the code will fail to compile when there are Clippy
// warnings. Check Clippy's suggestions from the output to solve the exercise.

fn main() {
    // TODO: Fix the Clippy lint in this line.
    const PI: f32 = std::f32::consts::PI;
    let radius: f32 = 5.0;

    let area = PI * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}

/*
What was the problem?

The original code declared let pi = 3.14, a hand-typed approximation of pi with only two
decimal places of precision. Clippy has a lint (approx_constant) that specifically
recognises common mathematical constants being manually retyped like this, and flags it
as a mistake, since a hardcoded approximation is both less accurate and easy to mistype
compared to using the constant the standard library already provides.

How does const PI: f32 = std::f32::consts::PI; fix this?

std::f32::consts::PI is the standard library's own predefined constant for pi, already
correct to the full precision an f32 can represent - so using it instead of a manually
typed decimal removes both the imprecision and the risk of a typo. Declaring it as
const PI: f32 = ... instead of let pi = ... also marks it as a genuine compile-time
constant rather than a regular variable, which better reflects that pi's value never
changes - matching the convention of using SCREAMING_CASE names for constants in Rust.
*/
