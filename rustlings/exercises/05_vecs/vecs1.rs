fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    
    let v =vec!(10, 20, 30, 40);

    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}

/*
What the problem was
`v` needed to hold the same elements as the array `a = [10, 20, 30, 40]`, using
the vector macro, but only a TODO comment marked where it should go.

Why is this a problem?
`assert_eq!(a, *v)` requires `v` to actually contain those four values as a
`Vec<i32>` — nothing produces that automatically from the array.

Why does `let v = vec![10, 20, 30, 40];` fix this?
`vec![...]` builds a `Vec<i32>` with the same elements as the array. Unlike the
array, a `Vec` owns a heap allocation and can grow or shrink at runtime — its
length isn't part of its type. The test compares `a` (an array) against `*v` (a
dereferenced `Vec`), which works because a `Vec<i32>` derefs to a `[i32]` slice
that can be compared against the array's contents — the array vs. `Vec`
distinction from primitive_types3, now in practice.
*/
