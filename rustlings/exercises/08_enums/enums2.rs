#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize {width: u64, height:u64},
    Move(Point),
    Echo(String),
    ChangeColor(u64, u64, u64),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

/*
What the problem was
`Message`'s variants had no data attached, but `main` constructs them with data
— `Message::Resize { width: 10, height: 30 }`, `Message::Move(Point { x: 10, y:
15 })`, `Message::Echo(String::from("hello world"))`, and so on — which doesn't
match a plain, dataless variant declaration.

Why is this a problem?
A variant declared as just `Resize` (no payload) can't be constructed as
`Resize { width, height }` — the shape used at construction has to match the
shape declared in the enum.

Why does giving each variant its declared payload fix this?
`Resize { width: u64, height: u64 }` (named fields, like a struct), `Move(Point)`
(wraps another type), `Echo(String)`, `ChangeColor(u64, u64, u64)` (positional,
like a tuple), and `Quit` (no data) each match exactly how `main` constructs
them. This is the real power of Rust enums over enums in most other languages:
each variant can have its own distinct data shape, all still under one type
(`Message`) that can be stored in one array or passed around uniformly. It's
effectively a type-safe tagged union — you can't accidentally read
`width`/`height` out of a `Message::Quit`, because the compiler won't let you
access fields without first matching on which variant you actually have.
*/
