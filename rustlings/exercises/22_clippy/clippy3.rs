// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Assume that you don't know the value of `my_option`.
    // In the case of `Some`, we want to print its value.
    if let Some(value) = my_option {
        println!("{value}");
    }

    let my_arr = &[
        -1, -2, -3
        ,-4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
  std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}

/*
What was the problem?

There were four separate Clippy lints here:

1. if my_option.is_none() { my_option.unwrap() } checked that the option was None, then
   immediately unwrapped it anyway. Clippy's unnecessary_unwrap / if-let lints flag this
   because unwrapping right after confirming there's no value would always panic - the
   logic was inverted and the whole pattern can be expressed directly.

2. The array literal was missing a comma between rows (-3 and -4 ran together), which
   Rust would actually parse as an expression `-3 - 4, -5, -6` rather than the six
   separate elements intended, silently changing the array's contents.

3. vec![1, 2, 3, 4, 5].resize(0, 5) called resize on a temporary Vec and discarded the
   result, since resize mutates in place and returns (), not a new Vec. my_empty_vec was
   being bound to () instead of an empty vector.

4. value_a = value_b; value_b = value_a; doesn't swap two values - by the time the second
   line runs, value_a has already been overwritten with value_b's value, so value_b just
   gets assigned the same value back and the original value_a is lost.

How do the fixes address this?

1. if let Some(value) = my_option directly expresses "only run this block when there's a
   value," removing the need to check is_none() and then unwrap() at all.

2. Adding the comma (here placed before -4 rather than after -3, either works) restores
   the array to the six intended elements instead of accidentally subtracting.

3. Splitting into `let mut my_empty_vec = vec![...]` followed by `my_empty_vec.clear()`
   keeps the Vec binding intact and empties it in place, which is what resize(0, _) was
   being used for anyway - clear() says that intent directly.

4. std::mem::swap(&mut value_a, &mut value_b) swaps the two values in place at the memory
   level, so neither value is overwritten before the other has been read, avoiding the
   data loss the manual reassignment caused.
*/
