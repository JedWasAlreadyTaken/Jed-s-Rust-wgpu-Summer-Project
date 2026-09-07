// TODO: Fix the compiler error by moving the whole definition of this macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
fn main() {
    my_macro!();
}

/*
What was the problem?

Unlike regular functions, which can be called before their definition appears later in the
same file (Rust resolves all item definitions in a scope before checking the bodies),
macro_rules! macros are different: they're only available to code that comes textually
after their definition, in the order the compiler reads the file top to bottom. The macro
definition was originally placed after main, which calls my_macro!() - so at the point the
compiler reached that call, my_macro hadn't been defined yet as far as macro expansion was
concerned, and the call failed to resolve to any known macro.

How does moving the definition above main fix this?

Moving the whole macro_rules! my_macro { ... } block so it appears before fn main() means
the compiler has already registered my_macro's definition by the time it reaches the
my_macro!() call inside main's body. Since macro resolution depends on textual order
rather than being resolved after a full first pass like function definitions are, the
call now successfully matches the macro's () => { ... } rule and expands correctly.
*/
