// TODO: We sometimes encourage you to keep trying things on a given exercise
// even after you already figured it out. If you got everything working and feel
// ready for the next exercise, enter `n` in the terminal.
//
// The exercise file will be reloaded when you change one of the lines below!
// Try adding a new `println!` and check the updated output in the terminal.

fn main() {
    println!(r#"       Welcome to...                      "#);
    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    println!();
    println!("This exercise compiles successfully. The remaining exercises contain a compiler");
    println!("or logic error. The central concept behind Rustlings is to fix these errors and");
    println!("solve the exercises. Good luck!");
    println!();
    println!("The file of this exercise is `exercises/00_intro/intro1.rs`. Have a look!");
    println!("The current exercise path will be always shown under the progress bar.");
    println!("You can click on the path to open the exercise file in your editor.");
}

/*
What the problem was
Nothing — this file compiles as-is. It exists purely to confirm the toolchain
and the rustlings runner both work before any real exercise starts.

Why is this a problem?
It isn't. But it's worth reading anyway, since it's the first real Rust source
you'll see: `fn main()` as the entry point, and `println!` as a macro (the `!` is
part of the name) rather than an ordinary function.

Why does this "fix" work?
`println!` being a macro is what lets it type-check its format string against
its arguments at compile time and accept a variable number of arguments — a
plain function couldn't do either. The banner text is wrapped in `r#"..."#`
(a raw string literal), which turns off escape-sequence processing, so the
backslashes in the ASCII art print literally instead of being interpreted.
*/
