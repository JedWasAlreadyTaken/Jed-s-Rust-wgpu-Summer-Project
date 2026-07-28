#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        Some( ref p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
/* What the Problem was
Without Ref, the Code would try to move the point out of the Option:

match optional_point {
    Some(p) => println!("Coordinates are {},{}", p.x, p.y),
    _ => panic!("No match!"),
}

Why is this a Problem?
Optional_point already Has a Point, which doesnt impliment copy as it's a custom Struct
when a pattern is matched with Some(p) Point is trying to be moved out of the option to p
and after match, optional_Point is tried to be used again, but it's moved therefore a compilor error

Why does ref fix this?

match optional_point {
    Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
    _ => panic!("No match!"),
}
ref p creates a reference (&Point) to the Point inside the Option
It borrows instead of moving
The Point stays inside optional_point, so using afterward is Ok

*/