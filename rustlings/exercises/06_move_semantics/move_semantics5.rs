#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

/*
What the problem was
The call site originally passed `data` (not `&data`) to `get_char`, and/or
passed `&data` to `string_uppercase` — the opposite of what each function
actually needs, per the comments marking one "Shouldn't take ownership" and the
other "Should take ownership".

Why is this a problem?
`get_char` takes `&String` — passing `data` by value there would move it,
leaving nothing for the later `string_uppercase(data)` call to use. Conversely,
`string_uppercase` takes `data: String` by value (it needs ownership to
reassign `data = data.to_uppercase()`), so passing only `&data` there wouldn't
satisfy that signature.

Why does `get_char(&data); string_uppercase(data);` fix this?
`get_char` only needs to read the string, so passing `&data` borrows it — `data`
stays valid afterward. `string_uppercase` genuinely needs ownership (to
consume and transform it), so passing `data` by value there moves it in for
good, which is fine since nothing in `main` needs `data` afterward. This is the
payoff of the whole move-semantics section: choosing `&T` vs `T` in a signature
is a real design decision. `&String` says "I just need to look, you keep
ownership"; `String` says "hand it over, I'm taking responsibility for it." A
function's signature alone should tell you which one you're dealing with.
*/
