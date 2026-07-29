#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,

     
}
fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}

/*
What the problem was
`Message` was an empty enum, but `main` already referenced five variants on it —
`Message::Resize`, `::Move`, `::Echo`, `::ChangeColor`, `::Quit` — none of which
existed yet.

Why is this a problem?
You can't reference an enum variant that isn't defined; each `Message::Xyz` in
`main` needs a matching variant declared inside the `enum Message { ... }`
block, or the compiler has nothing to resolve it to.

Why does adding the five variants fix this?
Declaring `Resize, Move, Echo, ChangeColor, Quit` inside `Message` gives each
of those references somewhere to point. None of them carry data yet — they're
just distinct named cases, each its own distinct value under the one `Message`
type. An enum defines a closed set of possible values: a `Message` can be
exactly one of these five variants, never something else and never more than
one at once. This is the foundation for `match` (used properly starting in
enums3), which the compiler can check is exhaustive precisely because the set
of variants is fixed and known.
*/
