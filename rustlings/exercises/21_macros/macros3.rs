// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;
}

fn main() {
    use crate::macros::my_macro;
    my_macro!();
}

/*
What was the problem?

my_macro was defined inside mod macros, a module with its own separate namespace. By
default, items inside a module (including macros) are private to that module - they
aren't automatically visible to code outside it, like main. main tried to call my_macro!()
without importing it from the macros module first, so the compiler couldn't resolve the
name at all - as far as main was concerned, no such macro existed in scope.

How does pub(crate) use my_macro; and use crate::macros::my_macro; fix this?

macro_rules! macros don't get a pub keyword directly on their definition the way
functions or structs do - instead, pub(crate) use my_macro; inside the module re-exports
the macro under a regular use statement, making it visible elsewhere in the crate (but not
outside it, since pub(crate) restricts visibility to code within this same crate). Then,
use crate::macros::my_macro; inside main brings that now-visible macro into scope there,
using its full path from the crate root (crate::) through the macros module. With both
pieces in place, my_macro!() inside main resolves correctly, since the macro has been
explicitly exported from its module and explicitly imported into the scope that calls it.
*/
