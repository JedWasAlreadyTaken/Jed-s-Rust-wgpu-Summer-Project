// TODO: Add some function with the name `call_me` without arguments or a return value.
fn call_me(){
    println!("Hello, I am a function!");
}

fn main() {
    call_me(); // Don't change this line
}

/*
What the problem was
`call_me` didn't exist yet — `main` already called it, but there was no matching
function definition anywhere in the file.

Why is this a problem?
`main` calls `call_me()` unconditionally, so without a `call_me` function
defined, the compiler has nothing to resolve that call to and rejects the file.

Why does adding `fn call_me() { ... }` fix this?
It defines a function matching exactly what `main` expects: no arguments, no
return value (the implicit `()` unit type). This is the baseline function shape
— `fn name(params) -> ReturnType { body }` — with both the parameter list and
`-> ReturnType` optional when there's nothing there. Everything from functions2
onward is just filling in one of those optional parts.
*/
