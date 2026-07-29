fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
     let a =["filler"; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}

/*
What the problem was
`a` needed to be an array of at least 100 elements, but the starting point is
just a TODO comment with no array literal — writing out 100 elements by hand
isn't practical.

Why is this a problem?
`a.len() >= 100` has to actually be true, or the `else` branch panics. Manually
typing a 100-element array literal is exactly the kind of tedious, error-prone
task Rust gives you a shortcut for instead.

Why does `let a = ["filler"; 100];` fix this?
`[value; count]` is array-repeat syntax — it makes a 100-element array where
every slot holds `"filler"`, without writing out each one. Arrays in Rust have
a fixed length that's part of their type (`[&str; 100]` here, known at compile
time), which is different from a `Vec`, which can grow/shrink at runtime.
Because the length is fixed and known, the compiler can lay the array out on
the stack and bounds-check indexing without any heap allocation — but it also
means you can't push onto an array or return one of a size decided at runtime.
*/
