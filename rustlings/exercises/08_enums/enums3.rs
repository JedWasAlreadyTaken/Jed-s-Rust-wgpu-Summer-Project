struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.
        match message{
            Message::Quit => self.quit(),
            Message::Echo(s) => self.echo(s),
            Message::Move(point) => self.move_position(point),
            Message::Resize{width, height} => self.resize(width,height),
            Message::ChangeColor(red, green, blue) => self.change_color(red, green, blue)
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}

/*
What the problem was
`process` had a TODO where its body should be — `State` already had methods
for every action (`resize`, `move_position`, `echo`, `change_color`, `quit`),
but nothing routed an incoming `Message` to the right one.

Why is this a problem?
The test calls `state.process(Message::Resize { .. })`,
`state.process(Message::Move(..))`, etc., and expects each to update the
matching field on `state` — without dispatch logic, none of that happens.

Why does `match message { ... }` fix this?
Each arm destructures one `Message` variant and calls the matching `State`
method with the extracted data — pulling `s` out of `Echo(s)`, `point` out of
`Move(point)`, `{width, height}` out of `Resize`, and so on. `match` on an enum
must be exhaustive — every variant needs an arm (or a catch-all `_`) — so if a
new `Message` variant were added later, the compiler would immediately flag
every `match` that forgot to handle it. That's a much stronger guarantee than
an `if`/`else if` chain checking a "type" field, or a switch statement with a
forgotten `default`, and it's the main reason enums + `match` are the idiomatic
way to model "one of several distinct shapes" in Rust.
*/
