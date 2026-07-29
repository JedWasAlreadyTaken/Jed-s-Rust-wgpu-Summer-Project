fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(8);
}

/*
What the problem was
The call site originally passed an argument that didn't fit `call_me`'s
parameter type — commonly a literal outside `u8`'s 0-255 range, or a value of
the wrong numeric type.

Why is this a problem?
`call_me` takes `num: u8`. Rust integers don't silently wrap or truncate on
assignment the way some languages do — passing a value that can't fit in a
`u8`, or one of a mismatched type, is a compile-time "literal out of range" or
type-mismatch error, not a runtime surprise.

Why does `call_me(8)` fix this?
`8` comfortably fits in `u8`, and since numeric literals don't have a fixed
type until context pins one down, the compiler infers this `8` as `u8` because
that's exactly what `call_me` expects.
*/
