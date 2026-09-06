macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!();
}

/*
What was the problem?

my_macro is defined with macro_rules!, and its one and only rule matches an empty
invocation - () => { ... } - taking no arguments. Calling a macro in Rust requires the !
right after its name, marking it as a macro invocation rather than a function call, and
the call needs to match one of the macro's defined patterns. The original call was missing
that ! entirely (or had some other mismatch with the macro's required call syntax), so it
either wasn't recognised as a macro invocation at all, or didn't match the macro's only
defined pattern.

How does my_macro!(); fix this?

Writing my_macro!() calls the macro with the ! that identifies it as a macro invocation,
and the empty parentheses match the macro's only rule, () => { ... }, which expands to
println!("Check out my macro!");. Since the call's shape now matches exactly what the
macro's rule expects, the macro expands correctly and the program compiles and runs.
*/
