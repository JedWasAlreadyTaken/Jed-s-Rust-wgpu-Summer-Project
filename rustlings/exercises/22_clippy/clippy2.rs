fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}

/*
What was the problem?

The original code used for x in option { ... } to loop over option, an Option<i32>.
Clippy has a lint (for_loops_over_fallibles) that flags iterating over an Option or
Result with a for loop, because it's misleading - it looks like it might loop multiple
times, but an Option can only ever be Some (one iteration) or None (zero iterations), so
using a general-purpose loop construct for something that's really a single conditional
check obscures what the code actually does.

How does if let Some(x) = option fix this?

if let Some(x) = option directly expresses the real intent: "if option happens to hold a
value, bind it to x and run this block once." It makes the at-most-once nature of the
check explicit in the syntax itself, rather than relying on a loop construct that could
misleadingly suggest repeated iteration. Functionally it behaves the same as the original
for loop did (running res += x once if option is Some, doing nothing if it's None), but
it communicates that behaviour directly instead of through a loop that just happens to
never run more than once.
*/
