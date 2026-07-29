fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4];
        // the & means i am borrowing(slicing) a refernce to a potion of the array
        //a is reffering back to the array we just made
        // the [1..4] is the range

        // so  &a[1..4] means that we're borring a chunk of the array a from index 1 to 4(not including)
        assert_eq!([2, 3, 4], nice_slice);
    }
}

/*
What the problem was
`nice_slice` didn't exist yet — the test needed a slice containing `[2, 3, 4]`
carved out of the array `a = [1, 2, 3, 4, 5]`, with only a TODO comment in place
of real code.

Why is this a problem?
`assert_eq!([2, 3, 4], nice_slice)` needs `nice_slice` to actually be a slice
holding those three elements, in order — nothing produces that on its own.

Why does `&a[1..4]` fix this?
`1..4` is a half-open range (includes index 1, excludes index 4), matching
elements `2, 3, 4`. The `&` means we're borrowing a view into `a`'s existing
memory rather than copying it into a new array. A slice (`&[T]`) is a fat
pointer — a pointer plus a length — so `nice_slice` doesn't own any data or need
its own allocation; it's tied to `a`'s lifetime. This is the same slicing
syntax and underlying idea used for `&str` (a string slice) later on.
*/
