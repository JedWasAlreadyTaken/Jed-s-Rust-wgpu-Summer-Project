fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    let (name, age) = cat;


    println!("{name} is {age} years old");
}

/*
What the problem was
`println!("{name} is {age} years old")` referenced `name` and `age`, but only
`cat` (the whole tuple) was ever bound — there was no line producing `name` or
`age` individually.

Why is this a problem?
`println!`'s `{name}`/`{age}` captures need bindings with exactly those names
in scope. `cat` alone doesn't provide them — you can't interpolate a field out
of a tuple by writing `{cat}` and expect just one element.

Why does `let (name, age) = cat;` fix this?
This destructures the tuple `("Furry McFurson", 3.5)` in one step, binding
`name` to the first element and `age` to the second — matching the tuple's
shape on the left side of `let`. Destructuring like this works anywhere a
pattern is expected (function parameters, `match` arms too), and it's usually
preferred over manual `.0`/`.1` field access (see primitive_types6) because it
gives each value a meaningful name right away instead of a positional index.
*/
