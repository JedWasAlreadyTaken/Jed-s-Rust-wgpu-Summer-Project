// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers: Vec<i16> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}

/*
What the Problem was
The empty vector numbers was made therefore no values can be inferred from it . Also with it being a Vec<i8> it was a wrong type for the from<u8> conversion.

How did : Vec<i16> fix this?
Both u8 and i8 can fit comfortably into the i16 range so it can convert without loss

*/