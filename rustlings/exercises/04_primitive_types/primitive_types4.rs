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
